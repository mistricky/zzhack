//! Tiny macro-friendly CLI framework with basic options, subcommands, and help generation.
//! Commands are defined declaratively with the [`command!`] macro and assembled into an app
//! using [`cli!`]. Execution is library-only; callers provide their own I/O or host environment.

extern crate self as micro_cli;

mod command;
mod error;
mod help;
pub mod macros;
mod parser;
#[cfg(test)]
mod tests;

pub use command::{
    CliApp, Command, CommandContext, CommandHandler, OptionKind, OptionSpec, ParsedOptions,
};
pub use error::CliError;
pub use micro_cli_derive::Parser;

/// Trait implemented by the derive macro for struct- and enum-based CLIs.
pub trait Parser: Sized {
    fn parse() -> Result<Self, CliError>;
    fn parse_from<I, T>(iter: I) -> Result<Self, CliError>
    where
        I: IntoIterator<Item = T>,
        T: Into<String>;
    fn help() -> String;
    fn description() -> String;
    fn name() -> &'static str;
}
