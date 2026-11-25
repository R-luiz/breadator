"""Merges the per-source research files into the files the app compiles in.

    py research/tools/merge_corpus.py            # writes research/data/{bibliography.json,claims.jsonl,bib-keys.json}
    py research/tools/merge_corpus.py --check    # fails if the merged files are out of date

Inputs (research/CONVENTIONS.md): research/data/references-<source>.json and
research/data/claims-<source>.jsonl. Each source agent used its own reference keys; this script
deduplicates references (DOI first, then normalised title), gives each a canonical key
(`<first-author-surname><year>[a-z]`, e.g. `zeng2023`), and rewrites every claim's `cites` to
canonical keys. A claim citing `doi:<doi>` that no references file lists gets a minimal entry from
the DOI alone, so no citation is lost. Problems (unparseable lines, duplicate claim ids,
citations that resolve to nothing) are printed and make the script exit 1.

Keys are permanent: articles cite them. `research/data/bib-keys.json` records every key ever
given with the paper's identity (DOI and normalised title). A paper found again keeps its key; a
new paper gets a key never used before, so adding references can never re-point a citation.
"""

from __future__ import annotations

import json
import re
import sys
import time
import unicodedata
import urllib.parse
import urllib.request
from pathlib import Path

DATA = Path(__file__).resolve().parent.parent / "data"
FIELDS = ("key", "authors", "year", "title", "venue", "doi", "url", "oa_url", "license")
# Crossref answers per DOI, committed so the merge is reproducible offline.
CROSSREF_CACHE = DATA / "crossref-cache.json"
# Every canonical key ever given, with the identity of its paper (see the module docstring).
KEY_REGISTRY = DATA / "bib-keys.json"


def norm_doi(doi: str | None) -> str | None:
    if not doi:
        return None
    doi = doi.strip().lower()
    doi = re.sub(r"^(https?://(dx\.)?doi\.org/|doi:)", "", doi)
    return doi or None


def ascii_fold(s: str) -> str:
    return unicodedata.normalize("NFKD", s).encode("ascii", "ignore").decode()


def norm_title(title: str | None) -> str:
    return re.sub(r"[^a-z0-9]+", " ", ascii_fold(title or "").lower()).strip()


def surname(authors: list[str]) -> str:
    if not authors:
        return "anon"
    # "Zeng, F." → zeng ; "Fang Zeng" → zeng ; "Di Cagno, Raffaella et al." → dicagno ;
    # "Smith et al." → smith
    first = re.sub(r"\s*\bet al\.?", "", authors[0]).strip()
    name = first.split(",")[0] if "," in first else (first.split() or [""])[-1]
    return re.sub(r"[^a-z]", "", ascii_fold(name).lower()) or "anon"


def crossref(doi: str, cache: dict) -> dict | None:
    """Authors, year, title and venue of a DOI from Crossref (cached; None when unknown)."""
    if doi in cache:
        return cache[doi]
    url = f"https://api.crossref.org/works/{urllib.parse.quote(doi, safe='/')}"
    req = urllib.request.Request(url, headers={"User-Agent": "breadator-research (mailto:contact@robinluiz.fr)"})
    try:
        with urllib.request.urlopen(req, timeout=20) as r:
            m = json.load(r)["message"]
    except Exception:  # noqa: BLE001 — network or 404: leave the entry as it is
        cache[doi] = None
        return None
    parts = (m.get("issued") or m.get("published") or {}).get("date-parts") or [[None]]
    found = {
        "authors": [f"{a.get('family', '')}, {a.get('given', '')}".strip(", ") for a in m.get("author", []) if a.get("family")],
        "year": parts[0][0],
        "title": (m.get("title") or [None])[0],
        "venue": (m.get("container-title") or [None])[0],
    }
    cache[doi] = found
    time.sleep(0.1)  # polite pool
    return found


def year_of(ref: dict) -> str:
    y = ref.get("year")
    return str(y) if isinstance(y, int) or (isinstance(y, str) and y.isdigit()) else "nd"


