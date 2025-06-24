use std::{ffi::OsString, fs};
// use std::io::{self, Write};
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
