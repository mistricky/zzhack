//! Minimal shell-like parser that can be validated against custom command definitions.
//! The parser is pure and avoids any system-specific API usage so it can run in any environment.
//!
//! # Example
//! ```
//! use shell_parser::ShellParser;
//!
//! let parser = ShellParser::new();
//! let invocations = parser.parse(r#"echo "Hello" > ./foo.log"#).unwrap();
//! assert_eq!(invocations[0].name, "echo");
//! assert_eq!(invocations[0].args, vec!["Hello", ">", "./foo.log"]);
//! ```

pub mod command;
pub mod error;
pub mod integration;
mod parser;
pub mod separator;
mod tokenizer;

pub use crate::command::{CommandInvocation, CommandSpec, ParsedCommand};
pub use crate::error::ShellParseError;
pub use crate::integration::{with_cli, CliRunner, ExecutableCommand, ShellCliError};
pub use crate::parser::ShellParser;
pub use crate::separator::Separator;

#[cfg(test)]
mod tests;
