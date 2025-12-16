use shell_parser::{CommandInvocation, ShellParser};

fn main() {
    let parser = ShellParser::new();
    let script = r#"
        function greet() {
            echo hello $1
            echo "args:" $#
        }

        repeat() {
            greet $1
            greet $2
        }

        greet "terminal"
        repeat "micro" "cli"
    "#;

    let invocations = parser.parse(script).expect("script should parse");
    for invocation in invocations {
        run(invocation);
    }
}

fn run(invocation: CommandInvocation) {
    match invocation.name.as_str() {
        "function" => println!("registered function {:?}", invocation.args),
        other => println!("{other} {:?}", invocation.args),
    }
}
