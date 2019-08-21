use crate::environment::Environment;
use crate::parser::Sexp;
use crate::token::*;

/// A runtime error.
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

#[derive(Clone)]
pub enum MankaiObject {
    Number(f64),
    String(String),
    List(Vec<MankaiObject>),
    Bool(bool),
    SpecialForm(fn(&mut Interpreter, Vec<&Sexp>) -> Result<MankaiObject, RuntimeError>),
    Native(fn(Vec<MankaiObject>) -> Result<MankaiObject, RuntimeError>),
    Function {
        name: Option<String>,
        arguments_identifiers: Vec<Token>,
        body: Sexp,
    },
}

impl std::fmt::Debug for MankaiObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MankaiObject::Number(n) => write!(f, "{}", n),
            MankaiObject::String(s) => write!(f, "{}", s),
            MankaiObject::List(list) => {
                write!(f, "( ")?;

                for elem in list {
                    elem.fmt(f)?;
                    write!(f, " ")?;
                }

                write!(f, " )")
            }
            MankaiObject::Bool(true) => write!(f, "true"),
            MankaiObject::Bool(false) => write!(f, "false"),
            MankaiObject::SpecialForm(_) => write!(f, "special form"),
            MankaiObject::Native(_) => write!(f, "native function"),
            MankaiObject::Function { .. } => write!(f, "user-defined function"),
        }
    }
}

impl PartialEq for MankaiObject {
    fn eq(&self, other: &Self) -> bool {
        match self {
            MankaiObject::Number(n) => match other {
                MankaiObject::Number(m) => n == m,
                _ => false,
            },
            MankaiObject::String(s) => match other {
                MankaiObject::String(t) => s == t,
                _ => false,
            },
            MankaiObject::List(l1) => match other {
                MankaiObject::List(l2) => l1 == l2,
                _ => false,
            },
            MankaiObject::Bool(b1) => match other {
                MankaiObject::Bool(b2) => b1 == b2,
                _ => false,
            },
            MankaiObject::SpecialForm(_) => false,
            MankaiObject::Native(_) => false,
            MankaiObject::Function { .. } => false,
        }
    }
}

impl ToString for MankaiObject {
    fn to_string(&self) -> String {
        match self {
            MankaiObject::Number(n) => n.to_string(),
            MankaiObject::String(s) => format!("\"{}\"", s),
            MankaiObject::List(list) => {
                let mut s = String::from("(");
                for (i, elem) in list.iter().enumerate() {
                    if i != 0 {
                        s.push(' ');
                    }
                    s.push_str(&elem.to_string());
                }
                s.push(')');

                s
            }
            MankaiObject::Bool(true) => String::from("true"),
            MankaiObject::Bool(false) => String::from("false"),
            MankaiObject::SpecialForm(_) => String::from("<special form>"),
            MankaiObject::Native(_) => String::from("<native function>"),
            MankaiObject::Function { .. } => String::from("<user-defined fucntion>"),
        }
    }
}

impl MankaiObject {
    /// Call the object with arguments.
    /// It the object is a function call it, if it's something else report a
    /// runtime error.
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<MankaiObject>,
    ) -> Result<MankaiObject, RuntimeError> {
        match self {
            MankaiObject::Native(function) => function(arguments),
            MankaiObject::Function {
                name,
                arguments_identifiers,
                body,
            } => {
                // Arity check.
                if arguments_identifiers.len() != arguments.len() {
                    let function_name = match name {
                        Some(string) => string,
                        None => "anonymous function",
                    };

                    return Err(RuntimeError::new(&format!(
                        "found {} arguments but '{}' requires {}!",
                        arguments.len(),
                        function_name,
                        arguments_identifiers.len()
                    )));
                }

                // Extend the environment.
                interpreter.environment.extend();

                for (identifier, value) in arguments_identifiers.iter().zip(arguments.iter()) {
                    interpreter.environment.define(identifier, value.clone());
                }

                // Evaluate the body of the function.
                let result = interpreter.evaluate(body);

                // Restrict the environment and return.
                interpreter.environment.restrict();
                result
            }
            _ => Err(RuntimeError::new(&format!(
                "'{}' is not callable!",
                self.to_string()
            ))),
        }
    }
}

/// A Mankai interepreter.
pub struct Interpreter {
    /// The environment.
    pub environment: Environment,
    /// Vector of reserved names for special forms.
    special_forms: Vec<String>,
    /// Vector of reserved names for native functions.
    native_functions: Vec<String>,
    /// Vector of reserved names for constants.
    constants: Vec<String>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Interpreter {
            environment: Environment::new(),
            special_forms: vec![
                String::from("if!"),
                String::from("lambda!"),
                String::from("set!"),
            ],
            native_functions: vec![
                String::from("+"),
                String::from("-"),
                String::from("*"),
                String::from("/"),
                String::from("="),
                String::from(">"),
                String::from("<"),
                String::from("and"),
                String::from("car"),
                String::from("cdr"),
                String::from("cons"),
                String::from("bool?"),
                String::from("list?"),
                String::from("number?"),
                String::from("string?"),
                String::from("list"),
                String::from("not"),
                String::from("or"),
                String::from("string-concat"),
                String::from("to-string"),
            ],
            constants: vec![String::from("true"), String::from("false")],
        }
    }
}

