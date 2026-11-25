// Saved formulas, starters, bakes and calibrations, kept in IndexedDB on this device and, when
// signed in, synchronized with the account (`/api/v1/sync`, see crates/breadator-server/src/sync.rs).
//
// Signed out, nothing here touches the network. Signed in (the server marks `<body data-signed-in>`),
// `startSync()` pulls on load, pushes shortly after every change, and again when the connection
// returns or the tab becomes visible. Rules shared with the server: client-chosen UUIDv7 ids,
// `updated_at` in milliseconds, deletions are tombstones, the newest `updated_at` wins.

export type Kind = "formulas" | "starters" | "bakes" | "calibrations";
export const KINDS: readonly Kind[] = ["formulas", "starters", "bakes", "calibrations"];

interface Synced {
  id: string;
  /** Milliseconds since the epoch, from this device's clock at the last edit. */
  updated_at: number;
  deleted: boolean;
}

/** Saved formula: `formula` is the engine's `Formula` (the same JSON the calculators take). */
export interface FormulaRecord extends Synced {
  name: string;
  formula: unknown;
}

export interface StarterRecord extends Synced {
  name: string;
  flour: string;
  hydration: number;
  notes: string;
}

export interface BakeRecord extends Synced {
  formula_id: string | null;
  starter_id: string | null;
  plan: unknown;
  actual: unknown;
  outcome: string;
  notes: string;
}

/** Per-culture vigour estimate; `id` is the culture key (a starter id or a yeast kind). */
export interface CalibrationRecord extends Synced {
  mean: number;
  variance: number;
  n: number;
}

export interface RecordOf {
  formulas: FormulaRecord;
  starters: StarterRecord;
  bakes: BakeRecord;
  calibrations: CalibrationRecord;
}

/** What callers give to `put`: the content; id and clocks are managed here. */
export type Draft<K extends Kind> = Omit<RecordOf[K], "id" | "updated_at" | "deleted"> & { id?: string };

/** What is stored locally: the record plus `dirty` (1 = changed since the server last saw it). */
type Stored<K extends Kind> = RecordOf[K] & { dirty: 0 | 1 };

const DB_NAME = "breadator";
const META = "meta";

let opened: Promise<IDBDatabase> | undefined;

function openDb(): Promise<IDBDatabase> {
  opened ??= new Promise((resolve, reject) => {
    if (typeof indexedDB === "undefined") {
      reject(new Error("IndexedDB is not available"));
      return;
    }
    const request = indexedDB.open(DB_NAME, 1);
    request.onupgradeneeded = () => {
      const db = request.result;
      for (const kind of KINDS) {
        db.createObjectStore(kind, { keyPath: "id" }).createIndex("dirty", "dirty");
      }
      db.createObjectStore(META, { keyPath: "key" });
    };
    request.onsuccess = () => resolve(request.result);
    request.onerror = () => reject(request.error ?? new Error("cannot open the store"));
    request.onblocked = () => reject(new Error("the store is blocked by another tab"));
  });
  // A failed open (private mode, blocked storage) may succeed later: do not cache the failure.
  opened.catch(() => (opened = undefined));
  return opened;
}

function wrap<T>(request: IDBRequest<T>): Promise<T> {
  return new Promise((resolve, reject) => {
    request.onsuccess = () => resolve(request.result);
    request.onerror = () => reject(request.error ?? new Error("store request failed"));
  });
}

function done(tx: IDBTransaction): Promise<void> {
  return new Promise((resolve, reject) => {
    tx.oncomplete = () => resolve();
    tx.onerror = () => reject(tx.error ?? new Error("store transaction failed"));
    tx.onabort = () => reject(tx.error ?? new Error("store transaction aborted"));
  });
}

async function getMeta<T>(key: string): Promise<T | undefined> {
  const db = await openDb();
  const row = await wrap<{ key: string; value: T } | undefined>(db.transaction(META).objectStore(META).get(key));
  return row?.value;
}

async function setMeta(key: string, value: unknown): Promise<void> {
  const db = await openDb();
  const tx = db.transaction(META, "readwrite");
  tx.objectStore(META).put({ key, value });
  await done(tx);
}

