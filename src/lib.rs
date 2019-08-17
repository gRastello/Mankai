/// Types of tokens.
#[derive(Debug, PartialEq, Clone)]
enum TokenKind {
    String(String),
    Number(f64),
    Identifier,
    LeftParen,
    RightParen,
    Eof,
}

/// A token.
#[derive(Debug, PartialEq, Clone)]
struct Token {
    /// Corresponding lexeme.
    lexeme: String,
    /// Kind of the token.
    kind: TokenKind,
}

impl Token {
    /// Create a new token from lexeme and kind.
    fn new(lexeme: String, kind: TokenKind) -> Self {
        Token { lexeme, kind }
    }
}

/// A lexing error.
struct ScanError {
    /// Error message.
    message: String,
    /// Start of the problematic token.
    position: usize,
}

impl ScanError {
    /// Make a new lexing error.
    fn new(message: &str, position: usize) -> Self {
        ScanError {
            message: String::from(message),
            position,
        }
    }
}

/// The lexer.
struct Lexer {
    /// The source code.
    source: String,
    /// The lexed tokens.
    tokens: Vec<Token>,
    /// Current token index.
    current: usize,
    /// Start of current lexeme.
    start: usize,
}

impl Lexer {
    /// Make a new lexer from some source code.
    fn new(source: &str) -> Self {
        Lexer {
            source: String::from(source),
            tokens: Vec::new(),
            current: 0,
            start: 0,
        }
    }

    /// Check if the lexer is at the end (or past the end) of the source code.
    fn is_at_end(&self) -> bool {
        self.current >= self.source.chars().count()
    }

    /// Advance the lexer.
    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    /// Peek the next character without advancing the lexer.
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    /// Peek the character adter the next one.
    fn peek_next(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    /// Return if the character is a separator (whitespace or parenthesis).
    fn is_separator(c: char) -> bool {
        c.is_whitespace() || c == '(' || c == ')'
    }

    /// Add a new token to the internal store with the given kind.
    fn add_token(&mut self, kind: TokenKind) {
        let lexeme: String = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();
        self.tokens.push(Token::new(lexeme, kind));
    }

    /// Tokenize a string.
    fn finish_string(&mut self) -> Result<(), ScanError> {
        loop {
            if self.is_at_end() {
                return Err(ScanError::new("unfinished string", self.start));
            }

            let next = self.advance();

            // Skip quoted characters, break if we hit the end of the string.
            if next == '\\' {
                self.current += 1;
            } else if next == '"' {
                break;
            }
        }

        let string: String = self
            .source
            .chars()
            .skip(self.start + 1)
            .take(self.current - self.start - 2)
            .collect();
        self.add_token(TokenKind::String(string));

        Ok(())
    }

    /// Tokenize a number.
    fn finish_number(&mut self) -> Result<(), ScanError> {
        // Consume the non-decimal part.
        while self.peek().is_digit(10) {
            self.current += 1;
        }

        // Check if the number has a decimal part.
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            // Consume the '.'.
            self.current += 1;

            // Consume the decimal part.
            while self.peek().is_digit(10) {
                self.current += 1;
            }
        }

        let number: f64 = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect::<String>()
            .parse()
            .unwrap();
        self.add_token(TokenKind::Number(number));

        Ok(())
    }

    /// Tokenize an identifier.
    fn finish_identifier(&mut self) -> Result<(), ScanError> {
        while !Lexer::is_separator(self.peek()) && !self.is_at_end() {
            self.current += 1;
        }

        self.add_token(TokenKind::Identifier);

        Ok(())
    }

    /// Scan a new token.
    fn scan_token(&mut self) -> Result<(), ScanError> {
        let c = self.advance();
        match c {
            ' ' | '\t' | '\n' | '\r' => Ok(()),
            '(' => Ok(self.add_token(TokenKind::LeftParen)),
            ')' => Ok(self.add_token(TokenKind::RightParen)),
            '"' => self.finish_string(),
            _ => {
                if c.is_digit(10) {
                    self.finish_number()
                } else {
                    self.finish_identifier()
                }
            }
        }
    }

