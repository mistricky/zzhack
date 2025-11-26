use crate::commands::{parse_cli, CommandContext};
use crate::vfs_data::format_path;
use micro_cli::Parser;
use shell_parser::integration::ExecutableCommand;
use shell_parser::CommandSpec;

#[derive(Parser, Debug, Default)]
#[command(about = "Print working directory")]
pub struct PwdCommand;

impl ExecutableCommand<CommandContext> for PwdCommand {
    fn name(&self) -> &'static str {
        "pwd"
    }

    fn description(&self) -> &'static str {
        "Print working directory"
    }

    fn spec(&self) -> CommandSpec {
        CommandSpec::new("pwd").with_max_args(0)
    }

    fn run(&self, args: &[String], ctx: &CommandContext) -> Result<(), String> {
        let _ = parse_cli::<PwdCommand>(args, ctx, self.name());
        ctx.terminal.push_text(format_path(&ctx.terminal.cwd()));
        Ok(())
    }
}
