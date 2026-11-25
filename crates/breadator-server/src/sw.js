// Appended to `const VERSION` and `const PRECACHE` by the server (/sw.js).
const STATIC_CACHE = "static-" + VERSION;
const PAGES_CACHE = "pages";
// Signing out (here or everywhere), revoking a session or deleting the account: the copies of
// pages show the account's name, so they go. ("Remove my data from this device" also clears them,
// from the page: `wipeLocal` in web/src/store.ts.)
const FORGET_PAGES = /^\/[a-z]{2}\/account\/(sign-out|sign-out-all|delete|sessions\/[^/]+\/revoke)$/;

self.addEventListener("install", (event) => {
  event.waitUntil(
    caches
      .open(STATIC_CACHE)
      .then((cache) => cache.addAll(PRECACHE))
      .then(() => self.skipWaiting()),
  );
});

self.addEventListener("activate", (event) => {
  event.waitUntil(
    caches
      .keys()
      .then((keys) =>
        Promise.all(
          keys.filter((k) => k !== STATIC_CACHE && k !== PAGES_CACHE).map((k) => caches.delete(k)),
        ),
      )
      .then(() => self.clients.claim()),
  );
});

self.addEventListener("fetch", (event) => {
  const request = event.request;
  const url = new URL(request.url);
  if (request.method === "POST" && url.origin === self.location.origin && FORGET_PAGES.test(url.pathname)) {
    event.waitUntil(caches.delete(PAGES_CACHE));
    return;
  }
  if (request.method !== "GET") return;
  if (url.origin !== self.location.origin || url.pathname.startsWith("/api/")) return;

  // Hashed, immutable assets: cache first.
  if (url.pathname.startsWith("/static/")) {
    event.respondWith(caches.match(request).then((hit) => hit || fetch(request)));
    return;
  }

  // Pages: network first, keep a copy, fall back to the copy (or the home page) offline.
  if (request.mode === "navigate") {
    event.respondWith(
      fetch(request)
        .then((response) => {
          // Personal pages (account, sign-in steps) say no-store: never keep a copy.
          if (response.ok && !/no-store/i.test(response.headers.get("cache-control") || "")) {
            const copy = response.clone();
            caches.open(PAGES_CACHE).then((cache) => cache.put(request, copy));
          }
          return response;
        })
        .catch(async () => {
          const cached = await caches.match(request);
          if (cached) return cached;
          const lang = url.pathname.startsWith("/fr") ? "/fr" : "/en";
          return (await caches.match(lang)) || Response.error();
        }),
    );
  }
});
