use std::collections::HashMap;
use std::sync::Arc;

use crate::error::CliError;
use crate::help::HelpBuilder;
use crate::parser::CommandMatch;

pub type CommandHandler = Arc<dyn Fn(&CommandContext) -> Result<(), CliError> + Send + Sync>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionKind {
    Flag,
    Value,
}

#[derive(Debug, Clone)]
pub struct OptionSpec {
    pub name: &'static str,
    pub short: Option<char>,
    pub long: Option<&'static str>,
    pub kind: OptionKind,
    pub help: &'static str,
}

impl OptionSpec {
    pub fn flag(
        name: &'static str,
        short: Option<char>,
        long: Option<&'static str>,
        help: &'static str,
    ) -> Self {
        Self {
            name,
            short,
            long,
            kind: OptionKind::Flag,
            help,
        }
    }

    pub fn value(
        name: &'static str,
        short: Option<char>,
        long: Option<&'static str>,
        help: &'static str,
    ) -> Self {
        Self {
            name,
            short,
            long,
            kind: OptionKind::Value,
            help,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ParsedOptions {
    flags: HashMap<String, bool>,
    values: HashMap<String, String>,
}

impl ParsedOptions {
    pub fn flag(&self, name: &str) -> bool {
        self.flags.get(name).copied().unwrap_or(false)
    }

    pub fn value(&self, name: &str) -> Option<&str> {
        self.values.get(name).map(|s| s.as_str())
    }

    pub(crate) fn insert_flag(&mut self, name: String) {
        self.flags.insert(name, true);
    }

    pub(crate) fn insert_value(&mut self, name: String, value: String) {
        self.values.insert(name, value);
    }
}

#[derive(Debug)]
pub struct CommandContext<'a> {
    pub options: &'a ParsedOptions,
    pub args: &'a [String],
    pub path: &'a [String],
}

#[derive(Clone)]
pub struct Command {
    pub name: &'static str,
    pub about: &'static str,
    pub options: Vec<OptionSpec>,
    pub subcommands: Vec<Command>,
    pub handler: CommandHandler,
}

impl std::fmt::Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Command")
            .field("name", &self.name)
            .field("about", &self.about)
            .field("options", &self.options)
            .field("subcommands", &self.subcommands)
            .finish_non_exhaustive()
    }
}

impl Command {
    pub fn new(name: &'static str, about: &'static str, handler: CommandHandler) -> Self {
        Self {
            name,
            about,
            options: Vec::new(),
            subcommands: Vec::new(),
            handler,
        }
    }

    pub fn with_options(mut self, options: Vec<OptionSpec>) -> Self {
        self.options = options;
        self
    }

    pub fn with_subcommands(mut self, subcommands: Vec<Command>) -> Self {
        self.subcommands = subcommands;
        self
    }

    pub(crate) fn find_subcommand(&self, name: &str) -> Option<&Command> {
        self.subcommands.iter().find(|c| c.name == name)
    }

    pub(crate) fn build_help(&self, path: &[String]) -> String {
        HelpBuilder::new(self, path).render()
    }
}

#[derive(Debug, Clone)]
pub struct CliApp {
    pub name: &'static str,
    pub about: &'static str,
    pub commands: Vec<Command>,
}

impl CliApp {
    pub fn new(name: &'static str, about: &'static str) -> Self {
        Self {
            name,
            about,
            commands: Vec::new(),
        }
    }

    pub fn commands(mut self, commands: Vec<Command>) -> Self {
        self.commands = commands;
        self
    }

    pub fn run(&self, args: &[String]) -> Result<(), CliError> {
        if args.is_empty() || args[0] == "--help" || args[0] == "-h" {
            return Err(CliError::Help(self.build_root_help()));
        }
        let root_match = crate::parser::parse_root(self, args)?;
        self.dispatch(root_match)
    }

    pub fn run_command(&self, name: &str, args: &[String]) -> Result<(), CliError> {
        let Some(cmd) = self.commands.iter().find(|c| c.name == name) else {
            return Err(CliError::UnknownCommand(name.to_string()));
        };

        let matched =
            crate::parser::parse_command(cmd, args, &[self.name.to_string(), name.to_string()])?;
        self.invoke(&matched)
    }

    fn build_root_help(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("Usage: {} <COMMAND> [ARGS]\n\n", self.name));
        out.push_str(&format!("{}\n\n", self.about));
        out.push_str("Commands:\n");
        for cmd in &self.commands {
            out.push_str(&format!("  {:<18} {}\n", cmd.name, cmd.about));
        }
        out.push_str("\nOptions:\n  -h, --help          Show help\n");
        out
    }

    fn dispatch(&self, root_match: CommandMatch<'_>) -> Result<(), CliError> {
        self.invoke(&root_match)
    }

    fn invoke(&self, matched: &CommandMatch<'_>) -> Result<(), CliError> {
        let ctx = CommandContext {
            options: &matched.options,
            args: &matched.args,
            path: &matched.path,
        };
        (matched.command.handler)(&ctx)
    }
}
