use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};

use crate::command::{CommandInvocation, CommandSpec, ParsedCommand};
use crate::error::ShellParseError;
use crate::tokenizer::{tokenize, CommandTokens, Token};

/// Parser that can tokenize shell-like input and validate against registered commands.
#[derive(Default)]
pub struct ShellParser {
    commands: HashMap<String, CommandSpec>,
    command_aliases: HashMap<String, String>,
    runtime_aliases: RefCell<HashMap<String, RuntimeAlias>>,
    runtime_functions: RefCell<HashMap<String, RuntimeFunction>>,
}

#[derive(Clone, Debug)]
struct RuntimeAlias {
    value: String,
}

#[derive(Clone, Debug)]
struct RuntimeFunction {
    commands: Vec<CommandTokens>,
}

#[derive(Clone)]
struct PendingCommand {
    tokens: CommandTokens,
    stack: Vec<String>,
}

struct FunctionHeader {
    name: String,
    position: usize,
    brace_inline: bool,
}

impl ShellParser {
    /// Create a parser without command validation.
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
            command_aliases: HashMap::new(),
            runtime_aliases: RefCell::new(HashMap::new()),
            runtime_functions: RefCell::new(HashMap::new()),
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
        let name = command.name.clone();
        for alias in &command.aliases {
            if alias != &name {
                self.command_aliases.insert(alias.clone(), name.clone());
            }
        }
        self.commands.insert(name, command);
    }

    /// Parse a script into a list of invocations.
    ///
    /// The parser supports:
    /// - Command separators: newline, `;`, `|`, or `&&`.
    /// - Comments starting with `#` until the end of the line.
    /// - Quoted arguments with `'` or `"`.
    /// - Escaping with `\` to include special characters.
    ///
    /// If the parser has registered commands, each invocation is validated
    /// against the corresponding [`CommandSpec`]. Unknown commands are
    /// rejected only when at least one command has been registered to avoid
    /// forcing validation on unconstrained parsers.
    pub fn parse(&self, input: &str) -> Result<Vec<CommandInvocation>, ShellParseError> {
        let command_tokens = self.collect_commands(input)?;
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
        let command_tokens = self.collect_commands(input)?;
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
        self.build_command_from_tokens(&tokens.tokens, args, validate_commands)
    }

    fn build_command_from_tokens(
        &self,
        tokens: &[Token],
        args: Vec<String>,
        validate_commands: bool,
    ) -> Result<CommandInvocation, ShellParseError> {
        let name_token = &tokens[0];

        if name_token.value == "builtin" {
            return self.build_builtin_invocation(tokens, args, validate_commands);
        }

        let mut resolved_name = name_token.value.clone();
        let spec = self.resolve_command(&name_token.value);

        if let Some((command_spec, canonical_name)) = spec {
            resolved_name = canonical_name.to_string();
            if validate_commands {
                validate_tokens(tokens, &args, command_spec)?;
            }
        } else if validate_commands && name_token.value != "alias" && name_token.value != "function"
        {
            return Err(ShellParseError::UnknownCommand {
                name: name_token.value.clone(),
                position: name_token.position,
            });
        }

        Ok(CommandInvocation {
            name: resolved_name,
            args,
            position: name_token.position,
        })
    }

    fn build_builtin_invocation(
        &self,
        tokens: &[Token],
        args: Vec<String>,
        validate_commands: bool,
    ) -> Result<CommandInvocation, ShellParseError> {
        if args.is_empty() {
            return Err(ShellParseError::InvalidArity {
                name: "builtin".into(),
                min_expected: 1,
                max_expected: None,
                found: 0,
                position: tokens[0].position,
            });
        }

        let target_name = args[0].clone();
        let target_token_position = tokens
            .get(1)
            .map(|token| token.position)
            .unwrap_or(tokens[0].position);
        let mut target_tokens = Vec::with_capacity(tokens.len() - 1);
        target_tokens.push(Token {
            value: target_name.clone(),
            position: target_token_position,
        });
        target_tokens.extend(tokens.iter().skip(2).cloned());
        let target_args: Vec<String> = args.into_iter().skip(1).collect();

        let Some((command_spec, canonical_name)) = self.resolve_command(&target_name) else {
            return Err(ShellParseError::UnknownCommand {
                name: target_name,
                position: target_token_position,
            });
        };

        if validate_commands {
            validate_tokens(&target_tokens, &target_args, command_spec)?;
        }

        Ok(CommandInvocation {
            name: canonical_name.to_string(),
            args: target_args,
            position: target_token_position,
        })
    }

    fn resolve_command(&self, name: &str) -> Option<(&CommandSpec, &str)> {
        if let Some(spec) = self.commands.get(name) {
            return Some((spec, spec.name.as_str()));
        }

        self.command_aliases.get(name).and_then(|canonical| {
            self.commands
                .get(canonical)
                .map(|spec| (spec, spec.name.as_str()))
        })
    }

    fn collect_commands(&self, input: &str) -> Result<Vec<CommandTokens>, ShellParseError> {
        let mut pending: VecDeque<PendingCommand> = tokenize(input)?
            .into_iter()
            .map(|tokens| PendingCommand {
                tokens,
                stack: Vec::new(),
            })
            .collect();
        let mut commands = Vec::new();

        while let Some(pending_command) = pending.pop_front() {
            let pending_command = pending_command;
            if pending_command.tokens.tokens.is_empty() {
                continue;
            }

            if self.try_handle_function_definition(&pending_command.tokens, &mut pending)? {
                continue;
            }

            if let Some(expanded) = self.try_expand_runtime_function(&pending_command)? {
                for cmd in expanded.into_iter().rev() {
                    pending.push_front(cmd);
                }
                continue;
            }

            if let Some(expanded) = self.try_expand_runtime_alias(&pending_command)? {
                for cmd in expanded.into_iter().rev() {
                    pending.push_front(cmd);
                }
                continue;
            }

            if self.is_alias_command(&pending_command.tokens) {
                self.apply_alias_definitions(&pending_command.tokens)?;
            }

            commands.push(pending_command.tokens);
        }

        Ok(commands)
    }

    fn try_handle_function_definition(
        &self,
        tokens: &CommandTokens,
        pending: &mut VecDeque<PendingCommand>,
    ) -> Result<bool, ShellParseError> {
        let Some(header) = self.parse_function_header(tokens)? else {
            return Ok(false);
        };

        if !header.brace_inline {
            let Some(brace_command) = pending.pop_front() else {
                return Err(ShellParseError::InvalidFunction {
                    name: header.name.clone(),
                    message: "expected '{' after function header".into(),
                    position: header.position,
                });
            };
            if !is_brace_command(&brace_command.tokens.tokens, "{") {
                return Err(ShellParseError::InvalidFunction {
                    name: header.name.clone(),
                    message: "expected '{' after function header".into(),
                    position: header.position,
                });
            }
        }

        let body = self.collect_function_body(&header.name, header.position, pending)?;
        self.runtime_functions
            .borrow_mut()
            .insert(header.name.clone(), RuntimeFunction { commands: body });

        Ok(true)
    }

    fn parse_function_header(
        &self,
        tokens: &CommandTokens,
    ) -> Result<Option<FunctionHeader>, ShellParseError> {
        if tokens.tokens.is_empty() {
            return Ok(None);
        }

        let first = &tokens.tokens[0];
        if first.value == "function" {
            if tokens.tokens.len() < 2 {
                return Err(ShellParseError::InvalidFunction {
                    name: String::new(),
                    message: "function name missing".into(),
                    position: first.position,
                });
            }
            let Some(name) = normalize_function_name(&tokens.tokens[1].value) else {
                return Err(ShellParseError::InvalidFunction {
                    name: tokens.tokens[1].value.clone(),
                    message: "invalid function name".into(),
                    position: tokens.tokens[1].position,
                });
            };
            let brace_inline = validate_inline_brace(tokens, &name)?;
            return Ok(Some(FunctionHeader {
                name,
                position: first.position,
                brace_inline,
            }));
        }

        if let Some(name) = normalize_bare_function_name(&first.value) {
            let brace_inline = validate_inline_brace(tokens, &name)?;
            return Ok(Some(FunctionHeader {
                name,
                position: first.position,
                brace_inline,
            }));
        }

        Ok(None)
    }

    fn collect_function_body(
        &self,
        name: &str,
        position: usize,
        pending: &mut VecDeque<PendingCommand>,
    ) -> Result<Vec<CommandTokens>, ShellParseError> {
        let mut body = Vec::new();
        let mut depth = 1;

        while let Some(command) = pending.pop_front() {
            if command.tokens.tokens.is_empty() {
                continue;
            }

            if is_brace_command(&command.tokens.tokens, "}") {
                depth -= 1;
                if depth == 0 {
                    return Ok(body);
                }
                body.push(command.tokens);
                continue;
            }

            let openings = count_open_braces(&command.tokens.tokens);
            if openings > 0 {
                depth += openings;
            }

            body.push(command.tokens);
        }

        Err(ShellParseError::InvalidFunction {
            name: name.to_string(),
            message: "missing closing '}' for function body".into(),
            position,
        })
    }

    fn try_expand_runtime_function(
        &self,
        pending: &PendingCommand,
    ) -> Result<Option<Vec<PendingCommand>>, ShellParseError> {
        let Some(name_token) = pending.tokens.tokens.first() else {
            return Ok(None);
        };

        if name_token.value == "function" {
            return Ok(None);
        }

        let function = {
            let functions = self.runtime_functions.borrow();
            functions.get(&name_token.value).cloned()
        };

        let Some(runtime_function) = function else {
            return Ok(None);
        };

        if pending.stack.contains(&name_token.value) {
            return Err(ShellParseError::AliasLoop {
                name: name_token.value.clone(),
                position: name_token.position,
            });
        }

        let args: Vec<String> = pending.tokens.tokens[1..].iter().map(token_value).collect();
        let mut stack = pending.stack.clone();
        stack.push(name_token.value.clone());

        let mut expanded: Vec<PendingCommand> = Vec::new();
        for command in runtime_function.commands.iter() {
            let tokens = expand_function_tokens(&command.tokens, &args, name_token.position);
            if tokens.is_empty() {
                continue;
            }
            expanded.push(PendingCommand {
                tokens: CommandTokens {
                    tokens,
                    separator: command.separator,
                },
                stack: stack.clone(),
            });
        }

        Ok(Some(expanded))
    }

    fn try_expand_runtime_alias(
        &self,
        pending: &PendingCommand,
    ) -> Result<Option<Vec<PendingCommand>>, ShellParseError> {
        let Some(name_token) = pending.tokens.tokens.first() else {
            return Ok(None);
        };

        if name_token.value == "alias" {
            return Ok(None);
        }

        if pending.stack.contains(&name_token.value) {
            return Err(ShellParseError::AliasLoop {
                name: name_token.value.clone(),
                position: name_token.position,
            });
        }

        let Some(commands) = self.expand_alias_commands(&name_token.value, &pending.tokens)? else {
            return Ok(None);
        };

        let mut stack = pending.stack.clone();
        stack.push(name_token.value.clone());

        let expanded = commands
            .into_iter()
            .map(|tokens| PendingCommand {
                tokens,
                stack: stack.clone(),
            })
            .collect();

        Ok(Some(expanded))
    }

    fn expand_alias_commands(
        &self,
        name: &str,
        original: &CommandTokens,
    ) -> Result<Option<Vec<CommandTokens>>, ShellParseError> {
        let alias_value = {
            let aliases = self.runtime_aliases.borrow();
            aliases.get(name).cloned()
        };

        let Some(runtime_alias) = alias_value else {
            return Ok(None);
        };

        if runtime_alias.value.is_empty() {
            self.runtime_aliases.borrow_mut().remove(name);
            return Ok(None);
        }

        let position = original.tokens[0].position;
        let mut commands =
            tokenize(&runtime_alias.value).map_err(|err| err.with_offset(position))?;

        if commands.is_empty() {
            self.runtime_aliases.borrow_mut().remove(name);
            return Ok(None);
        }

        for command in commands.iter_mut() {
            for token in command.tokens.iter_mut() {
                token.position = position;
            }
        }

        if let Some(last) = commands.last_mut() {
            last.tokens.extend(original.tokens.iter().skip(1).cloned());
            last.separator = original.separator;
        }

        Ok(Some(commands))
    }

    fn is_alias_command(&self, tokens: &CommandTokens) -> bool {
        tokens
            .tokens
            .first()
            .map(|token| token.value == "alias")
            .unwrap_or(false)
    }

    fn apply_alias_definitions(&self, tokens: &CommandTokens) -> Result<(), ShellParseError> {
        for token in tokens.tokens.iter().skip(1) {
            if let Some((name, value)) = token.value.split_once('=') {
                self.define_alias(name, value, token.position)?;
            }
        }
        Ok(())
    }

    fn define_alias(
        &self,
        name: &str,
        value: &str,
        token_position: usize,
    ) -> Result<(), ShellParseError> {
        if name.is_empty() {
            return Err(ShellParseError::InvalidAlias {
                name: String::from(name),
                message: "alias name cannot be empty".into(),
                position: token_position,
            });
        }

        if value.is_empty() {
            self.runtime_aliases.borrow_mut().remove(name);
        } else {
            self.runtime_aliases.borrow_mut().insert(
                name.to_string(),
                RuntimeAlias {
                    value: value.to_string(),
                },
            );
        }

        Ok(())
    }
}

