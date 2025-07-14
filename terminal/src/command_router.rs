use pwd::*;
use mkdir::*;
use rm::*;
use mv::*;
use cp::*;
use ls::*;

pub fn router(parts: Vec<String>, current_dir: &String) {
    if parts.is_empty() {
        return;
    }

    let args: Vec<&str> = parts[1..]
        .iter()
        .map(|s| s.as_str())
        .collect();

    match parts[0].as_str() {
        "pwd" => print_output("pwd", pwd()),
        "ls" => ls(&args),

        "echo" => {}
        "rm" => rm(&args),
        "mkdir" => mkdir(&current_dir, &args),
        "mv" => {
            mv(&args);
        }
        "cp" => {
            cp(&args);
        }
        "cd" => {}
        "exit" => {
            println!("terminal exited!");
            std::process::exit(0);
        }
        "clear" => {
            clear_screen();
        }
        _ => {

            println!("Command '{}' not found", parts[0]);
        }
    }
}

pub fn print_output(command: &str, result: Result<String, String>) {
    match result {
        Ok(out) => println!("{}", out),
        Err(err) => println!("{}: {}", command, err),
    }
}

pub fn clear_screen() {
    // ANSI escape code to clear screen and move cursor to top-left
    print!("\x1B[2J\x1B[1;1H");
    // Make sure to flush stdout so the escape code is sent immediately
    use std::io::{ stdout, Write };
    stdout().flush().unwrap();
}
