// Fetches YouTube metadata + caption transcripts for a list of videos.
//
//   node fetch-youtube.mjs <list> <outDir> [concurrency]
//
// <list>: one video per line, "id|title". Writes <outDir>/<id>.json (metadata, description,
// segments) and <outDir>/<id>.md (readable transcript). Already-fetched ids are skipped, so the
// run can be resumed. Failures go to <outDir>/_failed.txt.
//
// Why a browser: YouTube's timedtext endpoint answers 429 to yt-dlp and an empty body to any
// request without the player's PO token. The player's own caption request carries that token,
// so the script loads each watch page in a real browser (see below), lets the player fetch
// captions, and keeps the response.

import puppeteer from "puppeteer-core";
import fs from "node:fs/promises";
import path from "node:path";

const [listPath, outDir, concArg] = process.argv.slice(2);
if (!listPath || !outDir) {
  console.error("usage: node fetch-youtube.mjs <list> <outDir> [concurrency]");
  process.exit(2);
}
const concurrency = Number(concArg || 3);

const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const videos = (await fs.readFile(listPath, "utf8"))
  .split(/\r?\n/)
  .filter((l) => l.trim())
  .map((l) => {
    const [id, ...t] = l.split("|");
    return { id: id.trim(), title: t.join("|").trim() };
  });
await fs.mkdir(outDir, { recursive: true });
const done = new Set(
  (await fs.readdir(outDir)).filter((f) => f.endsWith(".json")).map((f) => f.slice(0, -5)),
);
const todo = videos.filter((v) => !done.has(v.id));
console.log(`${videos.length} listed, ${done.size} already fetched, ${todo.length} to fetch`);

// Default: the user's Brave over CDP (127.0.0.1:9222), inside a fresh incognito browser context
// (no cookies, no account, no watch history, nothing persisted). Puppeteer-launched Chrome does
// not work: headless gets caption responses with empty bodies (PO token refused) and an
// off-screen headful window never requests captions. The context window is visible while the
// script runs; teardown closes only that context and disconnects, never the browser.
const CDP_URL = process.env.CDP_URL || "http://127.0.0.1:9222";
const browser = await puppeteer.connect({ browserURL: CDP_URL, defaultViewport: null });
const context = await browser.createBrowserContext();

function pickTrack(tracks) {
  const en = (t) => /^en\b/.test(t.lang);
  return (
    tracks.find((t) => en(t) && t.kind !== "asr") ||
    tracks.find((t) => en(t) && t.kind === "asr") ||
    tracks.find((t) => t.kind === "asr") ||
    tracks[0] ||
    null
  );
}

function fmtTime(s) {
  s = Math.floor(s);
  const h = Math.floor(s / 3600);
  const m = Math.floor((s % 3600) / 60);
  const sec = String(s % 60).padStart(2, "0");
  return h ? `${h}:${String(m).padStart(2, "0")}:${sec}` : `${m}:${sec}`;
}

function toMarkdown(rec) {
  const lines = [
    `# ${rec.title}`,
    "",
    `- Channel: ${rec.channel}`,
    `- Video: https://www.youtube.com/watch?v=${rec.id}`,
    `- Published: ${rec.publishDate ?? "?"} · Duration: ${fmtTime(rec.lengthSeconds || 0)}`,
    `- Captions: ${rec.caption ? `${rec.caption.lang} (${rec.caption.kind || "manual"})` : "none"}`,
    "",
    "## Description",
    "",
    rec.description || "(empty)",
    "",
    "## Transcript",
    "",
  ];
  // One paragraph per ~30 s, prefixed with its start time, so notes can cite [mm:ss].
  let para = [];
  let start = null;
  for (const s of rec.segments) {
    if (start === null) start = s.t;
    para.push(s.text);
    if (s.t - start >= 30) {
      lines.push(`[${fmtTime(start)}] ${para.join(" ")}`, "");
      para = [];
      start = null;
    }
  }
  if (para.length) lines.push(`[${fmtTime(start)}] ${para.join(" ")}`, "");
  if (!rec.segments.length) lines.push("(no transcript)");
  return lines.join("\n");
}

