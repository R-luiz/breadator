# Breadator — architecture

Breadator (`https://breadator.robinluiz.fr`) is a bread-making web app for any baker: calculators
backed by a fermentation and dough engine, a cited knowledge base, recipes, troubleshooting, a
personal bake journal that calibrates predictions, and a grounded chatbot. French and English.
Public repository: code MIT, content CC BY-SA 4.0.

## Principles

1. **One engine, everywhere.** All bread arithmetic and science models live in one pure Rust crate
   (`breadator-engine`). The server links it; the browser runs the same code as WebAssembly; the
   chatbot and outside assistants (MCP) call it through tools. No formula is ever reimplemented
   in JavaScript or in a prompt.
2. **Works without JavaScript, better with it.** Shareable calculators render GET form results
   on the server; their WASM islands can recompute without a round trip. The existing preferment
   planner takes an online POST instead: observations stay out of the URL, aren't saved, and
   its response is not cacheable.
3. **Works without an account, better with one.** Saved anonymous data stays on the device
   (IndexedDB). Signing in syncs saved formulas, starters, bakes and calibration across devices.
   Chat requires an account (it spends the owner's OpenRouter credit).
4. **Every claim has a source.** Knowledge articles, recipes and engine parameters cite entries of
   the research corpus (`research/data/`). The build fails on an unresolved citation. Estimates
   carry an evidence label and an uncertainty range; extrapolation is shown as such.
5. **Small and fast.** One binary, SQLite on a volume, static assets with content hashes and
   immutable caching behind Cloudflare. Target: < 50 KB of critical CSS+HTML, WASM < 300 KB
   compressed, LCP < 1.5 s on a mid-range phone over 4G.

## Repository layout

```
Cargo.toml                 workspace
crates/
  breadator-engine/        pure library: units, formula (baker's math + stages), dough temperature,
                           scalds, yeast conversions, fermentation model, starter model, calibration.
                           No I/O. serde types are the public contract.
  breadator-wasm/          wasm-bindgen wrapper: JSON in, JSON out, one function per calculator.
  breadator-content/       content model + loader: articles (Markdown + TOML front matter), recipes
                           (TOML), sources (references, claims). Validates citations and
                           translations; renders Markdown to sanitized HTML. Used by build.rs of the
                           server to embed the compiled bundle.
  breadator-server/        axum app (binary `breadator`): pages (askama templates), JSON API,
                           auth, user data, sync, chat, tools and their MCP endpoint, search,
                           SQLite, backups, health.
web/                       TypeScript islands + CSS + service worker, built with esbuild into
                           crates/breadator-server/static/dist (hashed names, manifest JSON).
content/
  articles/<slug>/{fr,en}.md
  recipes/<slug>.toml      formula in baker's % + process; titles/notes in fr and en
  troubleshooting/<slug>/{fr,en}.md
research/                  sourced corpus (CONVENTIONS.md); raw/ is git-ignored, never published
migrations/                SQLite migrations (numbered .sql), embedded in the binary
docs/                      this file, operations, model documentation
Dockerfile, fly.toml, litestream.yml
.github/workflows/         ci.yml, deploy.yml
```

## Engine (`breadator-engine`)

| Module | Responsibility |
|---|---|
| `units` | °C/°F, g/oz/lb, volume helpers for display only (storage and computation are metric) |
| `ingredients` | ingredient kinds, default water/fat/sugar content (milk, egg, butter, honey, …), overridable |
| `formula` | total formula in baker's % (flour = 100 % of all flour in all stages); stages (levain, poolish, biga, sponge, scald, soaker); targets (total flour, dough weight, pieces × weight, baked weight with bake loss); per-stage and final-mix grams; hydration variants (water only, all liquids' water); inoculation conventions (levain/final-mix flour, levain/total flour, prefermented-flour %); validation errors instead of negative grams |
| `temperature` | water temperature for a desired dough temperature: classic base-temperature method (3/4 factors) and heat balance with specific heats, mixing temperature rise and ice |
| `scald` | tangzhong / yudane / empois / Brühstück / porridge sizing and its effect on hydration |
| `yeast` | fresh ↔ active dry ↔ instant ↔ osmotolerant conversions; sourdough equivalence is not a ratio (fermentation model instead) |
| `ferment` | progress integration over temperature phases (constant, ramp, Newton cooling/warming); time-to-endpoint and dose-for-time solving; endpoint definitions (aliquot-jar rise); rate modifiers (salt, sugar, enrichment, whole grain, scald…); uncertainty bands |
| `starter` | feed ratio × hydration × temperature → time to peak; levain build planning (seed:flour:water to reach a mass at a time) |
| `preferment` | lean-dough mass balance for a weighed sourdough preferment, reusable peak and bulk prediction, per-plan vigour calibration, alternative mixes when all constraints cannot be met |
| `flour` | flour types (ash class, protein, absorption, usual bread hydration, cold-proof class) and blends: protein, starting hydration with its range, water of additions and scalds, vital gluten for a protein target. The table in `flour/data.rs` is generated from `research/data/model-flour.json` by `research/tools/export_flour_table.py` (CI checks it is current) |
| `calibration` | per-culture vigour factor (log-normal), Bayesian update from logged bakes |
| `params` | model parameters with source keys, evidence labels and validity domains, compiled in from `params/*.toml` (generated from `research/data/model-*.json`) |

