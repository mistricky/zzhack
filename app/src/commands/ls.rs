use crate::commands::{parse_cli, CommandContext};
use crate::vfs_data::{find_node, format_path, resolve_path, VfsKind, VfsNode};
use micro_cli::Parser;
use shell_parser::integration::ExecutableCommand;
use shell_parser::CommandSpec;
use std::cmp::Ordering;
use yew::prelude::*;

#[derive(Parser, Debug, Default)]
#[command(about = "List directory contents")]
struct LsCli {
    #[arg(positional, help = "Path to list")]
    path: Option<String>,
    #[arg(
        short = 'p',
        long = "posts",
        help = "List markdown posts in the directory"
    )]
    posts: bool,
}

pub struct LsCommand;

impl ExecutableCommand<CommandContext> for LsCommand {
    fn name(&self) -> &'static str {
        "ls"
    }

    fn description(&self) -> &'static str {
        "List directory contents"
    }

    fn spec(&self) -> CommandSpec {
        CommandSpec::new("ls").with_max_args(1)
    }

    fn run(&self, args: &[String], ctx: &CommandContext) -> Result<(), String> {
        let Some(cli) = parse_cli::<LsCli>(args, ctx, self.name()) else {
            return Ok(());
        };
        let target = cli.path.as_deref().unwrap_or(".");
        let path = resolve_path(&ctx.terminal.cwd(), target);

        if cli.posts {
            return self.list_posts(ctx, &path);
        }

        match find_node(&ctx.vfs, &path) {
            Some(node) if node.kind == VfsKind::Directory => match &node.children {
                Some(children) if !children.is_empty() => {
                    let mut entries: Vec<(String, String)> = children
                        .iter()
                        .map(|child| {
                            let sort_key = child.name.clone();
                            let display = if child.kind == VfsKind::Directory {
                                format!(r#"<span class="text-sky-300">{}/</span>"#, child.name)
                            } else {
                                format!(r#"<span class="text-slate-100">{}</span>"#, child.name)
                            };
                            (sort_key, display)
                        })
                        .collect();
                    entries.sort_by(|a, b| a.0.cmp(&b.0));
                    let rendered = entries
                        .into_iter()
                        .map(|(_, display)| display)
                        .collect::<Vec<_>>()
                        .join("  ");
                    ctx.terminal.push_html(rendered);
                }
                _ => ctx.terminal.push_error("ls: empty directory"),
            },
            Some(_) => ctx
                .terminal
                .push_error(format!("ls: {}: not a directory", format_path(&path))),
            None => ctx.terminal.push_error(format!(
                "ls: {}: no such file or directory",
                format_path(&path)
            )),
        }
        Ok(())
    }
}

impl LsCommand {
    fn list_posts(&self, ctx: &CommandContext, path: &[String]) -> Result<(), String> {
        let Some(node) = find_node(&ctx.vfs, path) else {
            ctx.terminal.push_error(format!(
                "ls --posts: {}: no such directory",
                format_path(path)
            ));
            return Ok(());
        };

        if node.kind != VfsKind::Directory {
            ctx.terminal.push_error(format!(
                "ls --posts: {}: not a directory",
                format_path(path)
            ));
            return Ok(());
        }

        let Some(children) = &node.children else {
            ctx.terminal.push_error("ls --posts: empty directory");
            return Ok(());
        };

        let mut posts: Vec<PostEntry> = children
            .iter()
            .filter_map(|child| PostEntry::from_node(child))
            .collect();

        if posts.is_empty() {
            ctx.terminal
                .push_error("ls --posts: no markdown posts found");
            return Ok(());
        }

        posts.sort_by(|a, b| match (&a.modified, &b.modified) {
            (Some(la), Some(lb)) => lb.cmp(la),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => a.title.cmp(&b.title),
        });

        let rendered = html! {
            <div class="space-y-3">
                { for posts.iter().map(render_post) }
            </div>
        };

        ctx.terminal.push_component(rendered);
        Ok(())
    }
}

struct PostEntry {
    title: String,
    description: Option<String>,
    modified: Option<String>,
}

fn render_post(post: &PostEntry) -> Html {
    html! {
        <div class="flex flex-col gap-1">
            <div class="flex items-center gap-3">
                <span class="text-slate-100 font-semibold">{ &post.title }</span>
                <span class="text-slate-500 text-xs">
                    { post.modified.clone().unwrap_or_else(|| "-".to_string()) }
                </span>
            </div>
            if let Some(desc) = &post.description {
                <div class="text-slate-400 text-sm">{ desc }</div>
            }
        </div>
    }
}

impl PostEntry {
    fn from_node(node: &VfsNode) -> Option<Self> {
        match node.kind {
            VfsKind::File if is_markdown(node) => Some(Self {
                title: preferred_title(node, &node.name),
                description: node.description.clone(),
                modified: node.modified.clone(),
            }),
            VfsKind::Directory if node.is_post => {
                let index = node
                    .children
                    .as_ref()
                    .and_then(|children| {
                        children
                            .iter()
                            .find(|child| child.name.eq_ignore_ascii_case("index.md"))
                    })
                    .filter(|child| is_markdown(child))?;

                Some(Self {
                    title: preferred_title(index, &node.name),
                    description: index.description.clone(),
                    modified: index.modified.clone(),
                })
            }
            _ => None,
        }
    }
}

fn preferred_title(node: &VfsNode, fallback: &str) -> String {
    if let Some(title) = &node.title {
        return title.clone();
    }
    if let Some(ext) = &node.extension {
        if let Some(stripped) = fallback.strip_suffix(&format!(".{ext}")) {
            return stripped.to_string();
        }
    }
    fallback.to_string()
}

fn is_markdown(node: &VfsNode) -> bool {
    node.kind == VfsKind::File
        && node
            .extension
            .as_deref()
            .map(|ext| ext.eq_ignore_ascii_case("md"))
            == Some(true)
}
