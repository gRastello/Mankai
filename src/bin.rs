use mankailib::{Interpreter, Lexer, ParseError, Parser, RuntimeError, ScanError};
use std::io;
use std::io::prelude::*;

struct MankaiError {
    /// Error message.
    message: String,
}

impl From<ScanError> for MankaiError {
    fn from(err: ScanError) -> Self {
        let mut message = String::new();
        message.push_str("lexing error at ");
        message.push_str(&err.position.to_string());
        message.push_str(": ");
        message.push_str(&err.message);

        MankaiError { message }
    }
}

impl From<ParseError> for MankaiError {
    fn from(err: ParseError) -> Self {
        let mut message = String::new();
        message.push_str("parsing error");
        if let Some(token) = err.token {
            message.push_str(" at '");
            message.push_str(&token.lexeme);
            message.push_str("'");
        }
        message.push_str(": ");
        message.push_str(&err.message);

        MankaiError { message }
    }
}

impl From<RuntimeError> for MankaiError {
    fn from(err: RuntimeError) -> Self {
        MankaiError {
            message: err.message,
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut interpreter = Interpreter {};

    for line in stdin.lock().lines() {
        if let Ok(source) = line {
            if let Err(err) = run(source, &mut interpreter) {
                eprintln!("{}", err.message);
            }
        }
    }
}

fn run(source: String, interpreter: &mut Interpreter) -> Result<(), MankaiError> {
    let mut lexer = Lexer::new(source);
    lexer.scan()?;

    let mut parser = Parser::new(lexer.tokens);
    let sexp = parser.parse()?;

    let value = interpreter.eval(&sexp)?;
    println!("{}", value.to_string());

    Ok(())
}
