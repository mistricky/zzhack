mod cli {
    pub mod add_cli;
    pub mod echo_cli;
}

use cli::add_cli::AddCli;
use cli::echo_cli::EchoCli;
use shell_parser::integration::with_cli;

fn main() {
    let runner = with_cli(vec![
        Box::new(EchoCli::default()) as Box<dyn shell_parser::integration::ExecutableCommand>,
        Box::new(AddCli::default()) as Box<dyn shell_parser::integration::ExecutableCommand>,
    ]);

    // Execute a script through shell_parser and dispatch to the derived CLIs.
    let script = r#"
        echo --name Micro --count 2
        add --lhs 2 --rhs 3
        echo --help
        echo -v
        add -h
    "#;

    if let Err(err) = runner.run_script(script) {
        eprintln!("error: {err}");
    }

    runner.run_line("echo --help").unwrap();
}
