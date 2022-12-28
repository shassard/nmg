use regex::RegexSet;
use std::fs;
use std::path::PathBuf;

#[derive(Copy, Clone, Debug)]
struct Config<'r> {
    pub enable_rename: bool,
    pub skip_list: &'r RegexSet,
}

impl<'a> Config<'a> {
    /// check if a filename is protected from being renamed, in case an error occurs internally mark the file as protected.
    fn is_path_protected(&self, path: &PathBuf) -> bool {
        let filename = match path.file_name() {
            Some(x) => x,
            None => return true,
        };

        let filename_str = match filename.to_str() {
            Some(x) => x,
            None => return true,
        };

        self.skip_list.is_match(filename_str)
    }
}

/// returns a PathBuf with a cleaned up filename, or a clone of the original PathBuf if a failure occurs
fn fix_name(path: &PathBuf) -> Option<PathBuf> {
    let filename = match path.file_name() {
        Some(x) => x,
        None => return None
    };

    let filename_str = match filename.to_str() {
        Some(x) => x,
        None => return None
    };

    let new = PathBuf::from(
        filename_str
            .to_lowercase()
            .replace(' ', "-")
            .replace('_', "-")
            .replace(",-", ",")
            .replace("--", "-")
            .replace("&amp;", "and")
            .replace('&', "and")
            .replace("-(z-lib.org)", "")
            .replace("-epub.epub", ".epub"),
    );

    if new.file_name() == path.file_name() { return None }

    Some(new)
}

fn main() {
    let mut cnf = Config {
        enable_rename: false,
        skip_list: &RegexSet::new(&[r"^Cargo.*", r"^Makefile$", r"^\..*"]).unwrap(),
    };

    for arg in std::env::args() {
        if arg.as_str() == "-f" {
            cnf.enable_rename = true;
        }
    }

    if !cnf.enable_rename {
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

        if cnf.is_path_protected(&old_path) {
            println!("skipping: {}", old_path.display());
            continue;
        }

        let new_path = fix_name(&old_path);
        match new_path {
            None => continue,
            Some(v) => {
                println!("{:?} -> {:?}", old_path.display(), v.display());
                if !cnf.enable_rename { continue }

                match fs::rename(&old_path, &v) {
                    Ok(_) => continue,
                    Err(e) => println!("failed to rename: {:?} {:?}", old_path.display(), e),
                }
            }
        }
    }
}
