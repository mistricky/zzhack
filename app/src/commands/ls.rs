use crate::commands::{parse_cli, CommandContext};
use crate::vfs_data::{find_node, format_path, resolve_path, VfsKind};
use micro_cli::Parser;
use shell_parser::integration::ExecutableCommand;
use shell_parser::CommandSpec;

#[derive(Parser, Debug, Default)]
#[command(about = "List directory contents")]
struct LsCli {
    #[arg(positional, help = "Path to list")]
    path: Option<String>,
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
