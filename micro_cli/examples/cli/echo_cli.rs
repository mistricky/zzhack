use micro_cli::{CliError, Parser};
use shell_parser::command::CommandSpec;
use shell_parser::integration::ExecutableCommand;

#[derive(Parser, Debug, Default)]
#[command(about = "Echo text", version = "0.1.0")]
pub struct EchoCli {
    #[arg(short = 'n', long = "name", help = "Name to greet")]
    pub name: String,
    #[arg(
        short = 'c',
        long = "count",
        default_value_t = 1,
        help = "Repeat count"
    )]
    pub count: u8,
}

impl ExecutableCommand for EchoCli {
    fn name(&self) -> &'static str {
        "echo"
    }

    fn description(&self) -> &'static str {
        "Echo text"
    }

    fn spec(&self) -> CommandSpec {
        CommandSpec::new(self.name())
    }

    fn run(&self, args: &[String]) -> Result<(), String> {
        let parsed = match EchoCli::parse_from(args.to_vec()) {
            Ok(ok) => ok,
            Err(CliError::Help(text)) => {
                println!("{text}");
                return Ok(());
            }
            Err(err) => return Err(err.to_string()),
        };
        println!(
            "{} - {}",
            <EchoCli as Parser>::name(),
            <EchoCli as Parser>::description()
        );
        for _ in 0..parsed.count {
            println!("Hello {}!", parsed.name);
        }
        Ok(())
    }

    fn run_with_input(
        &self,
        args: &[String],
        _input: Option<String>,
    ) -> Result<Option<String>, String> {
        let parsed = match EchoCli::parse_from(args.to_vec()) {
            Ok(ok) => ok,
            Err(CliError::Help(text)) => {
                println!("{text}");
                return Ok(None);
            }
            Err(err) => return Err(err.to_string()),
        };
        let mut outputs = Vec::new();
        for _ in 0..parsed.count {
            outputs.push(parsed.name.clone());
        }
        let combined = outputs.join("\n");
        println!(
            "{} - {}",
            <EchoCli as Parser>::name(),
            <EchoCli as Parser>::description()
        );
        for line in combined.lines() {
            println!("Hello {}!", line);
        }
        Ok(Some(format!("Hello {}!", parsed.name)))
    }
}
