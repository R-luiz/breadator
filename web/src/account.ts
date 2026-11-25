// Island of the account and sign-in pages. Every page works without it; this adds:
//  - the magic-link confirmation pressed for you, only in the browser that asked for the link
//    (the server marks that page; anywhere else the visitor reads whose account it is and decides),
//  - passkeys (sign-in and adding one),
//  - clearing this device's saved data after "remove my data from this device",
//  - forgetting the offline copies of pages when a session ends.
import { PasskeyError, describePasskeyError, passkeysSupported, registerPasskey, signInWithPasskey } from "./account-passkey";
import { forgetPages, wipeLocal } from "./store";

/** Forms that end a session or the account, as `FORGET_PAGES` in the service worker. */
const ENDS_SESSION = /\/account\/(sign-out|sign-out-all|delete|sessions\/[^/]+\/revoke)$/;

function show(element: Element | null, text?: string): void {
  if (!element) return;
  if (text !== undefined) element.textContent = text;
  (element as HTMLElement).hidden = false;
}

/** Where to go after a passkey sign-in: the page asked for (`?next=`) if it is on this site. The
 * URL parser decides, as the browser will: `/\evil.example` or a tab after the slash is another
 * host. */
function nextPage(fallback: string): string {
  const next = new URLSearchParams(location.search).get("next");
  if (!next || !next.startsWith("/")) return fallback;
  let url: URL;
  try {
    url = new URL(next, location.origin);
  } catch {
    return fallback;
  }
  if (url.origin !== location.origin || url.pathname.startsWith("/api/")) return fallback;
  return url.pathname + url.search + url.hash;
}

export async function accountIsland(root: HTMLElement): Promise<void> {
  if (root.hasAttribute("data-wipe-local")) {
    try {
      await wipeLocal();
    } catch (error) {
      console.warn("cannot clear this device's saved data", error);
    }
  }

  // The service worker forgets its copies of pages when a session ends, but it does not control
  // every page (after a hard reload, for instance): do it from here too, then send the form.
  for (const form of root.querySelectorAll<HTMLFormElement>("form[method=post]")) {
    if (!ENDS_SESSION.test(new URL(form.action, location.origin).pathname)) continue;
    form.addEventListener("submit", (event) => {
      if (form.dataset.forgotten) return;
      event.preventDefault();
      form.dataset.forgotten = "1";
      void forgetPages().finally(() => form.requestSubmit((event as SubmitEvent).submitter));
    });
  }

  // The magic-link page: sign in without a second click.
  const auto = root.querySelector<HTMLFormElement>("form[data-autosubmit]");
  if (auto) {
    window.setTimeout(() => auto.requestSubmit(), 0);
    return;
  }

  if (!passkeysSupported()) return;

  const login = root.querySelector<HTMLElement>("[data-passkey-login]");
  if (login) {
    login.hidden = false;
    const button = login.querySelector<HTMLButtonElement>("[data-passkey-login-button]");
    const error = login.querySelector("[data-passkey-error]");
    button?.addEventListener("click", async () => {
      button.disabled = true;
      if (error) (error as HTMLElement).hidden = true;
      try {
        const target = await signInWithPasskey();
        location.assign(nextPage(target));
      } catch (e) {
        if (!(e instanceof PasskeyError)) console.warn("passkey sign-in failed", e);
        show(error, describePasskeyError(e));
        button.disabled = false;
      }
    });
  }

  const register = root.querySelector<HTMLElement>("[data-passkey-register]");
  if (register) {
    register.hidden = false;
    const button = register.querySelector<HTMLButtonElement>("[data-passkey-register-button]");
    const name = register.querySelector<HTMLInputElement>("[data-passkey-name]");
    const error = register.querySelector("[data-passkey-error]");
    button?.addEventListener("click", async () => {
      button.disabled = true;
      if (error) (error as HTMLElement).hidden = true;
      try {
        await registerPasskey(name?.value.trim() ?? "");
        // The page lists passkeys server-side: reload it with the confirmation banner.
        location.assign(`/${document.documentElement.lang}/account?msg=linked`);
      } catch (e) {
        if (!(e instanceof PasskeyError)) console.warn("passkey registration failed", e);
        show(error, describePasskeyError(e));
        button.disabled = false;
      }
    });
  }
}
