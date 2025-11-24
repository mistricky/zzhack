use crate::commands::{line_error, line_out, CommandContext, CommandHandler, CommandOutcome};
use crate::vfs_data::{du_bytes, find_node, format_path, resolve_path};
use async_trait::async_trait;
use shell_parser::CommandSpec;

pub struct DuCommand;

#[async_trait(?Send)]
impl CommandHandler for DuCommand {
    fn name(&self) -> &'static str {
        "du"
    }

    fn spec(&self) -> CommandSpec {
        CommandSpec::new("du").with_max_args(1)
    }

    async fn run(&self, args: &[String], ctx: &CommandContext) -> CommandOutcome {
        let target = args.get(0).map(String::as_str).unwrap_or(".");
        let path = resolve_path(&ctx.cwd, target);
        let mut lines = Vec::new();

        match find_node(&ctx.vfs, &path) {
            Some(node) => {
                let bytes = du_bytes(node);
                lines.push(line_out(format!(
                    "{} => {} bytes",
                    format_path(&path),
                    bytes
                )));
            }
            None => lines.push(line_error(format!("du: {}: not found", format_path(&path)))),
        }

        CommandOutcome {
            lines,
            new_cwd: None,
        }
    }
}
