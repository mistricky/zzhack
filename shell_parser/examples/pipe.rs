use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use shell_parser::{CommandInvocation, CommandSpec, ParsedCommand, Separator, ShellParser};

struct ShellState {
    cwd: PathBuf,
}

impl ShellState {
    fn new() -> io::Result<Self> {
        Ok(Self {
            cwd: std::env::current_dir()?,
        })
    }

    fn execute(&mut self, invocation: CommandInvocation, input: Option<String>) -> Option<String> {
        match invocation.name.as_str() {
            "echo" => self.cmd_echo(invocation, input),
            "upper" => self.cmd_upper(invocation, input),
            "append" => self.cmd_append(invocation, input),
            "save" => self.cmd_save(invocation, input),
            "cat" => self.cmd_cat(invocation, input),
            other => {
                println!("unknown command: {}", other);
                None
            }
        }
    }

    fn cmd_echo(&self, invocation: CommandInvocation, input: Option<String>) -> Option<String> {
        let text = if invocation.args.is_empty() {
            input.unwrap_or_default()
        } else {
            invocation.args.join(" ")
        };
        println!("{}", text);
        Some(text)
    }

    fn cmd_upper(&self, invocation: CommandInvocation, input: Option<String>) -> Option<String> {
        let text = if invocation.args.is_empty() {
            input.unwrap_or_default()
        } else {
            invocation.args.join(" ")
        };
        let upper = text.to_uppercase();
        println!("{}", upper);
        Some(upper)
    }

    fn cmd_append(&self, invocation: CommandInvocation, input: Option<String>) -> Option<String> {
        let base = input.unwrap_or_default();
        let suffix = invocation.args.join(" ");
        let result = if base.is_empty() {
            suffix
        } else if suffix.is_empty() {
            base
        } else {
            format!("{base}{suffix}")
        };
        println!("{}", result);
        Some(result)
    }

    fn cmd_save(&self, invocation: CommandInvocation, input: Option<String>) -> Option<String> {
        let Some(path) = invocation.args.get(0) else {
            println!("save: missing target path");
            return input;
        };
        let content = input.unwrap_or_default();
        let full_path = self.resolve(path);
        if let Some(parent) = full_path.parent() {
            if let Err(err) = fs::create_dir_all(parent) {
                println!("save: cannot create {}: {err}", parent.display());
                return None;
            }
        }
        match OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&full_path)
        {
            Ok(mut file) => {
                if let Err(err) = write!(file, "{content}") {
                    println!("save: cannot write {}: {err}", full_path.display());
                } else {
                    println!("save -> wrote {}", full_path.display());
                }
            }
            Err(err) => println!("save: cannot open {}: {err}", full_path.display()),
        }
        Some(content)
    }

    fn cmd_cat(&self, invocation: CommandInvocation, _input: Option<String>) -> Option<String> {
        let mut combined = String::new();
        for path in invocation.args {
            let full = self.resolve(&path);
            match fs::read_to_string(&full) {
                Ok(content) => {
                    print!("{content}");
                    combined.push_str(&content);
                }
                Err(err) => println!("cat: {}: {}", full.display(), err),
            }
        }
        Some(combined)
    }

    fn resolve(&self, path: &str) -> PathBuf {
        let p = Path::new(path);
        if p.is_absolute() {
            p.to_path_buf()
        } else {
            self.cwd.join(p)
        }
    }
}

fn execute_with_pipes(commands: Vec<ParsedCommand>, state: &mut ShellState) {
    let mut pipeline: Vec<CommandInvocation> = Vec::new();
    for parsed in commands {
        pipeline.push(parsed.invocation);
        let end_of_pipeline = parsed.separator != Some(Separator::Pipe);
        if end_of_pipeline {
            run_pipeline(&pipeline, state);
            pipeline.clear();
        }
    }
}

fn run_pipeline(pipeline: &[CommandInvocation], state: &mut ShellState) {
    let mut input: Option<String> = None;
    for invocation in pipeline {
        input = state.execute(invocation.clone(), input);
    }
}

fn main() -> io::Result<()> {
    let parser = ShellParser::with_commands([
        CommandSpec::new("echo").with_min_args(0),
        CommandSpec::new("upper").with_min_args(0),
        CommandSpec::new("append").with_min_args(0),
        CommandSpec::new("save").with_min_args(1).with_max_args(1),
        CommandSpec::new("cat").with_min_args(1),
    ]);

    let script = r#"
        # This pipeline uppercases, appends punctuation, saves, then prints the file
        echo "hello pipeline" | upper | append "!!!" | save ./example_out/piped.txt
        cat ./example_out/piped.txt
    "#;

    let commands = parser
        .parse_with_separators(script)
        .expect("script should parse");
    let mut state = ShellState::new()?;
    execute_with_pipes(commands, &mut state);
    Ok(())
}
