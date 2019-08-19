use crate::token::*;

/// An S-expression (sexp for brevity).
#[derive(Debug, PartialEq)]
pub enum Sexp {
    Atom(Token),
    List(Vec<Sexp>),
}

/// A parsing error.
#[derive(Debug, PartialEq)]
pub struct ParseError {
    /// Error message.
    pub message: String,
    /// Problematic token,
    pub token: Option<Token>,
}

impl ParseError {
    /// Make a new "full" error.
    fn new(message: &str, token: &Token) -> Self {
        ParseError {
            message: String::from(message),
            token: Some(token.clone()),
        }
    }

    /// Make a new error from just the message.
    fn from_message(message: &str) -> Self {
        ParseError {
            message: String::from(message),
            token: None,
        }
    }
}

/// The parser.
pub struct Parser {
    /// Token stream to parse.
    tokens: Vec<Token>,
    /// Current token.
    current: usize,
}

impl Parser {
    /// Make a new parser from a token stream.
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    /// Check if the parset has reached the end of the token stream.
    fn is_at_end(&self) -> bool {
        self.tokens.get(self.current).unwrap().kind == TokenKind::Eof
    }

    /// Advance the parser.
    fn advance(&mut self) -> &Token {
        self.current += 1;
        self.tokens.get(self.current - 1).unwrap()
    }

    /// Peek the next token without advancing the parser.
    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    /// Finish parsing a list.
    fn finish_list(&mut self) -> Result<Sexp, ParseError> {
        let mut sexps = Vec::new();
        sexps.push(self.parse_sexp()?);

        while self.peek().kind != TokenKind::RightParen && !self.is_at_end() {
            sexps.push(self.parse_sexp()?);
        }

        if self.peek().kind != TokenKind::RightParen {
            Err(ParseError::new("expected ')'", self.peek()))
        } else {
            self.current += 1;
            Ok(Sexp::List(sexps))
        }
    }

    /// Parse a single sexp.
    fn parse_sexp(&mut self) -> Result<Sexp, ParseError> {
        let token = self.advance();

        match token.kind {
            TokenKind::LeftParen => self.finish_list(),
            TokenKind::RightParen => Err(ParseError::new("expected atom or list", token)),
            _ => Ok(Sexp::Atom(token.clone())),
        }
    }

    pub fn parse(&mut self) -> Result<Sexp, ParseError> {
        if !self.is_at_end() {
            self.parse_sexp()
        } else {
            Err(ParseError::from_message("no tokens!"))
        }
    }
}

#[cfg(test)]
mod parser_test {
    use super::{ParseError, Parser, Sexp, Token, TokenKind};
    use crate::lexer::Lexer;

    #[test]
    fn parser_initialization_and_basic_operations() {
        let mut lexer = Lexer::new(String::from("(foo)"));
        if let Err(err) = lexer.scan() {
            panic!(err);
        }

        let mut parser = Parser::new(lexer.tokens);
        let mut token;

        token = parser.advance().clone();
        assert_eq!(token, Token::new(String::from("("), TokenKind::LeftParen));

        token = parser.advance().clone();
        assert_eq!(
            token,
            Token::new(String::from("foo"), TokenKind::Identifier)
        );

        token = parser.advance().clone();
        assert_eq!(token, Token::new(String::from(")"), TokenKind::RightParen));
    }

    #[test]
    fn parsing() {
        let mut lexer = Lexer::new(String::from("(car (\"2\" 3) \"foo\" 12.0)"));
        if let Err(err) = lexer.scan() {
            panic!(err);
        }

        let mut parser = Parser::new(lexer.tokens);
        match parser.parse() {
            Ok(sexp) => match sexp {
                Sexp::List(list) => {
                    // Check the first element of the list i.e. `car`.
                    if let Sexp::Atom(token) = list.get(0).unwrap() {
                        assert_eq!(
                            token.clone(),
                            Token::new(String::from("car"), TokenKind::Identifier)
                        );
                    } else {
                        panic!("expected atom!");
                    }

                    // Check that the second element is a list.
                    if let Sexp::List(_) = list.get(1).unwrap() {

                    } else {
                        panic!("expected list!");
                    }

                    // Check the third element of the list i.e. `"foo"`.
                    if let Sexp::Atom(token) = list.get(2).unwrap() {
                        assert_eq!(
                            token.clone(),
                            Token::new(
                                String::from("\"foo\""),
                                TokenKind::String(String::from("foo"))
                            )
                        );
                    } else {
                        panic!("expected atom!");
                    }

                    // Check the fourth element of the list i.e. 12.0.
                    if let Sexp::Atom(token) = list.get(3).unwrap() {
                        assert_eq!(
                            token.clone(),
                            Token::new(String::from("12.0"), TokenKind::Number(12.0))
                        );
                    } else {
                        panic!("expected atom!");
                    }
                }
                Sexp::Atom(_) => panic!("expected list!"),
            },
            Err(err) => panic!(err),
        }
    }

    #[test]
    fn unbalanced_expression() {
        let mut lexer = Lexer::new(String::from("(foo bar 32.66"));
        if let Err(err) = lexer.scan() {
            panic!(err);
        }

        let mut parser = Parser::new(lexer.tokens);
        match parser.parse() {
            Ok(_) => panic!("expected to fail parsing!"),
            Err(err) => assert_eq!(
                err,
                ParseError::new(
                    "expected ')'",
                    &Token::new(String::from(""), TokenKind::Eof)
                )
            ),
        }
        if let Ok(_) = parser.parse() {}
    }
}
