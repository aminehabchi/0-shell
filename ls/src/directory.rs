use std::path::Path;
use std::fs;
use std::os::unix::fs::MetadataExt;
//
use crate::file::File;
use crate::helpers::*;
use crate::file_helpers::FileType;
//
#[derive(Debug, Default)]
pub struct Directory {
    pub name: String,
    pub total: u64,
    pub files: Vec<File>,
    pub max_len: ((u8, u8, u8), u8, u8, u8),
    pub flags: Flag,
    pub is_files: bool,
}

impl Directory {
    pub fn new(name: &str, flags: &Flag) -> Result<Directory, String> {
        Ok(Directory {
            name: name.to_string(),
            total: 0,
            files: vec![],
            max_len: ((0, 0, 0), 0, 0, 0),
            flags: flags.clone(),
            is_files: false,
        })
    }
    pub fn fill_directory(&mut self) {
        // Check if directory first, without holding a reference
        if !Path::new(&self.name).is_dir() {
            return;
        }

        if self.flags.a {
            self.add_file_to_dir(".");
            self.add_file_to_dir("..");
        }

        // Create a fresh path reference for the read_dir call
        for entry_result in fs::read_dir(&self.name).unwrap() {
            let entry = match entry_result {
                Ok(e) => e,
                Err(err) => {
                    println!("{}", err);
                    continue;
                }
            };

            if !self.flags.a && is_hidden(&entry.file_name()) {
                continue;
            }

            let entry_path = entry.path();
            if let Some(path_str) = entry_path.to_str() {
                self.add_file_to_dir(path_str);
            }
        }

        self.sort_files_by_name()
    }

    pub fn add_file_to_dir(&mut self, entry_path: &str) {
        let file = File::new(entry_path, &self.flags);

        if self.flags.l {
            if let Ok(metadata) = fs::symlink_metadata(entry_path) {
                self.total += metadata.blocks();
            }
        }

        if file.file_type == FileType::CharDevice || file.file_type == FileType::BlockDevice {
            let major_len = file.major.to_string().len() as u8;
            let minor_len = file.minor.to_string().len() as u8;
            if major_len > self.max_len.0.1 {
                self.max_len.0.1 = major_len;
            }
            if minor_len > self.max_len.0.2 {
                self.max_len.0.2 = minor_len;
            }
            if major_len + minor_len + 2 > self.max_len.0.0 {
                self.max_len.0.0 = major_len + minor_len + 2;
            }
        } else {
            let size_len = file.size.to_string().len() as u8;
            if size_len > self.max_len.0.0 {
                self.max_len.0.0 = size_len;
            }
        }

        let nlink_len = file.nlink.to_string().len() as u8;
        if nlink_len > self.max_len.1 {
            self.max_len.1 = nlink_len;
        }

        let user_name_len = file.uid.to_string().len() as u8;
        if user_name_len > self.max_len.2 {
            self.max_len.2 = user_name_len;
        }

        let group_name_len = file.gid.to_string().len() as u8;
        if group_name_len > self.max_len.3 {
            self.max_len.3 = group_name_len;
        }

        self.files.push(file);
    }

    pub fn print(&self) {
        if self.flags.l && !self.is_files {
            println!("total {}", self.total / 2);
        }
        for i in 0..self.files.len() {
            self.files[i].print(&self.flags, &self.max_len);
            /********************/
            if i != self.files.len() - 1 {
                if self.flags.l {
                    println!("");
                } else {
                    print!(" ");
                }
            }
        }
    }

    pub fn sort_files_by_name(&mut self) {
        self.files.sort_by(|a, b|
            remove_special_char(&a.name.to_lowercase()).cmp(
                &remove_special_char(&b.name.to_lowercase())
            )
        );
    }
}
