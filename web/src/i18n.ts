// Translations and number formatting, mirroring crates/breadator-server/src/i18n.rs and
// pages/mod.rs so that a live result reads exactly like the server-rendered one.
import type { EngineError, Warning } from "./engine";

let catalog: Record<string, string> = {};
export const lang = document.documentElement.lang || "en";

/** Fetches the messages of the page's language: an asset the server names on <html>, cached for
 * good and precached by the service worker. Islands start after it; before, `t` knows nothing. */
export async function loadCatalog(): Promise<void> {
  const url = document.documentElement.dataset.i18n;
  if (!url) throw new Error("no message catalog");
  const response = await fetch(url);
  if (!response.ok) throw new Error(`message catalog: HTTP ${response.status}`);
  catalog = (await response.json()) as Record<string, string>;
}

/** Message `key` with its `{ $name }` placeholders filled. */
export function t(key: string, args: Record<string, string> = {}): string {
  const message = catalog[key];
  if (message === undefined) return key;
  return message.replace(/\{ \$(\w+) \}/g, (_, name: string) => args[name] ?? "");
}

const formats = new Map<number, Intl.NumberFormat>();

export function num(value: number, decimals: number): string {
  let format = formats.get(decimals);
  if (!format) {
    format = new Intl.NumberFormat(lang, {
      minimumFractionDigits: decimals,
      maximumFractionDigits: decimals,
      useGrouping: false,
    });
    formats.set(decimals, format);
  }
  const s = format.format(value);
  return /^-0([.,]0*)?$/.test(s) ? s.slice(1) : s;
}

/** Results also show ounces and °F: the server says so on <html> (the device's or account's choice). */
const US = document.documentElement.dataset.units === "us";
const GRAMS_PER_OUNCE = 28.349523125;

/** `0.018 oz`, `3.53 oz`, `14.1 oz`, `2 lb 0.5 oz`, as `Tr::ounces`. */
function ounces(grams: number): string {
  const oz = grams / GRAMS_PER_OUNCE;
  if (oz < 16) return `${num(oz, oz < 0.1 ? 3 : oz < 10 ? 2 : 1)} oz`;
  let pounds = Math.floor(oz / 16);
  let rest = Math.round((oz - 16 * pounds) * 10) / 10;
  if (rest >= 16) {
    pounds += 1;
    rest = 0;
  }
  return rest === 0 ? `${num(pounds, 0)} lb` : `${num(pounds, 0)} lb ${num(rest, 1)} oz`;
}

const grams = (value: number): string => {
  const a = Math.abs(value);
  return num(value, a < 1 ? 2 : a < 20 ? 1 : 0);
};

/** Grams as a baker weighs them (same steps as `Tr::g`); the caller writes the unit after it.
 * With US units the ounces come first: `2 lb 0.5 oz · 920`. */
export function g(value: number): string {
  return US && value >= 0 ? `${ounces(value)} · ${grams(value)}` : grams(value);
}

/** A range of weights (same as `Tr::g_range`): `25 – 42`, or `0.88 oz – 1.48 oz · 25 – 42`. */
export function gRange(low: number, high: number): string {
  const metric = `${grams(low)} – ${grams(high)}`;
  return US && low >= 0 ? `${ounces(low)} – ${ounces(high)} · ${metric}` : metric;
}

/** A temperature in °C (same as `Tr::temp`); the caller writes `°C` after it. */
export function temp(celsius: number): string {
  const metric = num(celsius, 1);
  return US ? `${num((celsius * 9) / 5 + 32, 0)} °F · ${metric}` : metric;
}

export function pct(value: number): string {
  return num(value, Math.abs(value) < 1 ? 2 : 1);
}

export function esc(s: string): string {
  return s.replace(/[&<>"']/g, (c) => `&#${c.charCodeAt(0)};`);
}

export function entryName(id: string): string {
  return catalog[`entry-${id}`] ?? id;
}

/** Same lookup as the server's `field_name` (Fluent ids have no dots). */
function fieldName(field: string): string {
  return catalog[`field-${field.replaceAll(".", "-")}`] ?? field;
}

export function describeError(e: EngineError): string {
  const n = (v: number) => num(v, 1);
  switch (e.code) {
    case "out_of_range":
      return t("err-out-of-range", { field: fieldName(e.field), value: n(e.value), min: n(e.min), max: n(e.max) });
    case "flour_sum":
      return t("err-flour-sum", { sum: n(e.sum) });
    case "no_flour":
      return t("err-no-flour");
    case "duplicate_id":
      return t("err-duplicate", { id: e.id });
    case "unknown_ingredient":
      return t("err-unknown", { id: e.id });
    case "overallocated":
      return t("err-overallocated", {
        ingredient: entryName(e.ingredient),
        needed: g(e.needed_g),
        available: g(e.available_g),
      });
    case "seed_too_wet":
      return t("err-seed-too-wet", { stage: entryName(e.stage) });
    case "invalid":
      return t("err-invalid", { field: fieldName(e.field), reason: e.reason });
  }
}

export function describeWarning(w: Warning): string {
  switch (w.code) {
    case "water_hot_for_yeast":
      return t("warn-water-hot", { t: num(w.water_c, 1) });
    case "ice_needed":
      return t("warn-ice-needed", { t: num(w.water_c, 1) });
    case "target_unreachable":
      return t("warn-unreachable", { t: num(w.best_dough_c, 1) });
    case "scald_evaporation_unknown":
      return t("warn-scald-evaporation", { stage: entryName(w.stage) });
  }
}
