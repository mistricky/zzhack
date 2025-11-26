use crate::commands::{parse_cli, CommandContext};
use micro_cli::Parser;
use shell_parser::integration::{CommandInfo, ExecutableCommand};

#[derive(Parser, Debug, Default)]
#[command(name = "clear", about = "Clear the terminal")]
pub struct ClearCommand;

impl ExecutableCommand<CommandContext> for ClearCommand {
    fn run(&self, args: &[String], ctx: &CommandContext) -> Result<(), String> {
        let _ = parse_cli::<ClearCommand>(args, ctx, self.command_name());
        ctx.terminal.clear();
        Ok(())
    }
}
