// `include_dir!` (static files, SQL migrations, content) and the Fluent loader (locales) embed files
// at compile time but do not tell cargo about them on stable Rust: without this, editing a
// stylesheet, a translation, a migration or an article would not rebuild the binary.
fn main() {
    for path in [
        "static",
        "locales",
        "templates",
        "../../migrations",
        "../../content",
        "../../research/data/bibliography.json",
        "../../research/data/claims.jsonl",
        "../../research/data/videos.json",
    ] {
        println!("cargo:rerun-if-changed={path}");
    }
}
