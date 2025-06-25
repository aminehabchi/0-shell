use std::fs;
use std::path::{Path, PathBuf};

pub fn cp(args: &[&str]) {
    if args.is_empty() {
        eprintln!("cp: missing file operand");
        return;
    }
    if args.len() == 1 {
        eprintln!("cp: missing destination file operand after '{}'", args[0]);
        return;
    }

    let (sources, destination) = args.split_at(args.len() - 1);
    let destination = Path::new(destination[0]);
    let dest_exists = destination.exists();
    let dest_is_dir = dest_exists && destination.is_dir();

    if sources.len() > 1 && !dest_is_dir {
        eprintln!("cp: target '{}' is not a directory", destination.display());
        return;
    }

    for src in sources {
        let src_path = Path::new(src);
        if !src_path.exists() {
            eprintln!("cp: cannot stat '{}': No such file or directory", src);
            continue;
        }

        if src_path.is_dir() {
            eprintln!("cp: -r not specified; omitting directory '{}'", src);
            continue;
        }

        let dest_file_path: PathBuf = if dest_is_dir {
            match src_path.file_name() {
                Some(name) => destination.join(name),
                None => {
                    eprintln!("cp: invalid source path '{}'", src);
                    continue;
                }
            }
        } else {
            destination.to_path_buf()
        };

        match fs::copy(src_path, &dest_file_path) {
            Ok(_) => {}
            Err(e) => eprintln!("cp: cannot copy '{}' to '{}': {}", src, dest_file_path.display(), e),
        }
    }
}
