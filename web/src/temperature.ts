// Water temperature island.
import { call, engine, formNumber, live, syncUrl } from "./engine";
import type { Warning } from "./engine";
import { describeError, describeWarning, esc, g, t, temp } from "./i18n";

interface Result {
  water_c: number;
  ice_g: number | null;
  tap_water_g: number | null;
  expected_dough_c: number;
  warnings: Warning[];
}

/** Form fields → engine `QuickTemperature` (missing numbers take the engine defaults). */
function quick(form: HTMLFormElement): Record<string, unknown> {
  const data = new FormData(form);
  const q: Record<string, unknown> = { method: data.get("method") === "classic" ? "classic" : "balance" };
  for (const name of ["desired", "flour_c", "room_c", "flour_g", "water_g", "pref_g", "pref_c", "pref_hyd", "rise", "friction", "tap_c"]) {
    const v = formNumber(data.get(name));
    if (v !== null) q[name] = v;
  }
  return q;
}

export async function temperatureIsland(root: HTMLElement): Promise<void> {
  const form = root.querySelector<HTMLFormElement>("form.inputs");
  const results = root.querySelector<HTMLElement>(".results");
  const explain = results?.querySelector(".note")?.outerHTML ?? "";
  if (!form || !results) return;
  const wasm = await engine();
  const update = () => {
    const input = quick(form);
    const reply = call<Result>(wasm.water_temperature_quick, input);
    let html: string;
    if ("error" in reply) {
      html = `<p class="error" role="alert">${esc(describeError(reply.error))}</p>`;
    } else {
      const r = reply.ok;
      html = `<p class="big">${esc(t("temperature-water-result"))} <strong>${temp(r.water_c)} °C</strong></p>`;
      if (r.ice_g !== null && r.tap_water_g !== null && typeof input.tap_c === "number") {
        const msg = t("temperature-ice-result", { tap: temp(input.tap_c), ice: g(r.ice_g), water: g(r.tap_water_g) });
        html += `<p class="big">${esc(msg)}</p>`;
      }
      html += `<p>${esc(t("temperature-expected"))} ${temp(r.expected_dough_c)} °C</p>`;
      html += r.warnings.map((w) => `<p class="warning">${esc(describeWarning(w))}</p>`).join("");
    }
    results.innerHTML = html + explain;
    syncUrl(form);
  };
  live(form, update);
}
