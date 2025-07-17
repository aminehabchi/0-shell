use crate::file::File;
use crate::helpers::FileType;
use colored::*;
use users::{ get_user_by_uid, get_group_by_gid };
use std::path::*;
use std::process::Command;

use crate::helpers::*;

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

/// Extract major device number
pub fn major(dev: u64) -> u64 {
    (dev >> 8) & 0xfff
}

/// Extract minor device number
pub fn minor(dev: u64) -> u64 {
    (dev & 0xff) | ((dev >> 12) & 0xfff00)
}
