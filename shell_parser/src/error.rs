use thiserror::Error;

/// Errors that can arise while parsing.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ShellParseError {
    #[error("unknown command '{name}' at {position}")]
    UnknownCommand { name: String, position: usize },
    #[error("invalid arity for '{name}' at {position}: expected {min_expected}..{max_expected:?}, found {found}")]
    InvalidArity {
        name: String,
        min_expected: usize,
        max_expected: Option<usize>,
        found: usize,
        position: usize,
    },
    #[error("unterminated quote {quote} at {position}")]
    UnterminatedQuote { quote: char, position: usize },
    #[error("trailing escape at {position}")]
    TrailingEscape { position: usize },
}