Rules: f64 SI-ish units (g, °C, h); no panics on user input (every entry point returns
`Result<_, EngineError>` with a stable error code and the offending field); results include
`warnings` (code + field + params) that the UI translates. Exhaustive unit tests, plus property
tests for mass conservation (sum of stage + final grams equals totals).

## Server (`breadator-server`)

- **axum 0.8**, tokio, tower-http (compression, cache headers, tracing, timeouts), askama templates.
- **SQLite** via rusqlite (bundled, FTS5), one writer connection behind a mutex + a small reader
  pool, WAL, `foreign_keys=ON`, migrations at start-up. Litestream replicates the database to a
  Cloudflare R2 bucket continuously (restore on an empty volume at boot).
- **i18n**: routes are prefixed `/fr/…` and `/en/…`; `/` redirects by cookie, then
  `Accept-Language`. Fluent (`.ftl`) catalogs compiled in; `hreflang` alternates on every page.
  Units: inputs and calculations are metric; with US units every weight and temperature of a
  result also shows ounces/pounds and °F, first (`2 lb 0.5 oz · 920 g`). The choice is the
  device's (`units` cookie, set by the switch in the footer, `GET /{lang}/units/{metric|us}`),
  else the account's setting; `auth::current_units()` is request-scoped, `Tr::g` / `Tr::temp`
  and their mirrors in `web/src/i18n.ts` (told by `<html data-units>`) format. Recipe steps are
  prose and stay metric. The islands read the same catalogs as a
  hashed, immutable asset (`/static/i18n/<lang>.<hash>.json`, named on `<html data-i18n>`),
  fetched once and precached by the service worker: pages do not carry the messages.
- **Pages** (built): home; `calculators` (index) and the calculators `formula`, `fermentation`,
  `preferment`, `starter`, `flour`, `temperature`, `yeast`, shown as tabs under the header;
  `learn` (articles by topic in reading order, `learn/<slug>` with a table of contents: every
  heading has an id), `sources` (bibliography), `search` (FTS5 over articles, recipes and
  claims), `recipes` (`recipes/<slug>`, solved weights, opens in the formula calculator);
  account pages, `my/formulas` and `chat`. A field that applies to one choice only (time or
  dose, sourdough or yeast) is hidden by CSS `:has()` while another is chosen. Planned:
  persistent bake journal and calibration profiles.
- **Knowledge** (`src/knowledge.rs`, `crates/breadator-content`): `content/` and the merged
  research files are compiled in and validated at start-up (unknown citation, missing
  translation, broken internal link or unsolvable recipe formula = the server refuses to start).
  Article citations render as numbered links to a source list; claim sources link to the video
  at its timestamp or to the page.
- **Auth** (optional accounts, `src/auth/`, `src/account/`): email magic link (Resend, from
  `no-reply@robinluiz.fr`; the link opens a confirmation page that consumes nothing; the browser
  that asked for the link holds a 15-minute cookie tied to the token and only there is the button
  pressed by the script; anywhere else the page shows the masked address, `a•••@example.com`, and
  waits for a press; served with `Referrer-Policy: same-origin` and `Cache-Control: no-store,
  no-transform`, so the token leaves neither the site nor through an injected analytics script;
  `no-referrer` would make browsers post the form with `Origin: null`), passkeys (WebAuthn,
  discoverable credentials, `webauthn-rs` pure Rust), Google (OpenID Connect) and GitHub OAuth
  (state + PKCE, verified emails only; a new address gets an account at once, but an identity
  whose address already has an account is linked only after a magic link mailed to that address
  is used; linking from the account page needs the signed-in session instead). Sessions: random
  256-bit token in an `HttpOnly; Secure; SameSite=Lax` cookie (`__Host-session`), SHA-256 of it
  stored server-side, 60-day sliding expiry. CSRF: SameSite + `Origin`/`Referer` must equal
  `PUBLIC_URL` on every non-GET (a cookie-less JSON `/api/` call is exempt). Rate limits in
  SQLite, plus a site-wide cap of 80 sign-in mails a day. Other modules use the `CurrentUser` /
  `MaybeUser` extractors.
