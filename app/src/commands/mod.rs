mod boa;
mod cat;
mod cd;
mod clear;
mod du;
mod echo;
mod email;
mod eval;
mod fetch;
mod help;
mod history;
mod ls;
mod navigate;
mod pwd;
mod render;
mod sleep;
mod stat;
mod theme;
mod whoami;

use crate::cache_service::CacheService;
use crate::config_service::AppConfig;
use crate::terminal::TerminalHandle;
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
pub use email::EmailCommand;
pub use eval::EvalCommand;
pub use fetch::FetchCommand;
pub use help::HelpCommand;
pub use history::HistoryCommand;
pub use ls::LsCommand;
pub use navigate::NavigateCommand;
pub use pwd::PwdCommand;
pub use render::RenderCommand;
pub use sleep::SleepCommand;
pub use stat::StatCommand;
pub use theme::ThemeCommand;
pub use whoami::WhoAmICommand;

#[derive(Clone)]
pub struct CommandContext {
    pub vfs: Rc<VfsNode>,
    pub cache: Option<Rc<CacheService>>,
    pub terminal: TerminalHandle,
    pub config: &'static AppConfig,
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
        Box::new(EchoCommand::default()),
        Box::new(LsCommand::default()),
        Box::new(CatCommand::default()),
        Box::new(EvalCommand::default()),
        Box::new(CdCommand::default()),
        Box::new(StatCommand::default()),
        Box::new(DuCommand::default()),
        Box::new(FetchCommand::default()),
        Box::new(BoaCommand::default()),
        Box::new(RenderCommand::default()),
        Box::new(ClearCommand::default()),
        Box::new(PwdCommand::default()),
        Box::new(EmailCommand::default()),
        Box::new(WhoAmICommand::default()),
        Box::new(ThemeCommand::default()),
        Box::new(NavigateCommand::default()),
        Box::new(SleepCommand::default()),
        Box::new(HistoryCommand::default()),
        Box::new(HelpCommand::default()),
    ]
}
