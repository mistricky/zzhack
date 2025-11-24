use crate::commands::{line_error, CommandContext, CommandHandler, CommandOutcome};
use crate::vfs_data::{find_node, format_path, resolve_path, VfsKind};
use async_trait::async_trait;
use shell_parser::CommandSpec;

pub struct CdCommand;

#[async_trait(?Send)]
impl CommandHandler for CdCommand {
    fn name(&self) -> &'static str {
        "cd"
    }

    fn spec(&self) -> CommandSpec {
        CommandSpec::new("cd").with_max_args(1)
    }

    async fn run(&self, args: &[String], ctx: &CommandContext) -> CommandOutcome {
        let target = args.get(0).map(String::as_str).unwrap_or("/");
        let path = resolve_path(&ctx.cwd, target);
        match find_node(&ctx.vfs, &path) {
            Some(node) if node.kind == VfsKind::Directory => CommandOutcome {
                lines: Vec::new(),
                new_cwd: Some(path),
            },
            Some(_) => CommandOutcome {
                lines: vec![line_error(format!(
                    "cd: {}: not a directory",
                    format_path(&path)
                ))],
                new_cwd: None,
            },
            None => CommandOutcome {
                lines: vec![line_error(format!(
                    "cd: {}: no such directory",
                    format_path(&path)
                ))],
                new_cwd: None,
            },
        }
    }
}
