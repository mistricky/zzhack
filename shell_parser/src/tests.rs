use crate::separator::Separator;
use crate::{CommandInvocation, CommandSpec, ShellParseError, ShellParser};

fn command(name: &str, min: usize, max: Option<usize>) -> CommandSpec {
    CommandSpec {
        name: name.to_string(),
        about: String::new(),
        min_args: min,
        max_args: max,
        aliases: Vec::new(),
    }
}

#[test]
fn parses_basic_commands() {
    let parser = ShellParser::new();
    let script = "echo hello world\nrun-task alpha; # comment\n next \"line two\"";
    let parsed = parser.parse(script).unwrap();

    assert_eq!(parsed.len(), 3);
    assert_eq!(
        parsed[0],
        CommandInvocation {
            name: "echo".into(),
            args: vec!["hello".into(), "world".into()],
            position: 0
        }
    );
    assert_eq!(parsed[1].name, "run-task");
    assert_eq!(parsed[1].args, vec!["alpha"]);
    assert_eq!(parsed[2].args, vec![String::from("line two")]);
}

#[test]
fn handles_quotes_and_escapes() {
    let parser = ShellParser::new();
    let parsed = parser
        .parse("say \"hello world\" 'and more' escaped\\ space \"mix\\\"ed\"")
        .unwrap();
    assert_eq!(parsed.len(), 1);
    assert_eq!(
        parsed[0].args,
        vec![
            String::from("hello world"),
            String::from("and more"),
            String::from("escaped space"),
            String::from("mix\"ed")
        ]
    );
}

#[test]
fn validates_registered_commands() {
    let parser =
        ShellParser::with_commands([command("echo", 1, Some(2)), command("exit", 0, Some(0))]);

    let ok = parser.parse("echo hi").unwrap();
    assert_eq!(ok[0].args, vec!["hi"]);

    let err = parser.parse("unknown arg").unwrap_err();
    assert!(matches!(
        err,
        ShellParseError::UnknownCommand { name, .. } if name == "unknown"
    ));

    let err = parser.parse("echo one two three").unwrap_err();
    assert!(matches!(
        err,
        ShellParseError::InvalidArity {
            name,
            min_expected: 1,
            max_expected: Some(2),
            found: 3,
            ..
        } if name == "echo"
    ));
}

#[test]
fn detects_unterminated_quote_and_escape() {
    let parser = ShellParser::new();
    let err = parser.parse("say \"oops").unwrap_err();
    assert!(matches!(
        err,
        ShellParseError::UnterminatedQuote { quote: '"', .. }
    ));

    let err = parser.parse("echo trailing \\").unwrap_err();
    assert!(matches!(err, ShellParseError::TrailingEscape { .. }));
}

#[test]
fn ignores_empty_commands_from_separators() {
    let parser = ShellParser::new();
    let parsed = parser.parse("  ;first;;second\n\n;third").unwrap();
    assert_eq!(parsed.len(), 3);
    assert_eq!(parsed[0].name, "first");
    assert_eq!(parsed[1].name, "second");
    assert_eq!(parsed[2].name, "third");
}

#[test]
fn parses_redirection_like_tokens_as_args() {
    let parser = ShellParser::new();
    let parsed = parser.parse(r#"echo "Hello" > ./foo.log"#).unwrap();
    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed[0].name, "echo");
    assert_eq!(parsed[0].args, vec!["Hello", ">", "./foo.log"]);
}

#[test]
fn preserves_pipe_separator() {
    let parser = ShellParser::new();
    let parsed = parser
        .parse_with_separators("echo hi | upper; next")
        .unwrap();

    assert_eq!(parsed.len(), 3);
    assert_eq!(parsed[0].separator, Some(Separator::Pipe));
    assert_eq!(parsed[1].separator, Some(Separator::Semicolon));
    assert_eq!(parsed[2].separator, None);
}

#[test]
fn resolves_aliases_to_canonical_names() {
    let parser = ShellParser::with_commands([CommandSpec::new("list", "List items")
        .with_alias("ls")
        .with_alias("ll")
        .with_max_args(0)]);

    let parsed = parser.parse("ll\nls").unwrap();
    assert_eq!(parsed.len(), 2);
    assert_eq!(parsed[0].name, "list");
    assert_eq!(parsed[1].name, "list");
}

#[test]
fn parses_runtime_alias_definitions() {
    let parser = ShellParser::new();
    let script = r#"
        alias greet="echo hello"
        greet world
    "#;

    let parsed = parser.parse(script).unwrap();
    assert_eq!(parsed.len(), 2);
    assert_eq!(parsed[0].name, "alias");
    assert_eq!(parsed[1].name, "echo");
    assert_eq!(parsed[1].args, vec!["hello", "world"]);
}

#[test]
fn runtime_aliases_can_expand_to_multiple_commands() {
    let parser = ShellParser::new();
    let script = r#"
        alias say='echo hi; echo bye'
        say everyone
    "#;

    let parsed = parser.parse(script).unwrap();
    assert_eq!(parsed.len(), 3);
    assert_eq!(parsed[1].name, "echo");
    assert_eq!(parsed[1].args, vec!["hi"]);
    assert_eq!(parsed[2].name, "echo");
    assert_eq!(parsed[2].args, vec!["bye", "everyone"]);
}

#[test]
fn runtime_aliases_persist_between_parses() {
    let parser = ShellParser::new();
    parser.parse(r#"alias ll="echo list""#).unwrap();
    let parsed = parser.parse("ll /tmp").unwrap();
    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed[0].name, "echo");
    assert_eq!(parsed[0].args, vec!["list", "/tmp"]);
}

#[test]
fn detects_alias_recursion() {
    let parser = ShellParser::new();
    let script = r#"
        alias first="second"
        alias second="first"
        first
    "#;
    let err = parser.parse(script).unwrap_err();
    assert!(matches!(
        err,
        ShellParseError::AliasLoop { name, .. } if name == "first"
    ));
}

#[test]
fn parses_and_separator() {
    let parser = ShellParser::new();
    let parsed = parser
        .parse_with_separators("first&&second && third")
        .unwrap();

    assert_eq!(parsed.len(), 3);
    assert_eq!(parsed[0].invocation.name, "first");
    assert_eq!(parsed[1].invocation.name, "second");
    assert_eq!(parsed[2].invocation.name, "third");
    assert_eq!(parsed[0].separator, Some(Separator::And));
    assert_eq!(parsed[1].separator, Some(Separator::And));
    assert_eq!(parsed[2].separator, None);
}
