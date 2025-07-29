use std::{ io::{ self, Write } };

pub fn parse_input(input: String) -> Vec<String> {
    let mut parts: Vec<String> = vec![String::new()];
    split_input(input, &mut parts, None);
    parts
}

pub fn split_input(input: String, parts: &mut Vec<String>, mut open_quote: Option<char>) {
    if parts.is_empty() {
        parts.push(String::new());
    }

    for ch in input.chars() {
        match open_quote {
            Some(quote) => {
                if ch == quote {
                    open_quote = None;
                } else {
                    parts.last_mut().unwrap().push(ch);
                }
            }
            None =>
                match ch {
                    '\'' | '"' => {
                        open_quote = Some(ch);
                    }
                    ' ' | '\t' => {
                        if !parts.last().unwrap().is_empty() {
                            parts.push(String::new());
                        }
                    }
                    _ => {
                        parts.last_mut().unwrap().push(ch);
                    }
                }
        }
    }

    if open_quote == None {
        if let Some(last_char) = input.chars().last() {
            if last_char == '\\' {
                open_quote = Some('z');
            }
        }
    }

    if let Some(quote) = open_quote {
        if quote == '"' {
            print!("dquote> ");
        } else if quote == '"' {
            print!("quote> ");
        } else {
            print!("> ");
            open_quote = None;
        }

        match io::stdout().flush() {
            Ok(_) => {}
            Err(r) => {
                print!("{r}");
                return;
            }
        }
        let mut new_input = String::new();
        match io::stdin().read_line(&mut new_input) {
            Ok(_) => {}
            Err(r) => {
                println!("{r}");
                return;
            }
        }
        split_input(new_input, parts, open_quote);
    }
}
