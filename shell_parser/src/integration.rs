use std::collections::HashMap;

use crate::{CommandSpec, ShellParseError, ShellParser};

/// Error surfaced when wiring parsed commands into executable handlers.
#[derive(Debug, thiserror::Error)]
pub enum ShellCliError {
    #[error("parse error: {0}")]
    Parse(#[from] ShellParseError),
    #[error("command failed: {command}: {message}")]
    Execution { command: String, message: String },
}

/// Common metadata for CLI commands.
pub trait CommandInfo {
    /// Name used in the script (e.g., `echo`).
    fn command_name(&self) -> &'static str;

    fn command_about(&self) -> &'static str;

    /// Optional aliases that should invoke the same command.
    fn command_aliases(&self) -> &'static [&'static str] {
        &[]
    }

    /// Specification for validation.
    fn command_spec(&self) -> CommandSpec {
        let mut spec = CommandSpec::new(self.command_name(), self.command_about());
        for alias in self.command_aliases() {
            spec = spec.with_alias(*alias);
        }
        spec
    }
}

/// Trait implemented by higher-level CLI commands that can be executed after parsing.
pub trait ExecutableCommand<C>: CommandInfo + Send + Sync {
    /// Specification (includes name) for validation.
    fn spec(&self) -> CommandSpec {
        self.command_spec()
    }
    /// Execute the command with already-parsed arguments.
    fn run(&self, args: &[String], context: &C) -> Result<(), String>;

    /// Execute with an optional piped input and return an optional piped output.
    fn run_with_input(
        &self,
        args: &[String],
        input: Option<String>,
        context: &C,
    ) -> Result<Option<String>, String> {
        self.run(args, context)?;
        Ok(input)
    }
}

/// Builder for integrating [`ShellParser`] with executable commands.
pub struct CliRunner<C> {
    parser: ShellParser,
    handlers: HashMap<String, Box<dyn ExecutableCommand<C>>>,
    specs: Vec<CommandSpec>,
    context: C,
}

impl<C> CliRunner<C> {
    /// Parse and execute a full script (multiple lines/commands).
    pub fn run_script(&self, script: &str) -> Result<(), ShellCliError> {
        let invocations = self.parser.parse(script)?;
        for inv in invocations {
            self.run_invocation(inv.name, inv.args, None)?;
        }
        Ok(())
    }

    /// Parse and execute a single command line.
    pub fn run_line(&self, line: &str) -> Result<(), ShellCliError> {
        self.run_script(line)
    }

    /// Parse and execute a script that may contain pipelines (`|`).
    pub fn run_pipeline_script(&self, script: &str) -> Result<(), ShellCliError> {
        let parsed = self.parser.parse_with_separators(script)?;
        let mut pipeline: Vec<(String, Vec<String>)> = Vec::new();

        for item in parsed {
            pipeline.push((item.invocation.name, item.invocation.args));
            let end_pipeline = item.separator != Some(crate::separator::Separator::Pipe);
            if end_pipeline {
                self.execute_pipeline(&pipeline)?;
                pipeline.clear();
            }
        }

        if !pipeline.is_empty() {
            self.execute_pipeline(&pipeline)?;
        }

        Ok(())
    }

    /// Render help text listing registered commands.
    pub fn help(&self) -> String {
        // let mut names: Vec<&str> = self.specs.iter().map(|spec| spec.name.as_str()).collect();
        // names.sort_unstable();
        let mut out = String::from("Commands:\n");

        for spec in self.specs.iter() {
            out.push_str(&format!("  {:<10}     {}\n", spec.name, spec.about));
        }

        out
    }

    fn run_invocation(
        &self,
        name: String,
        args: Vec<String>,
        input: Option<String>,
    ) -> Result<Option<String>, ShellCliError> {
        let handler = self
            .handlers
            .get(&name)
            .ok_or_else(|| ShellCliError::Execution {
                command: name.clone(),
                message: "no handler registered".into(),
            })?;
        handler
            .run_with_input(&args, input, &self.context)
            .map_err(|message| ShellCliError::Execution {
                command: name,
                message,
            })
    }

    fn execute_pipeline(&self, pipeline: &[(String, Vec<String>)]) -> Result<(), ShellCliError> {
        let mut input: Option<String> = None;
        for (name, args) in pipeline {
            input = self.run_invocation(name.clone(), args.clone(), input)?;
        }
        Ok(())
    }
}

/// Create a [`CliRunner`] by registering executable commands.
pub fn with_cli<C, I>(context: C, commands: I) -> CliRunner<C>
where
    I: IntoIterator<Item = Box<dyn ExecutableCommand<C>>>,
{
    let mut handlers = HashMap::new();
    let mut specs: Vec<CommandSpec> = Vec::new();

    for cmd in commands {
        let spec = cmd.spec();
        handlers.insert(spec.name.clone(), cmd);
        specs.push(spec);
    }

    let parser = ShellParser::with_commands(specs.clone());
    CliRunner {
        parser,
        handlers,
        specs,
        context,
    }
}
