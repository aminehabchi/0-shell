use std::ffi::OsStr;
use std::path::*;
use chrono::{ DateTime, Duration, Local, Utc };
use std::time::{ SystemTime };

use std::ffi::CString;
use std::fs;
use std::os::unix::ffi::OsStrExt;
use libc::{ lstat, stat as stat_t };

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

pub fn is_hidden(name: &OsStr) -> bool {
    name.to_string_lossy().starts_with(".")
}

pub fn format_date(time: &Option<SystemTime>) -> String {
    match *time {
        Some(t) => {
            let datetime_utc: DateTime<Utc> = t.into();

            // Add one hour to the time
            let adjusted = datetime_utc + Duration::hours(1);

            let now = Utc::now();
            let diff = now.signed_duration_since(datetime_utc);

            // Use the adjusted time for formatting
            let local = adjusted.with_timezone(&Local);

            if diff > Duration::days(365) {
                local.format("%b %d  %Y").to_string()
            } else {
                local.format("%b %d %H:%M").to_string()
            }
        }
        None => String::from("-- -- --:--"),
    }
}

pub fn is_alphanumeric_or_special(s: &str) -> bool {
    s.chars().all(|c| { c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' || c == '/' })
}

pub fn remove_special_char(name: &str) -> String {
    let name = name.strip_prefix('.').unwrap_or(name).to_string();
    // remove from name -.
    name
}

pub fn calculate_ls_total<P: AsRef<Path>>(dir_path: P, include_hidden: bool) -> u64 {
    let mut total_blocks = 0;
    let dir_path_ref = dir_path.as_ref();

    if include_hidden {
        for special in &[".", ".."] {
            let special_path = if *special == "." {
                dir_path_ref.to_path_buf()
            } else {
                dir_path_ref.join("..")
            };

            if let Ok(c_path) = CString::new(special_path.as_os_str().as_bytes()) {
                let mut stat_buf: stat_t = unsafe { std::mem::zeroed() };
                if (unsafe { lstat(c_path.as_ptr(), &mut stat_buf) }) == 0 {
                    total_blocks += ((stat_buf.st_blocks as u64) + 1) / 2;
                }
            }
        }
    }

    let entries = match fs::read_dir(dir_path_ref) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Failed to read directory: {}", e);
            return total_blocks;
        }
    };

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();

            if !include_hidden {
                if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                    if name.starts_with('.') {
                        continue;
                    }
                }
            }

            let c_path = match CString::new(path.as_os_str().as_bytes()) {
                Ok(cstr) => cstr,
                Err(_) => {
                    continue;
                }
            };

            let mut stat_buf: stat_t = unsafe { std::mem::zeroed() };
            let result = unsafe { lstat(c_path.as_ptr(), &mut stat_buf) };

            if result == 0 {
                total_blocks += ((stat_buf.st_blocks as u64) + 1) / 2;
            }
        }
    }

    total_blocks
}
