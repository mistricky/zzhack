use crate::commands::{parse_cli, CommandContext};
use crate::components::PostItem;
use crate::vfs_data::{find_node, format_path, resolve_path, VfsKind, VfsNode};
use micro_cli::Parser;
use shell_parser::integration::{CommandInfo, ExecutableCommand};
use std::cmp::Ordering;
use yew::prelude::*;

#[derive(Parser, Debug, Default)]
#[command(name = "ls", about = "List directory contents")]
pub struct LsCommand {
    #[arg(positional, help = "Path to list")]
    path: Option<String>,
    #[arg(
        short = 'p',
        long = "posts",
        help = "List markdown posts in the directory"
    )]
    posts: bool,
}

impl ExecutableCommand<CommandContext> for LsCommand {
    fn run(&self, args: &[String], ctx: &CommandContext) -> Result<(), String> {
        let Some(cli) = parse_cli::<LsCommand>(args, ctx, self.command_name()) else {
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
                    let mut entries: Vec<&VfsNode> = children.iter().collect();
                    entries.sort_by(|a, b| a.name.cmp(&b.name));
                    let rendered = html! {
                        <div class="grid grid-cols-[repeat(auto-fit,minmax(8rem,1fr))] gap-x-2">
                            { for entries.into_iter().map(|child| {
                                let class = if child.kind == VfsKind::Directory {
                                    "text-emerald-400 font-bold"
                                } else {
                                    "text-slate-100"
                                };
                                let label = if child.kind == VfsKind::Directory {
                                    format!("{}/", child.name)
                                } else {
                                    child.name.clone()
                                };
                                html! {
                                    <span class={class}>{label}</span>
                                }
                            }) }
                        </div>
                    };
                    ctx.terminal.push_component(rendered);
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

        let mut posts: Vec<PostEntry> = match node.kind {
            VfsKind::Directory => {
                let Some(children) = &node.children else {
                    ctx.terminal.push_error("ls --posts: empty directory");
                    return Ok(());
                };
                children
                    .iter()
                    .filter_map(|child| PostEntry::from_node(child))
                    .collect()
            }
            VfsKind::File if is_markdown(node) => vec![PostEntry::from_node(node).unwrap()],
            _ => {
                ctx.terminal.push_error(format!(
                    "ls --posts: {}: not a markdown file or directory",
                    format_path(path)
                ));
                return Ok(());
            }
        };

        if posts.is_empty() {
            ctx.terminal
                .push_error("ls --posts: no markdown posts found");
            return Ok(());
        }

        posts.sort_by(|a, b| match (&a.metadata.modified, &b.metadata.modified) {
            (Some(la), Some(lb)) => lb.cmp(la),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => a.metadata.name.cmp(&b.metadata.name),
        });

        let on_post_click = {
            let terminal = ctx.terminal.clone();
            Callback::from(move |metadata: VfsNode| {
                let path = format!("/posts/{}", metadata.path);
                terminal.execute_command(&format!("navigate {path}"));
            })
        };

        ctx.terminal
            .push_component(render_posts(&posts, on_post_click));
        Ok(())
    }
}

struct PostEntry {
    metadata: VfsNode,
}

impl PostEntry {
    fn from_node(node: &VfsNode) -> Option<Self> {
        match node.kind {
            VfsKind::File if is_markdown(node) => Some(Self {
                metadata: node.clone(),
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
                    metadata: index.clone(),
                })
            }
            _ => None,
        }
    }
}

fn render_posts(posts: &[PostEntry], on_click: Callback<VfsNode>) -> Html {
    html! {
        <div class="py-6 space-y-3">
            { for posts.iter().map(|post| {
                html! {
                    <PostItem metadata={post.metadata.clone()} on_click={on_click.clone()} />
                }
            }) }
        </div>
    }
}

fn is_markdown(node: &VfsNode) -> bool {
    node.kind == VfsKind::File
        && node
            .extension
            .as_deref()
            .map(|ext| ext.eq_ignore_ascii_case("md"))
            == Some(true)
}
