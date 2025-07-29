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
    let mut escape = false;
    let mut chars = input.chars().peekable();
    while let Some(ch) = chars.next() {
        if escape {
            parts.last_mut().unwrap().push(ch);
            escape = false;
            continue;
        }

        match ch {
            '\\' => {
                if chars.peek().is_some() {
                    escape = true;
                }
            }
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

    if open_quote == None {
        if let Some(last_char) = input.chars().last() {
            if last_char == '\\' {
                open_quote = Some('z');
            }
        }
    }

    if let Some(quote) = open_quote {
        if quote == '"' {
            parts.last_mut().unwrap().push('\n');
            print!("dquote> ");
        } else if quote == '\'' {
            parts.last_mut().unwrap().push('\n');
            print!("quote> ");
        } else {
            print!("> ");
            open_quote = None;
        }

        io::stdout().flush().unwrap();
        let mut new_input = String::new();
        if io::stdin().read_line(&mut new_input).is_ok() {
            split_input(new_input, parts, open_quote);
        }
    }
}
