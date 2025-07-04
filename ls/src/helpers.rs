#[allow(dead_code)]
use chrono::DateTime;
use chrono::Local;
use colored::*;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::{ffi::OsString, fs};

pub fn check_flags(args: &[&str], _file_name: OsString, current_dir: &str) -> Result<(), String> {
    // println!("flags in the args ==> {:?}", args);
    for (_i, arg) in args.iter().enumerate() {
        let _ = handle_flag(current_dir, *arg);
        break;
        // match *arg {
        //     "-l" => {
        //         let _ = handle_flag(current_dir, *arg);
        //         // break;
        //     }
        //     "-a" => {
        //         //Ok(());
        //     }
        //     "-F" => {
        //         println!("it is -F (F) !!!");
        //     }
        //     _ => {
        //         return Err("flag not found, Try -l , -a or -F".to_string());
        //     }
        // }
    }
    Ok(())
    // Err("flag not found, Try -l , -a or -F".to_string())
}

//ls -l

pub fn handle_flag(current_dir: &str, flag: &str) -> io::Result<()> {
    let curr_dir_data = fs::read_dir(&current_dir)?;
    let mut is_l = false;
    let mut is_f = false;
    let mut is_a = false;
    for data in curr_dir_data {
        let data = data?;
        let path = data.path();
        let metadata = data.metadata()?;
        let mode = metadata.mode();
        let is_dir = metadata.is_dir();
        //permissions
        let read_write = mode_to_string(mode);
        let mut permissions = String::new();
        let mut final_filename = String::new();
        let filename = path.file_name().unwrap().to_string_lossy();
        match flag {
            "-l" => {
                is_l = true;
                if metadata.is_dir() {
                    permissions.push('d');
                } else if metadata.is_symlink() {
                    permissions.push('l');
                } else {
                    permissions.push('-');
                }
            }
            "-F" => {
                is_f = true;

                final_filename.push_str(&filename);

                //add based on type
                if metadata.is_dir() {
                    // read_write.push_front('d');
                    final_filename.push('/');
                    let _ = &final_filename.blue().bold();
                } else if metadata.is_symlink() {
                    final_filename.push('@');
                } else {
                    final_filename.push('-');
                }
            }
            "-a" => is_a = true,
            _ => {}
        }
        // if metadata.is_dir() {
        //     // read_write.push_front('d');
        //     permissions.push('d');
        // } else if metadata.is_symlink() {
        //     permissions.push('l');
        // } else {
        //     permissions.push('-');
        // }
        permissions.push_str(&read_write);
        let size = metadata.len();

        let modified = metadata.modified()?;
        //time
        let datetime: DateTime<Local> = DateTime::from(modified);
        let formatted = datetime.format("%b %d %H:%M").to_string();
        // if flag == "-F" {}
        if !filename.starts_with(".") {
            if is_l {
                if is_dir {
                    print!(
                        "{} {} {} {}",
                        permissions,
                        size,
                        formatted,
                        colored_txt(filename.to_string())
                    );
                } else {
                    print!("{} {} {} {}", permissions, size, formatted, filename);
                }
            } else if is_f {
                if is_dir {
                    print!("{} ", colored_txt(final_filename));
                } else {
                    print!("{} ", final_filename);
                }
            }
        }
        if is_a {
            if is_dir {
                print!("{} ", colored_txt(filename.to_string()));
            } else {
                print!("{} ", filename);
            }
        }
    }
    println!("");
    Ok(())
}

//convert digit to valid permissions

pub fn digit_to_perm(digit: u8) -> String {
    let mut perm = String::new();

    // Check read bit (4)
    if digit & 4 != 0 {
        perm.push('r');
    } else {
        perm.push('-');
    }

    // Check write bit (2)
    if digit & 2 != 0 {
        perm.push('w');
    } else {
        perm.push('-');
    }

    // Check execute bit (1)
    if digit & 1 != 0 {
        perm.push('x');
    } else {
        perm.push('-');
    }

    perm
}

//get the last 3  digits from the mode
pub fn mode_to_string(mode: u32) -> String {
    let octal_str = format!("{:03o}", mode & 0o777);
    //println!("ocatal ==> {:?}", octal_str);
    let mut result = String::new();

    for ch in octal_str.chars() {
        let digit = ch.to_digit(8).unwrap() as u8;
        result.push_str(&digit_to_perm(digit));
    }
    result
}

pub fn it_contains(s: &str, flag: &str) -> bool {
    s.contains(flag)
}

fn colored_txt(s: String) -> ColoredString {
    s.blue().bold()
}
