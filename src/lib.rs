/// Types of tokens.
#[derive(Debug, PartialEq)]
enum TokenKind {
    String(String),
    Number(f64),
    Identifier,
    LeftParen,
    RightParen,
    Eof,
}

/// A token.
#[derive(Debug, PartialEq)]
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
        self.start = self.current;
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
            self.scan_token()?;
        }

        Ok(())
    }
}

mod test {
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
        let mut lexer = Lexer::new("(bar \"foo\" baz) 64.333 12 foo");
        let mut token;

        lexer.scan();

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
            Token::new(String::from("bar"), TokenKind::Identifier)
        );

        token = lexer.tokens.pop().unwrap();
        assert_eq!(token, Token::new(String::from("("), TokenKind::LeftParen));
    }
}

pub fn test_function() {
    println!("foo bar");
}
