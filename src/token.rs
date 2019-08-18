/// Types of tokens.
#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    String(String),
    Number(f64),
    Identifier,
    LeftParen,
    RightParen,
    Eof,
}

/// A token.
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    /// Corresponding lexeme.
    pub lexeme: String,
    /// Kind of the token.
    pub kind: TokenKind,
}

impl Token {
    /// Create a new token from lexeme and kind.
    pub fn new(lexeme: String, kind: TokenKind) -> Self {
        Token { lexeme, kind }
    }
}
