# Breadator chatbot model selection (OpenRouter, USD 1/month cap)

Run 2026-11-24. All prices, endpoints and benchmark figures come from OpenRouter's public API as fetched that day: `models-raw.json` (12:56 UTC), `endpoints-raw.json`, `zdr-endpoints-raw.json` and `benchmarks-raw.json`. Live calls used the owner's key. Prices change often, so rerun `costmodel.mjs` before acting on these numbers.

## Recommendation

| Setting | Value |
|---|---|
| **Primary model** | `z-ai/glm-5.3-flash` |
| **Fallback model** | `deepseek/deepseek-v4.1-flash-1017`, through the request's `models` array, so OpenRouter fails over automatically |
| **`:free` route** | **No.** Details in the free-route section below. |
| **Request body** | `"models": ["z-ai/glm-5.3-flash", "deepseek/deepseek-v4.1-flash-1017"]`, `"provider": {"zdr": true, "sort": "price", "max_price": {"prompt": 0.08, "completion": 0.6}}`, `"reasoning": {"effort": "low"}`, `"max_tokens": 800`, `"tool_choice": "auto"`, `"session_id": <conversation id>` |
| **Max output tokens** | 800 per call. The largest output seen in 138 benchmark calls was 655 tokens, reasoning included, and median final answers were 140–508 tokens. The server allows at most 3 LLM calls per answer, which means at most 2 tool round-trips. |
| **OpenRouter key** | A dedicated key for the site with **credit limit USD 1.00 and `limit_reset: monthly`**, so OpenRouter refuses requests itself once the month's dollar is spent. Under account privacy, turn on **ZDR enforcement for "All other models"** with **ZDR only** (Safety Retention excluded), and leave **"OpenRouter use of inputs/outputs"** and **Input/Output logging** off. Training opt-out is implied by ZDR. |
| **Global throttle (app side)** | Add up `usage.cost` from every response. Cut the chatbot off for the rest of the UTC day once the day's spend reaches **USD 0.032** (0.95 ÷ 30), and for the rest of the month once the month's spend reaches **USD 0.95**. Separately, allow at most 6 answers per minute site-wide and 2 requests in flight. |
| **Per-user quota** | **10 answers/day and 100/month** per signed-in account, one request in flight per user, questions capped at 1,000 characters, history cut to the last 4 turns (about 1k tokens). |

### Why GLM-5.3-Flash

All five live models scored between 113 and 119 out of 120 (scores below). The rubric's tie-break, fixed before the run, says the cheaper model wins within 5 points. GLM-5.3-Flash scored 117 and has the lowest projected cost per answer on its ZDR route: USD 0.00052–0.00059 against 0.00062–0.00094 for DeepSeek on its observed route and 0.0007–0.0029 for the others. It also has strong public science and tool-use scores:

- GPQA-Diamond 84.1 % and τ²-Bench airline 75.6 % (OpenRouter evals).
- MMLU-Pro: biology 93.9 %, chemistry 91.1 %, health 80.9 % (Vals).
- Artificial Analysis Healthcare & Medical index 36.6.

It made no tool-argument errors, invented no citation keys and never cited the irrelevant passage. Its slips were two unlabeled practical tips and one `[non sourcé]` label in an English answer.

DeepSeek V4.1 Flash 1017 is the fallback. It scored 115 with the same clean tool use and has the strongest public science numbers of the cheap tier: GPQA-D 90.1 %, AA Healthcare 41.3, MMLU-Pro 86.3 %. It is a different vendor on different provider infrastructure, so a GLM outage does not take it down too.

Muse Ember scored highest (119) but was not chosen, for three reasons:

- It costs 1.8× as much per answer.
- Its only ZDR-eligible route is Google Vertex with Safety Retention, so prompts may be kept for abuse review; that is not strict ZDR.
- It supports only `tool_choice: auto`.

### Cost arithmetic for the recommendation

The production token budget, per the assignment: system 1.5k, tool schemas 1k, history 1k, question about 50, retrieved passages 3k, each tool-call message about 80, each engine result about 300, answer 500.

- **1 round-trip** (search, then answer): call 1 takes 3,550 input tokens and outputs 80; call 2 takes 6,630 input and outputs 500.
- **2 round-trips** (search, then engine, then answer): a third call takes 7,010 input and outputs 500.
- **Caching:** each call reuses the previous call's prefix, and the shared 2.5k system-and-tools prefix is warm under steady traffic.

