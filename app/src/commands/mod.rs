mod cat;
mod cd;
mod clear;
mod du;
mod echo;
mod fetch;
mod ls;
mod render;
mod stat;

use crate::cache_service::CacheService;
use crate::terminal::Terminal;
use crate::vfs_data::VfsNode;
use async_trait::async_trait;
use shell_parser::{CommandSpec, ShellParser};
use std::collections::HashMap;
use std::rc::Rc;

pub use cat::CatCommand;
pub use cd::CdCommand;
pub use clear::ClearCommand;
pub use du::DuCommand;
pub use echo::EchoCommand;
pub use fetch::FetchCommand;
pub use ls::LsCommand;
pub use render::RenderCommand;
pub use stat::StatCommand;

#[derive(Clone)]
pub struct CommandContext {
    pub vfs: Rc<VfsNode>,
    pub cache: Option<Rc<CacheService>>,
    pub terminal: Terminal,
}

#[async_trait(?Send)]
pub trait CommandHandler {
    fn name(&self) -> &'static str;
    fn spec(&self) -> CommandSpec;
    async fn run(&self, args: &[String], ctx: &CommandContext);
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
        Box::new(RenderCommand),
        Box::new(ClearCommand),
    ]
}

pub async fn execute_command(
    input: &str,
    ctx: CommandContext,
    handlers: &[Box<dyn CommandHandler>],
) {
    let mut specs = Vec::with_capacity(handlers.len());
    let mut map: HashMap<&str, &Box<dyn CommandHandler>> = HashMap::new();
    for handler in handlers {
        specs.push(handler.spec());
        map.insert(handler.name(), handler);
    }

    let parser = ShellParser::with_commands(specs);

    let parsed = match parser.parse(input) {
        Ok(commands) => commands.into_iter().next(),
        Err(err) => {
            ctx.terminal.push_error(format!("parse error: {:?}", err));
            return;
        }
    };

    let Some(command) = parsed else {
        ctx.terminal.push_error("empty command");
        return;
    };

    let handler = match map.get(command.name.as_str()) {
        Some(h) => h,
        None => {
            ctx.terminal.push_error(format!(
                "unknown command: {} (TODO: implement)",
                command.name
            ));
            return;
        }
    };

    handler.run(&command.args, &ctx).await;
}
