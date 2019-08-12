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
}

mod test {
    use super::Lexer;

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
}

pub fn test_function() {
    println!("foo bar");
}
