mod cat;
mod cd;
mod clear;
mod du;
mod echo;
mod fetch;
mod boa;
mod ls;
mod pwd;
mod render;
mod stat;

use crate::cache_service::CacheService;
use crate::terminal::Terminal;
use crate::vfs_data::VfsNode;
use async_trait::async_trait;
use shell_parser::CommandSpec;
use std::rc::Rc;

pub use cat::CatCommand;
pub use cd::CdCommand;
pub use clear::ClearCommand;
pub use du::DuCommand;
pub use echo::EchoCommand;
pub use fetch::FetchCommand;
pub use boa::BoaCommand;
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
        Box::new(BoaCommand),
        Box::new(RenderCommand),
        Box::new(ClearCommand),
        Box::new(PwdCommand),
    ]
}
