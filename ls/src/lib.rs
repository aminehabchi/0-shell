mod directory;
mod file;
mod helpers;

use crate::helpers::Flag;
use crate::directory::Directory;

pub fn ls(args: &[&str]) {
    let mut args = args.to_vec();
    // read flags
    let flags = match parse_flags(&mut args) {
        Ok(f) => f,
        Err(err) => {
            println!("{err}");
            return;
        }
    };

    if args.is_empty() {
        args.push(".");
    }

    // read diroctiry
    let mut all_dirs: Vec<Directory> = vec![];
    for arg in args {
        if arg.starts_with('-') {
            continue;
        }

        let dir = match Directory::new(arg, &flags) {
            Ok(d) => d,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
        all_dirs.push(dir);
    }

    // print
    for dir in all_dirs {
        dir.print(&flags);
    }
    println!("");
}

fn parse_flags(args: &mut Vec<&str>) -> Result<Flag, String> {
    let mut flags = Flag::default();
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--" {
            args.remove(i);
            break;
        } else if args[i].starts_with('-') && args[i].len() > 1 {
            for ch in args[i].chars().skip(1) {
                match ch {
                    'a' => {
                        flags.a = true;
                    }
                    'A' => {
                        flags.a_upper = true;
                    }
                    'F' => {
                        flags.f_upper = true;
                    }
                    'l' => {
                        flags.l = true;
                    }
                    _ => {
                        return Err(format!("ls: invalid option -- '{}'", ch));
                    }
                }
            }
            args.remove(i);
        } else {
            i += 1;
        }
    }
    Ok(flags)
}
