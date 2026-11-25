// Island of the chat page. Without it the form posts and the conversation page waits for the
// answer; with it the question goes to POST /api/v1/chat and the answer streams in as
// server-sent events: text as it is written, a line while a tool runs, then the stored answer as
// the server renders it (markers linked to the numbered sources).
import { lang, t } from "./i18n";

interface StreamEvent {
  name: string;
  data: Record<string, unknown>;
}

/** Moves the complete events of `buffer` to `out`; returns the unfinished rest. */
function parseEvents(buffer: string, out: StreamEvent[]): string {
  let end = buffer.indexOf("\n\n");
  while (end >= 0) {
    const block = buffer.slice(0, end);
    buffer = buffer.slice(end + 2);
    let name = "message";
    let data = "";
    for (const line of block.split("\n")) {
      if (line.startsWith("event: ")) name = line.slice(7);
      else if (line.startsWith("data: ")) data += line.slice(6);
    }
    // Comment lines (`: ping`) carry no data.
    if (data) {
      try {
        out.push({ name, data: JSON.parse(data) as Record<string, unknown> });
      } catch {
        console.warn("unreadable chat event", name);
      }
    }
    end = buffer.indexOf("\n\n");
  }
  return buffer;
}

function element<K extends keyof HTMLElementTagNameMap>(tag: K, className: string, text?: string): HTMLElementTagNameMap[K] {
  const node = document.createElement(tag);
  node.className = className;
  if (text !== undefined) node.textContent = text;
  return node;
}

export async function chatIsland(root: HTMLElement): Promise<void> {
  const form = root.querySelector<HTMLFormElement>("form[data-ask]");
  const log = root.querySelector<HTMLElement>("[data-log]");
  const textarea = form?.querySelector("textarea");
  const button = form?.querySelector<HTMLButtonElement>("button[type=submit]");
  if (!form || !log || !textarea || !button) return;
  const status = form.querySelector<HTMLElement>("[data-status]");
  const left = form.querySelector<HTMLElement>("[data-left]");

  root.querySelector<HTMLFormElement>("form[data-delete]")?.addEventListener("submit", (event) => {
    if (!window.confirm(t("chat-delete-confirm"))) event.preventDefault();
  });

  // With a keyboard, Enter sends and Shift+Enter makes a new line (Ctrl/⌘+Enter sends too). On a
  // touch screen Enter stays a new line and the button sends.
  const keyboard = window.matchMedia("(pointer: fine)").matches;
  textarea.addEventListener("keydown", (event) => {
    if (event.key !== "Enter" || event.isComposing) return;
    if (event.ctrlKey || event.metaKey || (keyboard && !event.shiftKey)) {
      event.preventDefault();
      form.requestSubmit();
    }
  });

  /** A conversation just created: listed and marked as the current one, as a reload would show. */
  const list = root.querySelector<HTMLElement>(".chat-list");
  const listConversation = (id: string, title: string): void => {
    if (!list) return;
    list.querySelector("p.note")?.remove();
    list.querySelector('a[aria-current="page"]')?.removeAttribute("aria-current");
    const items = list.querySelector("ul") ?? list.appendChild(document.createElement("ul"));
    const link = element("a", "", title);
    link.href = `/${lang}/chat/${encodeURIComponent(id)}`;
    link.setAttribute("aria-current", "page");
    const item = document.createElement("li");
    item.append(link);
    items.prepend(item);
  };

  const setStatus = (text: string, error = false): void => {
    if (!status) return;
    status.textContent = text;
    status.className = error ? "error" : "note";
  };

  let busy = false;
  form.addEventListener("submit", (event) => {
    event.preventDefault();
    if (!busy) void ask();
  });

  async function ask(): Promise<void> {
    const question = textarea!.value.trim();
    if (!question) {
      setStatus(t("chat-refusal-empty"), true);
      return;
    }
    busy = true;
    button!.disabled = true;
    setStatus("");
    root.querySelector("[data-unanswered]")?.remove();
    const conversation = form!.querySelector<HTMLInputElement>("input[name=conversation]");

    const asked = element("li", "chat-q");
    asked.append(element("span", "who", t("chat-you")), element("p", "", question));
    const answer = element("li", "chat-a");
    const progress = element("p", "note chat-progress", t("chat-progress-write"));
    const text = element("div", "chat-text chat-streaming");
    answer.append(element("span", "who", "Breadator"), progress, text);
    log!.append(asked, answer);
    answer.scrollIntoView({ block: "nearest" });

    let finished = false;
    const handle = (event: StreamEvent): void => {
      const data = event.data;
      switch (event.name) {
        case "start": {
          const id = String(data.conversation ?? "");
          if (!conversation && id) {
            const input = document.createElement("input");
            input.type = "hidden";
            input.name = "conversation";
            input.value = id;
            form!.prepend(input);
            // The address of the new conversation, so that a reload shows it.
            history.replaceState(null, "", `/${lang}/chat/${encodeURIComponent(id)}`);
            listConversation(id, String(data.title ?? question));
          }
          break;
        }
        case "tool":
          progress.textContent = t(data.kind === "search" ? "chat-progress-search" : "chat-progress-compute");
          break;
        case "reset":
          text.textContent = "";
          break;
        case "text":
          progress.textContent = t("chat-progress-write");
          text.textContent += String(data.t ?? "");
          break;
        case "done":
          finished = true;
          // Rendered and sanitized by the server: the stored answer with its sources.
          answer.replaceChildren(element("span", "who", "Breadator"));
          answer.insertAdjacentHTML("beforeend", String(data.html ?? ""));
          if (left) left.textContent = t("chat-left", { n: String(data.left ?? "") });
          break;
        case "failed":
          finished = true;
          progress.textContent = String(data.message ?? t("chat-error-failed"));
          progress.className = "error";
          text.remove();
          textarea!.value = question;
          break;
      }
    };

    try {
      const response = await fetch("/api/v1/chat", {
        method: "POST",
        headers: { "content-type": "application/json", accept: "text/event-stream" },
        body: JSON.stringify({ question, conversation: conversation?.value ?? null, lang }),
      });
      if (!response.ok || !response.body) {
        let message = t("chat-refusal-failed");
        try {
          const body = (await response.json()) as { error?: { message?: string } };
          if (body.error?.message) message = body.error.message;
        } catch {
          // Not JSON: the generic message.
        }
        asked.remove();
        answer.remove();
        setStatus(message, true);
        finished = true;
        return;
      }
      textarea!.value = "";
      const reader = response.body.getReader();
      const decoder = new TextDecoder();
      let buffer = "";
      for (;;) {
        const { value, done } = await reader.read();
        if (done) break;
        const events: StreamEvent[] = [];
        buffer = parseEvents(buffer + decoder.decode(value, { stream: true }), events);
        events.forEach(handle);
      }
    } catch (error) {
      console.warn("chat stream failed", error);
    } finally {
      if (!finished) {
        // The answer may still have been stored: the page shows it once reloaded.
        progress.textContent = t("chat-error-network");
        progress.className = "error";
      }
      busy = false;
      button!.disabled = false;
    }
  }
}
