use crate::commands::{parse_cli, CommandContext};
use micro_cli::Parser;
use shell_parser::integration::{CommandInfo, ExecutableCommand};
use web_sys::console;

#[derive(Parser, Debug, Default)]
#[command(name = "help", about = "Print the given text to the console")]
pub struct HelpCommand;

impl ExecutableCommand<CommandContext> for HelpCommand {
    fn run(&self, args: &[String], ctx: &CommandContext) -> Result<(), String> {
        match ctx.terminal.help() {
            Ok(help_message) => ctx.terminal.push_text(help_message),
            Err(err) => return Err(format!("Failed to get help message: {}", err)),
        };

        Ok(())
    }
}
