use colored::*;
use users::{ get_user_by_uid, get_group_by_gid };
use std::ffi::OsStr;
use std::path::*;
use std::fs;
use std::os::unix::fs::*;
use chrono::{ DateTime, Duration, Local, TimeZone, Utc };
use std::time::{ SystemTime, UNIX_EPOCH };
use crate::file::File;

#[derive(Debug, Default, Clone)]
pub struct Flag {
    pub a: bool,
    pub a_upper: bool,
    pub f_upper: bool,
    pub l: bool,
}
pub fn is_executable(mode: u32) -> bool {
    (mode & 0o111) != 0
}
pub fn is_file(name: &str) -> bool {
    Path::new(name).is_file()
}
pub fn is_dir(name: &str) -> bool {
    Path::new(name).is_dir()
}

pub fn is_char_device<P: AsRef<Path>>(path: P) -> bool {
    if let Ok(metadata) = fs::symlink_metadata(path) {
        metadata.file_type().is_char_device()
    } else {
        false
    }
}

#[derive(PartialEq, Debug)]
pub enum FileType {
    Directory,
    File,
    Executable,
    Symlink(String),
    CharDevice,
    BlockDevice,
    NamedPipe,
    Socket,
    Other,
}

impl FileType {
    pub fn from_path(path: &Path) -> Self {
        let metadata = fs::symlink_metadata(path).unwrap();
        let file_type = metadata.file_type();

        if file_type.is_symlink() {
            let target = fs::read_link(path).unwrap();
            FileType::Symlink(target.to_string_lossy().to_string())
        } else if file_type.is_dir() {
            FileType::Directory
        } else if file_type.is_file() {
            let mode = metadata.permissions().mode();
            if (mode & 0o111) != 0 {
                FileType::Executable
            } else {
                FileType::File
            }
        } else if file_type.is_char_device() {
            FileType::CharDevice
        } else if file_type.is_block_device() {
            FileType::BlockDevice
        } else if file_type.is_fifo() {
            FileType::NamedPipe
        } else if file_type.is_socket() {
            FileType::Socket
        } else {
            FileType::Other
        }
    }
}

impl Default for FileType {
    fn default() -> Self {
        FileType::Other
    }
}

pub fn is_hidden(name: &OsStr) -> bool {
    name.to_string_lossy().starts_with(".")
}

pub fn format_date(time: &Option<SystemTime>) -> String {
    match *time {
        Some(t) => {
            let datetime_utc: DateTime<Utc> = t.into();

            let now = Utc::now();

            let diff = now.signed_duration_since(datetime_utc);

            if diff > Duration::days(365) {
                let local = datetime_utc.with_timezone(&Local);
                local.format("%b %d  %Y").to_string()
            } else {
                let local = datetime_utc.with_timezone(&Local);
                local.format("%b %d %H:%M").to_string()
            }
        }
        None => String::from("-- -- --:--"),
    }
}

pub fn mode_to_string(mode: &u32) -> String {
    let mut result = String::new();

    let permissions = [(mode >> 6) & 0o7, (mode >> 3) & 0o7, mode & 0o7];

    for &perm in &permissions {
        result.push(if (perm & 0b100) != 0 { 'r' } else { '-' });
        result.push(if (perm & 0b010) != 0 { 'w' } else { '-' });
        result.push(if (perm & 0b001) != 0 { 'x' } else { '-' });
    }

    result
}

pub fn file_name(name: &str, file_type: &FileType, flags: &Flag) -> String {
    match file_type {
        FileType::Directory => if flags.f_upper {
            format!("{}/", name.blue().bold())
        } else {
            format!("{}", name.blue().bold())
        }
        FileType::File => format!("{}", name),
        FileType::Symlink(target) => if flags.l {
            format!("{} -> {}", name.cyan(), target)
        } else if flags.f_upper {
            format!("{}@", name.cyan())
        } else {
            format!("{}", name.cyan())
        }
        FileType::Executable => if flags.f_upper {
            format!("{}*", name.yellow().bold())
        } else {
            format!("{}", name.yellow().bold())
        }
        FileType::CharDevice => { format!("{}", name) }
        FileType::BlockDevice => { format!("{}", name) }
        FileType::NamedPipe => { format!("{}`", name) }
        FileType::Socket => { format!("{}=", name) }
        FileType::Other => format!("{}", name),
    }
}

pub fn uid_to_username(uid: u32) -> String {
    get_user_by_uid(uid)
        .map(|user| user.name().to_string_lossy().to_string())
        .unwrap_or(uid.to_string())
}

pub fn gid_to_groupname(gid: u32) -> String {
    get_group_by_gid(gid)
        .map(|group| group.name().to_string_lossy().to_string())
        .unwrap_or(gid.to_string())
}

pub fn remove_leading_dot(name: &str) -> String {
    name.strip_prefix('.').unwrap_or(name).to_string()
}
