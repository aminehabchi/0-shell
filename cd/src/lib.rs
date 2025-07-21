use std::fs;
use std::path::Path;
use std::io;

pub fn visit_dir(dir: &Path, target: &str) -> (bool,String) {
    let mut is_found = false;
    let mut result = String::new();
    let file_name = dir.display().to_string();
    if target == "..".to_string() {
        return (true,dir.parent().unwrap().display().to_string());
    }
    if dir.is_dir() {
        match fs::read_dir(dir) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            let path = entry.path();
                            if path.is_dir() {
                                if let Some(dir_name) = path.file_name() {
                                    let name = dir_name.to_string_lossy();
                                    if target == name {
                                        is_found = true;
                                        result.push_str(&(file_name + "/" + &name));
                                        break;
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            println!("Error reading entry: {}", e);
                            continue;
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error reading directory: {}", e);
                return (false,String::new());
            }
        }
    }
    if !is_found {
        println!("No such directory: {}", target);
        return (false,String::new())
    }
    (true,result)
}
