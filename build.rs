use std::process::Command;

fn main() {
    Command::new("cargo")
        .args(["install", "xtr"])
        .status()
        .unwrap();
    Command::new("xtr").args(["src/main.rs"]).status().unwrap();
    Command::new("msgmerge")
        .args([
            "--update",
            "--no-fuzzy-matching",
            "--no-wrap",
            "messages-de.po",
            "messages.po",
        ])
        .status()
        .unwrap();
    Command::new("msgfmt")
        .args(["--check-format", "messages-de.po"])
        .status()
        .unwrap();
    Command::new("mkdir")
        .args(["--parents", "locale/de/LC_MESSAGES"])
        .status()
        .unwrap();
    Command::new("cp")
        .args(["messages.mo", "locale/de/LC_MESSAGES/hellorust.mo"])
        .status()
        .unwrap();
}
