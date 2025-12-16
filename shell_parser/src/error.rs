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
    #[error("alias/function loop detected for '{name}' at {position}")]
    AliasLoop { name: String, position: usize },
    #[error("invalid alias '{name}' at {position}: {message}")]
    InvalidAlias {
        name: String,
        message: String,
        position: usize,
    },
    #[error("invalid function '{name}' at {position}: {message}")]
    InvalidFunction {
        name: String,
        message: String,
        position: usize,
    },
}

impl ShellParseError {
    pub(crate) fn with_offset(self, offset: usize) -> Self {
        match self {
            ShellParseError::UnknownCommand { name, position } => ShellParseError::UnknownCommand {
                name,
                position: position + offset,
            },
            ShellParseError::InvalidArity {
                name,
                min_expected,
                max_expected,
                found,
                position,
            } => ShellParseError::InvalidArity {
                name,
                min_expected,
                max_expected,
                found,
                position: position + offset,
            },
            ShellParseError::UnterminatedQuote { quote, position } => {
                ShellParseError::UnterminatedQuote {
                    quote,
                    position: position + offset,
                }
            }
            ShellParseError::TrailingEscape { position } => ShellParseError::TrailingEscape {
                position: position + offset,
            },
            ShellParseError::AliasLoop { name, position } => ShellParseError::AliasLoop {
                name,
                position: position + offset,
            },
            ShellParseError::InvalidAlias {
                name,
                message,
                position,
            } => ShellParseError::InvalidAlias {
                name,
                message,
                position: position + offset,
            },
            ShellParseError::InvalidFunction {
                name,
                message,
                position,
            } => ShellParseError::InvalidFunction {
                name,
                message,
                position: position + offset,
            },
        }
    }
}
