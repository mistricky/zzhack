use crate::commands::{parse_cli, CommandContext};
use crate::vfs_data::{find_node, format_path, resolve_path, VfsKind};
use micro_cli::Parser;
use shell_parser::integration::{CommandInfo, ExecutableCommand};

#[derive(Parser, Debug, Default)]
#[command(name = "cd", about = "Change directory")]
pub struct CdCommand {
    #[arg(positional, help = "Directory path")]
    path: Option<String>,
}

impl ExecutableCommand<CommandContext> for CdCommand {
    fn run(&self, args: &[String], ctx: &CommandContext) -> Result<(), String> {
        let Some(cli) = parse_cli::<CdCommand>(args, ctx, self.command_name()) else {
            return Ok(());
        };
        let target = cli.path.as_deref().unwrap_or("/");
        let path = resolve_path(&ctx.terminal.cwd(), target);
        match find_node(&ctx.vfs, &path) {
            Some(node) if node.kind == VfsKind::Directory => {
                ctx.terminal.set_cwd(path.clone());
            }
            Some(_) => {
                ctx.terminal
                    .push_error(format!("cd: {}: not a directory", format_path(&path)));
            }
            None => {
                ctx.terminal
                    .push_error(format!("cd: {}: no such directory", format_path(&path)));
            }
        }
        Ok(())
    }
}
