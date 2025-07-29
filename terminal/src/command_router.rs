use pwd::*;
use cat::*;
use mkdir::*;
use rm::*;
use mv::*;
use cp::*;
use cd::*;
use ls::*;
use echo::*;
use std::io::{ Write };
 use std::io;
pub fn router(parts: Vec<String>, current_dir: &mut String) {
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

        "echo" => println!("{}",echo(args)),
        "rm" => rm(&args),
        "mkdir" => mkdir(&current_dir, &args),
        "mv" => {
            mv(&args);
        },
        "cat" => cat(&args),
        "cp" => {
            cp(&args);
        }
        "cd" => {
            let mut target : String = String::new();
            if args.is_empty() {
                //println!("cd: missing operand");
                
            } 
            match cd(current_dir, Some(&target)) {
                Ok(new_dir) => *current_dir = new_dir,
                Err(e) => {println!("cd: {}", e)}
            }
            
        }
        "exit" => {
            exit_message();
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
  
    match io::stdout().flush() {
            Ok(_) => {},
            Err(r) =>{
                print!("{r}");
                return
            } ,
        };
}
use colored::Colorize;

pub fn exit_message() {
    println!("\n\n{}", "0-shell: Disengaging from the Matrix...\n".bold().bright_magenta());
    println!(
        "{}",
        "Wake up, Neo. The terminal has closed.\n".bright_cyan().bold().underline()
    );
}
