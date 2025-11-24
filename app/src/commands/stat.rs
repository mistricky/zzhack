use crate::commands::{CommandContext, CommandHandler};
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

    async fn run(&self, args: &[String], ctx: &CommandContext) {
        let Some(target) = args.get(0) else {
            ctx.terminal.push_error("stat: missing operand");
            return;
        };
        let path = resolve_path(&ctx.terminal.cwd(), target);
        match find_node(&ctx.vfs, &path) {
            Some(node) => {
                ctx.terminal
                    .push_text(format!("{} => {}", format_path(&path), node_summary(node)))
            }
            None => ctx
                .terminal
                .push_error(format!("stat: {}: not found", format_path(&path))),
        }
    }
}
