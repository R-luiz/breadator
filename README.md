# Breadator

Bread-making calculators and science, for every baker: **https://breadator.robinluiz.fr**
(French and English).

- **Formula calculator**: baker's percentages for the whole bread, with levain, poolish and
  scald (tangzhong, yudane, empois, Kochstück); weights for each stage and the final mix, levain
  build from a seed starter, hydration with or without the water of milk, eggs and butter,
  inoculation in both conventions, dough yield. Saved formulas stay on the device, or sync
  across devices with an optional account (email link, passkey, Google, GitHub).
- **Fermentation time**: time to an endpoint (aliquot jar +25 % to doubled, ripe poolish) for a
  dose, or the dose for a time, over room / fridge / room phases. Follows the dough's own
  temperature while it cools and warms, applies salt, sugar, fat, whole-grain and other factors,
  and gives an 80 % range. Model and fits: `research/notes/science/fermentation/`.
- **Starter feeding**: when a refreshed starter peaks for a feed ratio (starter : flour : water),
  its hydration and the temperature; or the ratio that peaks at the time you want, with the
  grams of seed, flour and water for the amount of levain needed.
- **Existing preferment**: enter the grams already in a sourdough preferment, its age, desired
  baked weight and bulk time. Compare a final mix for those targets with ways to use the entire
  preferment. Optionally estimate culture vigour from up to four measured peaks or bulk rises,
  or enter a known factor. Out-of-domain estimates are marked.
- **Flour and hydration**: protein of a blend of flour types (French T45–T150, German, Italian,
  US, rye, spelt and other grains), a hydration to start from with its range, the water that
  bran, seeds, psyllium, vital gluten or a scald call for, and the vital gluten needed for a
  protein target. Data and sources: `research/data/model-flour.json`.
- **Water temperature**: heat balance from masses and heat capacities (Choi & Okos 1986), or the
  classic base-temperature rule; ice and water amounts when the water must be colder than the tap.
- **Yeast converter**: fresh, active dry, instant and osmotolerant yeast, with the range
  manufacturers give.
- **Learn**: articles where every fact cites its source (papers, *The Sourdough Framework*,
  bakers' documented tests), a bibliography, full-text search, and recipes that open in the
  formula calculator.
- **Chat**: questions answered from the knowledge base with numbered sources; weights, times and
  temperatures come from the calculators, not from the model. Signed-in accounts only, under a
  fixed monthly budget.
- **For AI assistants**: an [MCP](https://modelcontextprotocol.io) endpoint,
  `https://breadator.robinluiz.fr/mcp`, gives Claude and other assistants the calculators, the
  knowledge base with its sources and the recipes. No account and no key; see
  [Use it from an assistant](#use-it-from-an-assistant).

Coming: a persistent bake journal. See [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md).

Every calculator works without JavaScript. The shareable forms put their state in the URL and
can also recompute offline using the same Rust engine compiled to WebAssembly. The existing
preferment planner instead submits an online form without storing the calibration inputs or
putting them in the URL. The site installs as an app. French and English; results in grams and
°C, with ounces, pounds and °F alongside for those who switch.

## Use it from an assistant

The endpoint speaks the Model Context Protocol over Streamable HTTP, without authentication and
read-only: 14 tools (search, articles, recipes, bibliography and the eight calculators) and the
articles and recipes as resources, in English and French. The step-by-step guide for bakers,
with everything to paste, is on the site: https://breadator.robinluiz.fr/en/assistants
(opening `/mcp` in a browser leads there too).

- **Claude** (claude.ai, desktop, mobile): open https://claude.ai/customize/connectors (the page
  is called Plugins in recent versions) → Add → Add custom connector, name Breadator, URL
  `https://breadator.robinluiz.fr/mcp`, Continue, authentication "No sign in" (detected), Add.
- **Claude Code**: `claude mcp add --transport http breadator https://breadator.robinluiz.fr/mcp`.
- **Any other client**: the same URL; a quick look with the MCP Inspector:
  `npx -y @modelcontextprotocol/inspector --cli https://breadator.robinluiz.fr/mcp --transport http --method tools/list`.

An assistant then answers "two 800 g loaves at 75 % hydration with 20 % levain, and how long is
the bulk at 24 °C?" with the engine's numbers, and "does salt slow fermentation?" from passages
that carry their sources.

## Layout

| Path | Content |
|---|---|
| `crates/breadator-engine` | the engine: pure Rust, no I/O; every formula lives here |
| `crates/breadator-wasm` | the engine for the browser (JSON in, JSON out) |
| `crates/breadator-content` | knowledge base loader: articles, recipes, citation and link validation, full-text search |
| `crates/breadator-server` | axum server: pages (askama), JSON API, MCP endpoint, accounts and sync (SQLite), Fluent translations, embedded static files and content |
| `content/` | articles (`articles/<slug>/{fr,en}.md`) and recipes (`recipes/<slug>.toml`); how to write them: [`content/STYLE.md`](content/STYLE.md) |
| `migrations/` | SQLite schema, applied at start-up |
| `web/` | TypeScript islands, bundled by esbuild into the server's `static/dist/` |
| `research/` | the sourced corpus behind the engine and the content ([`research/CONVENTIONS.md`](research/CONVENTIONS.md)); `research/raw/` is never committed |
| `docs/` | architecture and operations |

The published research files (`research/data/bibliography.json`, `claims.jsonl`, `videos.json`)
are compiled into the server. After changing a source's claims or references, regenerate them
with `py research/tools/merge_corpus.py` (and `video_index.py` after fetching transcripts);
`cargo test -p breadator-content --test repo_content` validates every citation and link.
Bibliography keys are permanent: `research/data/bib-keys.json` records each key with its paper,
so a new reference never takes or re-letters a key an article cites. Never edit it by hand.

## Development

Requirements: Rust stable with the `wasm32-unknown-unknown` target, `wasm-bindgen-cli` 0.2.130
(same version as the crate), Node 24.

```sh
# engine for the browser, then the islands (writes crates/breadator-server/static/dist/)
cargo build --release --target wasm32-unknown-unknown -p breadator-wasm
wasm-bindgen --target web --out-dir web/pkg target/wasm32-unknown-unknown/release/breadator_wasm.wasm
(cd web && npm ci && npx tsc --noEmit && node build.mjs)

# server on http://localhost:8080 (PORT, PUBLIC_URL, DATABASE_PATH, LOG_LEVEL, LOG_FORMAT=json);
# the SQLite database is data/breadator.sqlite3. Without RESEND_API_KEY, sign-in links are
# logged instead of mailed (docs/OPERATIONS.md lists every variable and secret).
PUBLIC_URL=http://localhost:8080 cargo run -p breadator-server

# checks run by CI
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
```

Without the islands built, the server still runs: pages fall back to server-rendered forms.

On Windows (GNU toolchain), build with MSYS2 UCRT64 `gcc` first on a trimmed `PATH`, e.g.
`PATH='C:\Users\<you>\.cargo\bin;C:\msys64\ucrt64\bin;C:\Windows\System32;C:\Windows' cargo test`.

## Licence

Code: MIT. Content (articles, recipes, research notes): CC BY-SA 4.0; parts adapted from
*The Sourdough Framework* by Hendrik Kleinwächter (CC BY-SA 4.0) are credited where used.
