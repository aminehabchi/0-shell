use std::fs::{ self };
use std::os::unix::fs::{ MetadataExt };
use std::time::{ SystemTime };
use std::path::PathBuf;
use std::path::Path;

use crate::helpers::*;

#[derive(Debug, Default)]
pub struct File {
    pub name: String,
    pub path: PathBuf,
    pub file_type: FileType,
    pub size: u64,
    pub permissions: u32,
    pub nlink: u64,
    pub uid: u32, // owner user ID
    pub gid: u32, // owner group ID
    pub readonly: bool,
    pub modified: Option<SystemTime>,
    pub accessed: Option<SystemTime>,
    pub created: Option<SystemTime>,
}
impl File {
    pub fn new<P: AsRef<Path>>(path: P, flag: &Flag) -> File {
        let mut file = File::default();
        let path = path.as_ref().to_path_buf();
        file.path = path.clone();
        file.name = match path.file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => {
                let s = path.to_string_lossy().to_string();
                if s == "" {
                    ".".to_string()
                } else {
                    s
                }
            }
        };

        let metadata = fs::symlink_metadata(&path).unwrap();

        file.file_type = if metadata.is_dir() {
            FileType::Directory
        } else if metadata.file_type().is_symlink() {
            let target = fs
                ::read_link(&path)
                .map(|p| p.display().to_string())
                .unwrap_or_else(|_| "???".to_string());
            FileType::Symlink(target)
        } else if is_executable(metadata.mode()) {
            FileType::Executable
        } else if metadata.is_file() {
            FileType::File
        } else {
            FileType::Other
        };

        /*********** check flag -l ***************/
        if !flag.l {
            return file;
        }

        file.permissions = metadata.mode();
        file.uid = metadata.uid();
        file.gid = metadata.gid();
        file.nlink = metadata.nlink();
        file.size = metadata.len();
        file.readonly = metadata.permissions().readonly();
        file.modified = metadata.modified().ok();
        file.accessed = metadata.accessed().ok();
        file.created = metadata.created().ok();

        file
    }

    pub fn print(&self, flags: &Flag, max_len: &(u8, u8)) {
        if flags.l {
            let file_type = match &self.file_type {
                FileType::Directory => "d".to_string(),
                FileType::File => "-".to_string(),
                FileType::Symlink(_) => "l".to_string(),
                FileType::Executable => "-".to_string(),
                FileType::Other => "?".to_string(),
            };
            let mode = mode_to_string(&self.permissions);
            let hard_link = format!("{:width$}", self.nlink, width = max_len.1 as usize);
            let size = format!("{:width$}", self.size, width = max_len.0 as usize);
            let date = format_date(&self.modified);
            let name = file_name(&self.name, &self.file_type, &flags);
            //print
            print!(
                "{}{} {} {} {} {} {} {}",
                file_type,
                mode,
                hard_link,
                uid_to_username(self.uid),
                gid_to_groupname(self.gid),
                size,
                date,
                name
            );
        } else {
            print!("{}", file_name(&self.name, &self.file_type, &flags));
        }
    }
}
