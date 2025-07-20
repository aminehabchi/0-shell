use crate::helpers::*;
use crate::directory::Directory;

pub struct Ls {
    pub directorys: Vec<Directory>,
    pub files: Vec<String>,
    pub flags: Flag,
}

impl Ls {
    pub fn new(args: &[&str]) -> Option<Ls> {
        let mut args = args.to_vec();
        let flags = match parse_flags(&mut args) {
            Ok(f) => f,
            Err(err) => {
                println!("{err}");
                return None;
            }
        };
        /***************************/
        if args.is_empty() {
            args.push(".");
        }
        /***************************/
        let mut directorys: Vec<Directory> = vec![];
        let mut files: Vec<String> = vec![];
        for arg in args {
            if is_file(&arg) {
                files.push(arg.to_string());
                continue;
            }
            if !is_dir(&arg) {
                println!("ls: cannot access '{}': No such file or directory", arg);
                continue;
            }
            let mut dir = match Directory::new(arg, &flags) {
                Ok(d) => d,
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            };
            dir.fill_directory();
            directorys.push(dir);
        }
        Some(Ls {
            files,
            flags,
            directorys,
        })
    }

    pub fn print(&self) {
        if !self.files.is_empty() {
            let mut dir: Directory = Directory {
                name: "".to_string(),
                total: 0,
                files: vec![],
                max_len: ((0, 0, 0), 0, 0, 0),
                flags: self.flags.clone(),
                is_files: true,
            };

            for file in &self.files {
                dir.add_file_to_dir(&file);
            }

            dir.sort_files_by_name();
            dir.print();

            println!("\n");
        }

        let l = self.directorys.len();

        for i in 0..l {
            if l > 1 {
                if i != 0 {
                    println!("");
                }
                println!("{}:", self.directorys[i].name);
            }
            self.directorys[i].print();
            if !self.flags.l || i != l - 1 {
                println!("");
            }
        }
    }
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
