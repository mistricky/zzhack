use std::collections::HashMap;

use crate::command::{CommandInvocation, CommandSpec, ParsedCommand};
use crate::error::ShellParseError;
use crate::tokenizer::{tokenize, CommandTokens, Token};

/// Parser that can tokenize shell-like input and validate against registered commands.
#[derive(Default)]
pub struct ShellParser {
    commands: HashMap<String, CommandSpec>,
}

impl ShellParser {
    /// Create a parser without command validation.
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    /// Create a parser using the provided command specifications.
    pub fn with_commands<I>(commands: I) -> Self
    where
        I: IntoIterator<Item = CommandSpec>,
    {
        let mut parser = Self::new();
        for command in commands {
            parser.register_command(command);
        }
        parser
    }

    /// Register a command that may be used for validation.
    pub fn register_command(&mut self, command: CommandSpec) {
        self.commands.insert(command.name.clone(), command);
    }

    /// Parse a script into a list of invocations.
    ///
    /// The parser supports:
    /// - Command separators: newline, `;`, or `|`.
    /// - Comments starting with `#` until the end of the line.
    /// - Quoted arguments with `'` or `"`.
    /// - Escaping with `\` to include special characters.
    ///
    /// If the parser has registered commands, each invocation is validated
    /// against the corresponding [`CommandSpec`]. Unknown commands are
    /// rejected only when at least one command has been registered to avoid
    /// forcing validation on unconstrained parsers.
    pub fn parse(&self, input: &str) -> Result<Vec<CommandInvocation>, ShellParseError> {
        let command_tokens = tokenize(input)?;
        let validate_commands = !self.commands.is_empty();
        command_tokens
            .into_iter()
            .map(|tokens| self.build_invocation(tokens, validate_commands))
            .collect()
    }

    /// Parse a script into commands while preserving separators between them.
    pub fn parse_with_separators(
        &self,
        input: &str,
    ) -> Result<Vec<ParsedCommand>, ShellParseError> {
        let command_tokens = tokenize(input)?;
        let validate_commands = !self.commands.is_empty();
        command_tokens
            .into_iter()
            .map(|tokens| {
                let separator = tokens.separator;
                self.build_invocation(tokens, validate_commands)
                    .map(|invocation| ParsedCommand {
                        invocation,
                        separator,
                    })
            })
            .collect()
    }
}

fn validate_tokens(
    tokens: &[Token],
    args: &[String],
    spec: &CommandSpec,
) -> Result<(), ShellParseError> {
    let name_token = &tokens[0];
    if args.len() < spec.min_args {
        return Err(ShellParseError::InvalidArity {
            name: spec.name.clone(),
            min_expected: spec.min_args,
            max_expected: spec.max_args,
            found: args.len(),
            position: name_token.position,
        });
    }

    if let Some(max_args) = spec.max_args {
        if args.len() > max_args {
            return Err(ShellParseError::InvalidArity {
                name: spec.name.clone(),
                min_expected: spec.min_args,
                max_expected: spec.max_args,
                found: args.len(),
                position: name_token.position,
            });
        }
    }

    Ok(())
}

fn build_command_invocation(
    commands: &HashMap<String, CommandSpec>,
    tokens: &[Token],
    args: Vec<String>,
    validate_commands: bool,
) -> Result<CommandInvocation, ShellParseError> {
    let name_token = &tokens[0];

    if validate_commands {
        let spec =
            commands
                .get(&name_token.value)
                .ok_or_else(|| ShellParseError::UnknownCommand {
                    name: name_token.value.clone(),
                    position: name_token.position,
                })?;
        validate_tokens(tokens, &args, spec)?;
    }

    Ok(CommandInvocation {
        name: name_token.value.clone(),
        args,
        position: name_token.position,
    })
}

impl ShellParser {
    fn build_invocation(
        &self,
        tokens: CommandTokens,
        validate_commands: bool,
    ) -> Result<CommandInvocation, ShellParseError> {
        if tokens.tokens.is_empty() {
            return Err(ShellParseError::InvalidArity {
                name: String::new(),
                min_expected: 0,
                max_expected: None,
                found: 0,
                position: 0,
            });
        }

        let args: Vec<String> = tokens.tokens[1..].iter().map(token_value).collect();
        build_command_invocation(&self.commands, &tokens.tokens, args, validate_commands)
    }
}

fn token_value(token: &Token) -> String {
    token.value.clone()
}
