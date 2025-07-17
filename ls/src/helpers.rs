use std::ffi::OsStr;
use std::path::*;
use chrono::{ DateTime, Duration, Local, Utc };
use std::time::{ SystemTime };

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

pub fn is_alphanumeric_or_special(s: &str) -> bool {
    s.chars().all(|c| { c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' || c == '/' })
}

pub fn remove_special_char(name: &str) -> String {
    let name = name.strip_prefix('.').unwrap_or(name).to_string();
    // remove from name -.
    name
}
