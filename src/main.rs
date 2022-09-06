use regex::RegexSet;
use std::fs;
use std::path::PathBuf;

struct SkipList {
    pub protected_patterns: RegexSet,
}

impl SkipList {
    /// check if a filename is protected from being renamed, in case an error occurs internally mark the file as protected.
    fn is_path_protected(&self, path: PathBuf) -> bool {
        let filename = match path.file_name() {
            Some(x) => x,
            None => return true,
        };

        let filename_str = match filename.to_str() {
            Some(x) => x,
            None => return true,
        };

        self.protected_patterns.is_match(filename_str)
    }
}

/// returns a PathBuf with a cleaned up filename, or the original PathBuf if a failure occurs
fn fix_name(path: PathBuf) -> PathBuf {
    let filename = match path.file_name() {
        Some(x) => x,
        None => return path,
    };

    let filename_str = match filename.to_str() {
        Some(x) => x,
        None => return path,
    };

    PathBuf::from(
        filename_str
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
    let mut force = false;
    let skip_list = SkipList { protected_patterns: RegexSet::new(&[r"^Cargo.*", r"^Makefile$", r"^\..*"]).unwrap() };

    for arg in std::env::args() {
        if arg.as_str() == "-f" {
            force = true;
        }
    }

    if !force {
        println!("dry-run mode, pass '-f' argument to force renaming")
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

        // only mangle real files
        if !old_path.is_file() {
            continue;
        }

        if skip_list.is_path_protected(old_path.clone()) {
            println!("skipping: {}", old_path.display());
            continue;
        }

        let new_path = fix_name(old_path.clone());

        if old_path.file_name() != new_path.file_name() {
            println!("{:?} -> {:?}", old_path.display(), new_path.display());
            if force {
                match fs::rename(old_path.clone(), new_path.clone()) {
                    Ok(_) => continue,
                    Err(e) => println!("failed to rename: {:?} {:?}", old_path.display(), e),
                }
            }
        }
    }
}