/** UUIDv7: 48-bit millisecond timestamp, then random bits (ids sort by creation time). */
export function newId(): string {
  const bytes = crypto.getRandomValues(new Uint8Array(16));
  const ms = Date.now();
  bytes[0] = Math.floor(ms / 2 ** 40) & 0xff;
  bytes[1] = Math.floor(ms / 2 ** 32) & 0xff;
  bytes[2] = Math.floor(ms / 2 ** 24) & 0xff;
  bytes[3] = Math.floor(ms / 2 ** 16) & 0xff;
  bytes[4] = Math.floor(ms / 2 ** 8) & 0xff;
  bytes[5] = ms & 0xff;
  bytes[6] = 0x70 | ((bytes[6] ?? 0) & 0x0f);
  bytes[8] = 0x80 | ((bytes[8] ?? 0) & 0x3f);
  const hex = Array.from(bytes, (b) => b.toString(16).padStart(2, "0")).join("");
  return `${hex.slice(0, 8)}-${hex.slice(8, 12)}-${hex.slice(12, 16)}-${hex.slice(16, 20)}-${hex.slice(20)}`;
}

// ---- change notifications ---------------------------------------------------------------------

type Listener = (kind: Kind) => void;
const listeners = new Set<Listener>();
const localListeners = new Set<Listener>();
const channel = typeof BroadcastChannel === "undefined" ? undefined : new BroadcastChannel("breadator-store");

channel?.addEventListener("message", (event: MessageEvent<{ kind: Kind }>) => {
  for (const listener of listeners) listener(event.data.kind);
});

/** Called when a kind changes: by this tab, by another tab, or by a sync. Returns the unsubscribe. */
export function onChange(listener: Listener): () => void {
  listeners.add(listener);
  return () => listeners.delete(listener);
}

function emit(kind: Kind, edit: boolean): void {
  for (const listener of listeners) listener(kind);
  if (edit) for (const listener of localListeners) listener(kind);
  channel?.postMessage({ kind });
}

// ---- reading and writing ----------------------------------------------------------------------

function strip<K extends Kind>(record: Stored<K>): RecordOf[K] {
  const { dirty: _dirty, ...rest } = record;
  return rest as unknown as RecordOf[K];
}

/** Live (not deleted) records, most recently edited first. */
export async function list<K extends Kind>(kind: K): Promise<RecordOf[K][]> {
  const db = await openDb();
  const all = await wrap<Stored<K>[]>(db.transaction(kind).objectStore(kind).getAll());
  return all
    .filter((r) => !r.deleted)
    .sort((a, b) => b.updated_at - a.updated_at)
    .map(strip);
}

export async function get<K extends Kind>(kind: K, id: string): Promise<RecordOf[K] | undefined> {
  const db = await openDb();
  const record = await wrap<Stored<K> | undefined>(db.transaction(kind).objectStore(kind).get(id));
  return record && !record.deleted ? strip(record) : undefined;
}

/** Creates or updates a record (a new id when none is given) and schedules a sync. */
export async function put<K extends Kind>(kind: K, draft: Draft<K>): Promise<RecordOf[K]> {
  const db = await openDb();
  const tx = db.transaction(kind, "readwrite");
  const store = tx.objectStore(kind);
  const id = draft.id ?? newId();
  const existing = await wrap<Stored<K> | undefined>(store.get(id));
  // Strictly after the previous edit even if the clock stepped back: the newest edit must win.
  const updated_at = Math.max(Date.now(), (existing?.updated_at ?? 0) + 1);
  const record = { ...draft, id, updated_at, deleted: false, dirty: 1 } as unknown as Stored<K>;
  store.put(record);
  await done(tx);
  emit(kind, true);
  return strip(record);
}

/** Deletes a record: kept as a tombstone until the server has it, so other devices learn of it. */
export async function remove(kind: Kind, id: string): Promise<void> {
  const db = await openDb();
  const tx = db.transaction(kind, "readwrite");
  const store = tx.objectStore(kind);
  const existing = await wrap<Stored<Kind> | undefined>(store.get(id));
  if (!existing) {
    await done(tx);
    return;
  }
  const updated_at = Math.max(Date.now(), existing.updated_at + 1);
  store.put({ id, updated_at, deleted: true, dirty: 1 });
  await done(tx);
  emit(kind, true);
}