- **Sync** (`src/sync.rs`, `web/src/store.ts`): records (formulas, starters, bakes, calibrations)
  have client-generated UUIDv7 ids and `updated_at` (ms); `GET/POST /api/v1/sync` exchanges
  changes since a per-user cursor (change sequence), last-writer-wins per record, soft deletes
  (tombstones; deleting an id the server never stored is ignored). Limits: 16 KiB per record,
  500 records and 1 MiB per request, live records per kind (500 formulas, 50 starters, 2,000
  bakes, 100 calibrations), 5 MB stored per account with tombstones (`users.sync_bytes`, the
  running sum of each row's `bytes`, checked in the push's transaction: a push that would pass
  it is refused whole, `quota_exceeded`), and 30 pushes carrying records per ten minutes per
  account (`rate_limited`). A reply holds at most 500 records and about 1 MiB, cut from the
  stored sizes; records go out as the JSON text written from their columns (`RawValue`), never
  parsed. The browser pushes batches of at most 512 KiB, shows the two refusals and holds the
  refused changes until the next edit (or the end of the ten minutes).
- **Tools** (`src/tools/`): what a model can call, one registry for the chat below and for the
  MCP endpoint. Each has a name, a title, a description and the JSON Schema of its arguments,
  and runs over the embedded knowledge base and the engine:
  - knowledge: `search_knowledge` (the FTS5 index over articles, claims and recipes; returns
    passages in which each statement carries a number `[n]` standing for the citation keys the
    article gives it, or for the claim, recipe or article itself), `list_articles`,
    `read_article` (the article as Markdown, citation numbers after each statement, the
    sources listed at the end), `list_recipes`, `get_recipe` (formula, solved weights, steps,
    sources; scaled to a `target` on request), `list_sources` (the bibliography, by words);
  - calculators: `compute_formula` (`QuickForm`), `solve_formula` (any engine `Formula`:
    enrichments, several stages, a levain built from a seed; a key the type would ignore is
    refused rather than silently dropped), `water_temperature`, `predict_fermentation`,
    `predict_starter`, `plan_existing_preferment` (`QuickPreferment`), `blend_flour`,
    `convert_yeast`.

  A calculator's arguments are laid over its form (`QuickForm`, `QuickFerment`…): what is not
  given keeps the form's default, except the levain of `compute_formula`, absent unless asked
  for, and the predictors and `blend_flour` return the `inputs` they used. An unknown or wrong
  argument comes back as a message naming it and what is accepted, for the model to correct.
  Numbers are rounded as a baker reads them. The vigour calibration saved with an account
  (`vigour`, `anchor_c`) is not offered; `plan_existing_preferment` takes the planner page's
  own observations instead.
