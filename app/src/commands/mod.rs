mod boa;
mod cat;
mod cd;
mod clear;
mod du;
mod echo;
mod fetch;
mod ls;
mod pwd;
mod render;
mod stat;

use crate::cache_service::CacheService;
use crate::terminal::Terminal;
use crate::vfs_data::VfsNode;
use micro_cli::{CliError, Parser};
use shell_parser::integration::ExecutableCommand;
use std::rc::Rc;

pub use boa::BoaCommand;
pub use cat::CatCommand;
pub use cd::CdCommand;
pub use clear::ClearCommand;
pub use du::DuCommand;
pub use echo::EchoCommand;
pub use fetch::FetchCommand;
pub use ls::LsCommand;
pub use pwd::PwdCommand;
pub use render::RenderCommand;
pub use stat::StatCommand;

#[derive(Clone)]
pub struct CommandContext {
    pub vfs: Rc<VfsNode>,
    pub cache: Option<Rc<CacheService>>,
    pub terminal: Terminal,
}

pub fn parse_cli<T: Parser>(args: &[String], ctx: &CommandContext, label: &str) -> Option<T> {
    match T::parse_from(args.to_vec()) {
        Ok(parsed) => Some(parsed),
        Err(CliError::Help(text)) => {
            ctx.terminal.push_text(text);
            None
        }
        Err(err) => {
            ctx.terminal.push_error(format!("{label}: {err}"));
            None
        }
    }
}

pub fn command_handlers() -> Vec<Box<dyn ExecutableCommand<CommandContext>>> {
    vec![
        Box::new(EchoCommand),
        Box::new(LsCommand),
        Box::new(CatCommand),
        Box::new(CdCommand),
        Box::new(StatCommand),
        Box::new(DuCommand),
        Box::new(FetchCommand),
        Box::new(BoaCommand),
        Box::new(RenderCommand),
        Box::new(ClearCommand),
        Box::new(PwdCommand),
    ]
}
