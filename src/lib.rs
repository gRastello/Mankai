/// Types of tokens.
enum TokenKind {
    Plus,
    Minus,
    Star,
    Slash,
    String(String),
    Number(f64),
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

pub fn test_function() {
    println!("foo bar");
}
