use colored::*;
use users::{ get_user_by_uid, get_group_by_gid };
use std::fs;
use std::os::unix::fs::*;
use crate::helpers::*;

use std::path::*;

#[derive(PartialEq, Debug)]
pub enum FileType {
    Directory,
    File,
    Executable,
    Symlink(PathBuf),
    CharDevice,
    BlockDevice,
    NamedPipe,
    Socket,
    Other,
}

impl FileType {
    pub fn from_path(path: &Path) -> Self {
        let metadata = match fs::symlink_metadata(&path) {
            Ok(metadata) => metadata,
            Err(_) => {
                return FileType::File;
            }
        };

        let file_type = metadata.file_type();

        if file_type.is_symlink() {
            let target = match fs::read_link(path) {
                Ok(target) => target,
                Err(_) => {
                    return FileType::File;
                }
            };
            FileType::Symlink(target)
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

pub fn file_type(file_type: &FileType) -> char {
    match file_type {
        FileType::Directory => 'd',
        FileType::File => '-',
        FileType::Symlink(_) => 'l',
        FileType::CharDevice => 'c',
        FileType::BlockDevice => 'b',
        FileType::NamedPipe => 'p',
        FileType::Socket => 'S',
        FileType::Executable => '-',
        FileType::Other => '-',
    }
}

pub fn file_size(
    file_type: &FileType,
    size: &u64,
    major: &u64,
    minor: &u64,
    max_len: (&u8, &u8, &u8)
) -> String {
    match file_type {
        FileType::CharDevice | FileType::BlockDevice => {
            format!(
                "{:>major_width$}, {:>minor_width$}",
                *major,
                *minor,
                major_width = *max_len.1 as usize,
                minor_width = *max_len.2 as usize
            )
        }
        _ => format!("{:>width$}", size, width = *max_len.0 as usize),
    }
}

pub fn file_name(name: &str, file_type: &FileType, flags: &Flag) -> String {
    // Check if raw name needs quoting
    let needs_quote = !is_alphanumeric_or_special(name);

    let quoted_name = if needs_quote {
        if name.contains('\'') { format!("\"{}\"", name) } else { format!("'{}'", name) }
    } else {
        name.to_string()
    };

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
                let target_path = match fs::read_link(&target) {
                    Ok(raw_target) => raw_target,
                    Err(_) => target.clone(),
                };
                format!("{} -> {}", quoted_name.cyan(), target_path.display())
            } else if flags.f_upper {
                format!("{}@", quoted_name.cyan())
            } else {
                quoted_name.cyan().to_string()
            }
        }

        FileType::Executable => {
            if flags.f_upper {
                format!("{}*", quoted_name.green().bold())
            } else {
                quoted_name.green().bold().to_string()
            }
        }

        FileType::CharDevice | FileType::BlockDevice => quoted_name.yellow().bold().to_string(),

        FileType::NamedPipe => format!("{}|", quoted_name),

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

use xattr;
pub fn check_acl(path: &str) -> bool {
    if let Ok(attrs) = xattr::list(path) {
        for attr in attrs {
            if attr == "system.posix_acl_access" || attr == "system.posix_acl_default" {
                return true;
            }
        }
    }
    false
}

pub fn mode_to_string(mode: &u32, path: &str, file_type: &FileType) -> String {
    let mut result = String::new();

    let permissions = [(mode >> 6) & 0o7, (mode >> 3) & 0o7, mode & 0o7];

    for &perm in &permissions {
        result.push(if (perm & 0b100) != 0 { 'r' } else { '-' });
        result.push(if (perm & 0b010) != 0 { 'w' } else { '-' });
        result.push(if (perm & 0b001) != 0 { 'x' } else { '-' });
    }
    match file_type {
        FileType::Symlink(_) => {
            result.push(' ');
            return result;
        }
        _ => {}
    }

    if check_acl(path) {
        result.push('+');
    } else {
        result.push(' ');
    }

    result
}

/// Extract major device number
pub fn major(dev: u64) -> u64 {
    (dev >> 8) & 0xfff
}

/// Extract minor device number
pub fn minor(dev: u64) -> u64 {
    (dev & 0xff) | ((dev >> 12) & 0xfff00)
}
