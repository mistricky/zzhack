use crate::commands::{parse_cli, CommandContext};
use crate::vfs_data::{find_node, format_path, node_summary, resolve_path};
use micro_cli::Parser;
use shell_parser::integration::{CommandInfo, ExecutableCommand};

#[derive(Parser, Debug, Default)]
#[command(name = "stat", about = "Display file or directory metadata")]
pub struct StatCommand {
    #[arg(positional, help = "Path to inspect")]
    path: String,
}

impl ExecutableCommand<CommandContext> for StatCommand {
    fn run(&self, args: &[String], ctx: &CommandContext) -> Result<(), String> {
        let Some(cli) = parse_cli::<StatCommand>(args, ctx, self.command_name()) else {
            return Ok(());
        };
        let path = resolve_path(&ctx.terminal.cwd(), &cli.path);
        match find_node(&ctx.vfs, &path) {
            Some(node) => {
                ctx.terminal
                    .push_text(format!("{} => {}", format_path(&path), node_summary(node)))
            }
            None => ctx
                .terminal
                .push_error(format!("stat: {}: not found", format_path(&path))),
        }
        Ok(())
    }
}
