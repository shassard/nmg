use std::fs;
use std::path::PathBuf;

struct Config {
    force: bool,
}

/// check if a filename is protected from being renamed, in case an error occurs internally mark the file as protected.
fn is_protected(file: PathBuf) -> bool {
    let filename = match file.file_name() {
        Some(x) => x,
        None => return true,
    };

    let filestr = match filename.to_str() {
        Some(x) => x,
        None => return true,
    };

    match filestr {
        "Cargo.lock" => true,
        "Cargo.toml" => true,
        "Makefile" => true,
        _ => false,
    }
}

/// returns a PathBuf with a cleaned up filename, or the original PathBuf if a failure occurs
fn fix_name(file: PathBuf) -> PathBuf {
    let filename = match file.file_name() {
        Some(x) => x,
        None => return file,
    };

    let fname = match filename.to_str() {
        Some(x) => x,
        None => return file,
    };

    PathBuf::from(
        fname
            .to_lowercase()
            .replace(" ", "-")
            .replace("_", "-")
            .replace("--", "-")
            .replace("&amp;", "and")
            .replace("&", "and"),
    )
}

fn main() {
    let mut cnf = Config { force: false };
    for (k, v) in std::env::vars() {
        if k.as_str() == "FORCE" && v.as_str() == "1" {
            cnf.force = true;
        }
    }

    let paths = match fs::read_dir(".") {
        Ok(x) => x,
        Err(e) => panic!("Failed to read dir: {:?}", e),
    };
    for path in paths {
        let old_path = match path {
            Ok(x) => x.path(),
            Err(e) => {
                println!("bad path: {:?}", e);
                continue;
            }
        };

        if is_protected(old_path.clone()) {
            println!(
                "skipping: {}",
                old_path.file_name().unwrap().to_string_lossy()
            );
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
