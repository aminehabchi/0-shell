use std::process::Command;
use std::path::Path;
use std::io::{ self, Write };
use colored::*;
use pwd::*;
use cd::*;
use crate::parser::parse_input;
use crate::command_router::router;
use atty::Stream;
use crate::command_router::exit_message;
const ASCII: &str =
    r#"
_______         ______________  __________________ ______ 
__  __ \        __  ___/___  / / /___  ____/___  / ___  / 
_  / / /_____________ \ __  /_/ / __  __/   __  /  __  /  
/ /_/ / _/_____/____/ / _  __  /  _  /___   _  /____  /___
\____/          /____/  /_/ /_/   /_____/   /_____//_____/
                                                          "#;

 pub fn main_loop() {
let mut input = String::new();

    let is_tty = atty::is(Stream::Stdout);
    if is_tty {
        println!("{}\n", ASCII.blue());
    }
    let mut current_dir: String = "".to_string();
    loop {
        current_dir = match pwd() {
            Ok(path) => path,
            Err(_) => {
                let mut d: Vec<&str> = current_dir.split("/").collect();
                d.pop();
                let cc = d.join("/");
                match cd(&current_dir, &cc) {
                    Ok(new_dir) => current_dir = new_dir,
                    Err(_) => {
                        current_dir = cc;
                        //println!("cd: {}", e);
                    }
                }
                continue;
            }
        };
        let  binding = current_dir.clone();
        let  current_directory = Path::new(binding.as_str());
        if let Some(last_dir) = current_directory.file_name() {
            print!(
                "~ {} {}$ ",
                last_dir.to_string_lossy().blue().bold(),
                get_current_branch()
            );
        } else {
            print!("/");
        }

        match io::stdout().flush() {
            Ok(_) => {}
            Err(r) => {
                print!("{r}");
                return;
            }
        };
        input.clear();
        let bytes_read = io::stdin().read_line(&mut input);

        match bytes_read {
            Ok(0) => {
                // Ctrl + D
                exit_message();
                std::process::exit(0);
            }
            Ok(_) => {
                let trimmed_input = input.trim();
                if trimmed_input.is_empty() {
                    continue;
                }
                router(parse_input(trimmed_input.to_string()), &mut current_dir.to_string());
            }
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                break;
            }
        }
    }
}

fn get_current_branch() -> String {
    // return String::new();
    let output = Command::new("git").args(&["rev-parse", "--abbrev-ref", "HEAD"]).output();

    match output {
        Ok(output) if output.status.success() => {
            let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
            format!("git:({})", branch.red().bold())
        }
        _ => String::new(),
    }
}




