use shell_parser::{CommandSpec, ShellParser};

fn main() {
    let parser = ShellParser::with_commands([
        CommandSpec::new("echo", "Echo text"),
        CommandSpec::new("render", "Render documents"),
    ]);

    let script = r#"
        alias echo_raw="builtin echo"

        function echo_with_prompt() {
            echo_raw "$@"
            render --doc 02_help.md
        }

        alias echo="echo_with_prompt"

        echo "through alias"
        builtin echo "bypasses alias"
    "#;

    let invocations = parser.parse(script).expect("script parses");
    for invocation in invocations {
        println!("{invocation}");
    }
}
