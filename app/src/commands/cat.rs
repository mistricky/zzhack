use crate::commands::{line_error, CommandContext, CommandHandler, CommandOutcome, FetchCommand};
use async_trait::async_trait;
use shell_parser::CommandSpec;

pub struct CatCommand;

#[async_trait(?Send)]
impl CommandHandler for CatCommand {
    fn name(&self) -> &'static str {
        "cat"
    }

    fn spec(&self) -> CommandSpec {
        CommandSpec::new("cat").with_min_args(1).with_max_args(1)
    }

    async fn run(&self, args: &[String], ctx: &CommandContext) -> CommandOutcome {
        let Some(target) = args.get(0) else {
            return CommandOutcome {
                lines: vec![line_error("cat: missing operand".into())],
                new_cwd: None,
            };
        };

        let uri = format!("/data/{target}");
        let fetch = FetchCommand;
        let fetch_args = vec![uri];

        fetch.run(&fetch_args, ctx).await
    }
}
