use std::collections::HashMap;

use crate::interpreter::{MankaiObject, RuntimeError};
use crate::native_functions;
use crate::special_forms;
use crate::token::*;

pub struct Environment {
    /// Layers maps one-to-one to scopes. Thus the first layer is the global
    /// scope.
    layers: Vec<HashMap<String, MankaiObject>>,
}

impl Environment {
    /// Make a new environment.
    pub fn new() -> Self {
        // Make a new environment and a void global scope.
        let mut environment = Environment { layers: Vec::new() };
        environment.layers.push(HashMap::new());

        // Bring to scope some special forms.
        let if_special_form = MankaiObject::SpecialForm(special_forms::if_special_form);
        environment.define(
            &Token::new(String::from("if!"), TokenKind::Identifier),
            if_special_form,
        );

        let lambda = MankaiObject::SpecialForm(special_forms::lambda);
        environment.define(
            &Token::new(String::from("lambda!"), TokenKind::Identifier),
            lambda,
        );

        let set = MankaiObject::SpecialForm(special_forms::set);
        environment.define(
            &Token::new(String::from("set!"), TokenKind::Identifier),
            set,
        );

        // Bring to scope some native functions.
        let sum = MankaiObject::Native(native_functions::sum);
        environment.define(&Token::new(String::from("+"), TokenKind::Identifier), sum);

        let substraction = MankaiObject::Native(native_functions::substract);
        environment.define(
            &Token::new(String::from("-"), TokenKind::Identifier),
            substraction,
        );

        let multiplication = MankaiObject::Native(native_functions::multiplication);
        environment.define(
            &Token::new(String::from("*"), TokenKind::Identifier),
            multiplication,
        );

        let division = MankaiObject::Native(native_functions::division);
        environment.define(
            &Token::new(String::from("/"), TokenKind::Identifier),
            division,
        );

        let equals = MankaiObject::Native(native_functions::equals);
        environment.define(
            &Token::new(String::from("="), TokenKind::Identifier),
            equals,
        );

        let greater_than = MankaiObject::Native(native_functions::greater_than);
        environment.define(
            &Token::new(String::from(">"), TokenKind::Identifier),
            greater_than,
        );

        let is_boolean = MankaiObject::Native(native_functions::is_boolean);
        environment.define(
            &Token::new(String::from("bool?"), TokenKind::Identifier),
            is_boolean,
        );

        let is_list = MankaiObject::Native(native_functions::is_list);
        environment.define(
            &Token::new(String::from("list?"), TokenKind::Identifier),
            is_list,
        );

        let is_number = MankaiObject::Native(native_functions::is_number);
        environment.define(
            &Token::new(String::from("number?"), TokenKind::Identifier),
            is_number,
        );

        let is_string = MankaiObject::Native(native_functions::is_string);
        environment.define(
            &Token::new(String::from("string?"), TokenKind::Identifier),
            is_string,
        );

        let less_than = MankaiObject::Native(native_functions::less_than);
        environment.define(
            &Token::new(String::from("<"), TokenKind::Identifier),
            less_than,
        );

        let and = MankaiObject::Native(native_functions::and);
        environment.define(&Token::new(String::from("and"), TokenKind::Identifier), and);

        let car = MankaiObject::Native(native_functions::car);
        environment.define(&Token::new(String::from("car"), TokenKind::Identifier), car);

        let cdr = MankaiObject::Native(native_functions::cdr);
        environment.define(&Token::new(String::from("cdr"), TokenKind::Identifier), cdr);

        let cons = MankaiObject::Native(native_functions::cons);
        environment.define(
            &Token::new(String::from("cons"), TokenKind::Identifier),
            cons,
        );

        let list = MankaiObject::Native(native_functions::list);
        environment.define(
            &Token::new(String::from("list"), TokenKind::Identifier),
            list,
        );

        let not = MankaiObject::Native(native_functions::not);
        environment.define(&Token::new(String::from("not"), TokenKind::Identifier), not);

        let or = MankaiObject::Native(native_functions::or);
        environment.define(&Token::new(String::from("or"), TokenKind::Identifier), or);

        let string_concat = MankaiObject::Native(native_functions::string_concat);
        environment.define(
            &Token::new(String::from("string-concat"), TokenKind::Identifier),
            string_concat,
        );

        let to_string = MankaiObject::Native(native_functions::to_string);
        environment.define(
            &Token::new(String::from("to-string"), TokenKind::Identifier),
            to_string,
        );

        // Bring to scope some constants.
        environment.define(
            &Token::new(String::from("true"), TokenKind::Identifier),
            MankaiObject::Bool(true),
        );

        environment.define(
            &Token::new(String::from("false"), TokenKind::Identifier),
            MankaiObject::Bool(false),
        );

        environment
    }

