use std::fs;
use std::path::PathBuf;

/// check if a filename is blacklisted from being renamed
fn blacklisted(file: std::path::PathBuf) -> bool {
    match file.file_name().unwrap().to_str().unwrap() {
        "Cargo.lock" => true,
        "Cargo.toml" => true,
        "Makefile" => true,
        _ => false,
    }
}

/// returns a cleaned up filename
fn fix_name(file: std::path::PathBuf) -> std::path::PathBuf {
    PathBuf::from(file.file_name().unwrap().to_str().unwrap().to_lowercase()
        .replace(" ", "-")
        .replace("_", "-")
        .replace("--", "-"))
}

fn main() {
    let mut force = false;
    for (k, v) in std::env::vars() {
        if k == String::from("FORCE") && v == String::from("1") {
            force = true;
        }
    }

    let paths = fs::read_dir(".").unwrap();
    'outer: for path in paths {
        let old_path = path.unwrap().path();
        if blacklisted(old_path.clone()) {
            println!("skipping: {}", old_path.file_name().unwrap().to_string_lossy());
            continue 'outer;
        }
        let new_path = fix_name(old_path.clone());
        if old_path.file_name() != new_path.file_name() {
            println!("{} -> {}", old_path.display(), new_path.display());
            if force {
                fs::rename(old_path, new_path).unwrap();
            }
        }
    }
}
