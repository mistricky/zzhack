use crate::commands::{parse_cli, CommandContext};
use crate::vfs_data::{du_bytes, find_node, format_path, resolve_path};
use micro_cli::Parser;
use shell_parser::integration::ExecutableCommand;
use shell_parser::CommandSpec;

#[derive(Parser, Debug, Default)]
#[command(about = "Disk usage")]
struct DuCli {
    #[arg(positional, help = "Path to inspect")]
    path: Option<String>,
}

pub struct DuCommand;

impl ExecutableCommand<CommandContext> for DuCommand {
    fn name(&self) -> &'static str {
        "du"
    }

    fn description(&self) -> &'static str {
        "Disk usage"
    }

    fn spec(&self) -> CommandSpec {
        CommandSpec::new("du").with_max_args(1)
    }

    fn run(&self, args: &[String], ctx: &CommandContext) -> Result<(), String> {
        let Some(cli) = parse_cli::<DuCli>(args, ctx, self.name()) else {
            return Ok(());
        };
        let target = cli.path.as_deref().unwrap_or(".");
        let path = resolve_path(&ctx.terminal.cwd(), target);

        match find_node(&ctx.vfs, &path) {
            Some(node) => {
                let bytes = du_bytes(node);
                ctx.terminal
                    .push_text(format!("{} => {} bytes", format_path(&path), bytes));
            }
            None => ctx
                .terminal
                .push_error(format!("du: {}: not found", format_path(&path))),
        }
        Ok(())
    }
}
