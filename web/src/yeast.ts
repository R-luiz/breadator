// Yeast converter island.
import { call, engine, formNumber, live, syncUrl } from "./engine";
import { describeError, entryName, esc, g, gRange } from "./i18n";

const KINDS = ["fresh_yeast", "active_dry_yeast", "instant_yeast", "osmotolerant_yeast"];

export async function yeastIsland(root: HTMLElement): Promise<void> {
  const form = root.querySelector<HTMLFormElement>("form.inputs");
  const body = root.querySelector<HTMLElement>(".results tbody");
  if (!form || !body) return;
  const wasm = await engine();
  const update = () => {
    const data = new FormData(form);
    const grams = formNumber(data.get("grams")) ?? 10;
    const from = String(data.get("from") ?? KINDS[0]);
    let html = "";
    for (const to of KINDS) {
      if (to === from) continue;
      const reply = call<{ grams: number; low_g: number; high_g: number }>(wasm.convert_yeast, { grams, from, to });
      if ("error" in reply) {
        html = `<tr><td colspan="3" class="error" role="alert">${esc(describeError(reply.error))}</td></tr>`;
        break;
      }
      const c = reply.ok;
      html += `<tr><th scope="row">${esc(entryName(to))}</th><td class="g">${g(c.grams)} g</td><td>${gRange(c.low_g, c.high_g)} g</td></tr>`;
    }
    body.innerHTML = html;
    syncUrl(form);
  };
  live(form, update);
}
