use std::{ffi::OsString, fs};
use std::io::{self, Write};
use chrono::DateTime;
use chrono::Local;
pub fn ls(args: &[&str], current_dir: &str) {
    //
    println!("ls args ==> {:?}", args);
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
                                //io::stdout().flush().unwrap();
                                print!("{} ", file_name.to_string_lossy().trim());
                            }
                            //for later
                            // else if !args[0].is_empty() && args[0] == "-l" {
                            // print!("{} ", file_name.to_string_lossy().trim());
                            // }
                            // print!("{} ", file_name);
                        }
                        Err(e) => eprintln!("Error reading entry: {}", e),
                    }
                }
                println!("");
            }
            Err(e) => eprintln!("Error reading directory: {}", e),
        }
    } else {
        // eprintln!("No args yet!");
        match curr_dir_data {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            // let file_name1 = ;
                            let xx = check_flags(args, entry.file_name());
                            match xx {
                                Ok(val) => {
                                    print!("{} ", val.to_string_lossy());
                                }
                                Err(val) => {
                                    print!("{}", val);
                                    break;
                                }
                            }
                        }
                        Err(e) => eprintln!("Error reading entry: {}", e),
                    }
                }
                println!("");
            }
            Err(e) => eprintln!("Error reading directory: {}", e),
        }
    }
}

fn check_flags(args: &[&str], file_name: OsString) -> Result<OsString, String> {
    // println!("flags in the args ==> {:?}", args);
    for (_i, arg) in args.iter().enumerate() {
        match *arg {
            "-l" => {
                println!("it is -l (list) !!!");
                // Ok(())
            }
            "-a" => {
                // println!("it is -a (all) !!!");
                // print!("{} ", file_name.to_string_lossy());
                return Ok(file_name);
            }
            "-F" => {
                println!("it is -F (F) !!!");
            }
            _ => {
                //eprintln!("flag not found, Try : -l, -a, -F");
                return Err("flag not found, Try -l , -a or -F".to_string());
                // break;
            }
        }
    }
    Err("flag not found, Try -l , -a or -F".to_string())
}

fn handle_l(current_dir: &str) -> io::Result<()> {
    let curr_dir_data = fs::read_dir(&current_dir)?;
    for data in curr_dir_data {
        let data = data?;
        let path = data.path();
        let metadata = data.metadata()?;
        let mut permissions = String::new();

        if metadata.is_dir() {
            permissions.push('d')
        } else {
            permissions.push('-')
        };

        if metadata.permissions().readonly() {
            // "r--" // simple check
            permissions.push_str("r--");
        } else if !metadata.permissions().readonly() {
            permissions.push_str("rw-");
            // "rw-" // not precise, just an indicator
        };

        let size = metadata.len();

        let modified = metadata.modified()?;
        // let datetime = modified.duration_since(UNIX_EPOCH).unwrap();
        // let seconds = datetime.as_secs();
        let datetime: DateTime<Local> = DateTime::from(modified);
        let formatted = datetime.format("%b %d %H:%M").to_string();

        let filename = path.file_name().unwrap().to_string_lossy();
        let mut results : Vec<Vec<String>> = vec![];
        let mut l_results : Vec<String> = vec![];
        l_results.push(permissions);
        l_results.push(size.to_string());
        l_results.push(formatted);
        l_results.push(filename.to_string());
        results.push(l_results);
        results.sort();
        // println!("{permissions} {size:>8} {formatted} {filename}");
        println!("{:?}", results);
    }
    Ok(())
}
