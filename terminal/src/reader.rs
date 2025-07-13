use std::io::{ self, Write };
use colored::*;
use pwd::*;
use crate::parser::parse_input;
use crate::command_router::router;
pub fn main_loop() {
    let current_dir = match pwd() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("Failed to get current directory: {}", err);
            return;
        }
    };

    let mut input = String::new();

    loop {
        print!("{}$ ", current_dir.green().bold());
        io::stdout().flush().unwrap();
        input.clear();
        let bytes_read = io::stdin().read_line(&mut input);

        match bytes_read {
            Ok(0) => {
                // Ctrl + D
                println!("");
                break;
            }
            Ok(_) => {
                let trimmed_input = input.trim();
                if trimmed_input.is_empty() {
                    continue;
                }
                router(parse_input(trimmed_input.to_string()), &current_dir);
            }
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                break;
            }
        }
    }
}
