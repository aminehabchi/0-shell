use std::process::Command;
use std::path::Path;
use std::io::{ self, Write };
use colored::*;
use pwd::*;
use cd::*;
use colored;
use std::env;
use crate::parser::parse_input;
use crate::command_router::{ router, exit_message };
use atty::Stream;

const ASCII: &str =
    r#"
_______         ______________  __________________ ______ 
__  __ \        __  ___/___  / / /___  ____/___  / ___  / 
_  / / /_____________ \ __  /_/ / __  __/   __  /  __  /  
/ /_/ / _/_____/____/ / _  __  /  _  /___   _  /____  /___
\____/          /____/  /_/ /_/   /_____/   /_____//_____/
                                                          "#;

pub fn main_loop() {
    // Print ASCII art if stdout is a terminal
    if !atty::is(Stream::Stdout) {
       return;
    }
     println!("{}\n", ASCII.blue());
    let mut current_dir = String::new();
    let stdin = io::stdin();
    let stdout = io::stdout();

    loop {
        // Get current working directory or fallback logic on error (e.g. after cd ..)
        current_dir = match pwd() {
            Ok(path) => path,
            Err(_) => {
                // Try to go one directory up
                let mut parts: Vec<&str> = current_dir.split('/').collect();
                parts.pop();
                let parent = parts.join("/");

                current_dir = match cd(&current_dir, &parent) {
                    Ok(new_dir) => new_dir,
                    Err(_) => parent,
                };
                continue;
            }
        };

         // Prepare prompt: directory name + git branch if any
        let current_path = Path::new(&current_dir);
        if let Some(last_dir) = current_path.file_name() {
             let home = env::var("HOME").map_err(|_| "HOME not set".to_string())
                .unwrap_or_else(|_| String::from("/"));
            if current_dir == home {
                print!("{}","~$ ".blue());
              
            }else {
            print!("~ {} {}$ ", last_dir.to_string_lossy().blue().bold(), get_current_branch());

            }
        } else {
            print!("/ ");
        }
        // Flush stdout to show prompt immediately
        if let Err(e) = stdout.lock().flush() {
            eprintln!("Failed to flush stdout: {}", e);
            return;
        }

        // Read input line
        let mut input = String::new();
        match stdin.read_line(&mut input) {
            Ok(0) => {
                // Ctrl+D pressed — exit gracefully
                exit_message();
                std::process::exit(0);
            }
            Ok(_) => {
                let trimmed = input.trim();
                if trimmed.is_empty() {
                    continue;
                }
                router(parse_input(trimmed.to_string()), &mut current_dir);
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
        Ok(out) if out.status.success() => {
            let branch = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if branch.is_empty() {
                String::new()
            } else {
                format!("git:({})", branch.red().bold())
            }
        }
        _ => String::new(),
    }
}
