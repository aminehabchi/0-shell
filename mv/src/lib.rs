use std::fs::{ self, rename, copy };
use std::path::Path;

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
    let destination = destination[0];

    let is_dir = match fs::metadata(Path::new(destination)) {
        Ok(metadata) => metadata.is_dir(),
        Err(e) => {
            eprintln!("mv: cannot access '{}': {}", destination, e);
            return;
        }
    };

    for src in sources {
        if is_dir {
            let filename = match Path::new(src).file_name() {
                Some(name) => name,
                None => {
                    eprintln!("mv: invalid source path '{}'", src);
                    continue;
                }
            };

            let dest_path = Path::new(destination).join(filename);
            match copy(src, &dest_path) {
                Ok(_) => {}
                Err(e) =>
                    eprintln!("mv: cannot copy '{}' to '{}': {}", src, dest_path.display(), e),
            }
        } else {
            match rename(src, destination) {
                Ok(_) => {}
                Err(e) => eprintln!("mv: cannot move '{}' to '{}': {}", src, destination, e),
            }
        }
    }
}
