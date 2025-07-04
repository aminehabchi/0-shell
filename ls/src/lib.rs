use std::fs;
pub mod helpers;
#[allow(dead_code)]
// pub struct file_infos {
//     permission: String,
//     size: String,
//     date: String,
//     user_name: String,
//     group_name: String,
//     file_name: String,
// }

pub fn ls(args: &[&str], current_dir: &str) {
    //
    // println!("ls args ==> {:?}", args);
    let curr_dir_data = fs::read_dir(&current_dir);
    if args.is_empty() {
        match curr_dir_data {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            let file_name = entry.file_name();
                            // Skip hidden files (those starting with '.')
                            if !file_name.to_string_lossy().starts_with('.') {
                                print!("{} ", file_name.to_string_lossy().trim());
                            }
                        }
                        Err(e) => eprintln!("Error reading entry: {}", e),
                    }
                }
                println!("");
            }
            Err(e) => eprintln!("Error reading directory: {}", e),
        }
    } else {
        match curr_dir_data {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            let _ = helpers::check_flags(args, entry.file_name(), current_dir);
                            break;
                        }
                        Err(e) => eprintln!("Error reading entry: {}", e),
                    }
                }
                //println!("");
            }
            Err(e) => eprintln!("Error reading directory: {}", e),
        }
    }
}
