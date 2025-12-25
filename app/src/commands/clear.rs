use crate::commands::{parse_cli, CommandContext};
use micro_cli::Parser;
use shell_parser::integration::{CommandInfo, ExecutableCommand};

#[derive(Parser, Debug, Default)]
#[command(name = "clear", about = "Clear the terminal")]
pub struct ClearCommand {
    #[arg(
        short = 'n',
        long = "number",
        help = "Clear the {number} output counting from the end"
    )]
    num: Option<usize>,
}

impl ExecutableCommand<CommandContext> for ClearCommand {
    fn run(&self, args: &[String], ctx: &CommandContext) -> Result<(), String> {
        let Some(cli) = parse_cli::<ClearCommand>(args, ctx, self.command_name()) else {
            return Ok(());
        };

        if let Some(num) = cli.num {
            ctx.terminal.clear(Some(num));
        } else {
            ctx.terminal.clear(None);
        }

        Ok(())
    }
}
