// Fermentation calculator island: recomputes with the WASM engine as the user types.
// Mirrors the server's rendering (crates/breadator-server/src/pages/fermentation.rs).
import { call, engine, formNumber, live, syncUrl } from "./engine";
import { describeError, esc, num, pct, t } from "./i18n";

interface Modifier {
  code: string;
  factor: number;
  low: number;
  high: number;
  evidence: "measured" | "literature" | "creator_rule" | "extrapolation";
}

type Note =
  | { code: "extrapolation" }
  | { code: "use_osmotolerant"; sugar_pct: number }
  | { code: "very_short" }
  | { code: "endpoint_before_fridge" }
  | { code: "no_dose_fits"; too_slow: boolean }
  | { code: "rich_dough_cold" };

interface Result {
  hours: { low_h: number; mid_h: number; high_h: number };
  endpoint_phase: "before_fridge" | "fridge" | "room";
  dose_pct: number;
  schedule: { phases: { progress: number; end_temp_c: number }[] };
  modifiers: Modifier[];
  notes: Note[];
}

/** Same rounding as the server's `duration`: 5-minute steps. Shared with the starter island. */
export function duration(hours: number): string {
  const minutes = Math.round((hours * 60) / 5) * 5;
  if (minutes < 60) return t("duration-min", { m: String(Math.max(minutes, 5)) });
  return t("duration-hm", { h: String(Math.floor(minutes / 60)), m: String(minutes % 60).padStart(2, "0") });
}

const NUMBER_FIELDS: [string, string][] = [
  ["dose", "dose_pct"],
  ["starter_hyd", "starter_hydration_pct"],
  ["target_h", "target_h"],
  ["dough_c", "dough_c"],
  ["room_c", "room_c"],
  ["dough_g", "dough_g"],
  ["warm_h", "warm_h"],
  ["fridge_h", "fridge_h"],
  ["fridge_c", "fridge_c"],
];
const DOUGH_FIELDS: [string, string][] = [
  ["salt", "salt_pct"],
  ["sugar", "sugar_pct"],
  ["fat", "fat_pct"],
  ["hydration", "hydration_pct"],
  ["whole_grain", "whole_grain_pct"],
  ["spice", "spice_pct"],
  ["cocoa", "cocoa_pct"],
  ["scalded", "scalded_pct"],
];

/** Form → engine `QuickFerment` (missing numbers take the engine defaults, like the server). */
function quick(form: HTMLFormElement): Record<string, unknown> {
  const data = new FormData(form);
  const q: Record<string, unknown> = {};
  for (const name of ["solve", "leavening", "starter_state", "endpoint", "container"]) {
    const v = data.get(name);
    if (typeof v === "string" && v) q[name] = v;
  }
  for (const [field, key] of NUMBER_FIELDS) {
    const v = formNumber(data.get(field));
    if (v !== null) q[key] = v;
  }
  const dough: Record<string, unknown> = { milk: data.has("milk"), eggs: data.has("eggs") };
  for (const [field, key] of DOUGH_FIELDS) {
    const v = formNumber(data.get(field));
    if (v !== null) dough[key] = v;
  }
  q.dough = dough;
  return q;
}

function noteText(n: Note): string {
  switch (n.code) {
    case "extrapolation":
      return t("ferment-note-extrapolation");
    case "use_osmotolerant":
      return t("ferment-note-osmotolerant", { sugar: pct(n.sugar_pct) });
    case "very_short":
      return t("ferment-note-very-short");
    case "endpoint_before_fridge":
      return t("ferment-note-before-fridge");
    case "no_dose_fits":
      return t(n.too_slow ? "ferment-note-no-dose-slow" : "ferment-note-no-dose-fast");
    case "rich_dough_cold":
      return t("ferment-note-rich-cold");
  }
}

const EVIDENCE: Record<Modifier["evidence"], string> = {
  measured: "evidence-measured",
  literature: "evidence-literature",
  creator_rule: "evidence-creator-rule",
  extrapolation: "evidence-extrapolation",
};

function render(q: Record<string, unknown>, r: Result): string {
  const sourdough = (q.leavening ?? "sourdough") === "sourdough";
  const warm = Number(q.warm_h ?? 0);
  const fridge = Number(q.fridge_h ?? 0);
  const headline =
    q.solve === "dose"
      ? t(sourdough ? "ferment-dose-starter" : "ferment-dose-yeast", { pct: pct(r.dose_pct) })
      : t("ferment-time-result", { t: duration(r.hours.mid_h) });
  let html = `<p class="big"><strong>${esc(headline)}</strong></p>`;
  html += `<p>${esc(t("ferment-range", { low: duration(r.hours.low_h), high: duration(r.hours.high_h) }))}</p>`;
  if (fridge > 0) {
    const where =
      r.endpoint_phase === "before_fridge"
        ? t("ferment-where-before")
        : r.endpoint_phase === "fridge"
          ? t("ferment-where-fridge", { t: duration(r.hours.mid_h - warm) })
          : t("ferment-where-after", { t: duration(r.hours.mid_h - warm - fridge) });
    html += `<p>${esc(where)}</p>`;
  }
  html += r.notes.map((n) => `<p class="warning">${esc(noteText(n))}</p>`).join("");
  if (fridge > 0) {
    const labels: [string, number][] = [];
    if (warm > 0) labels.push(["ferment-phase-warm", warm]);
    labels.push(["ferment-phase-fridge", fridge]);
    const rows = labels
      .map(([key, hours], i) => {
        const p = r.schedule.phases[i];
        if (!p) return "";
        const detail = t("ferment-phase-detail", { p: num(Math.min(p.progress, 9.99) * 100, 0), t: num(p.end_temp_c, 1) });
        return `<tr><th scope="row">${esc(t(key))}</th><td class="g">${duration(hours)}</td><td>${esc(detail)}</td></tr>`;
      })
      .join("");
    html += `<h2>${esc(t("ferment-schedule"))}</h2><table class="lines"><tbody>${rows}</tbody></table>`;
  }
  const mods = r.modifiers
    .map(
      (m) =>
        `<tr><th scope="row">${esc(t(`modifier-${m.code}`))}</th><td class="g">×${num(m.factor, 2)}</td><td>${num(m.low, 2)} – ${num(m.high, 2)} · ${esc(t(EVIDENCE[m.evidence]))}</td></tr>`,
    )
    .join("");
  html += `<h2>${esc(t("ferment-factors"))}</h2><table class="lines"><tbody>${mods}</tbody></table>`;
  return html;
}

export async function fermentationIsland(root: HTMLElement): Promise<void> {
  const form = root.querySelector<HTMLFormElement>("form.inputs");
  const results = root.querySelector<HTMLElement>(".results");
  if (!form || !results) return;
  // The explanation and sources below the results stay as rendered by the server.
  const firstStatic = results.querySelector("h2:nth-last-of-type(1)");
  const staticPart = firstStatic ? collectFrom(firstStatic) : "";
  const wasm = await engine();
  const update = () => {
    const q = quick(form);
    const reply = call<Result>(wasm.ferment_quick, q);
    const head =
      "error" in reply ? `<p class="error" role="alert">${esc(describeError(reply.error))}</p>` : render(q, reply.ok);
    results.innerHTML = head + staticPart;
    syncUrl(form);
  };
  live(form, update);
}

/** HTML of `start` and every following sibling. */
function collectFrom(start: Element): string {
  let html = "";
  for (let el: Element | null = start; el; el = el.nextElementSibling) html += el.outerHTML;
  return html;
}
