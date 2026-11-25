//! Validates the repository's real content: every article, recipe and citation in `content/`
//! against the merged research files. Run after editing content:
//! `cargo test -p breadator-content --test repo_content`.

use std::fs;
use std::path::{Path, PathBuf};

use breadator_content::{Library, SourceFile};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn collect(dir: &Path, root: &Path, out: &mut Vec<(String, String)>) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect(&path, root, out);
        } else if let Ok(text) = fs::read_to_string(&path) {
            let rel = path.strip_prefix(root).unwrap().to_string_lossy().replace('\\', "/");
            out.push((rel, text));
        }
    }
}

#[test]
fn repository_content_is_valid() {
    let root = repo_root();
    let mut files = Vec::new();
    collect(&root.join("content"), &root, &mut files);
    for name in ["bibliography.json", "claims.jsonl", "videos.json"] {
        let path = root.join("research/data").join(name);
        let text = fs::read_to_string(&path).unwrap_or_else(|e| {
            panic!("{}: {e} (run research/tools/merge_corpus.py)", path.display())
        });
        files.push((format!("research/data/{name}"), text));
    }
    let lib = Library::load(files.iter().map(|(path, text)| SourceFile { path, text }));
    if let Err(errors) = lib {
        let list: Vec<String> = errors.iter().map(ToString::to_string).collect();
        panic!("{} content problems:\n{}", list.len(), list.join("\n"));
    }
}
