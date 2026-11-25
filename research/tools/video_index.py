"""Writes research/data/videos.json: id → {title, channel, published} for every fetched video.

    py research/tools/video_index.py

Run locally after fetching transcripts (research/raw/ is not in the repository). The site uses it
to show a video's title next to a citation of one of its claims. Titles, channel names and dates
are public metadata.
"""

import json
from pathlib import Path

RESEARCH = Path(__file__).resolve().parent.parent


def main() -> None:
    videos: dict[str, dict] = {}
    for meta in sorted((RESEARCH / "raw").rglob("*.json")):
        try:
            data = json.loads(meta.read_text(encoding="utf-8"))
        except (json.JSONDecodeError, UnicodeDecodeError):
            continue
        # fetch-youtube.mjs output: metadata + caption segments.
        if not isinstance(data, dict) or not {"id", "title", "segments"} <= data.keys():
            continue
        videos[data["id"]] = {
            "title": data["title"],
            "channel": data.get("channel"),
            "published": data.get("publishDate"),
        }
    out = RESEARCH / "data" / "videos.json"
    out.write_text(json.dumps(dict(sorted(videos.items())), ensure_ascii=False, indent=1) + "\n", encoding="utf-8", newline="\n")
    print(f"{len(videos)} videos → {out}")


if __name__ == "__main__":
    main()
