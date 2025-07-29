use std::env;
use std::path::{ PathBuf};
 
pub fn cd(current_dir: &str, target: Option<&str>) -> Result<String, String> {
    let basep = PathBuf::from(current_dir);
    let original_target = match target {
        Some(tar) => tar,
        None => ""
    };
    
    let resolved_target = if original_target == "~" || original_target.starts_with("~/") {
        match env::var("HOME") {
            Ok(home) => {
                if original_target == "~" {
                    home
                } else {
                    original_target.replacen("~", &home, 1)
                }
            }
            Err(_) => return Err("Could not determine home directory".to_string()),
        }
    } else {
        original_target.to_string()
    };
    
    let new_path = if PathBuf::from(&resolved_target).is_absolute() {
        PathBuf::from(&resolved_target).canonicalize()
    } else {
        basep.join(&resolved_target).canonicalize()
    };
    match new_path {
        Ok(path) => {
            if path.is_dir() {
                env::set_current_dir(&path).map_err(|e| e.to_string())?;
                Ok(path.display().to_string())
            } else {
                Err(format!("not a directory: {}", original_target))
            }
        }
        Err(_) => Err(format!("no such directory: {}", original_target)),
    }
}