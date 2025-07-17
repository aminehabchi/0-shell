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
    pub rdev: u64,
    pub major: u64,
    pub minor: u64,
    pub permissions: u32,
    pub nlink: u64,
    pub uid: String,
    pub gid: String,
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

        file.file_type = FileType::from_path(&path);

        /*********** check flag -l ***************/
        if !flag.l {
            return file;
        }

        file.permissions = metadata.mode();
        file.uid = uid_to_username(metadata.uid());
        file.gid = gid_to_groupname(metadata.gid());
        file.nlink = metadata.nlink();
        file.size = metadata.len();
        file.readonly = metadata.permissions().readonly();
        file.modified = metadata.modified().ok();
        file.accessed = metadata.accessed().ok();
        file.created = metadata.created().ok();
        file.rdev = metadata.rdev();
        file.major = major(file.rdev);
        file.minor = minor(file.rdev);
        file
    }

    pub fn print(&self, flags: &Flag, max_len: &((u8, u8, u8), u8, u8, u8)) {
        if flags.l {
            let file_type = match &self.file_type {
                FileType::Directory => "d".to_string(),
                FileType::File => "-".to_string(),
                FileType::Symlink(_) => "l".to_string(),
                FileType::CharDevice => "c".to_string(),
                FileType::BlockDevice => "b".to_string(),
                FileType::NamedPipe => "p".to_string(),
                FileType::Socket => "S".to_string(),
                FileType::Executable => "-".to_string(),
                FileType::Other => "-".to_string(),
            };
            let mode = mode_to_string(&self.permissions,&self.path.to_string_lossy());
            let hard_link = format!("{:width$}", self.nlink, width = max_len.1 as usize);
            let size = match &self.file_type {
                FileType::CharDevice | FileType::BlockDevice => {
                    // max_len.0.1 → width for major
                    // max_len.0.2 → width for minor
                    format!(
                        "{:>major_width$}, {:>minor_width$}",
                        self.major,
                        self.minor,
                        major_width = max_len.0.1 as usize,
                        minor_width = max_len.0.2 as usize
                    )
                }
                _ => format!("{:>width$}", self.size, width = max_len.0.0 as usize),
            };

            let date = format_date(&self.modified);
            let name = file_name(&self.name, &self.file_type, &flags);
            let user_name = format!("{:<width$}", self.uid, width = max_len.2 as usize);
            let group_name = format!("{:<width$}", self.gid, width = max_len.3 as usize);
            //print
            print!(
                "{}{} {} {} {} {} {} {}",
                file_type,
                mode,
                hard_link,
                user_name,
                group_name,
                size,
                date,
                name
            );
        } else {
            print!("{}", file_name(&self.name, &self.file_type, &flags));
        }
    }
}
