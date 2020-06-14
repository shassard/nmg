use std::fs;

/// check if a filename is blacklisted from being renamed
fn blacklisted(filename: &str) -> bool {
    match filename {
        "Cargo.lock" => true,
        "Cargo.toml" => true,
        "Makefile" => true,
        _ => false,
    }
}

/// returns a cleaned up path name
fn fix_name(path: String) -> String {
    path.to_lowercase()
        .replace(" ", "-")
        .replace("_", "-")
        .replace("--", "-")
}

fn main() {
    let paths = fs::read_dir(".").unwrap();
    'outer: for path in paths {
        let old_path = path.unwrap().path();
        let old_filename = old_path.file_name().unwrap().to_str().unwrap();
        if blacklisted(old_filename) {
                println!("matched blacklist: {}", old_filename);
                continue 'outer;
            }
        let new_name = fix_name(String::from(old_filename));
        if old_filename != new_name {
            println!("{} -> {}", old_filename, new_name);
            fs::rename(old_filename, new_name).unwrap();
        }
    }
}
