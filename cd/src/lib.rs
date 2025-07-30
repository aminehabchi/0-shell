use std::env;
use std::path::PathBuf;

// Static mutable variable for previous directory
static mut PREV_DIR: Option<String> = None;

pub fn cd(current_dir: &str, target: &str) -> Result<String, String> {
    // Handle "cd -" → go to previous directory
    if target == "-" {
        unsafe {
            if let Some(ref prev) = PREV_DIR {
                let path = PathBuf::from(prev);
                if path.is_dir() {
                    // Save current as previous before switching
                    PREV_DIR = Some(current_dir.to_string());
                    env::set_current_dir(&path).map_err(|e| e.to_string())?;
                    return Ok(path.display().to_string());
                } else {
                    return Err("previous path is not a directory".to_string());
                }
            } else {
                PREV_DIR = Some(current_dir.to_string())
            }
        }
    }

    // Handle "cd" and "cd ~" → go to $HOME
    let new_path = if target.is_empty() || target == "~" {
        let home = env::var("HOME").map_err(|_| "HOME not set".to_string())?;
        PathBuf::from(home)
    } else {
        PathBuf::from(current_dir).join(target)
    };

    let canonical = new_path.canonicalize();

    match canonical {
        Ok(path) => {
            if path.is_dir() {
                unsafe {
                    PREV_DIR = Some(current_dir.to_string());
                }
                env::set_current_dir(&path).map_err(|e| e.to_string())?;
                Ok(path.display().to_string())
            } else {
                Err(format!("not a directory: {}", target))
            }
        }
        Err(_) => {
                Err(format!("no previous directory or no such directory named : {}", target))
            
        } ,
    }
}