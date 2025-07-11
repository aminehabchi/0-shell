use colored::*;

#[derive(Debug, Default)]
pub struct Flag {
    pub a: bool,
    pub a_upper: bool,
    pub f_upper: bool,
    pub l: bool,
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

use std::ffi::OsStr;

pub fn is_hidden(name: &OsStr) -> bool {
    name.to_string_lossy().starts_with(".")
}

use std::time::{ SystemTime };
use chrono::{ DateTime, Local };

pub fn format_date(time: &Option<SystemTime>) -> String {
    match *time {
        Some(t) => {
            let datetime: DateTime<Local> = DateTime::<Local>::from(t);
            datetime.format("%b %d %H:%M").to_string()
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
            format!("{}/", name.blue())
        } else {
            format!("{}", name.blue())
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