/** The service worker's copies of visited pages (`crates/breadator-server/src/sw.js`). */
const PAGES_CACHE = "pages";

/** Forgets the offline copies of pages, which show the account's name. */
export async function forgetPages(): Promise<void> {
  if ("caches" in window) await caches.delete(PAGES_CACHE).catch(() => false);
}

/**
 * Forgets everything on this device (after "remove my data from this device", or another account),
 * including the offline copies of pages.
 */
export async function wipeLocal(): Promise<void> {
  await forgetPages();
  const db = await openDb();
  const tx = db.transaction([...KINDS, META], "readwrite");
  for (const name of [...KINDS, META]) tx.objectStore(name).clear();
  await done(tx);
  for (const kind of KINDS) emit(kind, false);
}

async function markAllDirty(): Promise<void> {
  const db = await openDb();
  const tx = db.transaction(KINDS, "readwrite");
  for (const kind of KINDS) {
    const store = tx.objectStore(kind);
    const cursorRequest = store.openCursor();
    cursorRequest.onsuccess = () => {
      const cursor = cursorRequest.result;
      if (!cursor) return;
      cursor.update({ ...cursor.value, dirty: 1 });
      cursor.continue();
    };
  }
  await done(tx);
}

// ---- synchronization --------------------------------------------------------------------------

export type SyncState = "idle" | "syncing" | "done" | "offline" | "error" | "signed-out";

export interface SyncStatus {
  state: SyncState;
  /** Records the server refused (too large, over the limit): they stay on this device only. */
  refused: number;
  /**
   * Catalog key of why the server refused this device's changes as a whole (the account's storage
   * is full, too many pushes): they stay here and are not sent again until the next change.
   */
  blocked?: string;
}

let status: SyncStatus = { state: "idle", refused: 0 };
const statusListeners = new Set<(s: SyncStatus) => void>();

export function syncStatus(): SyncStatus {
  return status;
}

export function onSyncStatus(listener: (s: SyncStatus) => void): () => void {
  statusListeners.add(listener);
  return () => statusListeners.delete(listener);
}

function setStatus(next: SyncStatus): void {
  status = next;
  for (const listener of statusListeners) listener(status);
}

const BATCH = 200;
/** Bytes of records per push, well under the server's 1 MiB per request. */
const BATCH_BYTES = 512 * 1024;
const MAX_ROUNDS = 50;
/** The server's window for pushes (`PUSH_WINDOW_SECS` in sync.rs). */
const PUSH_WINDOW_MS = 10 * 60_000;

interface Refusal {
  kind: Kind;
  id: string;
  reason: string;
}

interface SyncReply {
  cursor: number;
  more: boolean;
  reset: boolean;
  rejected: Refusal[];
  formulas: Synced[];
  starters: Synced[];
  bakes: Synced[];
  calibrations: Synced[];
}

