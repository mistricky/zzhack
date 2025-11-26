use crate::commands::fetch::fetch_text_with_cache;
use crate::commands::{parse_cli, CommandContext};
use crate::vfs_data::{find_node, format_path, resolve_path, VfsKind};
use boa_engine::{Context, Source};
use micro_cli::Parser;
use shell_parser::integration::{CommandInfo, ExecutableCommand};
use wasm_bindgen_futures::spawn_local;

#[derive(Parser, Debug, Default)]
#[command(name = "boa", about = "Run a JavaScript file with Boa")]
pub struct BoaCommand {
    #[arg(positional, help = "Script path")]
    path: String,
}

impl ExecutableCommand<CommandContext> for BoaCommand {
    fn run(&self, args: &[String], ctx: &CommandContext) -> Result<(), String> {
        let Some(cli) = parse_cli::<BoaCommand>(args, ctx, self.command_name()) else {
            return Ok(());
        };
        let ctx = ctx.clone();
        spawn_local(async move {
            run_boa(cli, ctx).await;
        });
        Ok(())
    }
}

async fn run_boa(cli: BoaCommand, ctx: CommandContext) {
    let target = &cli.path;
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
