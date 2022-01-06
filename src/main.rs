use std::fs;
use std::path::PathBuf;

struct Config {
    force: bool,
}

/// check if a filename is protected from being renamed
fn isprotected(file: PathBuf) -> bool {
    match file.file_name().unwrap().to_str().unwrap() {
        "Cargo.lock" => true,
        "Cargo.toml" => true,
        "Makefile" => true,
        _ => false,
    }
}

/// returns a cleaned up filename
fn fix_name(file: PathBuf) -> PathBuf {
    PathBuf::from(file.file_name().unwrap().to_str().unwrap().to_lowercase()
        .replace(" ", "-")
        .replace("_", "-")
        .replace("--", "-"))
}

fn main() {
    let mut cnf = Config {force: false};
    for (k, v) in std::env::vars() {
        if k.as_str() == "FORCE" && v.as_str() == "1" {
            cnf.force = true;
        }
    }

    let paths = fs::read_dir(".").unwrap();
    for path in paths {
        let old_path = path.unwrap().path();

        if isprotected(old_path.clone()) {
            println!("skipping: {}", old_path.file_name().unwrap().to_string_lossy());
            continue;
        }

        let new_path = fix_name(old_path.clone());

        if old_path.file_name() != new_path.file_name() {
            println!("{} -> {}", old_path.display(), new_path.display());
            if cnf.force {
                fs::rename(old_path, new_path).unwrap();
            }
        }
    }
}
