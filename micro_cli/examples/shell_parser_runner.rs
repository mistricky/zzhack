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

fn main() {
    let context = RunnerContext {
        prefix: "[ctx]".to_string(),
    };
    let commands: Vec<Box<dyn ExecutableCommand<RunnerContext>>> =
        vec![Box::new(EchoCli::default()), Box::new(AddCli::default())];
    let runner = with_cli(context, commands);

    // Execute a script through shell_parser and dispatch to the derived CLIs.
    let script = r#"
        echo --name Micro --count 2
        add --lhs 2 --rhs 3
        echo --help
        add -h
    "#;

    if let Err(err) = runner.run_script(script) {
        eprintln!("error: {err}");
    }

    runner.run_line("echo --help").unwrap();
}
