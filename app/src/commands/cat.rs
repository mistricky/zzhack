use crate::commands::fetch::fetch_text_with_cache;
use crate::commands::{parse_cli, CommandContext};
use crate::vfs_data::{find_node, format_path, resolve_path, VfsKind};
use micro_cli::Parser;
use shell_parser::integration::ExecutableCommand;
use shell_parser::CommandSpec;
use wasm_bindgen_futures::spawn_local;

#[derive(Parser, Debug, Default)]
#[command(about = "Print file contents")]
pub struct CatCommand {
    #[arg(positional, help = "Path to file")]
    path: String,
}

impl ExecutableCommand<CommandContext> for CatCommand {
    fn name(&self) -> &'static str {
        "cat"
    }

    fn description(&self) -> &'static str {
        "Print file contents"
    }

    fn spec(&self) -> CommandSpec {
        CommandSpec::new("cat").with_min_args(1).with_max_args(1)
    }

    fn run(&self, args: &[String], ctx: &CommandContext) -> Result<(), String> {
        let Some(cli) = parse_cli::<CatCommand>(args, ctx, self.name()) else {
            return Ok(());
        };
        let ctx = ctx.clone();
        spawn_local(async move {
            run_cat(cli, ctx).await;
        });
        Ok(())
    }
}

async fn run_cat(cli: CatCommand, ctx: CommandContext) {
    let target = &cli.path;

    let path = resolve_path(&ctx.terminal.cwd(), target);
    let Some(node) = find_node(&ctx.vfs, &path) else {
        ctx.terminal
            .push_error(format!("cat: {}: no such file", format_path(&path)));
        return;
    };

    if node.kind != VfsKind::File {
        ctx.terminal
            .push_error(format!("cat: {}: is a directory", format_path(&path)));
        return;
    }

    let Some(cache) = ctx.cache.clone() else {
        ctx.terminal
            .push_error("cat: cache unavailable (OPFS init failed)");
        return;
    };

    let uri = format!("/data/{}", path.join("/"));

    match fetch_text_with_cache(&uri, &cache).await {
        Ok(text) => ctx.terminal.push_text(text),
        Err(err) => ctx.terminal.push_error(format!("cat: {err}")),
    }
}
