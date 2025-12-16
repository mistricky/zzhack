mod cli {
    pub mod add_cli;
    pub mod echo_cli;

    #[derive(Debug, Clone)]
    pub struct RunnerContext {
        pub prefix: String,
    }
}

use cli::add_cli::AddCli;
use cli::echo_cli::EchoCli;
use cli::RunnerContext;
use shell_parser::integration::{with_cli, ExecutableCommand};
use shell_parser::ScriptResult;

fn main() {
    let context = RunnerContext {
        prefix: "[ctx]".to_string(),
    };
    let commands: Vec<Box<dyn ExecutableCommand<RunnerContext>>> =
        vec![Box::new(EchoCli::default()), Box::new(AddCli::default())];
    let runner = with_cli(context, commands);

    // Execute a script through shell_parser and dispatch to the derived CLIs.
    let script = r#"
        function greet() {
            echo --name hello $1
            echo --name "args:" $#
        }

        alias ll="echo";

        greet "world";

        echo --name Micro --count 2 && add --lhs 2 --rhs 3 && echo --name Done --count 1
        ll --help
        add -h
    "#;

    match runner.run_script(script) {
        Ok(ScriptResult::Completed) => {}
        Ok(ScriptResult::Paused {
            delay_ms,
            remainder,
        }) => {
            println!(
                "script paused for {delay_ms}ms; {} commands remaining",
                remainder.len()
            );
        }
        Err(err) => eprintln!("error: {err}"),
    }

    println!("{}", runner.help());
}
