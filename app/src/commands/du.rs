use crate::commands::{CommandContext, CommandHandler};
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

    async fn run(&self, args: &[String], ctx: &CommandContext) {
        let target = args.get(0).map(String::as_str).unwrap_or(".");
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
    }
}
