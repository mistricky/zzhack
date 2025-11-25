use crate::commands::fetch::fetch_text_with_cache;
use crate::commands::{parse_cli, CommandContext};
use crate::markdown_renderer::render_markdown_to_html;
use crate::vfs_data::{find_node, format_path, resolve_path, VfsKind};
use micro_cli::Parser;
use shell_parser::integration::ExecutableCommand;
use shell_parser::CommandSpec;
use wasm_bindgen_futures::spawn_local;

#[derive(Parser, Debug, Default)]
#[command(about = "Render markdown content to HTML")]
struct RenderCli {
    #[arg(positional, help = "Path to markdown file")]
    path: String,
}

pub struct RenderCommand;

impl ExecutableCommand<CommandContext> for RenderCommand {
    fn name(&self) -> &'static str {
        "render"
    }

    fn description(&self) -> &'static str {
        "Render markdown content to HTML"
    }

    fn spec(&self) -> CommandSpec {
        CommandSpec::new("render").with_min_args(1).with_max_args(1)
    }

    fn run(&self, args: &[String], ctx: &CommandContext) -> Result<(), String> {
        let Some(cli) = parse_cli::<RenderCli>(args, ctx, self.name()) else {
            return Ok(());
        };
        let ctx = ctx.clone();
        spawn_local(async move {
            run_render(cli, ctx).await;
        });
        Ok(())
    }
}

async fn run_render(cli: RenderCli, ctx: CommandContext) {
    let target = &cli.path;

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

    let uri = format!("/data/{}", path.join("/"));

    let image_extensions = ["png", "jpg", "jpeg", "gif", "bmp", "webp", "svg"];
    if node
        .extension
        .as_deref()
        .map(|ext| image_extensions.contains(&ext.to_ascii_lowercase().as_str()))
        .unwrap_or(false)
    {
        ctx.terminal
            .push_html(format!(r#"<img src="{uri}" alt="rendered image" />"#));
        return;
    }

    let Some(cache) = ctx.cache.clone() else {
        ctx.terminal
            .push_error("render: cache unavailable (OPFS init failed)");
        return;
    };

    match fetch_text_with_cache(&uri, &cache).await {
        Ok(content) => {
            let rendered = render_markdown_to_html(&content);
            ctx.terminal.push_html(rendered);
        }
        Err(err) => ctx.terminal.push_error(format!("render: {err}")),
    }
}
