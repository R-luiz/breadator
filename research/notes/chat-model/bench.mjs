// Breadator chat-model benchmark. Node >= 20, no dependencies.
// Usage: node bench.mjs <model-id>[@<provider-json>] ...   e.g.
//   node bench.mjs deepseek/deepseek-v4.1-flash-1017 'google/gemma-4-31b-it:free@{}'
// Default provider preferences: {"zdr":true,"sort":"price"} (what production would send; plain zdr:true
// let default routing pick a 10x-priced endpoint for DeepSeek in the smoke run).
// Reads the OpenRouter key from ~/.omp/agent/models.yml at runtime; never prints or writes it.
// Every call is appended to spend.jsonl; the run stops before a call once total spend >= BUDGET_STOP.
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";

const DIR = path.dirname(fileURLToPath(import.meta.url));
const BUDGET_STOP = 0.45;
const MAX_CALLS_PER_ANSWER = 5;
const MAX_TOKENS = 2500;

const KEY = (fs.readFileSync(path.join(os.homedir(), ".omp/agent/models.yml"), "utf8").match(/sk-or-v1-[A-Za-z0-9]+/) || [])[0];
if (!KEY) throw new Error("no OpenRouter key found");

const P = JSON.parse(fs.readFileSync(path.join(DIR, "prompts.json"), "utf8"));
const LEDGER = path.join(DIR, "spend.jsonl");
let spent = fs.existsSync(LEDGER)
  ? fs.readFileSync(LEDGER, "utf8").split("\n").filter(Boolean).reduce((s, l) => s + (JSON.parse(l).cost || 0), 0)
  : 0;

// ---------- mock engine ----------
const r1 = (x) => Math.round(x * 10) / 10;
function computeFormula(a) {
  const total = a.pieces * a.piece_weight_g;
  const h = a.hydration_pct / 100, s = a.salt_pct / 100, y = (a.instant_yeast_pct ?? 0) / 100;
  const lp = (a.levain_pct ?? 0) / 100, lh = (a.levain_hydration_pct ?? 100) / 100;
  const flour = total / (1 + h + s + y);
  const levain = lp * flour, levainFlour = levain / (1 + lh), levainWater = levain - levainFlour;
  return {
    total_dough_g: r1(total), total_flour_g: r1(flour), total_water_g: r1(h * flour),
    weigh_out: { flour_g: r1(flour - levainFlour), water_g: r1(h * flour - levainWater), levain_g: r1(levain), salt_g: r1(s * flour), instant_yeast_g: r1(y * flour) },
    levain_composition: { flour_g: r1(levainFlour), water_g: r1(levainWater), hydration_pct: lh * 100 },
  };
}
function predictBulk(a) {
  if (!(a.levain_pct > 0)) return { error: "levain_pct must be > 0" };
  const ff = { white: 1, whole_wheat: 0.85, rye: 0.75 }[a.flour ?? "white"] ?? 1;
  const h = 5.0 * Math.pow(20 / a.levain_pct, 0.45) * Math.pow(2, (24 - a.dough_temp_c) / 8) * ff * Math.pow((a.target_rise_pct ?? 50) / 50, 0.6);
  return { bulk_hours: r1(h), range_hours: [r1(h * 0.85), r1(h * 1.15)], dough_temp_c: a.dough_temp_c, levain_pct: a.levain_pct, flour: a.flour ?? "white", target_rise_pct: a.target_rise_pct ?? 50, note: "Watch the dough: end bulk at the target rise, not at the clock." };
}
function runTool(q, name, args, returned) {
  if (name === "search_knowledge") {
    q.passages.forEach((k) => returned.add(k));
    return { results: q.passages.map((k) => ({ key: k, source: P.passages[k].source, text: P.passages[k].text })) };
  }
  if (name === "compute_formula") return computeFormula(args);
  if (name === "predict_bulk_time") return predictBulk(args);
  return { error: `unknown tool ${name}` };
}

// ---------- API ----------
async function chat(body) {
  for (let attempt = 0; attempt < 3; attempt++) {
    const t0 = Date.now();
    const res = await fetch("https://openrouter.ai/api/v1/chat/completions", {
      method: "POST",
      headers: { Authorization: `Bearer ${KEY}`, "Content-Type": "application/json", "HTTP-Referer": "https://breadator.robinluiz.fr", "X-Title": "Breadator model benchmark" },
      body: JSON.stringify(body),
    });
    const text = await res.text();
    let j; try { j = JSON.parse(text); } catch { j = { error: { message: text.slice(0, 300) } }; }
    if (res.ok && !j.error) return { j, ms: Date.now() - t0 };
    if (attempt === 2 || ![429, 500, 502, 503, 504].includes(res.status)) return { j, ms: Date.now() - t0, status: res.status };
    await new Promise((r) => setTimeout(r, 4000 * (attempt + 1)));
  }
}

