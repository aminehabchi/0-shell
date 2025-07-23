pub fn echo(input: Vec<&str>) -> String {
    let mut result = String::new();
    for (i,item) in input.iter().enumerate() {
        let mut chars = item.chars().peekable(); 
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            if let Some(&next_ch) = chars.peek() {
                match next_ch {
                    'n' => {
                        result.push('\n');
                        chars.next();
                    }
                    't' => {
                        result.push('\t');
                        chars.next();
                    }
                    'r' => {
                        result.push('\r');
                        chars.next();
                    }
                    '\\' => {
                        result.push('\\');
                        chars.next();
                    }
                    _ => result.push(ch),
                }
            } else {
                result.push(ch);
            }
        } else {
            result.push(ch);
        }
        }
        if i < input.len()-1{

            result.push(' ');
        }

    }
    result
}

pub fn print_help() {
    println!("Usage: echo [OPTION]... [STRING]...");
    println!("Echo the STRINGs to standard output.");
    println!();
    println!("  -n     do not output the trailing newline");
    println!("  -e     enable interpretation of backslash escapes");
    println!("  -E     disable interpretation of backslash escapes (default)");
    println!("  --help display this help and exit");
    println!();
    println!("If -e is in effect, the following sequences are recognized:");
    println!("  \\n     new line");
    println!("  \\t     horizontal tab");
    println!("  \\r     carriage return");
    println!("  \\\\     backslash");
}