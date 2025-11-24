use crate::commands::{line_error, line_out, CommandContext, CommandHandler, CommandOutcome};
use crate::vfs_data::{find_node, format_path, node_summary, resolve_path};
use async_trait::async_trait;
use shell_parser::CommandSpec;

pub struct StatCommand;

#[async_trait(?Send)]
impl CommandHandler for StatCommand {
    fn name(&self) -> &'static str {
        "stat"
    }

    fn spec(&self) -> CommandSpec {
        CommandSpec::new("stat").with_min_args(1).with_max_args(1)
    }

    async fn run(&self, args: &[String], ctx: &CommandContext) -> CommandOutcome {
        let Some(target) = args.get(0) else {
            return CommandOutcome {
                lines: vec![line_error("stat: missing operand".into())],
                new_cwd: None,
            };
        };
        let path = resolve_path(&ctx.cwd, target);
        let mut lines = Vec::new();
        match find_node(&ctx.vfs, &path) {
            Some(node) => lines.push(line_out(format!(
                "{} => {}",
                format_path(&path),
                node_summary(node)
            ))),
            None => lines.push(line_error(format!(
                "stat: {}: not found",
                format_path(&path)
            ))),
        }

        CommandOutcome {
            lines,
            new_cwd: None,
        }
    }
}
