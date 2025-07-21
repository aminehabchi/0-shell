use std::fs::{rename, copy, remove_file, remove_dir_all};
use std::path::{Path, PathBuf};

pub fn mv(args: &[&str]) {
    if args.is_empty() {
        eprintln!("mv: missing file operand");
        return;
    }
    
    if args.len() == 1 {
        eprintln!("mv: missing destination file operand after '{}'", args[0]);
        return;
    }

    let (sources, destination) = args.split_at(args.len() - 1);
    let destination = Path::new(destination[0]);

    if sources.len() > 1 && !destination.is_dir() {
        eprintln!(
            "mv: target '{}' is not a directory",
            destination.display()
        );
        return;
    }

    for src in sources {
        let src_path = Path::new(src);

        let dest_path: PathBuf = if destination.is_dir() {
            match src_path.file_name() {
                Some(name) => destination.join(name),
                None => {
                    eprintln!("mv: invalid source path '{}'", src);
                    continue;
                }
            }
        } else {
            destination.to_path_buf()
        };

        match rename(&src_path, &dest_path) {
            Ok(_) => continue,
            Err(_) => {
                // Fallback: try copy and delete (across filesystems, etc.)
                if let Err(e) = copy(&src_path, &dest_path) {
                    eprintln!("mv: cannot move '{}' to '{}': {}", src, dest_path.display(), e);
                    continue;
                }

                // Remove source
                let _ = if src_path.is_dir() {
                    remove_dir_all(&src_path)
                } else {
                    remove_file(&src_path)
                };
            }
        }
    }
}