Input and output token counts are multiplied by each model's **tokenizer factor**. This is the ratio of first-call `prompt_tokens` for identical text, measured live: GLM 0.88, DeepSeek 0.96, Muse 1.04, Luna 0.72, Haiku 1.39. Measured reasoning tokens per answer are added on top.

**GLM-5.3-Flash on Relace fp4** (the route served in 22 of 24 benchmark calls; ZDR, USD 0.0356/M in, 0.0815/M out, 0.030/M cache read):

- 2 round-trips, no cache: input (3,550 + 6,630 + 7,010) × 0.88 = 15,127 tokens × 0.0356 = USD 0.000539. Output (80 + 80 + 500) × 0.88 = 581 tokens × 0.0815, plus 4 reasoning tokens = USD 0.000048. **Total USD 0.000587 per answer, so USD 1 buys 1,704 answers.**
- With caching: USD 0.000523, or 1,910 answers. Relace's cache read costs almost as much as uncached input, so caching saves only about 11 %.

**Fallback DeepSeek on Wafer/us** (ZDR, USD 0.036/M in, 0.50/M out, 0.010/M cache read): USD 0.00094 per answer without cache (1,059 answers per USD 1) and USD 0.00062 with cache (1,607). Pinned to Tenstorrent (USD 0.03/0.20, ZDR), it would cost USD 0.00029 cached, about 3,500 answers, but Auto Exacto routed only 3 of 24 calls there. OpenRouter reorders tool-calling requests by tool-call reliability, so that cheaper route cannot be counted on.

**Planning number: USD 0.0010 per answer.** This covers fallback to DeepSeek without cache and longer-than-budget histories. It gives **1,000 answers per month**, about **32 per day**, which is where the USD 0.032 daily cutoff comes from. At GLM's typical cost the same dollar cap allows about 55–60 answers per day.

**Worst case per answer** under `max_price` and the call caps: 3 calls of about 18k input tokens at the USD 0.08/M ceiling is USD 0.0014. Three outputs of 800 tokens at the USD 0.60/M ceiling is USD 0.0014. Total USD 0.0029, so even a month of nothing but worst-case answers allows about 340.

Abuse cannot push spend over the cap:

- The app-side dollar cutoffs stop the chatbot first.
- The OpenRouter key's USD 1 monthly limit is the hard backstop if the app's accounting fails.
- The per-user quota only spreads the budget: 10 answers/day means at least 3 users are needed to exhaust a day's allowance.

`max_price` matters: in the smoke run, `{"zdr": true}` alone routed DeepSeek to Reka at USD 0.30/1.20, ten times the list price. Sticky session routing then kept the same `session_id` on Reka (benchmark Q05, USD 0.00121 for one answer). With `max_price` set, endpoints like Reka are never eligible. The final check (`results/*.prodcfg.json`, 6 answers) was served by Relace and Wafer only, at USD 0.00013–0.00047 per answer.

## Candidate table

Benchmark sources:

