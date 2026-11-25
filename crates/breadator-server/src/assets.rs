//! Static files embedded in the binary, served under content-hashed names with immutable caching.
//!
//! `static/app.css` is served as `/static/app.<hash>.css`; templates ask [`Assets::url`] for the
//! current name, so a deploy can never serve stale CSS/JS/WASM from a cache. The islands'
//! translation catalogs are served the same way (`i18n/fr.json`), generated from `locales/`.

use std::collections::HashMap;

use include_dir::{Dir, File, include_dir};
use sha2::{Digest, Sha256};

use crate::i18n::Lang;

static STATIC_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/static");

pub struct Asset {
    pub bytes: &'static [u8],
    pub mime: &'static str,
}

pub struct Assets {
    /// "/static/app.1a2b3c4d.css" → file
    by_url: HashMap<String, Asset>,
    /// "app.css" → "/static/app.1a2b3c4d.css"
    url_of: HashMap<String, String>,
    /// Hash of every asset: changes whenever any asset changes (service worker cache name).
    pub version: String,
}

fn walk(dir: &'static Dir<'static>, out: &mut Vec<&'static File<'static>>) {
    out.extend(dir.files());
    for d in dir.dirs() {
        walk(d, out);
    }
}

fn mime(path: &str) -> &'static str {
    match path.rsplit('.').next().unwrap_or("") {
        "css" => "text/css; charset=utf-8",
        "js" | "mjs" => "text/javascript; charset=utf-8",
        "wasm" => "application/wasm",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "webp" => "image/webp",
        "ico" => "image/x-icon",
        "woff2" => "font/woff2",
        "json" => "application/json",
        "webmanifest" => "application/manifest+json",
        "txt" => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

impl Assets {
    pub fn load() -> Self {
        let mut files = Vec::new();
        walk(&STATIC_DIR, &mut files);
        files.sort_by_key(|f| f.path());
        let mut entries: Vec<(String, &'static [u8])> = files
            .into_iter()
            .map(|f| (f.path().to_string_lossy().replace('\\', "/"), f.contents()))
            .collect();
        entries.extend(
            Lang::ALL
                .iter()
                .map(|l| (format!("i18n/{}.json", l.code()), l.catalog_json().as_bytes())),
        );
        let mut by_url = HashMap::new();
        let mut url_of = HashMap::new();
        let mut all = Sha256::new();
        for (logical, bytes) in entries {
            let digest = Sha256::digest(bytes);
            all.update(digest);
            let hash = &hex(&digest)[..10];
            let url = match logical.rsplit_once('.') {
                Some((stem, ext)) if !stem.ends_with('/') => format!("/static/{stem}.{hash}.{ext}"),
                _ => format!("/static/{logical}.{hash}"),
            };
            by_url.insert(url.clone(), Asset { bytes, mime: mime(&logical) });
            url_of.insert(logical, url);
        }
        Self { by_url, url_of, version: hex(&all.finalize())[..12].to_owned() }
    }

    /// Hashed URL of `static/<logical>`; `None` if it is not embedded (e.g. front end not built).
    pub fn url(&self, logical: &str) -> Option<&str> {
        self.url_of.get(logical).map(String::as_str)
    }

    pub fn get(&self, url: &str) -> Option<&Asset> {
        self.by_url.get(url)
    }

    /// Every hashed URL, for the service worker's precache list.
    pub fn urls(&self) -> impl Iterator<Item = &str> {
        self.by_url.keys().map(String::as_str)
    }
}
