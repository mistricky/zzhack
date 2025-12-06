use shell_parser::{CommandInvocation, CommandSpec, ShellParser};

fn main() {
    let parser = ShellParser::with_commands([
        CommandSpec::new("list", "List directory entries").with_min_args(0),
        CommandSpec::new("echo", "Echo text"),
    ]);

    let script = r#"
        alias ll="list -l --color=always"
        alias greet='echo hello; echo welcome'
        alias warn="echo \"warning:\""

        ll src
        greet "to the demo"
        warn "mind the gap"
    "#;

    let invocations = parser.parse(script).expect("script parses");
    for invocation in invocations {
        run(invocation);
    }
}

fn run(invocation: CommandInvocation) {
    match invocation.name.as_str() {
        "list" => println!("list {:?} -> canonical handler", invocation.args),
        "echo" => println!("echo {:?}", invocation.args),
        "alias" => println!("registered alias {:?}", invocation.args),
        other => eprintln!("unhandled command: {other} {:?}", invocation.args),
    }
}
