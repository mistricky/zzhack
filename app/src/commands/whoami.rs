use crate::commands::{parse_cli, CommandContext};
use micro_cli::Parser;
use shell_parser::integration::{CommandInfo, ExecutableCommand};

#[derive(Parser, Debug, Default)]
#[command(name = "whoami", about = "Show configured author name")]
pub struct WhoAmICommand;

impl ExecutableCommand<CommandContext> for WhoAmICommand {
    fn run(&self, args: &[String], ctx: &CommandContext) -> Result<(), String> {
        let Some(_cli) = parse_cli::<WhoAmICommand>(args, ctx, self.command_name()) else {
            return Ok(());
        };
        let name = &ctx.config.author.name;
        if name.is_empty() {
            ctx.terminal
                .push_error("whoami: author.name is empty in App.toml");
        } else {
            ctx.terminal.push_text(name.clone());
        }
        Ok(())
    }
}
