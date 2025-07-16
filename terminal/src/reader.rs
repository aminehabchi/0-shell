use std::process::Command;
use std::io::{ self, Write };
use colored::*;
use pwd::*;
use crate::parser::parse_input;
use crate::command_router::router;

const ASCII: &str =
    r#"
_______         ______________  __________________ ______ 
__  __ \        __  ___/___  / / /___  ____/___  / ___  / 
_  / / /_____________ \ __  /_/ / __  __/   __  /  __  /  
/ /_/ / _/_____/____/ / _  __  /  _  /___   _  /____  /___
\____/          /____/  /_/ /_/   /_____/   /_____//_____/
                                                          "#;

pub fn main_loop() {
    let current_dir = match pwd() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("Failed to get current directory: {}", err);
            return;
        }
    };

    let mut input = String::new();
    println!("{}\n", ASCII.blue());
    loop {
        print!("~{}{}$ ", current_dir.green().bold(), get_current_branch());
        io::stdout().flush().unwrap();
        input.clear();
        let bytes_read = io::stdin().read_line(&mut input);

        match bytes_read {
            Ok(0) => {
                // Ctrl + D
                println!("");
                std::process::exit(0);
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

fn get_current_branch() -> String {
    let output = Command::new("git").args(&["rev-parse", "--abbrev-ref", "HEAD"]).output();

    match output {
        Ok(output) if output.status.success() => {
            let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
            format!(" git:({})", branch.red().bold())
        }
        _ => String::new(),
    }
}
