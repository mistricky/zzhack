use crate::commands::{parse_cli, CommandContext};
use micro_cli::Parser;
use shell_parser::integration::ExecutableCommand;
use shell_parser::CommandSpec;

#[derive(Parser, Debug, Default)]
#[command(about = "Show configured author name")]
struct WhoAmICli;

pub struct WhoAmICommand;

impl ExecutableCommand<CommandContext> for WhoAmICommand {
    fn name(&self) -> &'static str {
        "whoami"
    }

    fn description(&self) -> &'static str {
        "Display the configured author name from App.toml"
    }

    fn spec(&self) -> CommandSpec {
        CommandSpec::new("whoami").with_min_args(0).with_max_args(0)
    }

    fn run(&self, args: &[String], ctx: &CommandContext) -> Result<(), String> {
        let Some(_cli) = parse_cli::<WhoAmICli>(args, ctx, self.name()) else {
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
