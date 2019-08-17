use mankailib::{MankaiError, Interpreter, Lexer, Parser};
use std::io;
use std::io::prelude::*;

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
