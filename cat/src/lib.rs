use std::fs::File;
use std::io::{self, Read, Write};

pub fn cat(args: &[&str]) {
    // if no arguments, read from stdin and print
    if args.is_empty() {
        let mut buffer = String::new();
        if let Err(e) = io::stdin().read_to_string(&mut buffer) {
            eprintln!("cat: \x1b[31merror reading stdin\x1b[0m: {}", e);
        } else {
            print!("{}", buffer);
        }
        return;
    }

    for file in args {
        // if file is "-" or "--", treat it as stdin
        if *file == "-" || *file == "--" {
            let mut buffer = Vec::new();
            if let Err(e) = io::stdin().lock().read_to_end(&mut buffer) {
                eprintln!("cat: \x1b[31merror reading stdin\x1b[0m: {}", e);
                continue;
            }
            if let Err(e) = io::stdout().write_all(&buffer) {
                eprintln!("cat: \x1b[31mwrite error\x1b[0m: {}", e);
            }
            continue;
        }

        // try opening the file now
        match File::open(file) {
            Ok(mut f) => {
                let mut buffer = Vec::new();
                if let Err(e) = f.read_to_end(&mut buffer) {
                    eprintln!("cat: \x1b[31merror reading\x1b[0m '{}': {}", file, e);
                } else if let Err(e) = io::stdout().write_all(&buffer) {
                    eprintln!("cat: \x1b[31mwrite error\x1b[0m: {}", e);
                }
            }
            Err(e) => {
                eprintln!("cat: \x1b[31mcannot open\x1b[0m '{}': {}", file, e);
            }
        }
    }
}