- **MCP** (`src/mcp/`, `POST /mcp`): the tools for AI assistants (Claude custom connector,
  Claude Code, any Model Context Protocol client) over Streamable HTTP, through the official
  Rust SDK (`rmcp`).
  - Open and read-only: exactly what the site gives without an account. Every tool is
    annotated `readOnlyHint` with a title. No authentication, no cookie: the route is merged
    outside the session middleware, so a signed-in browser lends it nothing. Account data
    (saved formulas) is not exposed; doing so means OAuth 2.1 with the endpoint as a protected
    resource, not built.
  - Stateless, both protocol eras: the 2026-07-28 revision (per-request `_meta`,
    `server/discover`, `Mcp-Method`/`Mcp-Name` headers checked against the body, `ttlMs` and
    `cacheScope: public` on lists and reads) and the handshake-based ones (2025-11-25 and
    earlier), whose `initialize` is answered without a session id; claude.ai still opens with
    it. Replies are plain JSON (no event stream: no tool runs long); GET and DELETE answer 405.
  - A tool whose text exists in French and English takes `lang` here (default `en`); the
    chat takes the page's language instead. Results link to the site here and only here
    (`Context::site`; the chat lists an answer's sources itself and its model writes no link):
    a search's passages name the article or recipe page they were read on, and its result
    carries `sources`, each `[n]` with its citations (text and link). An article goes out as
    text, everything else as `structuredContent` plus its JSON text. A refused call is a tool
    result with `isError`, an unknown tool a JSON-RPC error.
  - Resources: every article (`breadator://articles/{lang}/{slug}`, Markdown) and recipe
    (`breadator://recipes/{lang}/{slug}`, JSON), for a person to attach to a conversation.
  - Guards: `Host` must be the site's and a browser's `Origin` the site's own (403 otherwise;
    requests without `Origin`, an assistant's backend, pass), bodies over 64 KiB get 413, and
    300 requests a minute per client address then 429 with `Retry-After` (in memory; hosted
    assistants share their servers' addresses, hence the height). One log line per call:
    tool, outcome, duration, protocol revision, user agent, never the arguments.
- **Chat** (`src/chat/`, `web/src/chat.ts`): OpenRouter chat completions, streamed, with seven
  of the tools above, run by the server: `search_knowledge`, `compute_formula`,
  `water_temperature`, `predict_fermentation`, `predict_starter`, `blend_flour` and
  `convert_yeast` (wrong arguments come back to the model as `{"error": …}`).
  - One answer = at most 3 model calls (two tool rounds, then a call with `tool_choice: none`),
    run in its own task so a closed page does not lose a paid answer. The system prompt
    (`prompt.rs`, French or English by the page) restricts the model to bread making, makes it
    search before stating facts, forbids computing numbers itself, asks for `[n]` after every
    sourced statement and `(non sourcé)` / `(unsourced)` after anything else, and forbids
    presenting sourdough as safe for coeliacs.
  - The server renumbers the cited `[n]` in order of citation, turns numbers no search returned
    into the unsourced label, and stores the answer (`chat_messages.content`, Markdown) with its
    sources (`citations`: keys, kind, page) and tool calls (`tool_calls`). Pages render the
    Markdown without links, images or raw HTML, the markers as links to a source list cited
    with `knowledge::cite` like an article's, and the tools used.
  - Pages: `/{lang}/chat` (signed out: what it is and a sign-in link; without
    `OPENROUTER_API_KEY`: unavailable) and `/{lang}/chat/{id}`. Without JavaScript the form
    posts, the answer is written in the background and the conversation page waits for it
    (Post/Redirect/Get). With JavaScript `POST /api/v1/chat` streams server-sent events: `start`
    (conversation id), `text`, `tool` (search / compute progress), `reset`, then `done` (the
    stored answer rendered by the server) or `failed`. Conversations are listed, deletable, and
    part of the account's export and deletion.
  - Models (benchmark 2026-11-24, `research/notes/chat-model/REPORT.md`): `z-ai/glm-5.3-flash`,
    fallback `deepseek/deepseek-v4.1-flash-1017`, sent as `models: [primary, fallback]` with
    `provider: {zdr: true, sort: "price", max_price: {prompt: 0.08, completion: 0.6}}`,
    `reasoning.effort: low`, `max_tokens: 800`, `usage.include`, `session_id` = conversation.
    No `:free` routes.
  - Budget USD 1/month (owner's hard cap): a dedicated OpenRouter key with a USD 1.00 monthly
    credit limit as backstop. Before each call, its ceiling (the price ceilings applied to the
    request size and a full output) is booked in `chat_spend` (the month row and the day row)
    and in `chat_usage` for the account and the client network, with the answer counted on its
    first call; once the call returns, the booking is replaced by its `usage.cost` (a `usage`
    without a cost counts at the larger of its tokens at the ceilings and that estimate; a call
    never billed gives the booking and the answer back). A deploy or a crash during a call
    leaves the ceiling booked, never nothing. A question is refused before any call when the
    day's spend is ≥ USD 0.032 or the month's ≥ USD 0.95 (bookings included; each call checks
    again), when the account has had 10 answers today or 100 this month or spent its share of
    the day (USD 0.008), when its network (IPv4 address or IPv6 /48) has had 15 answers today,
    when it already has an answer being written, or when the site has 2 answers in flight or
    started 6 in the last minute (these two in memory). The account's allowances are keyed on a
    SHA-256 of its lowercased address, so deleting the account and signing up again gives none
    back. Questions ≤ 1000 characters; the last 4 question-and-answer pairs are sent as
    history, without their markers.
- **Security headers**: strict CSP (`script-src 'self' 'wasm-unsafe-eval'`, no inline script),
  HSTS, `X-Content-Type-Options`, `Referrer-Policy`, `Permissions-Policy`.
- **Observability**: JSON logs (tracing), `/healthz`, minimal privacy-preserving metrics
  (Cloudflare Web Analytics script, as on robinluiz.fr).

## Data model (SQLite)

| Table | Content |
|---|---|
| `users` | id, email (unique, nullable for passkey-only), display name, locale, units, sync sequence and stored bytes, created_at |
| `identities` | user_id, provider (`google`, `github`), subject; unique (provider, subject) |
| `passkeys` | user_id, credential id, serialized credential, name, created_at, last_used_at |
| `email_tokens` | sha256(token), email, purpose (`signin`, or `link` with the provider and subject to link), sha256 of the asking browser's cookie, expires_at, used_at |
| `sessions` | sha256(token), user_id, created_at, expires_at, user_agent |
| `oauth_states` | sha256(state), provider, PKCE verifier, nonce, lang, link intent, expires_at: OAuth requests in flight |
| `rate_limits` | key (hashed), window, count, expires_at |
| `formulas` | id (uuid), user_id, name, json (engine `Formula`), updated_at, deleted, seq, bytes |
| `starters` | id, user_id, name, flour, hydration, notes, updated_at, deleted, seq, bytes |
| `bakes` | id, user_id, formula_id, starter_id, plan json, actual json (phases, temperatures, endpoint, times), outcome, notes, updated_at, deleted, seq, bytes |
| `calibrations` | user_id, culture key (starter id or yeast kind), log-vigour mean/variance, n, seq, bytes |
| `chat_conversations`, `chat_messages` | per user; messages with tool calls and citations |
| `chat_usage` | subject (`a:` + SHA-256 of an account's address, `u:` + SHA-256 of a user id without one, `c:` + SHA-256 of a client network), day, answers, tokens in/out, cost_usd (bookings included); not tied to `users`, rows of earlier months deleted |
| `chat_spend` | month (`YYYY-MM`, plus one `YYYY-MM-DD` row for the current day), cost_usd: the global ledger of the owner's OpenRouter credit, kept when accounts are deleted |
| `search` (FTS5) | rebuilt from the embedded content bundle at start-up |

## Front end (`web/`)

TypeScript, no framework: one island per shareable calculator (`data-island="formula"`), a shared WASM loader,
IndexedDB store for anonymous data and sync, chat client (SSE), service worker (precache shell,
CSS, JS, WASM; network-first for HTML; offline fallback page; its copies of pages are deleted on
sign-out, session revocation, account deletion and "remove my data from this device"). Mobile
first: single column, large touch targets, numeric keypads (`inputmode="decimal"`), results
pinned above the keyboard; desktop: inputs and results side by side. Light and dark themes from
the system setting.
Accessibility: labels on every field, results announced via `aria-live`, no colour-only status.

## Hosting and operations

- **Fly.io** app `breadator`, region `fra`, one `shared-cpu-1x` 256 MB machine with a volume
  `breadator_data` mounted at `/data`; image from `Dockerfile` (multi-stage: Rust release build +
  wasm-bindgen + esbuild, `debian:trixie-slim` runtime, non-root uid 10001, Litestream).
- **DNS**: Cloudflare zone `robinluiz.fr`, `breadator` CNAME → `breadator.fly.dev`, proxied;
  Fly certificate via the DNS-01 `_acme-challenge` CNAME. Cloudflare caches `/static/*`
  (immutable, hashed names).
- **Secrets** (Fly): `OPENROUTER_API_KEY`, `RESEND_API_KEY`, `GOOGLE_CLIENT_ID/SECRET`,
  `GITHUB_CLIENT_ID/SECRET`, `LITESTREAM_BUCKET`, `LITESTREAM_ENDPOINT`,
  `LITESTREAM_ACCESS_KEY_ID/SECRET_ACCESS_KEY` (no `SESSION_SECRET`: session tokens are random and
  stored hashed). Full list and one-time setup: `docs/OPERATIONS.md`.
- **CI** (GitHub Actions, same shape as LoL Rank Bot): fmt, clippy `-D warnings`, tests, content
  validation, front-end typecheck + build, Docker image build + smoke test (start, `/healthz`,
  non-root, clean SIGTERM). **Deploy**: after green CI on `main`, gated job deploys to Fly and
  smoke-tests the public URL.

## Research → product pipeline

`research/data/*` (claims, references, recipes, model parameters) is the evidence base.
Engine parameters are generated into `crates/breadator-engine/params/*.toml` by
`research/tools/build-params.py`, which keeps source keys. Articles and recipes in `content/` are
written from the corpus and cite claim ids / reference keys; `breadator-content` validates them.
