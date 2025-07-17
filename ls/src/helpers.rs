use colored::*;
use users::{ get_user_by_uid, get_group_by_gid };
use std::ffi::OsStr;
use std::path::*;
use std::fs;
use std::os::unix::fs::*;
use chrono::{ DateTime, Duration, Local, Utc };
use std::time::{ SystemTime };
use std::process::Command;

/// Extract major device number
pub fn major(dev: u64) -> u64 {
    (dev >> 8) & 0xfff
}

/// Extract minor device number
pub fn minor(dev: u64) -> u64 {
    (dev & 0xff) | ((dev >> 12) & 0xfff00)
}

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

pub fn mode_to_string(mode: &u32, path: &str) -> String {
    let mut result = String::new();

    let permissions = [(mode >> 6) & 0o7, (mode >> 3) & 0o7, mode & 0o7];

    for &perm in &permissions {
        result.push(if (perm & 0b100) != 0 { 'r' } else { '-' });
        result.push(if (perm & 0b010) != 0 { 'w' } else { '-' });
        result.push(if (perm & 0b001) != 0 { 'x' } else { '-' });
    }

    if check_acl(path) {
        result.push('+');
    } else {
        result.push(' ');
    }

    result
}
pub fn is_alphanumeric_or_special(s: &str) -> bool {
    s.chars().all(|c| { c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' || c == '/' })
}

use crate::file::File;
pub fn file_name(name: &str, file_type: &FileType, flags: &Flag) -> String {
    // Check if raw name needs quoting
    let needs_quote = !is_alphanumeric_or_special(name);

    // Quote raw name if needed (handle single quote inside name)
    let quoted_name = if needs_quote {
        if name.contains('\'') { format!("\"{}\"", name) } else { format!("'{}'", name) }
    } else {
        name.to_string()
    };

    // Now apply coloring/suffixes to the quoted or unquoted name (the string with quotes if any)
    match file_type {
        FileType::Directory => {
            if flags.f_upper {
                format!("{}/", quoted_name.blue().bold())
            } else {
                quoted_name.blue().bold().to_string()
            }
        }

        FileType::File => quoted_name,

        FileType::Symlink(target) => {
            if flags.l {
                let p = Path::new(target);
                if !p.exists() {
                    format!("{} -> {}", quoted_name.cyan(), target.underline())
                } else {
                    let fake_flag = Flag::default();
                    let t = File::new(p, &fake_flag);
                    format!(
                        "{} -> {}",
                        quoted_name.cyan(),
                        file_name(&target, &t.file_type, &fake_flag)
                    )
                }
            } else if flags.f_upper {
                format!("{}@", quoted_name.cyan())
            } else {
                quoted_name.cyan().to_string()
            }
        }

        FileType::Executable => {
            if flags.f_upper {
                format!("{}*", quoted_name.yellow().bold())
            } else {
                quoted_name.yellow().bold().to_string()
            }
        }

        FileType::CharDevice | FileType::BlockDevice => quoted_name.magenta().to_string(),

        FileType::NamedPipe => format!("{}`", quoted_name),

        FileType::Socket => format!("{}=", quoted_name),

        FileType::Other => quoted_name,
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

pub fn remove_special_char(name: &str) -> String {
    let name = name.strip_prefix('.').unwrap_or(name).to_string();
    // remove from name -.
    name
}

pub fn check_acl(path: &str) -> bool {
    let output = Command::new("getfacl").arg(path).output().expect("failed to execute getfacl");

    if !output.status.success() {
        return false;
    }

    let acl_text = String::from_utf8_lossy(&output.stdout);

    // Basic entries that always appear
    let basic_entries = ["user::", "group::", "other::"];

    // If any line does NOT start with these, it indicates extended ACL
    acl_text.lines().any(|line| {
        !basic_entries.iter().any(|prefix| line.starts_with(prefix)) &&
            !line.trim().is_empty() &&
            !line.starts_with('#') && // skip comments
            !line.starts_with("mask::") // mask is also standard in ACLs
    })
}
