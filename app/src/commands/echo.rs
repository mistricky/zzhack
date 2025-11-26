use crate::commands::{parse_cli, CommandContext};
use micro_cli::Parser;
use shell_parser::integration::{CommandInfo, ExecutableCommand};
use web_sys::console;

#[derive(Parser, Debug, Default)]
#[command(name = "echo", about = "Echo text")]
pub struct EchoCommand {
    #[arg(positional, help = "Text to echo")]
    message: Vec<String>,
}

impl ExecutableCommand<CommandContext> for EchoCommand {
    fn run(&self, args: &[String], ctx: &CommandContext) -> Result<(), String> {
        let Some(cli) = parse_cli::<EchoCommand>(args, ctx, self.command_name()) else {
            return Ok(());
        };
        let msg = cli.message.join(" ");
        console::log_1(&msg.clone().into());
        ctx.terminal.push_text(msg);
        Ok(())
    }
}
