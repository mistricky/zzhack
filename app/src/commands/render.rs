use crate::commands::fetch::fetch_text_with_cache;
use crate::commands::{parse_cli, CommandContext};
use crate::components::markdown_renderer::{Avatar, Header};
use crate::config_service::ConfigService;
use crate::markdown_renderer::MarkdownRenderer;
use crate::vfs_data::{find_node, format_path, resolve_path, VfsKind, VfsNode};
use micro_cli::Parser;
use shell_parser::integration::{CommandInfo, ExecutableCommand};
use wasm_bindgen_futures::spawn_local;
use yew::{html, Html};

#[derive(Parser, Debug, Default)]
#[command(name = "render", about = "Render markdown content to HTML")]
pub struct RenderCommand {
    #[arg(positional, help = "Path to markdown file")]
    path: String,

    #[arg(
        short = 'r',
        long = "raw",
        help = "Render markdown file without header"
    )]
    raw: bool,
}

impl ExecutableCommand<CommandContext> for RenderCommand {
    fn run(&self, args: &[String], ctx: &CommandContext) -> Result<(), String> {
        let Some(cli) = parse_cli::<RenderCommand>(args, ctx, self.command_name()) else {
            return Ok(());
        };
        let ctx = ctx.clone();
        spawn_local(async move {
            run_render(cli, ctx).await;
        });
        Ok(())
    }
}

async fn run_render(cli: RenderCommand, ctx: CommandContext) {
    let target = &cli.path;

    let mut path = resolve_path(&ctx.terminal.cwd(), target);
    let mut node = match find_node(&ctx.vfs, &path) {
        Some(node) => node,
        None => {
            ctx.terminal
                .push_error(format!("render: {}: no such file", format_path(&path)));
            return;
        }
    };

    if node.kind == VfsKind::Directory {
        let index = node.children.as_ref().and_then(|children| {
            children
                .iter()
                .find(|child| child.name.eq_ignore_ascii_case("index.md") && is_markdown(child))
        });

        match index {
            Some(idx) => {
                let mut new_path = path.clone();
                new_path.push(idx.name.clone());
                path = new_path;
                node = idx;
            }
            None => {
                ctx.terminal.push_error(format!(
                    "render: {}: is a directory without index.md",
                    format_path(&path)
                ));
                return;
            }
        }
    } else if node.kind != VfsKind::File {
        ctx.terminal
            .push_error(format!("render: {}: is not a file", format_path(&path)));
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

    let author = ConfigService::get().author.clone();

    match fetch_text_with_cache(&uri, &cache).await {
        Ok(content) => {
            let rendered = MarkdownRenderer::new().render(&content);
            let node: Html = if cli.raw {
                rendered
            } else {
                html! {
                    <div class="py-6 pb-9 text-base text-post">
                        <Header metadata={node.clone()} />
                        <div class="flex items-center">
                            <Avatar name={author.name.clone()} email={author.email.clone()} />
                            <span class="text-base text-white ml-3">{&author.name}</span>
                        </div>
                        { rendered }
                    </div>
                }
            };
            ctx.terminal.push_component(node);
        }
        Err(err) => ctx.terminal.push_error(format!("render: {err}")),
    }
}

fn is_markdown(node: &VfsNode) -> bool {
    node.extension
        .as_deref()
        .map(|ext| ext.eq_ignore_ascii_case("md"))
        == Some(true)
}
