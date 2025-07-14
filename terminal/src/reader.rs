use std::io::{ self, Write };
use pwd::*;
use mkdir::*;
use rm::*;
use cat::*;
use colored::*;
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
        print!("{}$ ", current_dir.blue().bold());
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        let trimmed_input = input.trim();
        if trimmed_input.is_empty() {
            input.clear();
            continue;
        }

        select_command(trimmed_input.to_string(), &current_dir);

        input.clear();
    }
}

fn select_command(input: String, current_dir: &str) {
    let args: Vec<&str> = input.split(" ").collect();
    match args[0] {
        "cat" => cat(&args[1..]) ,
        "pwd" => print_output("pwd", pwd()),
        "ls" => {}
        "echo" => {}
        "rm" => rm(&args[1..]),
        "mkdir" => mkdir(current_dir, &args[1..]),
        "mv" => {}
        "cp" => {}
        "cd" => {}
        "exit" => {
            println!("terminal exited!");
            std::process::exit(0);
        }
        _ => {
            let cmdnotfound = "Command Not Found!";
            println!("{}: {}", args[0].red().bold(),cmdnotfound.red().bold());
        }
    }
}

fn print_output(command: &str, result: Result<String, String>) {
    match result {
        Ok(out) => println!("{}", out),
        Err(err) => println!("{}: {}", command, err),
    }
}
