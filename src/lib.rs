/// Types of tokens.
#[derive(Debug, PartialEq)]
enum TokenKind {
    Plus,
    Minus,
    Star,
    Slash,
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
        self.source.chars().nth(self.current).unwrap()
    }

    /// Add a new token to the internal store with the given kind.
    fn add_token(&mut self, kind: TokenKind) -> () {
        let lexeme: String = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();
        self.tokens.push(Token::new(lexeme, kind));
    }

    /// Scan a new token.
    fn scan_token(&mut self) -> () {
        self.start = self.current;
        let c = self.advance();

        match c {
            ' ' | '\t' | '\n' | '\r' => (),
            '+' => self.add_token(TokenKind::Plus),
            '-' => self.add_token(TokenKind::Minus),
            '*' => self.add_token(TokenKind::Star),
            '/' => self.add_token(TokenKind::Slash),
            '(' => self.add_token(TokenKind::LeftParen),
            ')' => self.add_token(TokenKind::RightParen),
            _ => (),
        }
    }

    /// Scan the entire source code.
    fn scan(&mut self) -> () {
        while !self.is_at_end() {
            self.scan_token();
        }
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
        let mut lexer = Lexer::new("(+-)*/");
        let mut token;

        lexer.scan();

        token = lexer.tokens.pop().unwrap();
        assert_eq!(token, Token::new(String::from("/"), TokenKind::Slash));

        token = lexer.tokens.pop().unwrap();
        assert_eq!(token, Token::new(String::from("*"), TokenKind::Star));

        token = lexer.tokens.pop().unwrap();
        assert_eq!(token, Token::new(String::from(")"), TokenKind::RightParen));

        token = lexer.tokens.pop().unwrap();
        assert_eq!(token, Token::new(String::from("-"), TokenKind::Minus));

        token = lexer.tokens.pop().unwrap();
        assert_eq!(token, Token::new(String::from("+"), TokenKind::Plus));

        token = lexer.tokens.pop().unwrap();
        assert_eq!(token, Token::new(String::from("("), TokenKind::LeftParen));
    }
}

pub fn test_function() {
    println!("foo bar");
}
