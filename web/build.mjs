// Bundles the islands into crates/breadator-server/static/dist/, which the server embeds.
// Expects the wasm-bindgen output in web/pkg (see the Dockerfile / CI for the commands).
import { build } from "esbuild";
import { copyFile, mkdir, rm } from "node:fs/promises";
import { fileURLToPath } from "node:url";

const here = (p) => fileURLToPath(new URL(p, import.meta.url));
const out = here("../crates/breadator-server/static/dist/");

await rm(out, { recursive: true, force: true });
await mkdir(out, { recursive: true });
await build({
  entryPoints: [here("src/app.ts")],
  outfile: `${out}app.js`,
  bundle: true,
  format: "esm",
  minify: true,
  target: ["es2022"],
  legalComments: "none",
  logLevel: "info",
});
await copyFile(here("pkg/breadator_wasm_bg.wasm"), `${out}breadator_wasm_bg.wasm`);
