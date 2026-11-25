// "Save this formula": a small control the formula island adds under its results. The formula goes
// to the device store (and syncs to the account when signed in); `/my/formulas` lists it and opens
// it again in the calculator through `?s=` (the formula) and `&f=` (which saved record it is).
import { lang, t } from "./i18n";
import { get, put } from "./store";

/** The formula the server rendered the page from (`<script id="formula-state">`), if any. */
export function formulaFromPage(): unknown | null {
  const element = document.getElementById("formula-state");
  try {
    const value: unknown = JSON.parse(element?.textContent ?? "null");
    return value !== null && typeof value === "object" ? value : null;
  } catch {
    return null;
  }
}

function el<K extends keyof HTMLElementTagNameMap>(
  tag: K,
  props: Partial<HTMLElementTagNameMap[K]> = {},
  ...children: (Node | string)[]
): HTMLElementTagNameMap[K] {
  const node = Object.assign(document.createElement(tag), props);
  node.append(...children);
  return node;
}

/**
 * Adds the save control to `container`, which must outlive the island's re-rendering of its
 * results (append it to the island root, not inside `.results`). `getFormula` is called when the
 * button is pressed and returns the engine `Formula` currently on screen, or null when there is
 * none to save (the inputs are invalid).
 *
 * When the page was opened from a saved formula (`&f=<id>` in the address), the main button
 * updates that record and a second one saves a copy.
 */
export function mountSaveButton(container: HTMLElement, getFormula: () => unknown | null): void {
  const input = el("input", { type: "text", maxLength: 120, placeholder: t("save-name-placeholder") });
  input.autocomplete = "off";
  const message = el("p", { className: "note" });
  message.setAttribute("role", "status");
  const save = el("button", { type: "button", textContent: t("save-button") });
  const saveCopy = el("button", { type: "button", className: "secondary", textContent: t("save-as-new") });
  saveCopy.hidden = true;
  const actions = el("p", { className: "actions" }, save, saveCopy);
  const label = el("label", { className: "field" }, el("span", { className: "label", textContent: t("save-name-label") }), input);
  const box = el("div", { className: "save-formula" }, label, actions, message);
  container.append(box);

  let savedId: string | null = new URLSearchParams(location.search).get("f");

  void (async () => {
    if (!savedId) return;
    try {
      const existing = await get("formulas", savedId);
      if (!existing) {
        savedId = null;
        return;
      }
      input.value = existing.name;
      save.textContent = t("save-update", { name: existing.name });
      saveCopy.hidden = false;
    } catch {
      savedId = null;
    }
  })();

  const run = async (asCopy: boolean) => {
    const formula = getFormula();
    if (formula === null) {
      message.textContent = t("save-nothing");
      return;
    }
    const name = input.value.trim() || t("save-default-name", { date: new Date().toLocaleDateString(lang) });
    try {
      const record = await put("formulas", { id: asCopy ? undefined : (savedId ?? undefined), name, formula });
      savedId = record.id;
      input.value = record.name;
      save.textContent = t("save-update", { name: record.name });
      saveCopy.hidden = false;
      message.replaceChildren(
        `${t("save-saved")} `,
        el("a", { href: `/${lang}/my/formulas`, textContent: t("save-open-list") }),
      );
    } catch (error) {
      console.warn("cannot save the formula", error);
      message.textContent = t("save-error");
    }
  };
  save.addEventListener("click", () => void run(false));
  saveCopy.addEventListener("click", () => void run(true));
}
