use std::sync::{Arc, Mutex};

use crate::{cli, command, CliError, CommandContext, OptionSpec, Parser};

#[test]
fn runs_command_with_flag_and_args() {
    let output = Arc::new(Mutex::new(Vec::new()));
    let capture = output.clone();

    let echo = command! {
        name: "echo",
        about: "Print text",
        options: [
            OptionSpec::flag("no-newline", Some('n'), Some("no-newline"), "Skip trailing newline")
        ],
        run: |ctx: &CommandContext| {
            let text = ctx.args.join(" ");
            let out = if ctx.options.flag("no-newline") {
                text
            } else {
                format!("{text}\n")
            };
            capture.lock().unwrap().push(out);
            Ok(())
        }
    };

    let app = cli! {
        name: "demo",
        about: "Demo app",
        commands: [echo]
    };

    app.run(&vec![
        "echo".to_string(),
        "-n".to_string(),
        "hello".to_string(),
        "world".to_string(),
    ])
    .unwrap();

    assert_eq!(output.lock().unwrap().as_slice(), &["hello world"]);
}

#[test]
fn handles_subcommands_and_help() {
    let run_flag = Arc::new(Mutex::new(false));
    let flag_clone = run_flag.clone();

    let sub = command! {
        name: "child",
        about: "Child command",
        run: |_ctx: &CommandContext| {
            *flag_clone.lock().unwrap() = true;
            Ok(())
        }
    };

    let parent = command! {
        name: "parent",
        about: "Parent command",
        subcommands: [sub],
        run: |_ctx: &CommandContext| {
            Ok(())
        }
    };

    let app = cli! {
        name: "demo",
        about: "Demo app",
        commands: [parent]
    };

    app.run(&vec!["parent".into(), "child".into()]).unwrap();
    assert!(*run_flag.lock().unwrap());

    let err = app
        .run(&vec!["parent".into(), "--help".into()])
        .unwrap_err();
    match err {
        CliError::Help(text) => {
            assert!(text.contains("Usage"));
            assert!(text.contains("child"));
        }
        other => panic!("unexpected error: {:?}", other),
    }
}

#[test]
fn integrates_with_shell_parser_invocations() {
    let captured = Arc::new(Mutex::new(Vec::new()));
    let captured_clone = captured.clone();

    let echo = command! {
        name: "echo",
        about: "Echo text",
        run: |ctx: &CommandContext| {
            captured_clone.lock().unwrap().push(ctx.args.join(" "));
            Ok(())
        }
    };

    let app = cli! {
        name: "mini",
        about: "Mini shell",
        commands: [echo]
    };

    let parser = shell_parser::ShellParser::with_commands([shell_parser::CommandSpec::new(
        "echo",
        "Echo text",
    )]);
    let script = r#"echo hello world
echo "pipe friendly""#;
    let invocations = parser.parse(script).unwrap();

    for inv in invocations {
        app.run_command(&inv.name, &inv.args).unwrap();
    }

    let results = captured.lock().unwrap().clone();
    assert_eq!(
        results,
        vec!["hello world".to_string(), "pipe friendly".to_string()]
    );
}

#[derive(Parser, Debug)]
#[command(about = "Greet")]
struct GreetArgs {
    #[arg(short = 'n', long = "name", help = "Name")]
    name: String,
    #[arg(short = 'c', long = "count", default_value_t = 1, help = "Times")]
    count: u8,
}

#[test]
fn derives_parser_style() {
    let args = GreetArgs::parse_from(vec![
        "--name".to_string(),
        "Tester".to_string(),
        "--count".to_string(),
        "2".to_string(),
    ])
    .unwrap();
    assert_eq!(args.name, "Tester");
    assert_eq!(args.count, 2);

    let help = GreetArgs::help();
    assert!(help.contains("name"));
    assert!(help.contains("count"));
    assert!(!help.contains("version"));

    let help_err = GreetArgs::parse_from(Vec::<String>::new()).unwrap_err();
    assert!(matches!(help_err, CliError::Help(_)));

    let version_err = GreetArgs::parse_from(vec!["--version".to_string()]).unwrap_err();
    assert!(matches!(
        version_err,
        CliError::UnknownOption(ref flag) if flag == "--version"
    ));

    assert_eq!(GreetArgs::name(), "GreetArgs");
    assert_eq!(GreetArgs::description(), "Greet");
}