    /// Scan the entire source code.
    fn scan(&mut self) -> Result<(), ScanError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.start = self.current;
        self.add_token(TokenKind::Eof);
        Ok(())
    }
}

mod lexer_test {
    use super::{Lexer, Token, TokenKind};

    #[test]
    fn lexer_initialization_and_basic_operations() {
        let mut lexer = Lexer::new("(foo)");

        assert_eq!(lexer.advance(), '(');
        assert_eq!(lexer.advance(), 'f');
        assert_eq!(lexer.advance(), 'o');
        assert_eq!(lexer.advance(), 'o');
        assert_eq!(lexer.peek(), ')');
        assert_eq!(lexer.is_at_end(), false);
        assert_eq!(lexer.advance(), ')');
        assert_eq!(lexer.is_at_end(), true);
    }

    #[test]
    fn lexing() {
        let mut lexer = Lexer::new("(*bar+ \"foo\" baz) 64.333 12 foo");
        let mut token;

        if let Err(err) = lexer.scan() {
            panic!(err);
        }

        token = lexer.tokens.pop().unwrap();
        assert_eq!(token, Token::new(String::from(""), TokenKind::Eof));

        token = lexer.tokens.pop().unwrap();
        assert_eq!(
            token,
            Token::new(String::from("foo"), TokenKind::Identifier)
        );

        token = lexer.tokens.pop().unwrap();
        assert_eq!(
            token,
            Token::new(String::from("12"), TokenKind::Number(12.0))
        );

        token = lexer.tokens.pop().unwrap();
        assert_eq!(
            token,
            Token::new(String::from("64.333"), TokenKind::Number(64.333))
        );

        token = lexer.tokens.pop().unwrap();
        assert_eq!(token, Token::new(String::from(")"), TokenKind::RightParen));

        token = lexer.tokens.pop().unwrap();
        assert_eq!(
            token,
            Token::new(String::from("baz"), TokenKind::Identifier)
        );

        token = lexer.tokens.pop().unwrap();
        assert_eq!(
            token,
            Token::new(
                String::from("\"foo\""),
                TokenKind::String(String::from("foo"))
            )
        );

        token = lexer.tokens.pop().unwrap();
        assert_eq!(
            token,
            Token::new(String::from("*bar+"), TokenKind::Identifier)
        );

        token = lexer.tokens.pop().unwrap();
        assert_eq!(token, Token::new(String::from("("), TokenKind::LeftParen));
    }
}

/// An S-expression (sexp for brevity).
#[derive(Debug, PartialEq)]
enum Sexp {
    Atom(Token),
    List(Vec<Sexp>),
}

/// A parsing error.
struct ParseError {
    /// Error message.
    message: String,
    /// Problematic token,
    token: Option<Token>,
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
struct Parser {
    /// Token stream to parse.
    tokens: Vec<Token>,
    /// Current token.
    current: usize,
}

impl Parser {
    /// Make a new parser from a token stream.
    fn new(tokens: Vec<Token>) -> Self {
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

    fn parse(&mut self) -> Result<Sexp, ParseError> {
        if !self.is_at_end() {
            self.parse_sexp()
        } else {
            Err(ParseError::from_message("no tokens!"))
        }
    }
}

mod parser_test {
    use super::{Lexer, Parser, Sexp, Token, TokenKind};

    #[test]
    fn parser_initialization_and_basic_operations() {
        let mut lexer = Lexer::new("(foo)");
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
        let mut lexer = Lexer::new("(car (\"2\" 3) \"foo\" 12.0)");
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
}

pub fn test_function() {
    let mut lexer = Lexer::new("(car (\"2\" 3) \"foo\" 12.0)");
    if let Err(err) = lexer.scan() {
        panic!(err);
    }

    let mut parser = Parser::new(lexer.tokens);
    match parser.parse() {
        Ok(sexp) => println!("{:?}", sexp),
        Err(err) => panic!(err),
    }
}
