use pwd::*;
use std::fs;
use std::path::Path;
pub fn rm(args: &[&str]) {
    if args.is_empty() {
        eprintln!("rm: \x1b[31mmissing arg\x1b[0m");
        return;
    }

    let mut recursive = false;
    let mut files = Vec::new();

    for &arg in args {
        if arg.starts_with('-') {
            for c in arg.chars().skip(1) {
                match c {
                    'r' | 'R' => recursive = true,
                    _ => {
                        eprintln!("rm: \x1b[31munknown option\x1b[0m -- '{}'", c);
                        return;
                    }
                }
            }
        } else {
            files.push(arg);
        }
    }

    if files.is_empty() {
        eprintln!("rm: \x1b[31mmissing files\x1b[0m");
        return;
    }

    for file in files {
        let mut ispwd = false;
        if file == "." {
            ispwd = true;
        }else if file ==".." {
            fs::remove_dir_all(Path::new(".."))
            continue;
        }
        let path = Path::new(file);
        if recursive {
            if path.is_dir() {
                let pwd = match pwd() {
                    Ok(p) => p,
                    Err(_) => "".to_string(),
                };
                if ispwd {
                    if let Err(e) = fs::remove_dir_all(Path::new(&pwd)) {
                        eprintln!(
                            "rm: \x1b[31mfailed to remove directory\x1b[0m '{}': {}",
                            file, e
                        );
                    }
                } else {
                    if let Err(e) = fs::remove_dir_all(path) {
                        eprintln!(
                            "rm: \x1b[31mfailed to remove directory\x1b[0m '{}': {}",
                            file, e
                        );
                    }
                }
            } else if let Err(e) = fs::remove_file(path) {
                eprintln!("rm: \x1b[31mfailed to remove file\x1b[0m '{}': {}", file, e);
            }
        } else {
            if path.is_dir() {
                eprintln!(
                    "rm: \x1b[31mcan't remove '{}': is a directory (use -r)\x1b[0m",
                    file
                );
            } else if let Err(e) = fs::remove_file(path) {
                eprintln!("rm: \x1b[31mfailed to remove\x1b[0m '{}': {}", file, e);
            }
        }
    }
}
