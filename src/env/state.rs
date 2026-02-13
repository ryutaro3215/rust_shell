// This module defines the state management of shell
use std::collections::HashMap;
use std::env;

pub struct Environment {
    variables: HashMap<String, String>
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            variables: HashMap::new()
        }
    }

    pub fn loading_env(&mut self) {
        for (key, value) in env::vars() {
            self.valiables.insert(key, value);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loading_env() {
        let mut env = Environment::new();
        env.loading_env();
        assert!(env.variables.len() > 0); // Assuming there are some environment variables
    }
}
