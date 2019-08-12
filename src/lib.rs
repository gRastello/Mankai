/// Types of tokens.
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
struct Token {
    /// Corresponding lexeme.
    lexeme: String,
    /// Kind of the token.
    kind: TokenKind,
}

impl Token {
    /// Create a new token from lexeme and kind.
    fn new(lexeme: &str, kind: TokenKind) -> Self {
        Token {
            lexeme: String::from(lexeme),
            kind,
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
}

pub fn test_function() {
    println!("foo bar");
}
