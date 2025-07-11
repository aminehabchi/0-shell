use std::path::Path;
use std::fs;
//
use crate::file::File;
use crate::helpers::*;
use std::fs::metadata;
use std::os::unix::fs::MetadataExt;
//
#[derive(Debug, Default)]
pub struct Directory {
    pub name: String,
    pub total: u64,
    pub files: Vec<File>,
    pub max_len: (u8, u8), // size hlink
}

impl Directory {
    pub fn new(name: &str, flags: &Flag) -> Result<Directory, String> {
        let path = Path::new(name);

        if !path.exists() {
            return Err(format!("ls: cannot access '{name}': No such file or directory"));
        }

        if !path.is_dir() {
            println!("{}", name);
            return Err("".to_string());
        }

        let mut files: Vec<File> = Vec::new();
        let mut total = 0u64;
        let mut max_len: (u8, u8) = (0, 0);
        if flags.a {
            add_file_to_dir(Path::new("."), flags, &mut files, &mut total, &mut max_len);
            add_file_to_dir(Path::new(".."), flags, &mut files, &mut total, &mut max_len);
        }
        for entry_result in fs::read_dir(path).unwrap() {
            let entry = match entry_result {
                Ok(e) => e,
                Err(err) => {
                    println!("{}", err);
                    continue;
                }
            };
            if !flags.a && is_hidden(&entry.file_name()) {
                continue;
            }
            add_file_to_dir(&entry.path(), flags, &mut files, &mut total, &mut max_len);
        }
        Ok(Directory {
            name: name.to_string(),
            total,
            files,
            max_len,
        })
    }
    pub fn print(&self, flags: &Flag) {
        if flags.l {
            println!("total {}", self.total / 2);
        }
        for i in 0..self.files.len() {
            self.files[i].print(&flags, &self.max_len);
            /********************/
            if i != self.files.len() - 1 {
                if flags.l {
                    println!("");
                } else {
                    print!(" ");
                }
            }
        }
    }
}

fn add_file_to_dir(
    entry_path: &Path,
    flags: &Flag,
    files: &mut Vec<File>,
    total: &mut u64,
    max_len: &mut (u8, u8)
) {
    let file = File::new(entry_path, flags);

    let block = match fs::symlink_metadata(entry_path) {
        Ok(meta) => meta.blocks(), // blocks are in 512-byte units
        Err(_) => 0,
    };
    *total += block;

    let size_len = file.size.to_string().len() as u8;
    if size_len > max_len.0 {
        max_len.0 = size_len;
    }

    let nlink_len = file.nlink.to_string().len() as u8;
    if nlink_len > max_len.1 {
        max_len.1 = nlink_len;
    }

    files.push(file);
}
