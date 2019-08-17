use crate::{ParseError, RuntimeError, ScanError};

/// A general Mankai error (can be a parsing error or a runtime error).
pub struct MankaiError {
    /// Error message.
    pub message: String,
}

impl From<ScanError> for MankaiError {
    fn from(err: ScanError) -> Self {
        let mut message = String::new();
        message.push_str("Lexing error at ");
        message.push_str(&err.position.to_string());
        message.push_str(": ");
        message.push_str(&err.message);

        MankaiError { message }
    }
}

impl From<ParseError> for MankaiError {
    fn from(err: ParseError) -> Self {
        let mut message = String::new();
        message.push_str("Parsing error");
        if let Some(token) = err.token {
            message.push_str(" at '");
            message.push_str(&token.lexeme);
            message.push_str("'");
        }
        message.push_str(": ");
        message.push_str(&err.message);

        MankaiError { message }
    }
}

impl From<RuntimeError> for MankaiError {
    fn from(err: RuntimeError) -> Self {
        let mut message = String::new();
        message.push_str("Runtime error: ");
        message.push_str(&err.message);

        MankaiError { message }
    }
}
