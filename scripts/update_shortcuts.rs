use std::fs;
use std::path::Path;

fn replace_shortcuts(content: &str) -> String {
    let mut s = content.to_string();

    // 1. Protect existing Ctrl/Cmd references
    s = s.replace("Ctrl/Cmd", "\u{E001}");
    s = s.replace("Ctrl / Cmd", "\u{E001}");
    s = s.replace("Ctrl/CMD", "\u{E001}");

    // 2. HTML / Markdown tags:
    s = s.replace("<kbd>Ctrl</kbd>", "<kbd>Ctrl/Cmd</kbd>");
    s = s.replace("<kbd>Ctrl +", "<kbd>Ctrl/Cmd +");
    s = s.replace("<kbd>Ctrl+", "<kbd>Ctrl/Cmd+");

    // 3. Common inline markdown and text patterns:
    s = s.replace("`Ctrl+", "`Ctrl/Cmd+");
    s = s.replace("`Ctrl +", "`Ctrl/Cmd +");
    s = s.replace("(Ctrl+", "(Ctrl/Cmd+");
    s = s.replace("(Ctrl +", "(Ctrl/Cmd +");
    s = s.replace("/ Ctrl+", "/ Ctrl/Cmd+");
    s = s.replace("/ Ctrl +", "/ Ctrl/Cmd +");

    // 4. Any remaining "Ctrl +" and "Ctrl+"
    s = s.replace("Ctrl + ", "Ctrl/Cmd + ");
    s = s.replace("Ctrl+", "Ctrl/Cmd+");

    // 5. In quotes or tables
    s = s.replace("\"Ctrl + ", "\"Ctrl/Cmd + ");
    s = s.replace("\"Ctrl+", "\"Ctrl/Cmd+");
    s = s.replace(" Ctrl + ", " Ctrl/Cmd + ");
    s = s.replace(" Ctrl+ ", " Ctrl/Cmd+ ");

    // 6. Restore protected placeholder
    s = s.replace("\u{E001}", "Ctrl/Cmd");

    s
}

fn process_file(path: &Path) {
    if let Ok(content) = fs::read_to_string(path) {
        let updated = replace_shortcuts(&content);
        if updated != content {
            fs::write(path, updated).expect("Write updated file");
            println!("Updated: {:?}", path);
        }
    }
}

fn process_dir(dir: &Path, extension: &str) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let p = entry.path();
            if p.is_file() {
                if let Some(ext) = p.extension() {
                    if ext == extension {
                        process_file(&p);
                    }
                }
            }
        }
    }
}

fn main() {
    // 1. Root files
    process_file(Path::new("README.md"));
    process_file(Path::new("index.html"));

    // 2. docs/*.md and docs/*.html
    process_dir(Path::new("docs"), "md");
    process_dir(Path::new("docs"), "html");

    // 3. assets/locales/*.json
    process_dir(Path::new("assets/locales"), "json");

    // 4. src/ui/modals.rs
    process_file(Path::new("src/ui/modals.rs"));

    // 5. Any other src files
    process_file(Path::new("src/ui/top_bar.rs"));
    process_file(Path::new("src/i18n.rs"));

    println!("All shortcut references synchronized to Ctrl/Cmd!");
}
