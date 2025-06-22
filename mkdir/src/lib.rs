use std::fs;

pub fn mkdir(current_dir: &str, args: &[&str]) {
    if args.len() == 0 {
        println!("mkdir: missing operand");
    }

    for arg in args {
        let path = format!("{}/{}", current_dir, arg);
        match fs::create_dir(path) {
            Ok(()) => {}
            Err(err) => println!("mkdir: {}", err),
        }
    }
}
