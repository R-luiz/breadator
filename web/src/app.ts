// Entry point: starts the island of each calculator on the page and registers the service worker.
import { accountIsland } from "./account";
import { myFormulasIsland } from "./account-formulas";
import { chatIsland } from "./chat";
import { fermentationIsland } from "./fermentation";
import { flourIsland } from "./flour";
import { formulaIsland } from "./formula";
import { loadCatalog } from "./i18n";
import { starterIsland } from "./starter";
import { startSync } from "./store";
import { temperatureIsland } from "./temperature";
import { yeastIsland } from "./yeast";

const islands: Record<string, (root: HTMLElement) => Promise<void>> = {
  account: accountIsland,
  chat: chatIsland,
  fermentation: fermentationIsland,
  flour: flourIsland,
  formula: formulaIsland,
  "my-formulas": myFormulasIsland,
  starter: starterIsland,
  temperature: temperatureIsland,
  yeast: yeastIsland,
};

const roots = [...document.querySelectorAll<HTMLElement>("[data-island]")];
if (roots.length > 0) {
  // On failure (messages not reachable, old browser, blocked WASM) the server-rendered page keeps
  // working.
  loadCatalog()
    .then(() => {
      for (const root of roots) {
        const start = islands[root.dataset.island ?? ""];
        start?.(root).catch((error: unknown) => console.error("island failed", root.dataset.island, error));
      }
    })
    .catch((error: unknown) => console.error("messages unavailable", error));
}

// Signed in (the server marks the page): saved formulas, starters, bakes and calibration sync with
// the account. Visitors never reach the network from here.
startSync();

if ("serviceWorker" in navigator) {
  navigator.serviceWorker.register("/sw.js").catch(() => {});
}
