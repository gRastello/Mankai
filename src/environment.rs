use std::collections::HashMap;

use crate::interpreter::{MankaiObject, RuntimeError};
use crate::token::Token;

pub struct Environment {
    bindings: HashMap<String, MankaiObject>,
}

impl Environment {
    /// Make a new environment.
    pub fn new() -> Self {
        Environment {
            bindings: HashMap::new(),
        }
    }

    /// Define a new binding.
    pub fn define(&mut self, identifier: &Token, value: MankaiObject) {
        self.bindings.insert(identifier.lexeme.clone(), value);
    }

    /// Get a value out of the environment.
    pub fn get(&self, identifier: &Token) -> Result<MankaiObject, RuntimeError> {
        match self.bindings.get(&identifier.lexeme) {
            Some(value) => Ok(value.clone()),
            None => Err(RuntimeError::new(&format!(
                "unboud symbol '{}'",
                identifier.lexeme
            ))),
        }
    }
}

mod environment_test {
    use super::Environment;
    use crate::interpreter::MankaiObject;
    use crate::token::*;

    #[test]
    fn define_and_get_bindings() {
        let mut environment = Environment::new();

        // Define a couple of bindings.
        environment.define(
            &Token::new(String::from("foo"), TokenKind::Identifier),
            MankaiObject::Number(6.0),
        );

        environment.define(
            &Token::new(String::from("bar"), TokenKind::String(String::from("baz"))),
            MankaiObject::String(String::from("baz")),
        );

        // Try to get them out and test runtime errors.
        match environment.get(&Token::new(String::from("foo"), TokenKind::Identifier)) {
            Ok(value) => assert_eq!(value, MankaiObject::Number(6.0)),
            Err(err) => panic!(err),
        }

        match environment.get(&Token::new(String::from("bar"), TokenKind::Identifier)) {
            Ok(value) => assert_eq!(value, MankaiObject::String(String::from("baz"))),
            Err(err) => panic!(err),
        }

        if let Ok(_) = environment.get(&Token::new(String::from("oof"), TokenKind::Identifier)) {
            panic!("found nonexistent binding");
        }
    }
}