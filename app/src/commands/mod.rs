mod cat;
mod cd;
mod du;
mod echo;
mod fetch;
mod ls;
mod stat;

use crate::cache_service::CacheService;
use crate::types::TermLine;
use crate::vfs_data::VfsNode;
use async_trait::async_trait;
use shell_parser::{CommandSpec, ShellParser};
use std::collections::HashMap;
use std::rc::Rc;

pub use cat::CatCommand;
pub use cd::CdCommand;
pub use du::DuCommand;
pub use echo::EchoCommand;
pub use fetch::FetchCommand;
pub use ls::LsCommand;
pub use stat::StatCommand;

#[derive(Clone)]
pub struct CommandContext {
    pub cwd: Vec<String>,
    pub vfs: Rc<VfsNode>,
    pub cache: Option<Rc<CacheService>>,
}

pub struct CommandOutcome {
    pub lines: Vec<TermLine>,
    pub new_cwd: Option<Vec<String>>,
}

#[async_trait(?Send)]
pub trait CommandHandler {
    fn name(&self) -> &'static str;
    fn spec(&self) -> CommandSpec;
    async fn run(&self, args: &[String], ctx: &CommandContext) -> CommandOutcome;
}

pub fn command_handlers() -> Vec<Box<dyn CommandHandler>> {
    vec![
        Box::new(EchoCommand),
        Box::new(LsCommand),
        Box::new(CatCommand),
        Box::new(CdCommand),
        Box::new(StatCommand),
        Box::new(DuCommand),
        Box::new(FetchCommand),
    ]
}

pub async fn execute_command(
    input: &str,
    ctx: CommandContext,
    handlers: &[Box<dyn CommandHandler>],
) -> CommandOutcome {
    let mut specs = Vec::with_capacity(handlers.len());
    let mut map: HashMap<&str, &Box<dyn CommandHandler>> = HashMap::new();
    for handler in handlers {
        specs.push(handler.spec());
        map.insert(handler.name(), handler);
    }

    let parser = ShellParser::with_commands(specs);
    let mut output = Vec::new();

    let parsed = match parser.parse(input) {
        Ok(commands) => commands.into_iter().next(),
        Err(err) => {
            output.push(line_error(format!("parse error: {:?}", err)));
            return CommandOutcome {
                lines: output,
                new_cwd: None,
            };
        }
    };

    let Some(command) = parsed else {
        output.push(line_error("empty command".into()));
        return CommandOutcome {
            lines: output,
            new_cwd: None,
        };
    };

    let handler = match map.get(command.name.as_str()) {
        Some(h) => h,
        None => {
            output.push(line_error(format!(
                "unknown command: {} (TODO: implement)",
                command.name
            )));
            return CommandOutcome {
                lines: output,
                new_cwd: None,
            };
        }
    };

    handler.run(&command.args, &ctx).await
}

pub fn line_out(body: String) -> TermLine {
    TermLine {
        prompt: ">".into(),
        body,
        accent: false,
    }
}

pub fn line_error(body: String) -> TermLine {
    TermLine {
        prompt: "!".into(),
        body,
        accent: true,
    }
}
