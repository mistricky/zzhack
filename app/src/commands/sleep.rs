use micro_cli::Parser;
use shell_parser::integration::{CommandInfo, ExecutableCommand};
use shell_parser::pause_signal;

use crate::commands::{parse_cli, CommandContext};

#[derive(Parser, Debug, Default)]
#[command(name = "sleep", about = "Pause execution for a number of milliseconds")]
pub struct SleepCommand {
    #[arg(positional, help = "Duration in milliseconds")]
    millis: u32,
}

impl ExecutableCommand<CommandContext> for SleepCommand {
    fn run(&self, args: &[String], ctx: &CommandContext) -> Result<(), String> {
        let Some(cli) = parse_cli::<SleepCommand>(args, ctx, self.command_name()) else {
            return Ok(());
        };

        Err(pause_signal(cli.millis))
    }
}
