// Starter calculator island. Mirrors crates/breadator-server/src/pages/starter.rs.
import { call, engine, formNumber, live, syncUrl } from "./engine";
import { describeError, esc, g, num, pct, t } from "./i18n";
import { duration } from "./fermentation";

interface Result {
  hours: { low_h: number; mid_h: number; high_h: number };
  flour_parts: number;
  water_parts: number;
  hydration_pct: number;
  iota: number;
  build: [number, number, number] | null;
  notes: ({ code: "extrapolation" } | { code: "no_dose_fits"; too_slow: boolean } | { code: string })[];
}

const FIELDS: [string, string][] = [
  ["flour", "flour_parts"],
  ["water", "water_parts"],
  ["seed_hyd", "seed_hydration_pct"],
  ["hyd", "hydration_pct"],
  ["temp_c", "temp_c"],
  ["target_h", "target_h"],
  ["need_g", "need_g"],
];

function quick(form: HTMLFormElement): Record<string, unknown> {
  const data = new FormData(form);
  const q: Record<string, unknown> = {
    solve: data.get("solve") === "dose" ? "dose" : "time",
    seed_state: data.get("seed_state") === "from_fridge" ? "from_fridge" : "ripe",
  };
  for (const [field, key] of FIELDS) {
    const v = formNumber(data.get(field));
    if (v !== null) q[key] = v;
  }
  return q;
}

/** "1 : 5 : 5" like the server's `ratio`. */
function ratio(flour: number, water: number): string {
  const part = (x: number) => num(x, Math.abs(x - Math.round(x)) < 0.05 ? 0 : 1);
  return `1 : ${part(flour)} : ${part(water)}`;
}

function render(q: Record<string, unknown>, r: Result): string {
  const headline =
    q.solve === "dose"
      ? t("starter-ratio-result", { ratio: ratio(r.flour_parts, r.water_parts) })
      : t("starter-peak-result", { t: duration(r.hours.mid_h) });
  let html = `<p class="big"><strong>${esc(headline)}</strong></p>`;
  html += `<p>${esc(t("starter-range", { low: duration(r.hours.low_h), high: duration(r.hours.high_h) }))}</p>`;
  for (const n of r.notes) {
    const key =
      n.code === "extrapolation"
        ? "ferment-note-extrapolation"
        : "too_slow" in n
          ? n.too_slow
            ? "starter-note-too-fast"
            : "starter-note-too-slow"
          : null;
    if (key) html += `<p class="warning">${esc(t(key))}</p>`;
  }
  const details: [string, string][] = [
    [t("starter-feed"), ratio(r.flour_parts, r.water_parts)],
    [t("starter-hydration"), `${pct(r.hydration_pct)} %`],
    [t("starter-seed-share"), `${pct(r.iota * 100)} %`],
  ];
  html += `<dl class="metrics">${details.map(([k, v]) => `<div><dt>${esc(k)}</dt><dd>${esc(v)}</dd></div>`).join("")}</dl>`;
  if (r.build) {
    const [seed, flour, water] = r.build;
    const row = (label: string, grams: number) => `<tr><th scope="row">${esc(label)}</th><td class="g">${g(grams)} g</td></tr>`;
    html += `<h2>${esc(t("starter-build"))}</h2><table class="lines"><tbody>${row(t("starter-build-seed"), seed)}${row(t("entry-flour"), flour)}${row(t("entry-water"), water)}</tbody></table>`;
  }
  return html;
}

export async function starterIsland(root: HTMLElement): Promise<void> {
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
    const reply = call<Result>(wasm.starter_quick, q);
    const head = "error" in reply ? `<p class="error" role="alert">${esc(describeError(reply.error))}</p>` : render(q, reply.ok);
    results.innerHTML = head + staticPart;
    syncUrl(form);
  };
  live(form, update);
}
