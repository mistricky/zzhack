use crate::commands::{parse_cli, CommandContext};
use crate::router::run_route;
use micro_cli::Parser;
use shell_parser::integration::{CommandInfo, ExecutableCommand};
use wasm_bindgen_futures::spawn_local;

#[derive(Parser, Debug, Default)]
#[command(name = "navigate", about = "Navigate to a path and execute its route")]
pub struct NavigateCommand {
    #[arg(positional, help = "Path to navigate to")]
    path: String,
}

impl ExecutableCommand<CommandContext> for NavigateCommand {
    fn run(&self, args: &[String], ctx: &CommandContext) -> Result<(), String> {
        let Some(cli) = parse_cli::<NavigateCommand>(args, ctx, self.command_name()) else {
            return Ok(());
        };

        let path = cli.path;
        let terminal = ctx.terminal.clone();

        spawn_local(async move {
            terminal
                .execute_command(&format!("history --push {path}"))
                .await;
            match terminal.to_terminal() {
                Some(full_terminal) => run_route(&path, full_terminal),
                None => terminal.push_error("navigate: unable to execute route"),
            }
        });

        Ok(())
    }
}
