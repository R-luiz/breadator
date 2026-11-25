// The Rust engine compiled to WebAssembly. Loaded once, on the first island that needs it, from
// the hashed URL the server puts on <body data-wasm>.
import init, * as wasm from "../pkg/breadator_wasm.js";

export type EngineError =
  | { code: "out_of_range"; field: string; value: number; min: number; max: number }
  | { code: "flour_sum"; sum: number }
  | { code: "no_flour" }
  | { code: "duplicate_id"; id: string }
  | { code: "unknown_ingredient"; field: string; id: string }
  | { code: "overallocated"; ingredient: string; needed_g: number; available_g: number }
  | { code: "seed_too_wet"; stage: string }
  | { code: "invalid"; field: string; reason: string };

export type Warning =
  | { code: "water_hot_for_yeast"; water_c: number }
  | { code: "ice_needed"; water_c: number }
  | { code: "target_unreachable"; best_dough_c: number }
  | { code: "scald_evaporation_unknown"; stage: string };

export type Reply<T> = { ok: T } | { error: EngineError };

let ready: Promise<typeof wasm> | undefined;

export function engine(): Promise<typeof wasm> {
  if (!ready) {
    const url = document.body.dataset.wasm;
    ready = init({ module_or_path: url }).then(() => wasm);
  }
  return ready;
}

/** Calls a JSON-in/JSON-out engine export. */
export function call<T>(fn: (input: string) => string, input: unknown): Reply<T> {
  return JSON.parse(fn(JSON.stringify(input))) as Reply<T>;
}

/** Same encoding as the server's `?s=` (base64url of the JSON, no padding). */
export function encodeState(value: unknown): string {
  const bytes = new TextEncoder().encode(JSON.stringify(value));
  let binary = "";
  for (const b of bytes) binary += String.fromCharCode(b);
  return btoa(binary).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");
}

/** Parses a form number like the server does: empty → null, comma or dot decimals. */
export function formNumber(value: FormDataEntryValue | null): number | null {
  if (typeof value !== "string" || value.trim() === "") return null;
  const n = Number(value.trim().replace(",", "."));
  return Number.isFinite(n) ? n : null;
}

/** Recomputes on every edit (debounced) and on submit, without reloading the page. */
export function live(form: HTMLFormElement, update: () => void): void {
  form.querySelector('button[type="submit"]')?.setAttribute("hidden", "");
  let timer: number | undefined;
  form.addEventListener("input", () => {
    window.clearTimeout(timer);
    timer = window.setTimeout(update, 80);
  });
  form.addEventListener("submit", (event) => {
    event.preventDefault();
    update();
  });
}

/** Keeps the address bar in sync with the form, so reloading or sharing shows the same result. */
export function syncUrl(form: HTMLFormElement): void {
  const params = new URLSearchParams();
  for (const [key, value] of new FormData(form)) {
    if (typeof value === "string") params.set(key, value);
  }
  history.replaceState(null, "", `${location.pathname}?${params}`);
}
