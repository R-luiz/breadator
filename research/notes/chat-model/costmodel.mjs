// Projected cost per chatbot answer and answers per USD 1, per candidate.
// Inputs: models-raw.json (list price), zdr-endpoints-raw.json (ZDR endpoint prices), results/*.json (measured reasoning).
// Usage: node costmodel.mjs  -> prints a markdown table.
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
const DIR = path.dirname(fileURLToPath(import.meta.url));
const M = JSON.parse(fs.readFileSync(path.join(DIR, "models-raw.json"), "utf8")).data;
const Z = JSON.parse(fs.readFileSync(path.join(DIR, "zdr-endpoints-raw.json"), "utf8")).data;

// Token budget from the assignment.
const SYS = 1500, TOOLS = 1000, HIST = 1000, Q = 50, PASS = 3000, TC = 80, ENGINE = 300, ANS = 500;
const PREFIX = SYS + TOOLS; // identical for every user, so warm in cache under steady traffic
// Each call: [input tokens, of which a cacheable prefix already sent in an earlier call, output tokens]
const c1 = SYS + TOOLS + HIST + Q, c2 = c1 + TC + PASS, c3 = c2 + TC + ENGINE;
const SCEN = {
  one: [[c1, PREFIX, TC], [c2, c1 + TC, ANS]], // search -> answer
  two: [[c1, PREFIX, TC], [c2, c1 + TC, TC], [c3, c2 + TC, ANS]], // search -> engine tool -> answer
};

const CANDS = [
  "deepseek/deepseek-v4.1-flash-1017", "anthropic/claude-haiku-5.5", "openai/gpt-6-luna", "meta/muse-ember",
  "z-ai/glm-5.3-flash", "inclusionai/ling-3.1-flash", "qwen/qwen3.8-flash", "deepseek/deepseek-v4-flash", "xiaomi/mimo-v2.6-flash",
];

const price = (e, label) => ({ provider: label, pi: +e.pricing.prompt, po: +e.pricing.completion, cr: e.pricing.input_cache_read != null ? +e.pricing.input_cache_read : null, cw: e.pricing.input_cache_write != null ? +e.pricing.input_cache_write : null });

// Cheapest ZDR endpoint with tool support (what `sort: "price"` would pick before Auto Exacto reorders).
function zdrPrice(id) {
  const eps = Z.filter((e) => e.model_id === id && e.supported_parameters?.includes("tools"));
  if (!eps.length) return null;
  const w = (e) => (+e.pricing.prompt) * 14000 + (+e.pricing.completion) * 700;
  eps.sort((a, b) => w(a) - w(b));
  return price(eps[0], `cheapest ZDR: ${eps[0].provider_name} (${eps[0].tag})`);
}

function results(id) {
  const f = path.join(DIR, "results", id.replace(/[/:~]/g, "_") + ".json");
  if (!fs.existsSync(f)) return null;
  const a = JSON.parse(fs.readFileSync(f, "utf8")).answers.filter((x) => x.final);
  return a.length ? a : null;
}

// Tokenizer factor: first call of every answer sends identical text (system + tools + question), so its
// prompt_tokens ratio across models measures tokenizer density. Normalized to the mean of benchmarked models.
const BENCHED = CANDS.filter((id) => results(id));
const firstCall = (id) => { const a = results(id); return a.reduce((s, x) => s + x.calls[0].prompt_tokens, 0) / a.length; };
const REF = BENCHED.reduce((s, id) => s + firstCall(id), 0) / BENCHED.length;

// Route actually served in the benchmark (most frequent provider), priced from the ZDR endpoint list.
function observedRoute(id) {
  const a = results(id);
  if (!a) return null;
  const n = {};
  for (const c of a.flatMap((x) => x.calls)) n[c.provider] = (n[c.provider] || 0) + 1;
  const top = Object.entries(n).sort((x, y) => y[1] - x[1])[0][0];
  const eps = Z.filter((e) => e.model_id === id && e.provider_name === top);
  if (!eps.length) return null;
  eps.sort((x, y) => +x.pricing.prompt - +y.pricing.prompt); // several tags per provider: the cheapest one is assumed
  return price(eps[0], `observed: ${top} (${eps[0].tag})`);
}

function cost(p, scen, cached, reasoning, k) {
  let usd = 0;
  for (const [inp, prefix, out] of scen) {
    const read = cached && p.cr != null ? prefix * k : 0;
    const fresh = inp * k - read;
    const writeRate = cached && p.cw != null ? p.cw : p.pi;
    usd += read * p.cr + fresh * writeRate + out * k * p.po;
  }
  return usd + reasoning * p.po;
}

const rows = [];
for (const id of CANDS) {
  const m = M.find((x) => x.id === id);
  const p = observedRoute(id) ?? zdrPrice(id) ?? price(m, "no ZDR endpoint: list price");
  const a = results(id);
  const r = a ? a.reduce((s, x) => s + x.totals.reasoning_tokens, 0) / a.length : 0;
  const k = a ? firstCall(id) / REF : 1;
  const f = (x) => (x * 1000).toFixed(3);
  const n = (x) => Math.floor(1 / x).toLocaleString("en-US");
  const one = cost(p, SCEN.one, false, r, k), two = cost(p, SCEN.two, false, r, k), oneC = cost(p, SCEN.one, true, r, k), twoC = cost(p, SCEN.two, true, r, k);
  const meas = a ? (a.reduce((s, x) => s + x.totals.cost, 0) / a.length * 1000).toFixed(3) : "-";
  rows.push(`| ${id} | ${p.provider} | ${(p.pi * 1e6).toFixed(3)} / ${(p.po * 1e6).toFixed(3)} | ${p.cr != null ? (p.cr * 1e6).toFixed(4) : "-"} | ${k.toFixed(2)} | ${Math.round(r)} | ${meas} | ${f(one)} / ${f(two)} | ${f(oneC)} / ${f(twoC)} | ${n(two)} | ${n(twoC)} |`);
}
console.log(`Token budget per call (generic tokens, x tokenizer factor): ${JSON.stringify(SCEN)}; reference first-call prompt = ${REF.toFixed(0)} tokens`);
console.log("| Model | Route priced | $/M in / out | $/M cache read | Tokenizer factor | Measured reasoning tok/answer | Measured m$/answer (benchmark, ~4k-token context) | m$ per answer, 1 / 2 tool trips, no cache | m$ per answer, 1 / 2 trips, cached | Answers per $1 (2 trips, no cache) | Answers per $1 (2 trips, cached) |");
console.log("|---|---|---|---|---|---|---|---|---|---|---|");
console.log(rows.join("\n"));
