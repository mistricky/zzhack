use crate::commands::fetch::fetch_text_with_cache;
use crate::commands::{CommandContext, CommandHandler};
use crate::markdown_renderer::render_markdown_to_html;
use crate::vfs_data::{find_node, format_path, resolve_path, VfsKind};
use async_trait::async_trait;
use shell_parser::CommandSpec;

pub struct RenderCommand;

#[async_trait(?Send)]
impl CommandHandler for RenderCommand {
    fn name(&self) -> &'static str {
        "render"
    }

    fn spec(&self) -> CommandSpec {
        CommandSpec::new("render").with_min_args(1).with_max_args(1)
    }

    async fn run(&self, args: &[String], ctx: &CommandContext) {
        let Some(target) = args.get(0) else {
            ctx.terminal.push_error("render: missing file path");
            return;
        };

        let path = resolve_path(&ctx.terminal.cwd(), target);
        let Some(node) = find_node(&ctx.vfs, &path) else {
            ctx.terminal
                .push_error(format!("render: {}: no such file", format_path(&path)));
            return;
        };

        if node.kind != VfsKind::File {
            ctx.terminal
                .push_error(format!("render: {}: is a directory", format_path(&path)));
            return;
        }

        let Some(cache) = ctx.cache.clone() else {
            ctx.terminal
                .push_error("render: cache unavailable (OPFS init failed)");
            return;
        };

        let uri = format!("/data/{}", path.join("/"));
        match fetch_text_with_cache(&uri, &cache).await {
            Ok(content) => {
                let rendered = render_markdown_to_html(&content);
                ctx.terminal.push_html(rendered);
            }
            Err(err) => ctx.terminal.push_error(format!("render: {err}")),
        }
    }
}
