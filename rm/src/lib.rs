use std::fs;
use std::path::Path;

pub fn rm(args: &[&str]) {
    if args.is_empty() {
        eprintln!("rm: \x1b[31mmissing arg\x1b[0m");
        return;
    }
    let dashr = args.contains(&"-r") || args.contains(&"-R");
    let files: Vec<&str> = args.iter().filter(|&&arg| !arg.starts_with('-')).copied().collect();

    if files.is_empty() {
        eprintln!("rm: \x1b[31mmissing files\x1b[0m");
        return;
    }

    for file in files {
        let path = Path::new(file);
        if dashr {
            if let Err(e) = fs::remove_dir_all(path) {
                if let Err(e2) = fs::remove_file(path) {
                    eprintln!("rm: \x1b[31mfailed to remove\x1b[0m '{}': {} / {}", file, e, e2);
                }
            }
        } else {
            if path.is_dir() {
                eprintln!("rm: \x1b[31mcan't remove '{}': is a directory (use -r)\x1b[0m", file);
            } else if let Err(e) = fs::remove_file(path) {
                eprintln!("rm: \x1b[31mfailed to remove\x1b[0m '{}': {}", file, e);
            }
        }
    }
}