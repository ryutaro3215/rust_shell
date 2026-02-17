use std::io::{self, Write};

// Reads a line of input from standard input and returns it as a String trimmed of WhiteSpace and
// newline characters. If an error occurs while reading, it returns the error.

//In this function, we create a mutabel String variable.
pub fn read_input() -> io::Result<String> {
    let mut input = String::new();
    let mut buffer = String::new();
    let mut is_continuation = false;

    loop {
        if !is_quote_closed(&input) {
            print!("dquote> ");
        } else if input.contains('|') {
            print!("pipe> ");
        } else if is_continuation {
            print!("> ");
        } else {
            print!("$ ");
        } 
        io::stdout().flush()?;
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                // Trim newline char from the end of the input and return it as a String.
                // There are three situations 
                // 1. Normal input. It is one line and the user presses Enter.
                // 2. The User input a backslash at the end of the line. In this case, we should
                //    read the next line and concatenate it with the current input.
                // 3. The user input a line with unclosed quotes. In this case, we should read the
                //    next line until the quotes are closed.
                let trimmed_input = buffer.trim().to_string();
                if !is_quote_closed(&trimmed_input) {
                    if trimmed_input.ends_with('\\') {
                        input.push_str(&trimmed_input[..trimmed_input.len() - 1]);
                        is_continuation = true;
                    } else {
                        input.push_str(&trimmed_input);
                        input.push('\n');
                        is_continuation = false;
                    }
                    buffer.clear();
                    if is_quote_closed(&input) && !is_continuation {
                        break;
                    }
                } else if trimmed_input.ends_with('\\') {
                    input.push_str(&trimmed_input[..trimmed_input.len() - 1]);
                    buffer.clear();
                    is_continuation = true;
                } else {
                    input.push_str(&trimmed_input);
                    break;
                }
            },
            Err(e) => return Err(e),
        }
    }
    Ok(input)
}

pub fn is_quote_closed(input: &str) -> bool {
    let mut stack = Vec::new();

    for c in input.chars() {
        match c {
            '\'' => {
                if let Some(&last) = stack.last() {
                    if last == '\'' {
                        stack.pop();
                    } else {
                        stack.push(c);
                    }
                } else {
                    stack.push(c);
                }
            }
            '"' => {
                if let Some(&last) = stack.last() {
                    if last == '"' {
                        stack.pop();
                    } else {
                        stack.push(c);
                    }
                } else {
                    stack.push(c);
                }
            }
            _ => {}
        }
    }

    if stack.is_empty() {
        true 
    } else {
        false
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_read_input() {
        let input = read_input();
        println!("Input: {:?}", input);
    }
}