function isObject(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

function isSynced(value: unknown): value is Synced {
  return (
    isObject(value) &&
    typeof value.id === "string" &&
    typeof value.updated_at === "number" &&
    typeof value.deleted === "boolean"
  );
}

function isRefusal(value: unknown): value is Refusal {
  return isObject(value) && typeof value.kind === "string" && typeof value.id === "string" && typeof value.reason === "string";
}

/** The server's answer, checked: a proxy's error page or a newer protocol must not corrupt the store. */
function parseReply(value: unknown): SyncReply {
  if (!isObject(value) || typeof value.cursor !== "number") throw new Error("unexpected sync reply");
  const records = (key: Kind): Synced[] => (Array.isArray(value[key]) ? value[key].filter(isSynced) : []);
  return {
    cursor: value.cursor,
    more: value.more === true,
    reset: value.reset === true,
    rejected: Array.isArray(value.rejected) ? value.rejected.filter(isRefusal) : [],
    formulas: records("formulas"),
    starters: records("starters"),
    bakes: records("bakes"),
    calibrations: records("calibrations"),
  };
}

class SignedOut extends Error {}

/** The server refused the push as a whole: `key` is the catalog key of the message to show. */
class Refused extends Error {
  constructor(readonly key: string) {
    super(key);
  }
}

/** Push refusals the server words with a code (`{"error": {"code": …}}`), by catalog key. */
const REFUSALS: Record<string, string> = {
  quota_exceeded: "my-sync-quota",
  rate_limited: "my-sync-rate-limited",
};

const encoder = new TextEncoder();

/**
 * Changed records to push: at most `limit` of them and, past the first, at most `maxBytes` of JSON.
 * `full` says the batch was cut (more are waiting).
 */
async function dirtyBatch(
  limit: number,
  maxBytes: number,
): Promise<{ batch: { kind: Kind; record: Stored<Kind> }[]; full: boolean }> {
  const db = await openDb();
  const batch: { kind: Kind; record: Stored<Kind> }[] = [];
  let bytes = 0;
  for (const kind of KINDS) {
    if (batch.length >= limit) return { batch, full: true };
    const rows = await wrap<Stored<Kind>[]>(
      db.transaction(kind).objectStore(kind).index("dirty").getAll(IDBKeyRange.only(1), limit - batch.length),
    );
    for (const record of rows) {
      const size = encoder.encode(JSON.stringify(record)).length;
      if (batch.length > 0 && bytes + size > maxBytes) return { batch, full: true };
      bytes += size;
      batch.push({ kind, record });
    }
  }
  return { batch, full: batch.length >= limit };
}

/** Applies the server's version of a record unless this device has a newer unsent edit. */
async function merge(kind: Kind, incoming: Synced): Promise<boolean> {
  const db = await openDb();
  const tx = db.transaction(kind, "readwrite");
  const store = tx.objectStore(kind);
  const local = await wrap<Stored<Kind> | undefined>(store.get(incoming.id));
  let changed = false;
  if (!local || incoming.updated_at >= local.updated_at) {
    store.put({ ...incoming, dirty: 0 });
    changed = !local || local.updated_at !== incoming.updated_at || local.deleted !== incoming.deleted;
  }
  await done(tx);
  return changed;
}

async function settle(kind: Kind, id: string, pushedAt: number): Promise<void> {
  const db = await openDb();
  const tx = db.transaction(kind, "readwrite");
  const store = tx.objectStore(kind);
  const local = await wrap<Stored<Kind> | undefined>(store.get(id));
  // Only if it was not edited again while the request was in flight.
  if (local && local.updated_at === pushedAt) store.put({ ...local, dirty: 0 });
  await done(tx);
}

async function request(path: string, init?: RequestInit): Promise<Response> {
  const response = await fetch(path, { credentials: "same-origin", ...init });
  if (response.status === 401) throw new SignedOut();
  if (!response.ok) {
    const body: unknown = await response.json().catch(() => undefined);
    const code = isObject(body) && isObject(body.error) ? body.error.code : undefined;
    const key = typeof code === "string" ? REFUSALS[code] : undefined;
    if (key) throw new Refused(key);
    throw new Error(`sync failed: ${response.status}`);
  }
  return response;
}

/** Set when the server refused this device's changes as a whole: until the next local change (or
 * the end of the server's window, for too many pushes), syncs only pull. */
let blocked: string | undefined;

async function syncOnce(): Promise<number> {
  const me: unknown = await (await request("/api/v1/me")).json();
  if (!isObject(me) || typeof me.id !== "string") throw new Error("unexpected /api/v1/me reply");
  const userId = me.id;
  const known = await getMeta<string>("user");
  if (known && known !== userId) {
    // Another account used this browser before: its data is not ours to upload.
    await wipeLocal();
  }
  if (known !== userId) {
    // First sign-in here (or after a wipe): what is on the device joins the account.
    await setMeta("user", userId);
    await setMeta("cursor", 0);
    await markAllDirty();
  }

  let refused = 0;
  for (let round = 0; round < MAX_ROUNDS; round++) {
    const { batch: pending, full } = blocked ? { batch: [], full: false } : await dirtyBatch(BATCH, BATCH_BYTES);
    const cursor = (await getMeta<number>("cursor")) ?? 0;
    const batches: Record<Kind, unknown[]> = { formulas: [], starters: [], bakes: [], calibrations: [] };
    for (const { kind, record } of pending) {
      const { dirty: _dirty, ...wire } = record;
      batches[kind].push(wire);
    }
    const answer: unknown = await (
      await request("/api/v1/sync", {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ since: cursor, ...batches }),
      })
    ).json();
    const reply = parseReply(answer);

    const reasons = new Map(reply.rejected.map((r) => [`${r.kind}/${r.id}`, r.reason]));
    for (const { kind, record } of pending) {
      const reason = reasons.get(`${kind}/${record.id}`);
      // "stale": the server's version is in this reply and merged below. Any other refusal (too
      // large, over the limit, invalid) will never be accepted: stop retrying that record.
      if (reason !== undefined && reason !== "stale") refused++;
      if (reason !== "stale") await settle(kind, record.id, record.updated_at);
    }
    for (const kind of KINDS) {
      let changed = false;
      for (const incoming of reply[kind]) changed = (await merge(kind, incoming)) || changed;
      if (changed) emit(kind, false);
    }
    await setMeta("cursor", reply.reset ? 0 : reply.cursor);
    // A server that went back in time (restored from a backup) gets everything this device holds.
    if (reply.reset) await markAllDirty();
    // Done when the server has nothing more to say and nothing local is waiting.
    if (!reply.more && !reply.reset && !full) break;
  }
  return refused;
}

