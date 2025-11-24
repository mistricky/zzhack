use crate::commands::{CommandContext, CommandHandler};
use crate::vfs_data::format_path;
use async_trait::async_trait;
use shell_parser::CommandSpec;

pub struct PwdCommand;

#[async_trait(?Send)]
impl CommandHandler for PwdCommand {
    fn name(&self) -> &'static str {
        "pwd"
    }

    fn spec(&self) -> CommandSpec {
        CommandSpec::new("pwd").with_max_args(0)
    }

    async fn run(&self, _args: &[String], ctx: &CommandContext) {
        ctx.terminal.push_text(format_path(&ctx.terminal.cwd()));
    }
}
