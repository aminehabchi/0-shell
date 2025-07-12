use chrono::{ DateTime, Local, Utc };
use colored::*;
use users::{ get_user_by_uid, get_group_by_gid };
use std::ffi::OsStr;
use std::time::{ SystemTime };
use std::path::Path;

#[derive(Debug, Default, Clone)]
pub struct Flag {
    pub a: bool,
    pub a_upper: bool,
    pub f_upper: bool,
    pub l: bool,
}

pub fn is_file(name: &str) -> bool {
    Path::new(name).is_file()
}
pub fn is_dir(name: &str) -> bool {
    Path::new(name).is_dir()
}

#[derive(PartialEq, Debug)]
pub enum FileType {
    Directory,
    File,
    Symlink(String),
    Other,
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
            let utc: DateTime<Utc> = t.into();
            let local = utc.with_timezone(&Local);
            local.format("%b %d %H:%M").to_string()
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
