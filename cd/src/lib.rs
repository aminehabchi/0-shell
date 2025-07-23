use std::env;
use std::path::{ PathBuf};

pub fn cd(current_dir: &str, target: &str) -> Result<String, String> {
    let basep = PathBuf::from(current_dir);
    let new_path = basep.join(target).canonicalize();

    match new_path {
        Ok(path) => {
            if path.is_dir() {
                env::set_current_dir(&path).map_err(|e| e.to_string())?;
                Ok(path.display().to_string())
            } else {
                Err(format!("not a directory: {}", target))
            }
        }
        Err(_) => Err(format!("no such directory: {}", target)),
    }
}