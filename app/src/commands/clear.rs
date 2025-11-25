use crate::commands::{parse_cli, CommandContext};
use micro_cli::Parser;
use shell_parser::integration::ExecutableCommand;
use shell_parser::CommandSpec;

#[derive(Parser, Debug, Default)]
#[command(about = "Clear the terminal")]
struct ClearCli;

pub struct ClearCommand;

impl ExecutableCommand<CommandContext> for ClearCommand {
    fn name(&self) -> &'static str {
        "clear"
    }

    fn description(&self) -> &'static str {
        "Clear the terminal"
    }

    fn spec(&self) -> CommandSpec {
        CommandSpec::new("clear").with_max_args(0)
    }

    fn run(&self, args: &[String], ctx: &CommandContext) -> Result<(), String> {
        let _ = parse_cli::<ClearCli>(args, ctx, self.name());
        ctx.terminal.clear();
        Ok(())
    }
}
