// Island of /{lang}/my/formulas: the formulas saved on this device (and synced to the account when
// signed in), drawn from IndexedDB. Open one in the calculator, rename it, delete it.
import { encodeState } from "./engine";
import { lang, t } from "./i18n";
import { type FormulaRecord, list, onChange, onSyncStatus, put, remove, startSync, syncStatus } from "./store";

function el<K extends keyof HTMLElementTagNameMap>(
  tag: K,
  props: Partial<HTMLElementTagNameMap[K]> = {},
  ...children: (Node | string)[]
): HTMLElementTagNameMap[K] {
  const node = Object.assign(document.createElement(tag), props);
  node.append(...children);
  return node;
}

export async function myFormulasIsland(root: HTMLElement): Promise<void> {
  root.querySelector("[data-nojs]")?.remove();
  const listElement = root.querySelector<HTMLElement>("[data-list]");
  const empty = root.querySelector<HTMLElement>("[data-empty]");
  const statusElement = root.querySelector<HTMLElement>("[data-status]");
  if (!listElement || !empty) return;

  const open = (record: FormulaRecord) =>
    `/${lang}/calculators/formula?s=${encodeState(record.formula)}&f=${encodeURIComponent(record.id)}`;

  /** Row of a formula; `mode` swaps the buttons for the rename form or the delete confirmation. */
  const row = (record: FormulaRecord, mode: "view" | "rename" | "delete") => {
    const item = el("li");
    const redraw = (next: "view" | "rename" | "delete") => item.replaceWith(row(record, next));
    if (mode === "rename") {
      const input = el("input", { type: "text", value: record.name, maxLength: 120, required: true });
      input.setAttribute("aria-label", t("my-rename"));
      const form = el(
        "form",
        {},
        input,
        el("button", { type: "submit", className: "small", textContent: t("my-rename-save") }),
        el("button", { type: "button", className: "secondary small", textContent: t("my-rename-cancel") }),
      );
      form.querySelector<HTMLButtonElement>('button[type="button"]')?.addEventListener("click", () => redraw("view"));
      form.addEventListener("submit", (event) => {
        event.preventDefault();
        const name = input.value.trim();
        if (name) void put("formulas", { id: record.id, name, formula: record.formula });
        else redraw("view");
      });
      item.append(form);
      queueMicrotask(() => input.focus());
      return item;
    }
    const when = new Date(record.updated_at).toLocaleDateString(lang);
    item.append(
      el(
        "span",
        { className: "grow" },
        el("strong", { textContent: record.name }),
        el("br"),
        el("span", { className: "note", textContent: t("my-updated-on", { date: when }) }),
      ),
    );
    if (mode === "delete") {
      const yes = el("button", { type: "button", className: "danger small", textContent: t("my-delete-yes") });
      const no = el("button", { type: "button", className: "secondary small", textContent: t("my-rename-cancel") });
      yes.addEventListener("click", () => void remove("formulas", record.id));
      no.addEventListener("click", () => redraw("view"));
      item.append(el("span", { className: "note", textContent: t("my-delete-confirm") }), yes, no);
      return item;
    }
    const rename = el("button", { type: "button", className: "secondary small", textContent: t("my-rename") });
    const del = el("button", { type: "button", className: "secondary small", textContent: t("my-delete") });
    rename.addEventListener("click", () => redraw("rename"));
    del.addEventListener("click", () => redraw("delete"));
    item.append(el("a", { className: "button small", href: open(record), textContent: t("my-open") }), rename, del);
    return item;
  };

  let editing = false;
  const render = async () => {
    let records: FormulaRecord[];
    try {
      records = await list("formulas");
    } catch (error) {
      console.warn("cannot read the saved formulas", error);
      listElement.replaceChildren();
      empty.hidden = true;
      if (statusElement) statusElement.textContent = t("my-storage-error");
      return;
    }
    // Leave an open rename or delete prompt alone while other tabs or a sync redraw the list.
    if (editing) return;
    listElement.replaceChildren(...records.map((r) => row(r, "view")));
    empty.hidden = records.length > 0;
  };
  listElement.addEventListener("focusin", (event) => {
    editing = (event.target as HTMLElement).tagName === "INPUT";
  });
  listElement.addEventListener("focusout", () => {
    editing = false;
  });

  const describe = () => {
    if (!statusElement || !document.body.hasAttribute("data-signed-in")) return;
    const { state, blocked } = syncStatus();
    // A refusal of the server (account full, too many pushes) says what to do; it outlasts syncs.
    const key =
      blocked ??
      { syncing: "my-sync-syncing", done: "my-sync-done", offline: "my-sync-offline", error: "my-sync-error" }[
        state as "syncing" | "done" | "offline" | "error"
      ];
    statusElement.textContent = key ? t(key) : "";
  };

  onChange((kind) => {
    if (kind === "formulas") void render();
  });
  onSyncStatus(describe);
  await render();
  describe();
  startSync();
}
