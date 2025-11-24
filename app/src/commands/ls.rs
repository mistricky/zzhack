use crate::commands::{line_error, line_out, CommandContext, CommandHandler, CommandOutcome};
use crate::vfs_data::{find_node, format_path, list_children, resolve_path, VfsKind};
use async_trait::async_trait;
use shell_parser::CommandSpec;

pub struct LsCommand;

#[async_trait(?Send)]
impl CommandHandler for LsCommand {
    fn name(&self) -> &'static str {
        "ls"
    }

    fn spec(&self) -> CommandSpec {
        CommandSpec::new("ls").with_max_args(1)
    }

    async fn run(&self, args: &[String], ctx: &CommandContext) -> CommandOutcome {
        let target = args.get(0).map(String::as_str).unwrap_or(".");
        let path = resolve_path(&ctx.cwd, target);
        let mut lines = Vec::new();

        match find_node(&ctx.vfs, &path) {
            Some(node) if node.kind == VfsKind::Directory => {
                if let Some(names) = list_children(node) {
                    lines.push(line_out(names.join("  ")));
                } else {
                    lines.push(line_error("ls: empty directory".into()));
                }
            }
            Some(_) => lines.push(line_error(format!(
                "ls: {}: not a directory",
                format_path(&path)
            ))),
            None => lines.push(line_error(format!(
                "ls: {}: no such file or directory",
                format_path(&path)
            ))),
        }

        CommandOutcome {
            lines,
            new_cwd: None,
        }
    }
}
