#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CliError {
    #[error("unknown command: {0}")]
    UnknownCommand(String),
    #[error("unknown option: {0}")]
    UnknownOption(String),
    #[error("missing value for option: {0}")]
    MissingOptionValue(String),
    #[error("missing argument: {0}")]
    MissingArgument(&'static str),
    #[error("help")]
    Help(String),
}
