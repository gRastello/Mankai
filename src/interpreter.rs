use crate::environment::Environment;
use crate::parser::Sexp;
use crate::token::*;

pub struct RuntimeError {
    /// Error message.
    pub message: String,
}

impl RuntimeError {
    pub fn new(message: &str) -> Self {
        RuntimeError {
            message: String::from(message),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MankaiObject {
    Number(f64),
    String(String),
    // Function and NativeFunction
}

impl ToString for MankaiObject {
    fn to_string(&self) -> String {
        match self {
            MankaiObject::Number(n) => n.to_string(),
            MankaiObject::String(s) => s.to_string(),
        }
    }
}

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    /// Create a new interpreter.
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
        }
    }

    /// Evaluate an atom.
    pub fn evaluate_atom(&self, atom: &Token) -> Result<MankaiObject, RuntimeError> {
        match &atom.kind {
            TokenKind::Number(n) => Ok(MankaiObject::Number(*n)),
            TokenKind::String(s) => Ok(MankaiObject::String(s.to_string())),
            TokenKind::Identifier => self.environment.get(atom),
            _ => Err(RuntimeError::new("failed to convert atom to value")),
        }
    }

    /// Evaluate an expression.
    pub fn evaluate(&mut self, expr: &Sexp) -> Result<MankaiObject, RuntimeError> {
        match expr {
            Sexp::Atom(token) => self.evaluate_atom(token), // MankaiObject::from_token(token),
            Sexp::List(_) => Err(RuntimeError::new("I can't evaluate function calls (yet)")),
        }
    }
}

mod interpreter_test {
    use super::{Interpreter, MankaiObject};
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::token::*;

    #[test]
    fn atom_evaluating() {
        // Number literal.
        let mut lexer = Lexer::new(String::from("5"));
        if let Err(err) = lexer.scan() {
            panic!(err.message);
        }

        let mut parser = Parser::new(lexer.tokens);
        let mut interpreter = Interpreter::new();

        match parser.parse() {
            Ok(expr) => match interpreter.evaluate(&expr) {
                Ok(value) => assert_eq!(value, MankaiObject::Number(5.0)),
                Err(err) => panic!(err.message),
            },
            Err(err) => panic!(err.message),
        }

        // String literal.
        lexer = Lexer::new(String::from("\"foo\""));
        if let Err(err) = lexer.scan() {
            panic!(err.message);
        }

        parser = Parser::new(lexer.tokens);
        interpreter = Interpreter::new();

        match parser.parse() {
            Ok(expr) => match interpreter.evaluate(&expr) {
                Ok(value) => assert_eq!(value, MankaiObject::String(String::from("foo"))),
                Err(err) => panic!(err.message),
            },
            Err(err) => panic!(err.message),
        }

        // Symbol non-binded.
        lexer = Lexer::new(String::from("foo"));
        if let Err(err) = lexer.scan() {
            panic!(err.message);
        }

        parser = Parser::new(lexer.tokens);
        interpreter = Interpreter::new();

        match parser.parse() {
            Ok(expr) => {
                if let Ok(_) = interpreter.evaluate(&expr) {
                    panic!("found nonexistent bidning");
                }
            }
            Err(err) => panic!(err.message),
        }

        // Symbol binded.
        lexer = Lexer::new(String::from("bar"));
        if let Err(err) = lexer.scan() {
            panic!(err.message);
        }

        parser = Parser::new(lexer.tokens);
        interpreter = Interpreter::new();

        interpreter.environment.define(
            &Token::new(String::from("bar"), TokenKind::Identifier),
            MankaiObject::Number(2.0),
        );

        match parser.parse() {
            Ok(expr) => match interpreter.evaluate(&expr) {
                Ok(value) => assert_eq!(value, MankaiObject::Number(2.0)),
                Err(err) => panic!(err.message),
            },
            Err(err) => panic!(err.message),
        }
    }
}
