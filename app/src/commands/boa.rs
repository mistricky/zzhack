use crate::commands::fetch::fetch_text_with_cache;
use crate::commands::{CommandContext, CommandHandler};
use crate::vfs_data::{find_node, format_path, resolve_path, VfsKind};
use async_trait::async_trait;
use boa_engine::{Context, Source};
use shell_parser::CommandSpec;

pub struct BoaCommand;

#[async_trait(?Send)]
impl CommandHandler for BoaCommand {
    fn name(&self) -> &'static str {
        "boa"
    }

    fn spec(&self) -> CommandSpec {
        CommandSpec::new("boa").with_min_args(1).with_max_args(1)
    }

    async fn run(&self, args: &[String], ctx: &CommandContext) {
        let Some(target) = args.get(0) else {
            ctx.terminal.push_error("boa: missing script path");
            return;
        };

        let path = resolve_path(&ctx.terminal.cwd(), target);
        let Some(node) = find_node(&ctx.vfs, &path) else {
            ctx.terminal
                .push_error(format!("boa: {}: no such file", format_path(&path)));
            return;
        };

        if node.kind != VfsKind::File {
            ctx.terminal
                .push_error(format!("boa: {}: is a directory", format_path(&path)));
            return;
        }

        let Some(cache) = ctx.cache.clone() else {
            ctx.terminal
                .push_error("boa: cache unavailable (OPFS init failed)");
            return;
        };

        let uri = format!("/data/{}", path.join("/"));

        let source = match fetch_text_with_cache(&uri, &cache).await {
            Ok(text) => text,
            Err(err) => {
                ctx.terminal.push_error(format!("boa: {err}"));
                return;
            }
        };

        let mut boa_ctx = Context::default();
        match boa_ctx.eval(Source::from_bytes(source.as_bytes())) {
            Ok(value) => match value.to_string(&mut boa_ctx) {
                Ok(out) => ctx.terminal.push_text(out.to_std_string_escaped()),
                Err(err) => ctx
                    .terminal
                    .push_error(format!("boa: failed to stringify result: {err}")),
            },
            Err(err) => ctx.terminal.push_error(format!("boa: {err}")),
        }
    }
}
