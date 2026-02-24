mod env;
mod io_process;
mod lexer;

use crate::env::state::Environment;
use crate::io_process::input::read_input;
use crate::lexer::tokenize::tokenize;

fn main() {
    let mut env = Environment::new();
    env.loading_env();
    
    loop {
        match read_input() {
            Ok(input) => {
                if input.trim().is_empty() {
                    continue; // Skip empty input
                }
                let tokens = tokenize(&input);
                println!("Tokens: {:?}", tokens);
            },
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
}
