// Flour calculator island. Mirrors crates/breadator-server/src/pages/flour.rs.
import { call, engine, formNumber, live, syncUrl } from "./engine";
import { describeError, esc, g, gRange, num, t } from "./i18n";

interface Band {
  low: number;
  mid: number;
  high: number;
}

type Note =
  | { code: "rye_needs_acidity" | "additions_beyond_range" | "psyllium_beyond_range" | "scald_beyond_range" | "target_below_blend" }
  | { code: "protein_far_from_type" | "no_hydration_data" | "no_absorption_data"; id: string };

interface Result {
  protein_pct: number;
  protein_with_gluten_pct: number | null;
  fibre_pct: number;
  absorption_pct: number | null;
  hydration: Band | null;
  water_g: Band | null;
  from_reference: boolean;
  flours_points: number;
  additions_points: number;
  scald_points: number;
  additions: { kind: string; grams: number; water_g: number }[];
  gluten: { pct: number; grams: number; extra_water_g: number } | null;
  rye_pct: number;
  network_pct: number;
  retard_h: number | null;
  notes: Note[];
}

/** Form field → engine kind, as `ADDITIONS` in pages/flour.rs. */
const ADDITIONS: [string, string][] = [
  ["gluten", "vital_wheat_gluten"],
  ["bran", "wheat_bran"],
  ["oat_bran", "oat_bran"],
  ["psyllium", "psyllium_husk"],
  ["chia", "chia_ground"],
  ["flax", "flaxseed_whole_soaked"],
  ["seeds", "seeds_sunflower_sesame_pumpkin"],
  ["grains", "cracked_grain_oat_flakes_buckwheat_groats_quinoa"],
];
const ROWS = 3;

interface Quick {
  flours: { id: string; pct: number; protein_pct: number | null }[];
  total_flour_g?: number;
  reference?: { flours: { id: string; pct: number; protein_pct: null }[]; hydration_pct: number };
  additions: { kind: string; pct: number }[];
  scald_pct: number;
  scald: string;
  target_protein_pct: number | null;
  gluten_protein_pct?: number;
}

function quick(form: HTMLFormElement): Quick {
  const data = new FormData(form);
  const q: Quick = {
    flours: [],
    additions: [],
    scald_pct: formNumber(data.get("scald_pct")) ?? 0,
    scald: String(data.get("scald") ?? "yudane"),
    target_protein_pct: formNumber(data.get("target")),
  };
  for (let i = 1; i <= ROWS; i++) {
    const id = String(data.get(`f${i}`) ?? "");
    if (id) q.flours.push({ id, pct: formNumber(data.get(`p${i}`)) ?? 0, protein_pct: formNumber(data.get(`prot${i}`)) });
  }
  for (const [field, kind] of ADDITIONS) {
    const pct = formNumber(data.get(field));
    if (pct) q.additions.push({ kind, pct });
  }
  const flour = formNumber(data.get("flour_g"));
  if (flour !== null) q.total_flour_g = flour;
  const gluten = formNumber(data.get("gluten_protein"));
  if (gluten !== null) q.gluten_protein_pct = gluten;
  // Both the flour and the hydration of the working recipe are needed to start from it.
  const reference = String(data.get("ref") ?? "");
  const hydration = formNumber(data.get("ref_h"));
  if (reference && hydration !== null) {
    q.reference = { flours: [{ id: reference, pct: 100, protein_pct: null }], hydration_pct: hydration };
  }
  return q;
}

const pct1 = (v: number) => `${num(v, 1)} %`;
const range = (b: Band) => `${num(b.low, 0)}–${num(b.high, 0)}`;

function note(n: Note): string {
  switch (n.code) {
    case "rye_needs_acidity":
      return t("flour-note-rye");
    case "additions_beyond_range":
      return t("flour-note-additions");
    case "psyllium_beyond_range":
      return t("flour-note-psyllium");
    case "scald_beyond_range":
      return t("flour-note-scald");
    case "target_below_blend":
      return t("flour-note-target");
    case "protein_far_from_type":
      return t("flour-note-protein", { flour: t(`flour-${n.id}`) });
    case "no_hydration_data":
      return t("flour-note-no-data", { flour: t(`flour-${n.id}`) });
    case "no_absorption_data":
      return t("flour-note-no-absorption", { flour: t(`flour-${n.id}`) });
  }
}