let running: Promise<void> | undefined;
let rerun = false;
let retryTimer: number | undefined;

/** Synchronizes now; calls made while a sync runs are folded into one more run. */
export function syncNow(): Promise<void> {
  if (running) {
    rerun = true;
    return running;
  }
  running = (async () => {
    do {
      rerun = false;
      if (!navigator.onLine) {
        setStatus({ state: "offline", refused: status.refused });
        return;
      }
      setStatus({ state: "syncing", refused: status.refused, blocked });
      try {
        const refused = await syncOnce();
        window.clearTimeout(retryTimer);
        setStatus({ state: refused > 0 || blocked ? "error" : "done", refused, blocked });
      } catch (error) {
        if (error instanceof SignedOut) {
          setStatus({ state: "signed-out", refused: 0 });
          return;
        }
        window.clearTimeout(retryTimer);
        if (error instanceof Refused) {
          // Sending the same changes again would be refused again: they wait on this device.
          blocked = error.key;
          setStatus({ state: "error", refused: status.refused, blocked });
          if (error.key === REFUSALS.rate_limited) {
            retryTimer = window.setTimeout(() => {
              blocked = undefined;
              void syncNow();
            }, PUSH_WINDOW_MS);
          }
          return;
        }
        console.warn("sync failed", error);
        setStatus({ state: "error", refused: status.refused, blocked });
        retryTimer = window.setTimeout(() => void syncNow(), 30_000);
        return;
      }
    } while (rerun);
  })().finally(() => {
    running = undefined;
  });
  return running;
}

let started = false;

/** Starts syncing for a signed-in page (a no-op for visitors and when called twice). */
export function startSync(): void {
  if (started || !document.body.hasAttribute("data-signed-in")) return;
  started = true;
  void syncNow();
  let timer: number | undefined;
  localListeners.add(() => {
    // A new change may fit where the refused ones did not (a deletion frees room); too many
    // pushes wait for the end of the server's window.
    if (blocked !== REFUSALS.rate_limited) blocked = undefined;
    window.clearTimeout(timer);
    timer = window.setTimeout(() => void syncNow(), 1500);
  });
  window.addEventListener("online", () => void syncNow());
  let last = Date.now();
  const refresh = () => {
    if (document.visibilityState === "visible" && Date.now() - last > 30_000) {
      last = Date.now();
      void syncNow();
    }
  };
  document.addEventListener("visibilitychange", refresh);
  // An open tab picks up what another device saved.
  window.setInterval(refresh, 60_000);
}
