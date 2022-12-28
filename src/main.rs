use regex::RegexSet;
use std::fs;
use std::path::PathBuf;

#[derive(Copy, Clone, Debug)]
struct Config<'r, 'p> {
    pub enable_rename: bool,
    pub skip_list: &'r RegexSet,
    pub replacement_patterns: &'p Vec<(String, String)>,
}

impl<'r, 'p> Config<'r, 'p> {
    /// check if a filename is protected from being renamed. when an error occurs, safely note the file as protected.
    fn is_path_protected(&self, path: &PathBuf) -> Option<bool> {
        Some(self.skip_list.is_match(path.file_name()?.to_str()?))
    }
}

/// returns a PathBuf with a cleaned up filename, or None if a failure occurs or the filename wouldn't change.
fn fix_name(path: &PathBuf, cnf: &Config) -> Option<PathBuf> {
    let mut new_name = path.file_name()?.to_str()?.to_lowercase();

    for replacement in cnf.replacement_patterns {
        new_name = new_name.replace(replacement.0.as_str(), replacement.1.as_str())
    }

    let new = PathBuf::from(new_name);

    if new.file_name() == path.file_name() {
        return None;
    }

    Some(new)
}

/// do the work of renaming the filename from the DirEntry, following the rules defined in cnf.
fn normalize_file(r: fs::DirEntry, cnf: &Config) {
    let old_path = r.path();

    // only mangle real files (not directories, symlinks, etc)
    if !old_path.is_file() {
        return;
    }

    // proceed when the file is not protected and we haven't encountered an error
    match cnf.is_path_protected(&old_path) {
        Some(false) => {}
        _ => {
            println!("skipping: {}", old_path.display());
            return;
        }
    }

    match fix_name(&old_path, &cnf) {
        None => return,
        Some(v) => {
            println!("{:?} -> {:?}", old_path.display(), v.display());
            if !cnf.enable_rename {
                return;
            }

            match fs::rename(&old_path, &v) {
                Ok(_) => return,
                Err(e) => println!("failed to rename: {:?} {:?}", old_path.display(), e),
            }
        }
    }
}

fn main() {
    // configure sensible defaults
    let mut cnf = Config {
        enable_rename: false,
        skip_list: &RegexSet::new(&[r"^Cargo.*", r"^Makefile$", r"^\..*"]).unwrap(),
        replacement_patterns: &vec![
            (" ".to_string(), "-".to_string()),
            (",-".to_string(), ",".to_string()),
            ("_".to_string(), "-".to_string()),
            (",-".to_string(), ",".to_string()),
            ("--".to_string(), "-".to_string()),
            ("&amp;".to_string(), "and".to_string()),
            ("&".to_string(), "and".to_string()),
            ("-(z-lib.org)".to_string(), "".to_string()),
            ("-epub.epub".to_string(), ".epub".to_string()),
        ],
    };

    for arg in std::env::args() {
        match arg.as_str() {
            "-f" => cnf.enable_rename = true,
            _ => {}
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
        match path {
            Ok(x) => normalize_file(x, &cnf),
            _ => continue,
        }
    }
}
