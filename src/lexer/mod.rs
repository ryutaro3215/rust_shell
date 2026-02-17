pub use tokenize;




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let input = "echo Hello, World!";
        let tokens = tokenize(input);
        assert_eq!(tokens, vec!["echo", "Hello,", "World!"]);
    }
}