fn token_value(token: &Token) -> String {
    token.value.clone()
}

fn normalize_function_name(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }
    let name = trimmed.strip_suffix("()").unwrap_or(trimmed);
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}

fn normalize_bare_function_name(value: &str) -> Option<String> {
    let trimmed = value.trim();
    let name = trimmed.strip_suffix("()")?;
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}

fn validate_inline_brace(tokens: &CommandTokens, name: &str) -> Result<bool, ShellParseError> {
    if let Some((idx, brace)) = tokens
        .tokens
        .iter()
        .enumerate()
        .find(|(_, token)| token.value == "{")
    {
        if idx != tokens.tokens.len() - 1 {
            return Err(ShellParseError::InvalidFunction {
                name: name.to_string(),
                message: "opening '{' must end the line for functions".into(),
                position: brace.position,
            });
        }
        return Ok(true);
    }
    Ok(false)
}

fn is_brace_command(tokens: &[Token], brace: &str) -> bool {
    tokens.len() == 1 && tokens[0].value == brace
}

fn count_open_braces(tokens: &[Token]) -> usize {
    tokens.iter().filter(|token| token.value == "{").count()
}

fn expand_function_tokens(tokens: &[Token], args: &[String], position: usize) -> Vec<Token> {
    let mut expanded: Vec<Token> = Vec::new();
    for token in tokens {
        match token.value.as_str() {
            "$@" => {
                for arg in args {
                    expanded.push(Token {
                        value: arg.clone(),
                        position,
                    });
                }
            }
            "$*" => {
                if !args.is_empty() {
                    expanded.push(Token {
                        value: args.join(" "),
                        position,
                    });
                }
            }
            "$#" => expanded.push(Token {
                value: args.len().to_string(),
                position,
            }),
            value => {
                if let Some(idx) = positional_index(value) {
                    if let Some(arg) = args.get(idx - 1) {
                        if !arg.is_empty() {
                            expanded.push(Token {
                                value: arg.clone(),
                                position,
                            });
                        }
                    }
                } else {
                    let mut cloned = token.clone();
                    cloned.position = position;
                    expanded.push(cloned);
                }
            }
        }
    }
    expanded
}

fn positional_index(value: &str) -> Option<usize> {
    if !value.starts_with('$') {
        return None;
    }
    let digits = &value[1..];
    if digits.is_empty() || digits.chars().any(|ch| !ch.is_ascii_digit()) {
        return None;
    }
    let idx = digits.parse::<usize>().ok()?;
    if idx == 0 {
        None
    } else {
        Some(idx)
    }
}
