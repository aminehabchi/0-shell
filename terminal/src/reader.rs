use std::io::{ self, Write };
use std::path::Path;
use pwd::*;
use mkdir::*;
use rm::*;
use mv::*;
use cp::*;
use echo::*;
use cd::*;


pub fn main_loop() {
    let mut current_dir : String = match pwd() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("Failed to get current directory: {}", err);
            return;
        }
    };

    let mut input = String::new();

    loop {
        print!("{}$ ", current_dir);
        io::stdout().flush().unwrap();
        input.clear();
        let bytes_read = io::stdin().read_line(&mut input);

        match bytes_read {
            Ok(0) => {
                println!("");
                break;
            }
            Ok(_) => {
                let trimmed_input = input.trim();
                if trimmed_input.is_empty() {
                    continue;
                }
                select_command(trimmed_input.to_string(), &mut current_dir);
            }
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                break;
            }
        }
    }
}

fn select_command(input: String, current_dir: &mut String) {
    let args: Vec<&str> = input.split(" ").collect();
    match args[0] {
        "pwd" => print_output("pwd", pwd()),
        "ls" => {},
        "echo" => println!("{}",process_escape_sequences(&args[1][1..args[1].len()-1])),
        "rm" => rm(&args[1..]),
        "mkdir" => mkdir(current_dir, &args[1..]),
        "mv" => { mv(&args[1..]) }
        "cp" => { cp(&args[1..]) }
        "cd" => {
            let (is_valid,content) = visit_dir(Path::new(current_dir),&args[1]);
            if is_valid {
                *current_dir = content;
            }
        },
        "exit" => {
            println!("terminal exited!");
            std::process::exit(0);
        }
        _ => {
            println!("Command '{}' not found", args[0]);
        }
    }
}

fn print_output(command: &str, result: Result<String, String>) {
    match result {
        Ok(out) => println!("{}", out),
        Err(err) => println!("{}: {}", command, err),
    }
}
