use crate::commands::CommandContext;
use micro_cli::Parser;
use shell_parser::integration::ExecutableCommand;
use yew::html;

#[derive(Parser, Debug, Default)]
#[command(name = "help", about = "Print the given text to the console")]
pub struct HelpCommand;

impl ExecutableCommand<CommandContext> for HelpCommand {
    fn run(&self, _args: &[String], ctx: &CommandContext) -> Result<(), String> {
        match ctx.terminal.help() {
            Ok(help_message) => ctx.terminal.push_component(html! {
                <span class="whitespace-break-spaces">{help_message}</span>
            }),
            Err(err) => return Err(format!("Failed to get help message: {}", err)),
        };

        Ok(())
    }
}
