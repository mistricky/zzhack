use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use shell_parser::{CommandInvocation, CommandSpec, ShellParser};

/// Minimal executor to show how callers can hook command implementations.
struct ShellState {
    cwd: PathBuf,
}

impl ShellState {
    fn new() -> io::Result<Self> {
        let cwd = std::env::current_dir()?;
        Ok(Self { cwd })
    }

    fn run(&mut self, invocation: CommandInvocation) {
        match invocation.name.as_str() {
            "cd" => self.cmd_cd(invocation),
            "echo" => self.cmd_echo(invocation),
            "cat" => self.cmd_cat(invocation),
            other => println!("unknown command: {}", other),
        }
    }

    fn cmd_cd(&mut self, invocation: CommandInvocation) {
        let Some(target) = invocation.args.get(0) else {
            println!("cd: missing target directory");
            return;
        };

        let new_path = self.resolve(target);
        if let Err(err) = fs::create_dir_all(&new_path) {
            println!("cd: cannot create {}: {err}", new_path.display());
            return;
        }
        self.cwd = new_path;
        println!("cd -> {}", self.cwd.display());
    }

    fn cmd_echo(&mut self, invocation: CommandInvocation) {
        let mut parts = invocation.args.into_iter();
        let mut message: Vec<String> = Vec::new();
        let mut redirect: Option<String> = None;

        while let Some(part) = parts.next() {
            if part == ">" {
                redirect = parts.next();
                break;
            } else {
                message.push(part);
            }
        }

        let output = message.join(" ");
        match redirect {
            Some(path) => {
                let full_path = self.resolve(&path);
                if let Some(parent) = full_path.parent() {
                    if let Err(err) = fs::create_dir_all(parent) {
                        println!("echo: cannot create {}: {err}", full_path.display());
                        return;
                    }
                }
                let mut file = match OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(&full_path)
                {
                    Ok(file) => file,
                    Err(err) => {
                        println!("echo: cannot open {}: {err}", full_path.display());
                        return;
                    }
                };
                if let Err(err) = writeln!(file, "{}", output) {
                    println!("echo: cannot write {}: {err}", full_path.display());
                    return;
                }
                println!(
                    "echo (redirect) -> {} wrote {}",
                    output,
                    full_path.display()
                );
            }
            None => println!("{}", output),
        }
    }

    fn cmd_cat(&mut self, invocation: CommandInvocation) {
        for path in invocation.args {
            let full_path = self.resolve(&path);
            match fs::read_to_string(&full_path) {
                Ok(content) => print!("{}", content),
                Err(err) => println!("cat: {}: {}", full_path.display(), err),
            }
        }
    }

    fn resolve(&self, target: &str) -> PathBuf {
        let path = Path::new(target);
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.cwd.join(path)
        }
    }
}

fn main() {
    let parser = ShellParser::with_commands([
        CommandSpec::new("echo", "Print text to stdout").with_min_args(1),
        CommandSpec::new("cd", "Change the working directory")
            .with_min_args(1)
            .with_max_args(1),
        CommandSpec::new("cat", "Print the contents of a file").with_min_args(1),
    ]);

    let script = r#"
        # This script illustrates simple shell-like parsing and execution
        cd ./example_out
        echo "Hello world" > ./foo.log
        cat ./foo.log; echo done
    "#;

    let invocations = parser.parse(script).expect("script should parse");
    let mut state = ShellState::new().expect("cwd available");
    for invocation in invocations {
        state.run(invocation);
    }
}