async function fetchOne(page, id) {
  // Every caption response the player receives, keyed "lang:kind" (translations skipped).
  const got = new Map();
  const onResponse = async (r) => {
    const u = r.url();
    if (!u.includes("/api/timedtext")) return;
    const q = new URL(u).searchParams;
    if (q.get("tlang")) return;
    const key = `${q.get("lang")}:${q.get("kind") || ""}`;
    try {
      const body = await r.text();
      if (process.env.DEBUG) console.log(`  timedtext ${r.status()} ${key} len=${body.length}`);
      if (body.length > 50 && !got.has(key)) got.set(key, body);
    } catch (e) {
      // CORS preflights carry no body; the real request follows.
      if (process.env.DEBUG) console.log(`  timedtext ${key} no body: ${e.message}`);
    }
  };
  page.on("response", onResponse);
  try {
    // A French UI makes the player fetch a caption track by itself; with an English UI it
    // fetches nothing. Once it has, the player is asked for the wanted track (below): its
    // request carries a valid token, whereas an edited caption URL comes back empty.
    await page.goto(`https://www.youtube.com/watch?v=${id}&hl=fr`, {
      waitUntil: "domcontentloaded",
      timeout: 45000,
    });
    if (page.url().includes("consent.")) {
      await page.evaluate(() => {
        const b = [...document.querySelectorAll("button")].find((b) =>
          /reject all|tout refuser/i.test(b.innerText),
        );
        b?.click();
      });
      await page.waitForNavigation({ timeout: 20000 }).catch(() => {});
    }
    await page.waitForFunction(() => window.ytInitialPlayerResponse?.videoDetails, {
      timeout: 25000,
    });
    const meta = await page.evaluate(() => {
      const pr = window.ytInitialPlayerResponse;
      const vd = pr.videoDetails;
      const mf = pr.microformat?.playerMicroformatRenderer;
      const tracks = (pr.captions?.playerCaptionsTracklistRenderer?.captionTracks || []).map(
        (t) => ({ lang: t.languageCode, kind: t.kind || "", baseUrl: t.baseUrl }),
      );
      return {
        title: vd.title,
        channel: vd.author,
        lengthSeconds: Number(vd.lengthSeconds),
        description: vd.shortDescription || "",
        keywords: vd.keywords || [],
        publishDate: mf?.publishDate || null,
        tracks,
      };
    });
    const want = pickTrack(meta.tracks);
    let caption = null;
    let segments = [];
    if (want) {
      const wantKey = `${want.lang}:${want.kind}`;
      await page
        .waitForFunction(() => document.getElementById("movie_player")?.setOption, { timeout: 20000 })
        .catch(() => {});
      for (let i = 0; i < 40 && !got.has(wantKey); i++) {
        // Ask for the wanted track once the player has loaded its first one, and again later
        // in case the first request was dropped.
        if ((got.size > 0 && i % 8 === 0) || i === 20) {
          await page
            .evaluate((t) => {
              const p = document.getElementById("movie_player");
              p?.loadModule?.("captions");
              p?.setOption?.("captions", "track", t.kind ? { languageCode: t.lang, kind: t.kind } : { languageCode: t.lang });
            }, want)
            .catch(() => {});
        }
        if (i === 30) await page.click(".ytp-subtitles-button").catch(() => {});
        await sleep(500);
      }
      const body = got.get(wantKey);
      if (!body) throw new Error(`no ${wantKey} caption response (got: ${[...got.keys()].join(",") || "none"})`);
      const j = JSON.parse(body);
      segments = (j.events || [])
        .filter((e) => e.segs)
        .map((e) => ({
          t: (e.tStartMs || 0) / 1000,
          text: e.segs
            .map((s) => s.utf8)
            .join("")
            .replace(/\s+/g, " ")
            .trim(),
        }))
        .filter((s) => s.text);
      caption = { lang: want.lang, kind: want.kind };
    }
    await page.evaluate(() => document.querySelector("video")?.pause()).catch(() => {});
    const { tracks, ...rest } = meta;
    return { id, ...rest, availableCaptions: tracks.map((t) => `${t.lang}:${t.kind}`), caption, segments };
  } finally {
    page.off("response", onResponse);
  }
}

let ok = 0;
let failed = 0;
const queue = [...todo];
async function worker(n) {
  const page = await context.newPage();
  await page.setExtraHTTPHeaders({ "Accept-Language": "fr-FR,fr;q=0.9" });
  // Lowest stream quality: the video must load for the player to fetch captions, but only a
  // few seconds of it are needed.
  await page.evaluateOnNewDocument(() => {
    try {
      const now = Date.now();
      localStorage.setItem(
        "yt-player-quality",
        JSON.stringify({ data: JSON.stringify({ quality: 144, previousQuality: 144 }), expiration: now + 864e5, creation: now }),
      );
    } catch {}
  });
  while (queue.length) {
    const v = queue.shift();
    let rec = null;
    for (let attempt = 1; attempt <= 2 && !rec; attempt++) {
      try {
        rec = await fetchOne(page, v.id);
      } catch (e) {
        if (attempt === 2) {
          failed++;
          await fs.appendFile(path.join(outDir, "_failed.txt"), `${v.id}|${v.title}|${e.message}\n`);
          console.log(`FAIL ${v.id} ${e.message}`);
        } else await sleep(3000);
      }
    }
    if (!rec) continue;
    await fs.writeFile(path.join(outDir, `${v.id}.json`), JSON.stringify(rec, null, 1));
    await fs.writeFile(path.join(outDir, `${v.id}.md`), toMarkdown(rec));
    ok++;
    console.log(
      `[w${n}] ${ok + failed}/${todo.length} ${v.id} ${rec.caption ? rec.caption.lang : "no-captions"} ${rec.segments.length} segs`,
    );
    await sleep(800 + Math.random() * 1200);
  }
  await page.close();
}

try {
  await Promise.all(Array.from({ length: concurrency }, (_, i) => worker(i + 1)));
} finally {
  await context.close().catch(() => {});
  await browser.disconnect();
}
console.log(`done: ${ok} ok, ${failed} failed`);
