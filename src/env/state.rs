// This module defines the state management of shell
use std::collections::HashMap;
use std::env;
use errno::{Errno, errno, set_errno};

pub struct Environment {
    variables: HashMap<String, String>,
    errno: Errno
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            variables: HashMap::new(),
            errno: errno()
        }
    }

    pub fn loading_env(&mut self) {
        for (key, value) in env::vars() {
            self.variables.insert(key, value);
        }
    }

    pub fn get_specific_env(&self, key: &str) -> Option<&String> {
        self.variables.get(key)
    }

    pub fn set_specific_env(&mut self, key: &str, value: &str) {
        self.variables.insert(key.to_string(), value.to_string());
    }

    pub fn get_errno(&self) -> Errno {
        self.errno
    }

    pub fn set_errno(&mut self, err: Errno) {
        self.errno = err;
        set_errno(err);
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

    #[test]
    pub fn test_get_specific_env() {
        let mut env = Environment::new();
        env.loading_env();
        let path = env.get_specific_env("PATH");
        dbg!(path);
        assert!(path.is_some()); // Assuming PATH is set in the environment
    }

    #[test]
    pub  fn test_set_specific_env() {
        let mut env = Environment::new();
        env.set_specific_env("TEST_VAR", "test_value");
        let value = env.get_specific_env("TEST_VAR");
        dbg!(value);
        assert_eq!(value, Some(&"test_value".to_string()));
    }

    #[test]
    pub fn test_get_errno() {
        let env = Environment::new();
        let err = env.get_errno();
        dbg!(err);
        assert_eq!(err, errno());
    }

    #[test]
    pub fn test_set_errno() {
        let mut env = Environment::new();
        env.set_errno(Errno(1)); // Set to a specific error code
        let err = env.get_errno();
        dbg!(err);
        assert_eq!(err, Errno(1));
    }
}
