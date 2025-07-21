use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};

pub fn cat(args: &[&str]) {
    if args.is_empty() {
         let mut buffer = String::new();
        if let Err(e) = io::stdin().read_to_string(&mut buffer) {
            eprintln!("cat: \x1b[31merror reading stdin\x1b[0m: {}", e);
            return;
        }
    }

    let mut input_files = Vec::new();
    let mut redirect_mode = None;

    let mut i = 0;
    while i < args.len() {
        match args[i] {
            ">" | ">>" => {
                if i + 1 >= args.len() {
                    eprintln!("cat: \x1b[31mredirection without file\x1b[0m");
                    return;
                }
                redirect_mode = Some((args[i], args[i + 1]));
                i += 2;
            }

            _ => {
                input_files.push(args[i]);
                i += 1;
            }
        }
    }
    // for flags , ya ima t overwriti wla t appendi
    let mut output: Box<dyn Write> = match redirect_mode {
        Some((">", filename)) => match File::create(filename) {
            Ok(f) => Box::new(f),
            Err(e) => {
                eprintln!("cat: \x1b[31mcannot write to\x1b[0m '{}': {}", filename, e);
                return;
            }
        },
        Some((">>", filename)) => {
            match OpenOptions::new().append(true).create(true).open(filename) {
                Ok(f) => Box::new(f),
                Err(e) => {
                    eprintln!("cat: \x1b[31mcannot append to\x1b[0m '{}': {}", filename, e);
                    return;
                }
            }
        }
        _ => Box::new(io::stdout()),
    };

    // If no input files walakin kayn redirection : read from stdin
    if input_files.is_empty() && redirect_mode.is_some() {
        let mut buffer = String::new();
        if let Err(e) = io::stdin().read_to_string(&mut buffer) {
            eprintln!("cat: \x1b[31merror reading stdin\x1b[0m: {}", e);
            return;
        }
        if let Err(e) = write!(output, "{}", buffer) {
            eprintln!("cat: \x1b[31mwrite error\x1b[0m: {}", e);
        }
        return;
    }

    for file in input_files {
        if file == "-"  || file == "--" {
            let stdin = io::stdin();
            let mut line = String::new();
            while stdin.read_line(&mut line).unwrap_or(0) > 0 {
                if let Err(e) = output.write_all(line.as_bytes()) {
                    eprintln!("cat: write error: {}", e);
                    return;
                }
            }
            continue;
        }

        match fs::File::open(file) {
            Ok(mut f) => {
                let mut contents = String::new();
                if let Err(e) = f.read_to_string(&mut contents) {
                    eprintln!("cat: \x1b[31merror reading\x1b[0m '{}': {}", file, e);
                } else {
                    if let Err(e) = write!(output, "{}", contents) {
                        eprintln!("cat: \x1b[31mwrite error\x1b[0m: {}", e);
                        return;
                    }
                }
            }
            Err(e) => {
                eprintln!("cat: \x1b[31mcannot open\x1b[0m '{}': {}", file, e);
            }
        }
    }
}
