use micro_cli::{CliError, Parser};
use shell_parser::command::CommandSpec;
use shell_parser::integration::ExecutableCommand;

#[derive(Parser, Debug, Default)]
#[command(about = "Add two numbers", version = "0.1.0")]
pub struct AddCli {
    #[arg(short = 'l', long = "lhs", help = "Left-hand side")]
    pub lhs: i32,
    #[arg(short = 'r', long = "rhs", help = "Right-hand side")]
    pub rhs: i32,
}

impl ExecutableCommand for AddCli {
    fn name(&self) -> &'static str {
        "add"
    }

    fn description(&self) -> &'static str {
        "Add two numbers"
    }

    fn spec(&self) -> CommandSpec {
        CommandSpec::new(self.name())
    }

    fn run(&self, args: &[String]) -> Result<(), String> {
        let parsed = match AddCli::parse_from(args.to_vec()) {
            Ok(ok) => ok,
            Err(CliError::Help(text)) => {
                println!("{text}");
                return Ok(());
            }
            Err(err) => return Err(err.to_string()),
        };
        println!(
            "{} - {}",
            <AddCli as Parser>::name(),
            <AddCli as Parser>::description()
        );
        println!(
            "{} + {} = {}",
            parsed.lhs,
            parsed.rhs,
            parsed.lhs + parsed.rhs
        );
        Ok(())
    }

    fn run_with_input(
        &self,
        args: &[String],
        input: Option<String>,
    ) -> Result<Option<String>, String> {
        let parsed = match AddCli::parse_from(args.to_vec()) {
            Ok(ok) => ok,
            Err(CliError::Help(text)) => {
                println!("{text}");
                return Ok(None);
            }
            Err(err) => return Err(err.to_string()),
        };
        let lhs = input
            .and_then(|v| v.trim().parse::<i32>().ok())
            .unwrap_or(parsed.lhs);
        let rhs = parsed.rhs;
        let sum = lhs + rhs;
        println!(
            "{} - {}",
            <AddCli as Parser>::name(),
            <AddCli as Parser>::description()
        );
        println!("{} + {} = {}", lhs, rhs, sum);
        Ok(Some(sum.to_string()))
    }
}