// ---------- checks ----------
const LABEL = /\[(non sourcé|non source|not sourced)\]/gi;
function autoChecks(q, toolCalls, final, returned) {
  const cited = [...new Set([...(final.matchAll(/\[([a-z0-9][a-z0-9-]*[a-z0-9])\]/g))].map((m) => m[1]))];
  const fr = (final.match(/\b(le|la|les|des|est|une|pour|que|dans|pas|avec)\b/gi) || []).length;
  const en = (final.match(/\b(the|is|and|of|to|with|for|that|your|are)\b/gi) || []).length;
  const out = {
    searched: toolCalls.some((t) => t.name === "search_knowledge"),
    n_search: toolCalls.filter((t) => t.name === "search_knowledge").length,
    engine_calls: toolCalls.filter((t) => t.name !== "search_knowledge").map((t) => ({ name: t.name, args: t.args })),
    cited,
    invented_keys: cited.filter((k) => !returned.has(k)),
    irrelevant_cited: cited.filter((k) => returned.has(k) && q.relevant.length && !q.relevant.includes(k)),
    unsourced_labels: (final.match(LABEL) || []).length,
    words: final.split(/\s+/).filter(Boolean).length,
    lang_guess: fr > en ? "fr" : "en",
  };
  const e = q.expect;
  const close = (a, b) => typeof b === "number" ? Math.abs((a ?? NaN) - b) <= 0.11 : a === b;
  const argsMatch = (got, want) => Object.entries(want).every(([k, v]) => close(got?.[k], v));
  if (e.args) out.args_ok = out.engine_calls.some((c) => c.name === e.tool && argsMatch(c.args, e.args));
  if (e.calls) out.args_ok = e.calls.every((w) => out.engine_calls.some((c) => c.name === e.tool && argsMatch(c.args, w)));
  return out;
}

// ---------- one answer ----------
async function answer(model, provider, q) {
  const messages = [{ role: "system", content: P.system }, { role: "user", content: q.text }];
  const calls = [], toolCalls = [], returned = new Set();
  let final = "", error = null;
  for (let i = 0; i < MAX_CALLS_PER_ANSWER; i++) {
    if (spent >= BUDGET_STOP) { error = `budget stop at ${spent.toFixed(4)}`; break; }
    const body = { model, messages, tools: P.tools, tool_choice: "auto", max_tokens: MAX_TOKENS, reasoning: { effort: "low" }, provider, usage: { include: true }, session_id: `bench-${model}-${q.id}` };
    if (model.startsWith("anthropic/")) body.cache_control = { type: "ephemeral" }; // Anthropic needs opt-in caching; others cache implicitly
    const { j, ms, status } = await chat(body);
    if (j.error) { error = `${status} ${j.error.message} ${JSON.stringify(j.error.metadata || {}).slice(0, 300)}`; break; }
    const u = j.usage || {};
    const rec = { q: q.id, i, model, provider: j.provider, prompt_tokens: u.prompt_tokens, completion_tokens: u.completion_tokens, reasoning_tokens: u.completion_tokens_details?.reasoning_tokens ?? 0, cached_tokens: u.prompt_tokens_details?.cached_tokens ?? 0, cache_write_tokens: u.prompt_tokens_details?.cache_write_tokens ?? 0, cost: u.cost ?? 0, ms, finish: j.choices?.[0]?.finish_reason };
    spent += rec.cost; calls.push(rec);
    fs.appendFileSync(LEDGER, JSON.stringify({ t: new Date().toISOString(), ...rec }) + "\n");
    const msg = j.choices?.[0]?.message || {};
    const tcs = msg.tool_calls || [];
    if (!tcs.length) { final = msg.content || ""; break; }
    const keep = { role: "assistant", content: msg.content ?? "", tool_calls: tcs };
    if (msg.reasoning_details) keep.reasoning_details = msg.reasoning_details;
    messages.push(keep);
    for (const tc of tcs) {
      let args; try { args = JSON.parse(tc.function.arguments || "{}"); } catch { args = { _unparsable: tc.function.arguments }; }
      toolCalls.push({ name: tc.function.name, args });
      messages.push({ role: "tool", tool_call_id: tc.id, content: JSON.stringify(runTool(q, tc.function.name, args, returned)) });
    }
  }
  if (!final && !error) error = "no final answer within call limit";
  const sum = (k) => calls.reduce((s, c) => s + (c[k] || 0), 0);
  return {
    id: q.id, lang: q.lang, kind: q.kind, question: q.text, final, error, tool_calls: toolCalls,
    totals: { calls: calls.length, cost: sum("cost"), prompt_tokens: sum("prompt_tokens"), completion_tokens: sum("completion_tokens"), reasoning_tokens: sum("reasoning_tokens"), cached_tokens: sum("cached_tokens"), ms: sum("ms"), providers: [...new Set(calls.map((c) => c.provider))] },
    calls, auto: autoChecks(q, toolCalls, final, returned),
  };
}

async function runModel(spec) {
  const at = spec.indexOf("@");
  const model = at < 0 ? spec : spec.slice(0, at);
  const provider = at < 0 ? { zdr: true, sort: "price" } : JSON.parse(spec.slice(at + 1));
  const out = { model, provider, started: new Date().toISOString(), answers: [] };
  const only = process.env.ONLY ? process.env.ONLY.split(",") : null;
  for (const q of P.questions) {
    if (only && !only.includes(q.id)) continue;
    const a = await answer(model, provider, q);
    out.answers.push(a);
    console.log(`${model} ${q.id} calls=${a.totals.calls} cost=${a.totals.cost.toFixed(5)} total=${spent.toFixed(4)} ${a.error ?? ""}`);
    if (spent >= BUDGET_STOP) break;
  }
  const file = path.join(DIR, "results", model.replace(/[/:~]/g, "_") + (process.env.TAG ? "." + process.env.TAG : "") + ".json");
  fs.mkdirSync(path.dirname(file), { recursive: true });
  fs.writeFileSync(file, JSON.stringify(out, null, 1));
}

console.log(`ledger total before run: ${spent.toFixed(4)}`);
await Promise.all(process.argv.slice(2).map(runModel));
console.log(`ledger total after run: ${spent.toFixed(4)}`);
