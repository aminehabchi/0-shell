use std::env;
pub fn pwd() -> Result<String, String> {
    match env::current_dir() {
        Ok(path) => Ok(path.display().to_string()),
        Err(err) => Err(err.to_string()),
    }
}
