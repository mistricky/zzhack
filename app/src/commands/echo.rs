use crate::commands::{parse_cli, CommandContext};
use micro_cli::Parser;
use shell_parser::integration::ExecutableCommand;
use shell_parser::CommandSpec;
use web_sys::console;

#[derive(Parser, Debug, Default)]
#[command(about = "Echo text")]
struct EchoCli {
    #[arg(positional, help = "Text to echo")]
    message: Vec<String>,
}

pub struct EchoCommand;

impl ExecutableCommand<CommandContext> for EchoCommand {
    fn name(&self) -> &'static str {
        "echo"
    }

    fn description(&self) -> &'static str {
        "Echo text"
    }

    fn spec(&self) -> CommandSpec {
        CommandSpec::new("echo")
    }

    fn run(&self, args: &[String], ctx: &CommandContext) -> Result<(), String> {
        let Some(cli) = parse_cli::<EchoCli>(args, ctx, self.name()) else {
            return Ok(());
        };
        let msg = cli.message.join(" ");
        console::log_1(&msg.clone().into());
        ctx.terminal.push_text(msg);
        Ok(())
    }
}