- **GPQA-D** and **τ²-air**: OpenRouter's own evals, from `GET /api/v1/benchmarks?source=openrouter` (as of 2026-11-17; https://openrouter.ai/rankings). τ²-air is τ²-Bench airline, which tests multi-turn agentic tool calling.
- **AA-Int** and **AA-Health**: the Artificial Analysis Intelligence and Healthcare & Medical indexes, via the same endpoint (as of 2026-11-24; https://artificialanalysis.ai).
- **MMLU-Pro overall**: Kaggle Open Benchmarks (https://www.kaggle.com/benchmarks/open-benchmarks/mmlu-pro) where the model is listed; otherwise Vals (https://www.vals.ai/benchmarks/mmlu_pro).
- **MMLU-Pro biology / chemistry / health**: Vals per-subject data embedded in that page, extracted to `vals-mmlu-pro-subjects.json`.
- **BFCL**: the Berkeley leaderboard CSV (`bfcl-data_overall.csv`, https://gorilla.cs.berkeley.edu/leaderboard.html) has no 2026 model, so τ²-air stands in for tool use.
- **Multilingual**: AA's Multilingual index lists none of the shortlisted models, and BenchLM's MMMLU page has only lab-reported scores for older models. French fluency was therefore measured live (6 French questions).

"–" means no published score was found. Prices are USD per million tokens.

| Model | List in/out (cache read) | Cheapest ZDR route with tools | Context | tool_choice | GPQA-D | τ²-air | AA-Int / AA-Health | MMLU-Pro overall; bio / chem / health | Data policy of routes |
|---|---|---|---|---|---|---|---|---|---|
| **z-ai/glm-5.3-flash** | 0.12 / 0.38 (0.02) | Relace fp4 0.036 / 0.081 | 1,048,576 | yes (Relace: auto/none only) | 84.1 | 75.6 | 27.8 / 36.6 | 86.1 (Vals); 93.9 / 91.1 / 80.9 | 26 of 35 endpoints ZDR. Z.AI first-party is not ZDR. |
| **deepseek/deepseek-v4.1-flash-1017** | 0.03 / 0.20 (0.001) | Tenstorrent fp8 0.03 / 0.20; served mostly by Wafer/us 0.036 / 0.50 | 1,048,576 | yes (Tenstorrent: no `required`) | 90.1 | 73.3 | 32.6 / 41.3 | 86.3 (Kaggle, V4.1 Flash); subjects – (V4 Flash 0731 on Vals: 92.9 / 91.0 / 80.7) | 34 of 43 endpoints ZDR. DeepSeek first-party, Novita, SiliconFlow, Baidu, Alibaba and others are not. |
| **anthropic/claude-haiku-5.5** | 0.10 / 0.50 (0.01; cache write 0.125) | Vertex or Bedrock 0.10 / 0.50 | 1,000,000 | yes (no `required`) | 86.6 | 72.8 | 36.5 / 37.5 | 85.8 (Kaggle); subjects – | Vertex and Bedrock are strict ZDR. Anthropic first-party, Azure and Claude-on-AWS are not. |
| **openai/gpt-6-luna** | 0.10 / 0.50 (0.01; cache write 0.125) | Azure 0.10 / 0.50 | 1,050,000 | yes | 87.4 | 67.3 | 27.9 / 34.8 | 85.4 (Kaggle); subjects – (GPT-5.6 Luna: 93.3 / 91.0 / 81.4) | Only Azure is ZDR. OpenAI first-party and Bedrock are not. Moderated. |
| **meta/muse-ember** | 0.10 / 0.50 (0.004) | Vertex 0.10 / 0.50 | 1,048,576 | auto only | – | – | 36.0 / 41.3 | – | Only Vertex is listed, and it uses **Safety Retention** (not strict ZDR). Moderated. |
| inclusionai/ling-3.1-flash | 0.075 / 0.225 (0.015) | none | 1,048,576 | yes | 81.3 | 79.3 | 29.6 / 38.3 | – (Ling 3.0 Flash 82.0 Vals; 91.8 / 90.5 / 79.0) | Ant Digital only. **Not ZDR**; it passes `data_collection: deny`, so OpenRouter lists it as not training. |
| qwen/qwen3.8-flash | 0.15 / 0.47 (0.016) | none | 1,000,000 | yes (no `required`) | 88.6 | 71.3 | 28.6 / 36.4 | – | Alibaba only. **Not ZDR**; passes `data_collection: deny`. |
| deepseek/deepseek-v4-flash | 0.09 / 0.18 (0.018) | Relace fp4 0.05 / 0.45 | 1,048,576 | yes | 86.1 | 72.3 | – | 0731 snapshot 86.2 (Vals) | 8 of 11 endpoints ZDR. |
| xiaomi/mimo-v2.6-flash | 0.14 / 0.28 (0.003) | Cohere fp8 0.14 / 0.28 | 1,048,576 | yes | 73.2 | 77.9 | 26.7 / 35.1 | – | 5 of 7 endpoints ZDR. Xiaomi first-party is not. |
| google/gemma-4-31b-it:free | 0 / 0 | none | 262,144 | yes | 82.5 | 75.3 | 4.4 / – | 86.7 (Kaggle, Gemma 4 31B) | Google AI Studio free route: **not ZDR**; passes `data_collection: deny`. Rate-limited upstream (below). |
| nvidia/nemotron-3.5-super-vl:free | 0 / 0 | none | 1,000,000 | yes | 86.8 | 72.9 | 13.6 / 21.0 | – | **Trains on prompts**: rejected with "No endpoints found matching your data policy (Free model training)" under `data_collection: deny`. |

Bold rows were benchmarked live.

Excluded from the shortlist:

- `amazon/nova-micro-v1` has a high OpenRouter GPQA score but no `tools` support.
- `openai/gpt-oss-120b` scores GPQA 74.5 and AA Healthcare 5.8.
- `*:batch` variants are asynchronous, so they cannot serve chat.

`nvidia/nemotron-3-ultra-550b-a55b:free` also fails `data_collection: deny`, so it trains on prompts like the other NVIDIA free route.

### Projected cost per answer and answers per month under USD 1

Output of `node costmodel.mjs` (also saved as `costmodel-output.md`). Each benchmarked model is priced on the route OpenRouter actually served; the others on their cheapest ZDR endpoint, or list price when none exists. The cost columns are in milli-USD, so 0.587 means USD 0.000587. Because the cap is USD 1 per month, "answers per USD 1" equals answers per month.

| Model | Route priced | In / out per M | Cache read per M | Tokenizer factor | Reasoning tokens per answer (measured) | Measured per answer, benchmark (~4k context) | 1 / 2 trips, no cache | 1 / 2 trips, cached | Answers per USD 1, 2 trips, no cache | Answers per USD 1, 2 trips, cached |
|---|---|---|---|---|---|---|---|---|---|---|
| z-ai/glm-5.3-flash | Relace (fp4) | 0.036 / 0.081 | 0.030 | 0.88 | 4 | 0.149 | 0.361 / 0.587 | 0.331 / 0.523 | **1,704** | **1,910** |
| deepseek/deepseek-v4.1-flash-1017 | Wafer (us) | 0.036 / 0.500 | 0.010 | 0.96 | 60 | 0.386 | 0.662 / 0.943 | 0.509 / 0.622 | 1,059 | 1,607 |
| openai/gpt-6-luna | Azure | 0.100 / 0.500 | 0.010 | 0.72 | 0 | 0.325 | 0.947 / 1.483 | 0.621 / 0.726 | 674 | 1,378 |
| meta/muse-ember | Vertex | 0.100 / 0.500 | 0.004 | 1.04 | 179 | 0.524 | 1.451 / 2.223 | 0.839 / 0.940 | 449 | 1,064 |
| anthropic/claude-haiku-5.5 | Vertex | 0.100 / 0.500 | 0.010 (write 0.125) | 1.39 | 88 | 1.057 | 1.864 / 2.895 | 1.237 / 1.438 | 345 | 695 |
| inclusionai/ling-3.1-flash | list (no ZDR) | 0.075 / 0.225 | 0.015 | 1.00* | 0* | – | 0.894 / 1.438 | 0.526 / 0.667 | 695 | 1,498 |
| qwen/qwen3.8-flash | list (no ZDR) | 0.150 / 0.470 | 0.016 | 1.00* | 0* | – | 1.800 / 2.889 | 0.978 / 1.168 | 346 | 856 |
| deepseek/deepseek-v4-flash | Relace (fp4) | 0.050 / 0.450 | 0.050 | 1.00* | 0* | – | 0.770 / 1.157 | 0.770 / 1.157 | 864 | 864 |
| xiaomi/mimo-v2.6-flash | Cohere (fp8) | 0.140 / 0.280 | 0.003 | 1.00* | 0* | – | 1.588 / 2.591 | 0.748 / 0.832 | 385 | 1,201 |

\* Not measured. These rows use factor 1.00 and no reasoning tokens, so they are optimistic.

Notes on caching:

- The cached columns assume the shared 2.5k system-and-tools prefix is warm, and that call n+1 reuses call n's prefix.
- Anthropic and OpenAI GPT-5.6+ charge 1.25× input for cache writes; DeepSeek charges writes at the input price.
- On Haiku, only 2 of 12 benchmark answers got any cache read (`cached_tokens` > 0). The ~4k-token benchmark prompts were probably under its minimum cacheable size, so Haiku's cached column is a best case.

## `:free` routes: not for production

- **Limits** (https://openrouter.ai/docs/api_reference/limits): 20 requests per minute and 50 per day, rising to **1,000 per day once the account has bought ≥ USD 10 of credit in total**. This account qualifies: `GET /api/v1/key` reports `free_model_daily_requests.limit = 1000`. Credit raises only the daily count. It does not add upstream capacity, and a negative balance blocks free models too.
- **Availability, measured:** `google/gemma-4-31b-it:free` answered **1 of 37 benchmark requests**; every other request got HTTP 429 "temporarily rate-limited upstream" from Google AI Studio. Earlier probes of `inclusionai/ling-3.2-flash:free`, `baidu/ernie-5.5-flash-preview:free` and `google/gemma-4-26b-a4b-it:free` also mostly returned 429. Free routes are shared capacity, and OpenRouter's 1,000/day counter does not guarantee it.
- **Privacy:** no `:free` endpoint is ZDR. Every one tested returned 404 "No endpoints found matching your data policy (Zero data retention)" under `zdr: true`. The NVIDIA free routes also train on prompts.
- **Billing trap:** with a `:free` model first in a `models` array, every failure falls through to the paid model and is billed (https://openrouter.ai/docs/guides/routing/model-variants/free). A free primary would therefore save little, add a 429 round-trip to most answers, and send user questions to non-ZDR endpoints.

## Live benchmark

### Setup

| Item | Detail |
|---|---|
| Files | `bench.mjs` (Node, no dependencies), `prompts.json` (system prompt, 3 tool schemas, 18 passages, 12 questions with expected arguments and key facts), `rubric.md` (written before any live call), raw transcripts in `results/<model>.json`, per-call ledger in `spend.jsonl` |
| Calls | `POST /api/v1/chat/completions`, `tool_choice: auto`, `reasoning.effort: low`, `max_tokens: 2500`, up to 5 calls per answer |
| Provider settings | `{"zdr": true, "sort": "price"}`; the free model used `{"data_collection": "deny"}`. Anthropic got a top-level `cache_control` to opt into caching. |
| Mock `search_knowledge` | Returns the question's 3 passages, one of them always irrelevant |
| Mock `compute_formula` | Baker's-percentage weights, levain counted in total flour and water |
| Mock `predict_bulk_time` | Q10-style temperature model |
| Questions | Q01–Q04 mechanisms (rye enzymes and acidification, tangzhong vs yudane, salt and fermentation, stiff vs liquid starter). Q05–Q08 numeric: the right tool must be called with exact arguments, including 75 °F → 23.9 °C and one call per temperature scenario. Q09–Q12 grounding: Q09's core temperature and Q12's medical claims are outside the passages and must be labeled. French: Q01, Q03, Q05, Q07, Q09, Q11. English: the rest. |
| Grading | `rubric.md`: correctness, tool use, citation fidelity, language, brevity, each scored 0–2, so 10 per question and 120 per model. I graded every answer myself from the full transcript; the script's automatic checks (argument match, invented or irrelevant keys, label count, word count) served as evidence. |
| Context size | Benchmark prompts were about 3–6k tokens, smaller than the 6.5k production budget. Cost projections use the production budget, not benchmark costs. |

### Scores

| Model | Correctness | Tool use | Citation fidelity | Language | Brevity | **Total /120** | Tool-argument accuracy (Q05–Q08) | Invented keys / irrelevant cited | Measured USD per answer | Median latency per answer |
|---|---|---|---|---|---|---|---|---|---|---|
| meta/muse-ember | 24 | 23 | 24 | 24 | 24 | **119** | 4/4 | 0 / 0 | 0.00052 | 4.3 s |
| **z-ai/glm-5.3-flash** | 24 | 24 | 22 | 23 | 24 | **117** | 4/4 | 0 / 0 | 0.00015 | 6.8 s (max 22 s) |
| deepseek/deepseek-v4.1-flash-1017 | 24 | 24 | 22 | 22 | 23 | **115** | 4/4 | 0 / 0 | 0.00039 | 5.7 s |
| openai/gpt-6-luna | 21 | 24 | 22 | 23 | 24 | **114** | 4/4 | 0 / 0 | 0.00033 | 5.2 s |
| anthropic/claude-haiku-5.5 | 22 | 24 | 21 | 22 | 24 | **113** | 4/4 | 0 / 0 | 0.00106 | 4.5 s |
| google/gemma-4-31b-it:free | – | – | – | – | – | **0 answers** | – | – | 0 | 36 of 37 requests got 429 |

Every paid model called the expected engine tool with exact arguments on all four numeric questions: 75 °F was converted to 23.9 °C, Q07 got two separate `predict_bulk_time` calls, and Q08 passed `levain_pct: 0` with `instant_yeast_pct: 0.5`. No model computed weights or times itself, invented a citation key, or cited the irrelevant passage. The differences are in labeling discipline and in completeness.

### Per-question grades and evidence

Each cell is correctness / tool use / citation fidelity / language / brevity, then the total. "Clean" means every key fact was present, the tool calls were right, the citations were accurate, the language idiomatic, and the answer at most 250 words. Each model's transcript is in `results/<model>.json`; the production-config spot check is in `results/*.prodcfg.json`.

#### z-ai/glm-5.3-flash, 117/120 (routed to Relace fp4 for 22 of 24 calls)

| Q | Score | Evidence |
|---|---|---|
| Q01 FR | 2/2/2/2/2 = 10 | Amylase, 52–62 °C, starch attack, pH 4.0–4.3, pentosans; one search; 157 words. |
| Q02 EN | 2/2/2/1/2 = 9 | Both methods correct and cited. Its closing recommendation is labeled `[non sourcé]` in an English answer. |
| Q03 FR | 2/2/2/2/2 = 10 | Osmotic pressure, glycerol, 10–20 % at 1.8–2.2 %, above 2.5 %, gluten. |
| Q04 EN | 2/2/1/2/2 = 9 | Acetic vs lactic, electron acceptor, temperatures. The practical tip "refreshed less often" is unsourced and unlabeled. |
| Q05 FR | 2/2/2/2/2 = 10 | Exact arguments. Weights 915/661/203/20 g, with the levain split explained; 89 words. |
| Q06 EN | 2/2/2/2/2 = 10 | Called with 23.9 °C and 15 %; reports 5.7 h (4.9–6.6 h); 43 words. |
| Q07 FR | 2/2/2/2/2 = 10 | Two calls; 8.9 h and 5.7 h with ranges. "About a third shorter" is correct arithmetic on the tool's numbers. |
| Q08 EN | 2/2/2/2/2 = 10 | Exact arguments, weights rounded correctly. Its baked-weight estimate is labeled `[not sourced]`. |
| Q09 FR | 2/2/2/2/2 = 10 | Both causes cited. Core temperature 96–99 °C labeled `[non sourcé]`; 242 words. |
| Q10 EN | 2/2/1/2/2 = 9 | Content correct and cited. The "45–60 min, water holdback" tip is unlabeled. |
| Q11 FR | 2/2/2/2/2 = 10 | Chlorine levels, chloramine, young starters, hardness 50–100 mg/L, mineral water unnecessary. Did not cite the irrelevant tangzhong passage. |
| Q12 EN | 2/2/2/2/2 = 10 | Answers "No", cites both passages, gives the 20 mg/kg and < 10 ppm figures, labels the medical advice. |

Production-config check (`max_price`): Q07, Q09 and Q12 again had correct arguments and labels, routed to Relace and Wafer, at USD 0.00013–0.00025 per answer.

#### deepseek/deepseek-v4.1-flash-1017, 115/120 (Wafer 19 calls, Tenstorrent 3, Reka 2)

| Q | Score | Evidence |
|---|---|---|
| Q01 FR | 10 | Clean. The rye-levain 20–40 % tip is labeled `[non sourcé]`. |
| Q02 EN | 10 | Clean, 182 words. |
| Q03 FR | 2/2/1/2/2 = 9 | Turns the 10–20 % gas-rate figure into "bulk about 10–20 % longer" without a label. That is an unsourced inference about fermentation time. |
| Q04 EN | 10 | Clean, including temperature. Two practical additions are properly labeled `[not sourced]`. |
| Q05 FR | 10 | Exact arguments and correct weights; the autolyse tip is labeled. Sticky routing kept this session on Reka (USD 0.00121). |
| Q06 EN | 2/2/1/2/2 = 9 | Correct conversion and numbers, plus an unneeded search. Labels the engine output itself "[not sourced — engine output…]". |
| Q07 FR | 2/2/2/1/2 = 9 | Two correct calls. Writes "l'apprêt peut arriver vite" about the end of bulk, but *apprêt* means final proof, not bulk. |
| Q08 EN | 10 | Exact arguments; mixing tips labeled. |
| Q09 FR | 2/2/2/2/1 = 9 | Best-grounded Q09 answer: both causes cited, and an explicit "le corpus ne donne pas de valeur" before 96–99 °C `[non sourcé]`. 253 words, just over the limit. |
| Q10 EN | 2/2/2/1/2 = 9 | Content and citations correct, but uses `[non sourcé]` labels in an English answer. |
| Q11 FR | 10 | Clean. |
| Q12 EN | 10 | Clean. Cross-contamination and the doctor advice are labeled. |

#### meta/muse-ember, 119/120 (Vertex, Safety Retention)

| Q | Score | Evidence |
|---|---|---|
| Q01–Q08, Q10–Q12 | 10 each | All key facts, every sentence cited, no unlabeled additions, 38–175 words. Q05 shows totals and the levain split; Q07 cites `lab-temperature` and labels its advice. |
| Q09 FR | 2/1/2/2/2 = 9 | Content and labeling clean (96–98 °C `[non sourcé]`), but it made 5 searches over 3 calls, which is redundant. |

#### openai/gpt-6-luna, 114/120 (Azure)

| Q | Score | Evidence |
|---|---|---|
| Q01–Q03, Q05–Q08 | 10 each | Shortest answers in the set (47–146 words), correct and cited. Q05 is a bare French table, 49 words. |
| Q04 EN | 1/2/2/2/2 = 9 | Gives the acetic vs lactic trend but omits the mechanism (electron acceptor) and the temperature passage. |
| Q09 FR | 1/2/2/2/2 = 9 | Says the sources give no core temperature, then gives none. The system prompt asks for a labeled best general answer. |
| Q10 EN | 2/2/1/2/2 = 9 | Malformed label "[non sourced]" on the fermentolyse remark. |
| Q11 FR | 1/2/1/2/2 = 8 | Skips water hardness entirely, and attaches "l'eau minérale n'est donc pas nécessaire" to `[chlorine-water]`, which does not say that. |
| Q12 EN | 2/2/2/1/2 = 9 | Correct, but labeled `[non sourcé]` in an English answer. |

#### anthropic/claude-haiku-5.5, 113/120 (Vertex)

| Q | Score | Evidence |
|---|---|---|
| Q01 FR | 2/2/2/1/2 = 9 | Correct and cited, but contains a truncated comparison: "à plus basse température … contre celle du blé". |
| Q02 EN | 1/2/1/2/2 = 8 | Unsupported, unlabeled claims: "Tangzhong gelatinizes more fully at a lower percentage", "Yudane adds more flour and water". |
| Q03, Q05–Q09, Q11 | 10 each | Q09 is a model grounding answer: opens with "la base de connaissances … ne donne pas de température", then labels 96–98 °C. |
| Q04 EN | 1/2/1/2/2 = 8 | Claims a cool fermentation gives "a less acidic flavour overall" and cites `lab-temperature`, which does not say that. This contradicts its own previous paragraph (cool and stiff gives a sharper sour). |
| Q10 EN | 2/2/2/1/2 = 9 | Uses `[non sourcé]` labels in an English answer. |
| Q12 EN | 2/2/1/2/2 = 9 | Correct "No", but the medical guidance (certified gluten-free, cross-contamination, doctor) is not labeled. |

## Total spend

| Measure | Amount |
|---|---|
| This run's own calls (`spend.jsonl`, 140 rows: smoke test, 5 paid models × 12 questions, 6 production-config answers, routing and policy probes; `usage.cost` summed) | **USD 0.0321** |
| Key usage delta from `GET /api/v1/key`, 14.738360 → 14.775218 | USD 0.0369 |

Both figures are far under the USD 0.50 limit and the USD 0.45 stop threshold. The key delta is higher than the ledger because other processes used the same key concurrently: `usage_daily` was already USD 0.019 before the first call, and it rose by about USD 0.002 at times when this run made no calls. Remaining key credit: USD 5.23.

## Reproduce

```sh
node bench.mjs z-ai/glm-5.3-flash deepseek/deepseek-v4.1-flash-1017      # default provider {"zdr":true,"sort":"price"}
ONLY=Q07,Q09 TAG=x node bench.mjs 'model@{"zdr":true,"max_price":{"prompt":0.08,"completion":0.6}}'
node costmodel.mjs                                                          # cost table from the saved JSON files
```

`bench.mjs` reads the key at runtime from `~/.omp/agent/models.yml`, appends every call to `spend.jsonl`, and stops issuing calls once the ledger reaches USD 0.45.
