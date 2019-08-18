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

impl MankaiObject {
    /// Create a new MankaiObject from a token.
    fn from_token(token: &Token) -> Result<Self, RuntimeError> {
        match &token.kind {
            TokenKind::Number(n) => Ok(MankaiObject::Number(*n)),
            TokenKind::String(s) => Ok(MankaiObject::String(s.to_string())),
            _ => Err(RuntimeError::new("failed to convert atom to value")),
        }
    }
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

    /// Evaluate an expression.
    pub fn evaluate(&mut self, expr: &Sexp) -> Result<MankaiObject, RuntimeError> {
        match expr {
            Sexp::Atom(token) => MankaiObject::from_token(token),
            Sexp::List(_) => Err(RuntimeError::new("I can't evaluate function calls (yet)")),
        }
    }
}

mod interpreter_test {
    use super::{Interpreter, MankaiObject};
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn atom_evaluating() {
        // Number literal.
        let mut lexer = Lexer::new(String::from("5"));
        if let Err(err) = lexer.scan() {
            panic!(err);
        }

        let mut parser = Parser::new(lexer.tokens);
        let mut interpreter = Interpreter {};

        match parser.parse() {
            Ok(expr) => match interpreter.eval(&expr) {
                Ok(value) => assert_eq!(value, MankaiObject::Number(5.0)),
                Err(err) => panic!(err),
            },
            Err(err) => panic!(err),
        }

        // String literal.
        lexer = Lexer::new(String::from("\"foo\""));
        if let Err(err) = lexer.scan() {
            panic!(err);
        }

        parser = Parser::new(lexer.tokens);
        interpreter = Interpreter {};

        match parser.parse() {
            Ok(expr) => match interpreter.eval(&expr) {
                Ok(value) => assert_eq!(value, MankaiObject::String(String::from("foo"))),
                Err(err) => panic!(err),
            },
            Err(err) => panic!(err),
        }
    }
}
