// Formula calculator island: recomputes the simple form with the WASM engine as the user types.
// Formulas opened from `?s=` (full editor, recipes) stay server-rendered until the editor exists.
import { call, encodeState, engine, formNumber, live, syncUrl } from "./engine";
import { describeError, describeWarning, entryName, esc, g, num, pct, t } from "./i18n";
import { formulaFromPage, mountSaveButton } from "./save";

interface Line {
  id: string;
  grams: number;
  pct: number;
}

interface StageSolution {
  id: string;
  flour_g: number;
  water_g: number;
  mass_g: number;
  hydration_pct: number;
  lines: Line[];
  evaporation_water_g: number;
  seed: { seed_g: number; fresh_flour_g: number; fresh_water_g: number } | null;
}

interface Solution {
  totals: Line[];
  stages: StageSolution[];
  final_mix: Line[];
  warnings: Parameters<typeof describeWarning>[0][];
  metrics: {
    total_flour_g: number;
    dough_g: number;
    hydration_water_pct: number;
    hydration_total_pct: number;
    dough_yield: number;
    prefermented_flour_pct: number;
    scalded_flour_pct: number;
    salt_pct: number;
    sugar_pct: number;
    fat_pct: number;
    levains: { inoculation_final_mix_pct: number | null; inoculation_total_flour_pct: number }[];
  };
}

const rows = (lines: Line[]) =>
  `<table class="lines"><tbody>${lines
    .map(
      (l) =>
        `<tr><th scope="row">${esc(entryName(l.id))}</th><td class="g">${g(l.grams)} g</td><td class="pct">${pct(l.pct)} %</td></tr>`,
    )
    .join("")}</tbody></table>`;

function render(s: Solution): string {
  let html = s.warnings.map((w) => `<p class="warning">${esc(describeWarning(w))}</p>`).join("");
  for (const st of s.stages) {
    html += `<h2>${esc(entryName(st.id))} <small>${g(st.mass_g)} g · ${esc(t("hydration"))} ${pct(st.hydration_pct)} %</small></h2>`;
    html += rows(st.lines);
    if (st.seed) {
      const msg = t("formula-seed-build", {
        seed: g(st.seed.seed_g),
        flour: g(st.seed.fresh_flour_g),
        water: g(st.seed.fresh_water_g),
      });
      html += `<p class="note">${esc(msg)}</p>`;
    }
    if (st.evaporation_water_g > 0) {
      const msg = t("formula-evaporation", { extra: g(st.evaporation_water_g), mass: g(st.mass_g) });
      html += `<p class="note">${esc(msg)}</p>`;
    }
  }
  html += `<h2>${esc(t("formula-final-mix"))}</h2>${rows(s.final_mix)}`;
  html += `<h2>${esc(t("formula-totals"))}</h2>${rows(s.totals)}`;
  const m = s.metrics;
  const metrics: [string, string][] = [
    ["metric-total-flour", `${g(m.total_flour_g)} g`],
    ["metric-dough", `${g(m.dough_g)} g`],
    ["metric-hydration-water", `${pct(m.hydration_water_pct)} %`],
    ["metric-hydration-total", `${pct(m.hydration_total_pct)} %`],
    ["metric-salt", `${pct(m.salt_pct)} %`],
  ];
  if (m.prefermented_flour_pct > 0) metrics.push(["metric-prefermented", `${pct(m.prefermented_flour_pct)} %`]);
  for (const lv of m.levains) {
    if (lv.inoculation_final_mix_pct !== null) {
      metrics.push(["metric-inoculation-final", `${pct(lv.inoculation_final_mix_pct)} %`]);
    }
    metrics.push(["metric-inoculation-total", `${pct(lv.inoculation_total_flour_pct)} %`]);
  }
  if (m.scalded_flour_pct > 0) metrics.push(["metric-scalded", `${pct(m.scalded_flour_pct)} %`]);
  if (m.sugar_pct > 0) metrics.push(["metric-sugar", `${pct(m.sugar_pct)} %`]);
  if (m.fat_pct > 0) metrics.push(["metric-fat", `${pct(m.fat_pct)} %`]);
  metrics.push(["metric-dough-yield", num(m.dough_yield, 0)]);
  html += `<dl class="metrics">${metrics
    .map(([key, value]) => `<div><dt>${esc(t(key))}</dt><dd>${value}</dd></div>`)
    .join("")}</dl>`;
  return html;
}

/** Form fields → engine `QuickForm` (same defaults as the server's `quick_form`). */
function quickForm(form: HTMLFormElement): Record<string, unknown> {
  const data = new FormData(form);
  const n = (name: string) => formNumber(data.get(name));
  const text = (name: string) => String(data.get(name) ?? "").trim();
  const code = (name: string) => {
    const v = text(name);
    return v === "" || v === "none" ? null : v;
  };
  const quick: Record<string, unknown> = {
    by: text("by") || "dough",
    flour1: text("flour1") || t("default-flour1"),
    flour2: text("flour2"),
    yeast_kind: code("yeast_kind"),
    scald_method: code("scald_method"),
  };
  for (const name of ["grams", "count", "loss", "flour2_pct", "water", "salt", "yeast", "levain", "levain_hyd", "poolish", "poolish_yeast", "scald"]) {
    const v = n(name);
    if (v !== null) quick[name] = v;
  }
  return quick;
}

export async function formulaIsland(root: HTMLElement): Promise<void> {
  const form = root.querySelector<HTMLFormElement>("form.inputs");
  const results = root.querySelector<HTMLElement>(".results");
  if (!form || !results) return;
  // The formula on screen: the one the server rendered, then whatever the island computes.
  let current: unknown | null = formulaFromPage();
  mountSaveButton(root, () => current);
  // A formula opened from ?s= may not fit the simple form: keep the server's rendering.
  if (new URLSearchParams(location.search).has("s")) return;
  const wasm = await engine();
  const update = () => {
    const reply = call<{ formula: unknown; solution: Solution }>(wasm.solve_quick, quickForm(form));
    let html: string;
    if ("error" in reply) {
      current = null;
      html = `<p class="error" role="alert">${esc(describeError(reply.error))}</p>`;
    } else {
      current = reply.ok.formula;
      const share = `${location.origin}${location.pathname}?s=${encodeState(reply.ok.formula)}`;
      html = `${render(reply.ok.solution)}<p class="share"><a href="${esc(share)}" rel="nofollow">${esc(t("share-link"))}</a></p>`;
    }
    results.innerHTML = html;
    syncUrl(form);
  };
  live(form, update);
}
