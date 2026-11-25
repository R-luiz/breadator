// Copy buttons for the blocks of text a page gives to paste elsewhere (`.copyable` around a
// `<pre>`; `data-label` says what the block is, for screen readers). Without JavaScript or
// without clipboard access there is no button and the text stays there to select.
import { t } from "./i18n";

export async function copyIsland(root: HTMLElement): Promise<void> {
  if (!navigator.clipboard) return;
  for (const block of root.querySelectorAll<HTMLElement>(".copyable")) {
    const source = block.querySelector("pre");
    if (!source) continue;
    const button = document.createElement("button");
    button.type = "button";
    button.className = "small copy";
    button.textContent = t("copy-button");
    if (block.dataset.label) button.setAttribute("aria-label", t("copy-label", { what: block.dataset.label }));
    let reset = 0;
    button.addEventListener("click", () => {
      navigator.clipboard
        .writeText(source.textContent ?? "")
        .then(() => {
          button.textContent = t("copy-done");
          window.clearTimeout(reset);
          reset = window.setTimeout(() => {
            button.textContent = t("copy-button");
          }, 2000);
        })
        // Refused (permissions, an insecure page): the text is selected for Ctrl+C instead.
        .catch(() => window.getSelection()?.selectAllChildren(source));
    });
    block.append(button);
  }
}
