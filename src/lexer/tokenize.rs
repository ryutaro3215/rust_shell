use crate::io_process::input::is_quote_closed;

//single meta character: | & ; ( ) < > { }
//double meta character: || && << >> 

pub fn is_meta_character(input: char) -> bool {
    let meta_characters = ['|', '&', ';', '(', ')', '<', '>', '{', '}'];
    if meta_characters.contains(&input) {
        true
    } else {
        false
    }
}

pub fn is_double_meta_character(current_token: &str, c: char) -> bool {
    let double_meta_characters = ["||", "&&", "<<", ">>"];
    let combined = format!("{}{}", current_token, c);
    if double_meta_characters.contains(&combined.as_str()) {
        true
    } else {
        false
    }
}


pub fn process_meta_character(tokens: &mut Vec<String>, current_token: &mut String, c: char) {
    if is_double_meta_character(current_token, c) {
        current_token.push(c);
        tokens.push(current_token.clone());
        current_token.clear();
    } else if is_meta_character(c) {
        if !current_token.is_empty() {
            tokens.push(current_token.clone());
            current_token.clear();
        }
        current_token.push(c);
    } else {
        if !current_token.is_empty() {
            if is_meta_character(current_token.chars().last().unwrap()) {
                tokens.push(current_token.clone());
                current_token.clear();
            }
        }
        current_token.push(c);
    }
}


pub fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut is_in_quote = false;

    for c in input.chars() {
        match c {
            ' ' if !is_in_quote => {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
            },
            _ => {
                if is_in_quote {
                    current_token.push(c);
                } else {
                    process_meta_character(&mut tokens, &mut current_token, c);
                }
            }
        }
        if !is_quote_closed(&current_token) {
            is_in_quote = true;
        } else {
            is_in_quote = false;
        }
    }
    if !current_token.is_empty() {
        tokens.push(current_token);
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_meta_character() {
        assert!(is_meta_character('|'));
        assert!(is_meta_character('&'));
        assert!(is_meta_character(';'));
        assert!(is_meta_character('('));
        assert!(is_meta_character(')'));
        assert!(is_meta_character('<'));
        assert!(is_meta_character('>'));
        assert!(is_meta_character('{'));
        assert!(is_meta_character('}'));
        assert!(!is_meta_character('a'));
    }

    #[test]
    fn test_is_double_meta_character() {
        assert!(is_double_meta_character("|", '|'));
        assert!(is_double_meta_character("&", '&'));
        assert!(is_double_meta_character("<", '<'));
        assert!(is_double_meta_character(">", '>'));
        assert!(!is_double_meta_character("|", '&'));
        assert!(!is_double_meta_character("&", '|'));
        assert!(!is_double_meta_character("<", '>'));
    }

    #[test]
    fn test_process_meta_character() {
        let mut tokens = Vec::new();
        let mut current_token = String::new();

        current_token.push('|');
        process_meta_character(&mut tokens, &mut current_token, '|');
        assert_eq!(tokens, vec!["||"]);
        assert_eq!(current_token, "");

        current_token.push('&');
        process_meta_character(&mut tokens, &mut current_token, '&');
        assert_eq!(tokens, vec!["||", "&&"]);
        assert_eq!(current_token, "");

        current_token.push('>');
        process_meta_character(&mut tokens, &mut current_token, '>');
        assert_eq!(tokens, vec!["||", "&&", ">>"]);
        assert_eq!(current_token, "");

        current_token.push('<');
        process_meta_character(&mut tokens, &mut current_token, '<');
        assert_eq!(tokens, vec!["||", "&&", ">>", "<<"]);
        assert_eq!(current_token, "");

        current_token.push_str("command");
        process_meta_character(&mut tokens, &mut current_token, '|');
        assert_eq!(tokens, vec!["||", "&&", ">>", "<<", "command"]);
        assert_eq!(current_token, "|");
        current_token.clear();

        process_meta_character(&mut tokens, &mut current_token, '&');
        assert_eq!(tokens, vec!["||", "&&", ">>", "<<", "command"]);
        assert_eq!(current_token, "&");

        process_meta_character(&mut tokens, &mut current_token, '&');
        assert_eq!(tokens, vec!["||", "&&", ">>", "<<", "command", "&&"]);
        assert_eq!(current_token, "");
    }

    #[test]
    fn test_tokenize() {
        let input = "echo \"Hello World\" && ls -l | grep \"test\" > output.txt";
        let expected_tokens = vec![
            "echo", "\"Hello World\"", "&&", "ls", "-l", "|", "grep", "\"test\"", ">", "output.txt"
        ];
        assert_eq!(tokenize(input), expected_tokens);

        let input2 = "echo \"hello world\"|cat -e";
        let expected_tokens2 = vec![
            "echo", "\"hello world\"", "|", "cat", "-e"
        ];
        assert_eq!(tokenize(input2), expected_tokens2);

        let input3 = "echo \"hello world\"|cat -e > output.txt";
        let expected_tokens3 = vec![
            "echo", "\"hello world\"", "|", "cat", "-e", ">", "output.txt"
        ];
        assert_eq!(tokenize(input3), expected_tokens3);
    }
}