    /// Define a new binding.
    pub fn define(&mut self, identifier: &Token, value: MankaiObject) {
        if let Some(layer) = self.layers.last_mut() {
            layer.insert(identifier.lexeme.clone(), value);
        } else {
            panic!("the environment has no layer!");
        }
    }

    /// Get a value out of the environment.
    pub fn get(&self, identifier: &Token) -> Result<MankaiObject, RuntimeError> {
        // Start searching for the key from the outermost layer.
        for layer in self.layers.iter().rev() {
            if let Some(value) = layer.get(&identifier.lexeme) {
                return Ok(value.clone());
            }
        }

        // If nothing is found return a runtime errror.
        Err(RuntimeError::new(&format!(
            "unboud symbol '{}'",
            identifier.lexeme
        )))
    }

    /// Extend the environment with a new layer.
    pub fn extend(&mut self) {
        self.layers.push(HashMap::new());
    }

    /// Remove the last layer of the environment (panics if trying to remove the
    /// global scope).
    pub fn restrict(&mut self) {
        if self.layers.len() > 1 {
            self.layers.pop();
        } else {
            panic!("trying to remove global scope");
        }
    }
}

#[cfg(test)]
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
            &Token::new(String::from("bar"), TokenKind::Identifier),
            MankaiObject::String(String::from("baz")),
        );

        // Try to get them out and test runtime errors.
        match environment.get(&Token::new(String::from("foo"), TokenKind::Identifier)) {
            Ok(value) => assert_eq!(value, MankaiObject::Number(6.0)),
            Err(err) => panic!(err.message),
        }

        match environment.get(&Token::new(String::from("bar"), TokenKind::Identifier)) {
            Ok(value) => assert_eq!(value, MankaiObject::String(String::from("baz"))),
            Err(err) => panic!(err.message),
        }

        if let Ok(_) = environment.get(&Token::new(String::from("oof"), TokenKind::Identifier)) {
            panic!("found nonexistent binding");
        }
    }

    #[test]
    fn layers() {
        let mut environment = Environment::new();

        // Put something in the global scope.
        environment.define(
            &Token::new(String::from("foo"), TokenKind::Identifier),
            MankaiObject::Number(6.0),
        );

        environment.define(
            &Token::new(String::from("bar"), TokenKind::Identifier),
            MankaiObject::String(String::from("baz")),
        );

        // Extend the environment and define something again.
        environment.extend();

        environment.define(
            &Token::new(String::from("foo"), TokenKind::Identifier),
            MankaiObject::Number(12.0),
        );

        environment.define(
            &Token::new(String::from("baz"), TokenKind::Identifier),
            MankaiObject::Number(0.0),
        );

        // Check that the extended environment acts properly.
        match environment.get(&Token::new(String::from("foo"), TokenKind::Identifier)) {
            Ok(value) => assert_eq!(value, MankaiObject::Number(12.0)),
            Err(err) => panic!(err.message),
        }

        match environment.get(&Token::new(String::from("bar"), TokenKind::Identifier)) {
            Ok(value) => assert_eq!(value, MankaiObject::String(String::from("baz"))),
            Err(err) => panic!(err.message),
        }

        match environment.get(&Token::new(String::from("baz"), TokenKind::Identifier)) {
            Ok(value) => assert_eq!(value, MankaiObject::Number(0.0)),
            Err(err) => panic!(err.message),
        }

        // Restrict the environment.
        environment.restrict();

        // Check that the restricted environment acts properly.
        match environment.get(&Token::new(String::from("foo"), TokenKind::Identifier)) {
            Ok(value) => assert_eq!(value, MankaiObject::Number(6.0)),
            Err(err) => panic!(err.message),
        }

        match environment.get(&Token::new(String::from("bar"), TokenKind::Identifier)) {
            Ok(value) => assert_eq!(value, MankaiObject::String(String::from("baz"))),
            Err(err) => panic!(err.message),
        }
    }
}
