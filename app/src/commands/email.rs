use crate::commands::{parse_cli, CommandContext};
use js_sys::encode_uri_component;
use micro_cli::Parser;
use shell_parser::integration::{CommandInfo, ExecutableCommand};
use web_sys::window;

#[derive(Parser, Debug, Default)]
#[command(
    name = "email",
    about = "Display or send an email to the configured author"
)]
pub struct EmailCommand {
    #[arg(
        positional,
        help = "Optional subject/body to include in the mailto link"
    )]
    message: Vec<String>,
}

impl ExecutableCommand<CommandContext> for EmailCommand {
    fn run(&self, args: &[String], ctx: &CommandContext) -> Result<(), String> {
        let Some(cli) = parse_cli::<EmailCommand>(args, ctx, self.command_name()) else {
            return Ok(());
        };

        let email = ctx.config.author.email.trim();
        if email.is_empty() {
            ctx.terminal
                .push_error("email: author.email is empty in App.toml");
            return Ok(());
        }

        if cli.message.is_empty() {
            ctx.terminal.push_text(email.to_string());
            return Ok(());
        }

        let subject = cli.message.join(" ");
        let encoded = encode_uri_component(&subject);
        let mailto = format!("mailto:{email}?subject={encoded}");

        match window().and_then(|win| win.location().set_href(&mailto).ok()) {
            Some(_) => {
                ctx.terminal.push_text(format!(
                    "Opening mailto for {email} with subject \"{subject}\""
                ));
            }
            None => {
                ctx.terminal.push_error("email: failed to open mailto link");
            }
        }
        Ok(())
    }
}
