use std::fs;
use std::path::PathBuf;
use regex::RegexSet;

struct Config {
    force: bool, // runs in dry-run otherwise
    protected: RegexSet, // all protected regex patterns
}

/// check if a filename is protected from being renamed, in case an error occurs internally mark the file as protected.
fn is_protected(file: &PathBuf, protections: &RegexSet) -> bool {
    let filename = match file.file_name() {
        Some(x) => x,
        None => return true,
    };

    let filestr = match filename.to_str() {
        Some(x) => x,
        None => return true,
    };

    protections.is_match(filestr)
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
            .replace(' ', "-")
            .replace('_', "-")
            .replace(",-", ",")
            .replace("--", "-")
            .replace("&amp;", "and")
            .replace('&', "and")
            .replace("-(z-lib.org)", ""),
    )
}

fn main() {
    let mut cnf = Config {
        force: false,
        protected: RegexSet::new(&[
            r"^Cargo.*",
            r"^Makefile$",
            r"^\..*",
        ]).unwrap() };

    for arg in std::env::args() {
        if arg.as_str() == "-f" {
            cnf.force = true;
            println!("enabled file renaming")
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

        if is_protected(&old_path, &cnf.protected) {
            println!("skipping: {}", old_path.display());
            continue;
        }

        let new_path = fix_name(old_path.clone());

        if old_path.file_name() != new_path.file_name() && cnf.force {
            match fs::rename(old_path.clone(), new_path.clone()) {
                Ok(_) => println!("{:?} -> {:?}", old_path.display(), new_path.display()),
                Err(e) => println!("failed to rename: {:?} {:?}", old_path.display(), e),
            }
        }
    }
}
