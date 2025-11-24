use crate::commands::{CommandContext, CommandHandler};
use async_trait::async_trait;
use shell_parser::CommandSpec;

pub struct ClearCommand;

#[async_trait(?Send)]
impl CommandHandler for ClearCommand {
    fn name(&self) -> &'static str {
        "clear"
    }

    fn spec(&self) -> CommandSpec {
        CommandSpec::new("clear").with_max_args(0)
    }

    async fn run(&self, _args: &[String], ctx: &CommandContext) {
        ctx.terminal.clear();
    }
}
