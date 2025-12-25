use crate::commands::fetch::fetch_text_with_cache;
use crate::commands::{parse_cli, CommandContext};
use crate::vfs_data::{find_node, format_path, resolve_path, VfsKind};
use micro_cli::Parser;
use shell_parser::integration::{CommandInfo, ExecutableCommand};
use wasm_bindgen_futures::spawn_local;

#[derive(Parser, Debug, Default)]
#[command(name = "eval", about = "Execute commands from a file")]
pub struct EvalCommand {
    #[arg(positional, help = "Path to the script file")]
    path: String,
}

impl ExecutableCommand<CommandContext> for EvalCommand {
    fn run(&self, args: &[String], ctx: &CommandContext) -> Result<(), String> {
        let Some(cli) = parse_cli::<EvalCommand>(args, ctx, self.command_name()) else {
            return Ok(());
        };

        let ctx = ctx.clone();
        spawn_local(async move {
            run_eval(cli, ctx).await;
        });

        Ok(())
    }
}

async fn run_eval(cli: EvalCommand, ctx: CommandContext) {
    let path = resolve_path(&ctx.terminal.cwd(), &cli.path);
    let Some(node) = find_node(&ctx.vfs, &path) else {
        ctx.terminal
            .push_error(format!("eval: {}: no such file", format_path(&path)));
        return;
    };

    if node.kind != VfsKind::File {
        ctx.terminal
            .push_error(format!("eval: {}: is a directory", format_path(&path)));
        return;
    }

    let Some(cache) = ctx.cache.clone() else {
        ctx.terminal
            .push_error("eval: cache unavailable (OPFS init failed)");
        return;
    };

    let uri = format!("/data/{}", path.join("/"));
    match fetch_text_with_cache(&uri, &cache).await {
        Ok(text) => {
            let script = text.trim();
            if script.is_empty() {
                ctx.terminal
                    .push_error(format!("eval: {}: file is empty", format_path(&path)));
                return;
            }
            ctx.terminal.execute_command(script);
        }
        Err(err) => {
            ctx.terminal.push_error(format!("eval: {err}"));
        }
    }
}
