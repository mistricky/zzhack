use crate::cache_service::CacheService;
use crate::commands::{CommandContext, CommandHandler};
use crate::types::{OutputKind, TermLine};
use crate::vfs_data::format_path;
use crate::vfs_data::VfsNode;
use shell_parser::{ShellParseError, ShellParser};
use std::collections::HashMap;
use std::rc::Rc;
use yew::UseStateHandle;

#[derive(Clone)]
pub struct Terminal {
    lines: UseStateHandle<Vec<TermLine>>,
    cwd: UseStateHandle<Vec<String>>,
}

impl Terminal {
    pub fn new(lines: UseStateHandle<Vec<TermLine>>, cwd: UseStateHandle<Vec<String>>) -> Self {
        Self { lines, cwd }
    }

    pub fn snapshot(&self) -> Vec<TermLine> {
        (*self.lines).clone()
    }

    pub fn push_line(&self, line: TermLine) {
        let mut next = self.snapshot();
        next.push(line);
        self.lines.set(next);
    }

    pub fn push_text(&self, body: impl Into<String>) {
        self.push_line(TermLine {
            prompt: String::new(),
            body: body.into(),
            accent: false,
            kind: OutputKind::Text,
        });
    }

    pub fn push_error(&self, body: impl Into<String>) {
        self.push_line(TermLine {
            prompt: String::new(),
            body: body.into(),
            accent: true,
            kind: OutputKind::Error,
        });
    }

    pub fn push_html(&self, body: impl Into<String>) {
        self.push_line(TermLine {
            prompt: String::new(),
            body: body.into(),
            accent: false,
            kind: OutputKind::Html,
        });
    }

    #[allow(dead_code)]
    pub fn extend(&self, lines: impl IntoIterator<Item = TermLine>) {
        let mut next = self.snapshot();
        next.extend(lines);
        self.lines.set(next);
    }

    pub fn clear(&self) {
        self.lines.set(Vec::new());
    }

    pub fn cwd(&self) -> Vec<String> {
        (*self.cwd).clone()
    }

    pub fn set_cwd(&self, cwd: Vec<String>) {
        self.cwd.set(cwd);
    }

    pub fn prompt(&self) -> String {
        let path = format_path(&self.cwd());
        if path == "/" {
            "guest@zzhack >".into()
        } else {
            format!("guest@zzhack {} >", path)
        }
    }

    pub async fn execute_command(
        &self,
        input: &str,
        vfs: Rc<VfsNode>,
        cache: Option<Rc<CacheService>>,
        handlers: &[Box<dyn CommandHandler>],
    ) {
        let ctx = CommandContext {
            vfs,
            cache,
            terminal: self.clone(),
        };

        let mut specs = Vec::with_capacity(handlers.len());
        let mut map: HashMap<&str, &Box<dyn CommandHandler>> = HashMap::new();
        for handler in handlers {
            specs.push(handler.spec());
            map.insert(handler.name(), handler);
        }

        let parser = ShellParser::with_commands(specs);

        let parsed = match parser.parse(input) {
            Ok(commands) => commands.into_iter().next(),
            Err(err) => {
                let message = match err {
                    ShellParseError::UnknownCommand { name, .. } => {
                        format!("Unknown command {name}")
                    }
                    other => format!("parse error: {other}"),
                };
                ctx.terminal.push_error(message);
                return;
            }
        };

        let Some(command) = parsed else {
            ctx.terminal.push_error("empty command");
            return;
        };

        let handler = match map.get(command.name.as_str()) {
            Some(h) => h,
            None => {
                ctx.terminal.push_error(format!(
                    "unknown command: {} (TODO: implement)",
                    command.name
                ));
                return;
            }
        };

        handler.run(&command.args, &ctx).await;
    }
}
