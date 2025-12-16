# shell_parser

Pure shell-like parser that works in any environment without system API dependencies. It tokenizes simple shell syntax, validates commands against user-provided specs, and lets callers wire their own command implementations or pipeline handling.

## Features
- Tokenizes commands with spaces, quotes (`'`/`"`), escapes (`\`), comments (`#`), separators (`;`, newline, `|`, `&&`).
- Optional command validation via `CommandSpec` (min/max args, unknown-command errors).
- Access to parsed separators through `parse_with_separators` to build pipelines.
- Command aliases declared through specs *and* runtime `alias name="value"` statements that behave like real shells.
- Runtime shell functions declared with `function name() { ... }` (or `name() { ... }`) that expand at call sites with positional parameters (`$1`, `$@`, `$*`, `$#`).
- Zero system calls in the library; you provide execution logic.

## Installation
Add to your `Cargo.toml`:
```toml
[dependencies]
shell_parser = { path = "shell_parser" } # adjust path or version as needed
```

## Quick start
```rust
use shell_parser::{CommandSpec, ShellParser};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Register commands for validation (optional)
    let parser = ShellParser::with_commands([
        CommandSpec::new("echo", "Print text")
            .with_alias("say")
            .with_min_args(1),
        CommandSpec::new("upper", "Uppercase the previous output").with_min_args(0),
    ]);

    let script = r#"echo "hello world" | upper"#;
    let commands = parser.parse_with_separators(script)?;

    // Wire your own executor
    let mut last_output = None;
    for cmd in commands {
        let out = match cmd.invocation.name.as_str() {
            "echo" => Some(cmd.invocation.args.join(" ")),
            "upper" => last_output.take().map(|s| s.to_uppercase()),
            _ => None,
        };
        last_output = out;
        // Use cmd.separator to decide when to flush a pipeline
    }

    Ok(())
}
```

## Examples
Run the bundled examples to see end-to-end usage:
- Runtime alias definitions and expansions:
  ```bash
  cargo run -p shell_parser --example alias
  ```
  Demonstrates how `alias ll="list -l"` and `alias greet='echo hi; echo bye'` expand into multiple commands.

- Runtime functions with positional parameters:
  ```bash
  cargo run -p shell_parser --example function
  ```
  Shows how `function greet() { ... }` definitions expand and how `$1`/`$@`/`$#` values are substituted.

- Basic commands with real file I/O for `echo`, `cd`, and `cat`:
  ```bash
  cargo run -p shell_parser --example basic
  ```
  Writes `example_out/foo.log` in the workspace.

- Pipelines with separators:
  ```bash
  cargo run -p shell_parser --example pipe
  ```
  Demonstrates `echo | upper | append | save` and reads back the result.

## API highlights
- `ShellParser::parse(&str) -> Vec<CommandInvocation>`: basic parsing into commands/args.
- `ShellParser::parse_with_separators(&str) -> Vec<ParsedCommand>`: includes trailing separators (`Separator::Pipe`, `Separator::Semicolon`, `Separator::And`, `Separator::Newline`).
- `CommandSpec`: configure min/max args for validation.
- `CommandSpec::with_alias`/`with_aliases`: register alternate names that resolve to the canonical command.
- Runtime aliases via the `alias` builtin: `ShellParser` learns definitions while parsing and expands future invocations.
- `ShellParseError`: detailed errors for unknown commands, arity issues, and malformed input.

## Command aliases
Register aliases directly on a `CommandSpec`. The parser accepts those names and returns the canonical
command in [`CommandInvocation::name`], so executors only handle one identifier:

```rust
use shell_parser::{CommandSpec, ShellParser};

let parser = ShellParser::with_commands([
    CommandSpec::new("list", "Show files").with_aliases(["ls", "ll"]),
]);

let cmds = parser.parse("ls\nll\nlist").unwrap();
assert!(cmds.iter().all(|cmd| cmd.name == "list"));
```

## Runtime alias definitions
Shell-style aliases declared with the `alias` command are parsed and expanded in order, including multi-command bodies:

```rust
use shell_parser::ShellParser;

let parser = ShellParser::new();
let script = r#"
    alias greet='echo hi; echo bye'
    alias warn="echo \"warning:\""
    greet there
    warn "be careful"
"#;
let parsed = parser.parse(script).unwrap();
assert_eq!(parsed[1].name, "echo"); // greet expanded
assert_eq!(parsed[2].args, vec!["bye", "there"]);
assert_eq!(parsed[3].args, vec!["warning:", "be careful"]);
```

Alias definitions persist across successive `parse` calls while the parser lives, and invalid recursive aliases surface a `ShellParseError::AliasLoop`.

## Runtime functions
Shell functions declared with either `function name() { ... }` or the bare `name() { ... }` syntax are parsed once and expanded only when invoked later. Function bodies can span multiple commands and may use positional parameters:

```rust
use shell_parser::ShellParser;

let parser = ShellParser::new();
let script = r#"
    function greet() {
        echo hello $1
        echo "args:" $#
    }

    twice() {
        greet $1
        greet $2
    }

    greet "terminal"
    twice "micro" "cli"
"#;
let parsed = parser.parse(script).unwrap();
assert_eq!(parsed[0].args, vec!["hello", "terminal"]);
assert_eq!(parsed[1].args, vec!["args:", "1"]);
assert_eq!(parsed[2].args, vec!["hello", "micro"]);
assert_eq!(parsed[4].args, vec!["hello", "cli"]);
```

Function definitions expect the opening `{` at the end of the header line (or the following line by itself) and the closing `}` on its own line. Definition lines are consumed during parsing so executors never see a `function` command. Positional placeholders `$1`, `$2`, …, `$@`, `$*`, and `$#` are substituted at expansion time, while other `$var` references are left untouched for executors to handle.

## Notes
- The library never executes commands; it only parses. You control execution and side effects.
- Output paths in examples stay under `example_out/` to keep the workspace tidy.