impl Interpreter {
    /// Create a new interpreter.
    pub fn new() -> Self {
        Interpreter::default()
    }

    /// Check if the identifier is reserved for a special form.
    pub fn is_special_form(&self, identifier: &Token) -> bool {
        self.special_forms.iter().any(|s| *s == identifier.lexeme)
    }

    /// Check if the identifier is reserved for a native function.
    pub fn is_native_fucntion(&self, identifier: &Token) -> bool {
        self.native_functions
            .iter()
            .any(|s| *s == identifier.lexeme)
    }

    /// Check if the identifier is reserved for a constant.
    pub fn is_constant(&self, identifier: &Token) -> bool {
        self.constants.iter().any(|s| *s == identifier.lexeme)
    }

    /// Evaluate an atom.
    fn evaluate_atom(&self, atom: &Token) -> Result<MankaiObject, RuntimeError> {
        match &atom.kind {
            TokenKind::Number(n) => Ok(MankaiObject::Number(*n)),
            TokenKind::String(s) => Ok(MankaiObject::String(s.to_string())),
            TokenKind::Identifier => self.environment.get(atom),
            _ => Err(RuntimeError::new("failed to convert atom to value")),
        }
    }

    /// Evaluate a list: can result in evaluating a special form or a function
    /// (user-defined or native).
    fn evaluate_list(&mut self, list: &[Sexp]) -> Result<MankaiObject, RuntimeError> {
        let callee = self.evaluate(list.get(0).unwrap())?;
        let arguments: Vec<&Sexp> = list.iter().skip(1).collect();

        match callee {
            MankaiObject::SpecialForm(special_form) => special_form(self, arguments),
            _ => {
                // Evaluate the arguments.
                let mut evaluated_arguments = Vec::new();
                for expr in arguments {
                    let value = self.evaluate(expr)?;
                    evaluated_arguments.push(value);
                }

                // Call the function.
                callee.call(self, evaluated_arguments)
            }
        }
    }

    /// Evaluate an expression.
    pub fn evaluate(&mut self, expr: &Sexp) -> Result<MankaiObject, RuntimeError> {
        match expr {
            Sexp::Atom(token) => self.evaluate_atom(token),
            Sexp::List(list) => self.evaluate_list(list),
        }
    }
}

#[cfg(test)]
mod interpreter_test {
    use super::{Interpreter, MankaiObject};
    use crate::lexer::Lexer;
    use crate::parser::{Parser, Sexp};
    use crate::token::*;

    #[test]
    fn atom_evaluation() {
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

    #[test]
    fn set_special_form() {
        let mut lexer = Lexer::new(String::from("(set! foo \"bar\")"));
        if let Err(err) = lexer.scan() {
            panic!(err.message);
        }

        let mut parser = Parser::new(lexer.tokens);
        let mut interpreter = Interpreter::new();

        match parser.parse() {
            Ok(expr) => match interpreter.evaluate(&expr) {
                Ok(value) => assert_eq!(value, MankaiObject::String(String::from("bar"))),
                Err(err) => panic!(err.message),
            },
            Err(err) => panic!(err.message),
        }

        lexer = Lexer::new(String::from("foo"));
        if let Err(err) = lexer.scan() {
            panic!(err.message);
        }

        parser = Parser::new(lexer.tokens);

        match parser.parse() {
            Ok(expr) => match interpreter.evaluate(&expr) {
                Ok(value) => assert_eq!(value, MankaiObject::String(String::from("bar"))),
                Err(err) => panic!(err.message),
            },
            Err(err) => panic!(err.message),
        }
    }

    #[test]
    fn anonymous_function_call() {
        let mut lexer = Lexer::new(String::from("(my-addition 1 2)"));
        if let Err(err) = lexer.scan() {
            panic!(err.message);
        }

        let mut parser = Parser::new(lexer.tokens);
        let mut interpreter = Interpreter::new();

        // Bring to scope "my-addition": a function that performs addition of
        // two numbers using the native '+'.
        interpreter.environment.define(
            &Token::new(String::from("my-addition"), TokenKind::Identifier),
            MankaiObject::Function {
                name: Some(String::from("my-addition")),
                arguments_identifiers: vec![
                    Token::new(String::from("first"), TokenKind::Identifier),
                    Token::new(String::from("second"), TokenKind::Identifier),
                ],
                body: Sexp::List(vec![
                    Sexp::Atom(Token::new(String::from("+"), TokenKind::Identifier)),
                    Sexp::Atom(Token::new(String::from("first"), TokenKind::Identifier)),
                    Sexp::Atom(Token::new(String::from("second"), TokenKind::Identifier)),
                ]),
            },
        );

        match parser.parse() {
            Ok(expr) => match interpreter.evaluate(&expr) {
                Ok(value) => assert_eq!(value, MankaiObject::Number(3.0)),
                Err(err) => panic!(err.message),
            },
            Err(err) => panic!(err.message),
        }
    }
}