def load_registry() -> dict[str, dict]:
    """key -> {"doi", "title"}. Seeded from the current bibliography the first time, so the keys
    articles already cite keep their meaning."""
    if KEY_REGISTRY.exists():
        return json.loads(KEY_REGISTRY.read_text(encoding="utf-8"))
    bib = DATA / "bibliography.json"
    if not bib.exists():
        return {}
    return {
        e["key"]: {"doi": norm_doi(e.get("doi")), "title": norm_title(e.get("title") or "")}
        for e in json.loads(bib.read_text(encoding="utf-8"))
    }


def candidate_keys(base: str):
    """base, base+a … base+z, base+aa …: the first one not in the registry is the new key."""
    yield base
    letters = "abcdefghijklmnopqrstuvwxyz"
    yield from (base + c for c in letters)
    yield from (base + a + b for a in letters for b in letters)


# Sources distilled from private material (git-ignored): used locally, never merged into the
# published files. Skipping them also keeps `--check` identical on a fresh clone (CI).
PRIVATE_SOURCES = {"project"}


def load_sources() -> tuple[list[tuple[str, dict]], list[tuple[str, dict]], list[str]]:
    problems: list[str] = []
    refs: list[tuple[str, dict]] = []
    for path in sorted(DATA.glob("references-*.json")):
        source = path.stem.removeprefix("references-")
        if source in PRIVATE_SOURCES:
            continue
        try:
            data = json.loads(path.read_text(encoding="utf-8"))
        except json.JSONDecodeError as e:
            problems.append(f"{path.name}: {e}")
            continue
        if isinstance(data, dict):
            data = list(data.values())
        refs.extend((source, r) for r in data if isinstance(r, dict))
    claims: list[tuple[str, dict]] = []
    for path in sorted(DATA.glob("claims-*.jsonl")):
        source = path.stem.removeprefix("claims-")
        if source in PRIVATE_SOURCES:
            continue
        for n, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
            if not line.strip():
                continue
            try:
                claims.append((source, json.loads(line)))
            except json.JSONDecodeError as e:
                problems.append(f"{path.name}:{n}: {e}")
    return refs, claims, problems


