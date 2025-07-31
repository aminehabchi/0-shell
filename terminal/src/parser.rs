use std::io::{ self, Write };

pub fn parse_input(input: String) -> Vec<String> {
    let mut parts: Vec<String> = vec![String::new()];
    split_input(input, &mut parts, None);
    parts
}

pub fn split_input(input: String, parts: &mut Vec<String>, mut open_quote: Option<char>) {
    if parts.is_empty() {
        parts.push(String::new());
    }
    let mut chars = input.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '\'' | '"' => {
                if open_quote.is_some() {
                    if open_quote.unwrap() == ch {
                        open_quote = None;
                    } else {
                        parts.last_mut().unwrap().push(ch);
                    }
                } else {
                    open_quote = Some(ch);
                }
            }
            ' ' | '\t' => {
                if open_quote.is_some() {
                    parts.last_mut().unwrap().push(ch);
                } else if !parts.last().unwrap().is_empty() {
                    parts.push(String::new());
                }
            }
            _ => {
                parts.last_mut().unwrap().push(ch);
            }
        }
    }

    let mut is = false;
    if let Some(last_char) = input.chars().last() {
        if last_char == '\\' {
            is = true;
        }
    }

    if open_quote.is_some() || is {
        if let Some(quote) = open_quote && !is {
            if quote == '"' {
                print!("dquote> ");
            } else if quote == '\'' {
                print!("quote> ");
            }
            parts.last_mut().unwrap().push('\n');
        } else {
            print!("> ");
        }

        io::stdout().flush().unwrap();
        let mut new_input = String::new();
        if io::stdin().read_line(&mut new_input).is_ok() {
            let trimed = new_input.trim().to_string();
            split_input(trimed, parts, open_quote);
        }
    }
}
