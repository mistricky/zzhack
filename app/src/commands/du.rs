use crate::commands::{parse_cli, CommandContext};
use crate::vfs_data::{du_bytes, find_node, format_path, resolve_path};
use micro_cli::Parser;
use shell_parser::integration::{CommandInfo, ExecutableCommand};

#[derive(Parser, Debug, Default)]
#[command(name = "du", about = "Disk usage")]
pub struct DuCommand {
    #[arg(positional, help = "Path to inspect")]
    path: Option<String>,
}

impl ExecutableCommand<CommandContext> for DuCommand {
    fn run(&self, args: &[String], ctx: &CommandContext) -> Result<(), String> {
        let Some(cli) = parse_cli::<DuCommand>(args, ctx, self.command_name()) else {
            return Ok(());
        };
        let target = cli.path.as_deref().unwrap_or(".");
        let path = resolve_path(&ctx.terminal.cwd(), target);

        match find_node(&ctx.vfs, &path) {
            Some(node) => {
                let bytes = du_bytes(node);
                ctx.terminal
                    .push_text(format!("{} => {} bytes", format_path(&path), bytes));
            }
            None => ctx
                .terminal
                .push_error(format!("du: {}: not found", format_path(&path))),
        }
        Ok(())
    }
}