def merge() -> tuple[list[dict], list[dict], list[str], dict[str, dict]]:
    refs, claims, problems = load_sources()

    # 1. Group references: same DOI, else same normalised title (+ year when both have one).
    groups: list[dict] = []
    by_doi: dict[str, dict] = {}
    by_title: dict[str, dict] = {}
    alias: dict[tuple[str, str], dict] = {}  # (source, local key) → group
    for source, r in refs:
        doi = norm_doi(r.get("doi"))
        title = norm_title(r.get("title"))
        group = (doi and by_doi.get(doi)) or (title and by_title.get(title))
        if group is None:
            group = {"members": [], "doi": doi, "title": title}
            groups.append(group)
        group["members"].append(r)
        if doi:
            by_doi.setdefault(doi, group)
            group["doi"] = group["doi"] or doi
        if title:
            by_title.setdefault(title, group)
        if r.get("key"):
            alias[(source, re.sub(r"^(bib|ref):", "", str(r["key"])))] = group

    # 2. Claims citing a DOI no references file lists: a minimal entry from the DOI.
    for source, c in claims:
        for cite in c.get("cites") or []:
            doi = norm_doi(cite) if str(cite).lower().startswith(("doi:", "10.", "https://doi.org")) else None
            if doi and doi not in by_doi:
                group = {"members": [{"doi": doi, "title": f"doi:{doi}"}], "doi": doi, "title": ""}
                groups.append(group)
                by_doi[doi] = group

    # 3. One merged entry per group; fields from the most complete member, gaps filled from
    #    Crossref when the entry has a DOI (authors, year, title, venue).
    cache = json.loads(CROSSREF_CACHE.read_text(encoding="utf-8")) if CROSSREF_CACHE.exists() else {}
    merged: list[dict] = []
    for g in groups:
        best = max(g["members"], key=lambda m: sum(1 for f in FIELDS if m.get(f)))
        entry = {f: best.get(f) for f in FIELDS if f != "key"}
        for m in g["members"]:
            for f in FIELDS:
                if f != "key" and not entry.get(f) and m.get(f):
                    entry[f] = m[f]
        entry["doi"] = g["doi"]
        entry["authors"] = entry.get("authors") or []
        if isinstance(entry["authors"], str):
            entry["authors"] = [entry["authors"]]
        if entry["title"] and entry["title"].startswith("doi:"):
            entry["title"] = None
        if g["doi"] and not (entry["authors"] and entry.get("year") and entry.get("title")):
            found = crossref(g["doi"], cache) or {}
            for f in ("authors", "year", "title", "venue"):
                if not entry.get(f) and found.get(f):
                    entry[f] = found[f]
        y = entry.get("year")
        entry["year"] = int(y) if isinstance(y, (int, str)) and str(y).isdigit() else None
        entry["title"] = entry.get("title") or (f"doi:{g['doi']}" if g["doi"] else "(untitled)")
        g["entry"] = entry
        merged.append(entry)
    CROSSREF_CACHE.write_text(json.dumps(cache, ensure_ascii=False, indent=1, sort_keys=True) + "\n", encoding="utf-8")

    # 4. Canonical keys, permanent (see the module docstring).
    registry = load_registry()
    key_by_doi = {v["doi"]: k for k, v in registry.items() if v.get("doi")}
    key_by_title = {v["title"]: k for k, v in registry.items() if v.get("title")}
    merged.sort(key=lambda e: (surname(e["authors"]), year_of(e), norm_title(e["title"])))
    assigned: set[str] = set()
    for e in merged:
        key = (e["doi"] and key_by_doi.get(e["doi"])) or key_by_title.get(norm_title(e["title"]))
        if key is None or key in assigned:
            base = f"{surname(e['authors'])}{year_of(e)}"
            key = next(k for k in candidate_keys(base) if k not in registry)
            registry[key] = {"doi": e["doi"], "title": norm_title(e["title"])}
        assigned.add(key)
        e["key"] = key

    # 5. Claims with canonical citations.
    out_claims: list[dict] = []
    seen_ids: set[str] = set()
    for source, c in claims:
        cid = c.get("id")
        if not cid or cid in seen_ids:
            problems.append(f"claims-{source}: missing or duplicate claim id {cid!r}")
            continue
        seen_ids.add(cid)
        cites = []
        for cite in c.get("cites") or []:
            # Some sources prefix their own keys (`bib:<key>`, `ref:<key>`); DOIs keep `doi:`.
            cite = re.sub(r"^(bib|ref):", "", str(cite))
            group = alias.get((source, cite))
            if group is None:
                doi = norm_doi(cite) if cite.lower().startswith(("doi:", "10.", "https://doi.org")) else None
                group = by_doi.get(doi) if doi else None
            if group is None:
                # Another source's key, or a key equal to a canonical key.
                group = next((g for (s, k), g in alias.items() if k == cite), None)
            if group is None:
                problems.append(f"claim {cid}: citation {cite!r} matches no reference")
                continue
            key = group["entry"]["key"]
            if key not in cites:
                cites.append(key)
        out_claims.append({**c, "cites": cites})
    out_claims.sort(key=lambda c: c["id"])
    return merged, out_claims, problems, registry


def main() -> int:
    check = "--check" in sys.argv[1:]
    # Reports problems and counts without writing the merged files (two people editing sources
    # at once can each validate their own without racing on the outputs).
    dry_run = "--dry-run" in sys.argv[1:]
    bibliography, claims, problems, registry = merge()
    bib_text = json.dumps(bibliography, ensure_ascii=False, indent=1) + "\n"
    claims_text = "".join(json.dumps(c, ensure_ascii=False) + "\n" for c in claims)
    registry_text = json.dumps(dict(sorted(registry.items())), ensure_ascii=False, indent=1) + "\n"
    targets = {
        DATA / "bibliography.json": bib_text,
        DATA / "claims.jsonl": claims_text,
        KEY_REGISTRY: registry_text,
    }
    if check:
        stale = [p.name for p, t in targets.items() if not p.exists() or p.read_text(encoding="utf-8") != t]
        if stale:
            problems.append(f"out of date: {', '.join(stale)} (run merge_corpus.py)")
    elif not dry_run:
        for path, text in targets.items():
            path.write_text(text, encoding="utf-8", newline="\n")
    for p in problems:
        print(p, file=sys.stderr)
    print(f"{len(bibliography)} references, {len(claims)} claims, {len(problems)} problems")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
