use std::env;
use std::fs;
use std::io::{self, Write, Read};
use std::path::Path;
use std::process::{Command};

fn rm(args: &[&str]) {
    if args.is_empty() {
        eprintln!("rm: \x1b[31mmissing arg\x1b[0m");
        return;
    }
    let dashr = args.contains(&"-r") || args.contains(&"-R");
    let files: Vec<&str> = args.iter().filter(|&&arg| !arg.starts_with('-')).copied().collect();

    if files.is_empty() {
        eprintln!("rm: \x1b[31mmissing files\x1b[0m");
        return;
    }

    for file in files {
        let path = Path::new(file);
        if dashr {
            if let Err(e) = fs::remove_dir_all(path) {
                if let Err(e2) = fs::remove_file(path) {
                    eprintln!("rm: \x1b[31mfailed to remove\x1b[0m '{}': {} / {}", file, e, e2);
                }
            }
        } else {
            if path.is_dir() {
                eprintln!("rm: \x1b[31mcan't remove '{}': is a directory (use -r)\x1b[0m", file);
            } else if let Err(e) = fs::remove_file(path) {
                eprintln!("rm: \x1b[31mfailed to remove\x1b[0m '{}': {}", file, e);
            }
        }
    }
}

fn cat(args: &[&str]) {
    if args.is_empty() {
        eprintln!("cat: \x1b[31mmissing file operand\x1b[0m");
        return;
    }

    for file in args {
        match fs::File::open(file) {
            Ok(mut f) => {
                let mut contents = String::new();
                if let Err(e) = f.read_to_string(&mut contents) {
                    eprintln!("cat: \x1b[31merror reading\x1b[0m '{}': {}", file, e);
                } else {
                    println!("{}", contents);
                }
            }
            Err(e) => {
                eprintln!("cat: \x1b[31mcannot open\x1b[0m '{}': {}", file, e);
            }
        }
    }
}

fn myloop() {
    loop {
        print!("$ ");
       io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Error reading input");
            continue;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0];
        let args = &parts[1..];

        match command {
            "exit" => {
                println!("Goodbye!");
                break;
            }
            "cd" => {
                let new_dir = args.get(0).copied().unwrap_or("/");
                if let Err(e) = env::set_current_dir(new_dir) {
                    eprintln!("cd: \x1b[31mcannot change directory to '{}': {}\x1b[0m", new_dir, e);
                }
            }
            "pwd" => {
                match env::current_dir() {
                    Ok(path) => println!("{}", path.display()),
                    Err(e) => eprintln!("pwd: \x1b[31m{}\x1b[0m", e),
                }
            }
            "rm" => rm(args),
            "cat" => cat(args),
            _ => {
                match Command::new(command).args(args).status() {
                    Ok(status) => {
                        if !status.success() {
                            eprintln!("\x1b[31mCommand exited with code {}\x1b[0m", status.code().unwrap_or(-1));
                        }
                    }
                    Err(_) => eprintln!("Command '{}' not found", command),
                }
            }
        }
    }
}
fn main() {
    myloop();
}
