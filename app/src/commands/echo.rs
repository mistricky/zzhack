use crate::commands::{CommandContext, CommandHandler};
use async_trait::async_trait;
use shell_parser::CommandSpec;
use web_sys::console;

pub struct EchoCommand;

#[async_trait(?Send)]
impl CommandHandler for EchoCommand {
    fn name(&self) -> &'static str {
        "echo"
    }

    fn spec(&self) -> CommandSpec {
        CommandSpec::new("echo")
    }

    async fn run(&self, args: &[String], ctx: &CommandContext) {
        let msg = args.join(" ");
        console::log_1(&msg.clone().into());
        ctx.terminal.push_text(msg);
    }
}