function render(q: Quick, r: Result): string {
  const headline = r.hydration
    ? t("flour-hydration-result", { mid: num(r.hydration.mid, 0), range: range(r.hydration) })
    : t("flour-hydration-unknown");
  let html = `<p class="big"><strong>${esc(headline)}</strong></p>`;
  if (r.water_g) {
    html += `<p>${esc(t("flour-water-result", { mid: g(r.water_g.mid), range: gRange(r.water_g.low, r.water_g.high), flour: g(q.total_flour_g ?? 1000) }))}</p>`;
  }
  for (const n of r.notes) html += `<p class="warning">${esc(note(n))}</p>`;
  if (r.hydration && (r.additions_points > 0 || r.scald_points > 0 || r.from_reference)) {
    const row = (label: string, value: string) => `<tr><th scope="row">${esc(label)}</th><td class="g">${esc(value)}</td></tr>`;
    const base = r.from_reference ? "flour-part-reference" : "flour-part-flours";
    html += `<table class="lines"><tbody>${row(t(base), pct1(r.flours_points))}`;
    if (r.additions_points > 0) html += row(t("flour-part-additions"), `+ ${pct1(r.additions_points)}`);
    if (r.scald_points > 0) html += row(t("flour-part-scald"), `+ ${pct1(r.scald_points)}`);
    html += "</tbody></table>";
  }
  const metrics: [string, string][] = [[t("flour-protein"), pct1(r.protein_pct)]];
  if (r.protein_with_gluten_pct !== null) metrics.push([t("flour-protein-with-gluten"), pct1(r.protein_with_gluten_pct)]);
  metrics.push([t("flour-fibre"), pct1(r.fibre_pct)]);
  if (r.absorption_pct !== null) metrics.push([t("flour-absorption"), pct1(r.absorption_pct)]);
  if (r.rye_pct > 0) metrics.push([t("flour-rye-share"), pct1(r.rye_pct)]);
  if (r.network_pct < 100) metrics.push([t("flour-no-network"), pct1(100 - r.network_pct)]);
  if (r.retard_h !== null) metrics.push([t("flour-retard"), t("flour-retard-hours", { h: num(r.retard_h, 0) })]);
  html += `<dl class="metrics">${metrics.map(([k, v]) => `<div><dt>${esc(k)}</dt><dd>${esc(v)}</dd></div>`).join("")}</dl>`;
  if (r.additions.length > 0) {
    const name = (kind: string) => {
      const field = ADDITIONS.find(([, k]) => k === kind)?.[0];
      return field ? t(`flour-add-${field}`) : kind;
    };
    html += `<h2>${esc(t("flour-additions-water"))}</h2><table class="lines"><thead><tr><th scope="col">${esc(t("flour-addition"))}</th><th scope="col">${esc(t("flour-weight"))}</th><th scope="col">${esc(t("flour-its-water"))}</th></tr></thead><tbody>`;
    for (const l of r.additions) {
      html += `<tr><th scope="row">${esc(name(l.kind))}</th><td class="g">${g(l.grams)} g</td><td class="g">${g(l.water_g)} g</td></tr>`;
    }
    html += "</tbody></table>";
  }
  if (r.gluten) {
    html += `<h2>${esc(t("flour-target-legend"))}</h2><p>${esc(t("flour-gluten-result", { grams: g(r.gluten.grams), pct: num(r.gluten.pct, 1), water: g(r.gluten.extra_water_g) }))}</p>`;
  }
  return html;
}

export async function flourIsland(root: HTMLElement): Promise<void> {
  const form = root.querySelector<HTMLFormElement>("form.inputs");
  const results = root.querySelector<HTMLElement>(".results");
  if (!form || !results) return;
  // The explanation under the results stays as the server rendered it.
  const how = [...results.querySelectorAll("h2")].at(-1);
  let staticPart = "";
  for (let el: Element | null = how ?? null; el; el = el.nextElementSibling) staticPart += el.outerHTML;
  const wasm = await engine();
  const update = () => {
    const q = quick(form);
    const reply = call<Result>(wasm.flour_quick, q);
    const head = "error" in reply ? `<p class="error" role="alert">${esc(describeError(reply.error))}</p>` : render(q, reply.ok);
    results.innerHTML = head + staticPart;
    syncUrl(form);
  };
  live(form, update);
}
