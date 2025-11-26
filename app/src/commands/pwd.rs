use crate::commands::{parse_cli, CommandContext};
use crate::vfs_data::format_path;
use micro_cli::Parser;
use shell_parser::integration::{CommandInfo, ExecutableCommand};

#[derive(Parser, Debug, Default)]
#[command(name = "pwd", about = "Print working directory")]
pub struct PwdCommand;

impl ExecutableCommand<CommandContext> for PwdCommand {
    fn run(&self, args: &[String], ctx: &CommandContext) -> Result<(), String> {
        let _ = parse_cli::<PwdCommand>(args, ctx, self.command_name());
        ctx.terminal.push_text(format_path(&ctx.terminal.cwd()));
        Ok(())
    }
}
