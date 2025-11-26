use crate::cache_service::CacheService;
use crate::commands::{command_handlers, CommandContext};
use crate::commands_history_service::CommandHistory;
use crate::types::{OutputKind, TermLine};
use crate::vfs_data::{load_vfs, VfsNode};
use shell_parser::integration::ExecutableCommand;
use shell_parser::{ShellParseError, ShellParser};
use std::collections::HashMap;
use std::rc::Rc;
use yew::UseStateHandle;

#[derive(Clone)]
pub struct Terminal {
    lines: UseStateHandle<Vec<TermLine>>,
    cwd: UseStateHandle<Vec<String>>,
    vfs: Rc<VfsNode>,
    cache: Option<Rc<CacheService>>,
    handlers: Rc<Vec<Box<dyn ExecutableCommand<CommandContext>>>>,
}

impl Terminal {
    pub async fn new(
        lines: UseStateHandle<Vec<TermLine>>,
        cwd: UseStateHandle<Vec<String>>,
    ) -> Self {
        let cache = match CacheService::new().await {
            Ok(service) => Some(Rc::new(service)),
            Err(err) => {
                web_sys::console::error_1(&err);
                None
            }
        };

        Self {
            lines,
            cwd,
            vfs: Rc::new(load_vfs()),
            cache,
            handlers: Rc::new(command_handlers()),
        }
    }

    pub fn update_state_handles(
        &mut self,
        lines: UseStateHandle<Vec<TermLine>>,
        cwd: UseStateHandle<Vec<String>>,
    ) {
        self.lines = lines;
        self.cwd = cwd;
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
            body: body.into(),
            accent: false,
            kind: OutputKind::Text,
            node: None,
        });
    }

    pub fn push_error(&self, body: impl Into<String>) {
        self.push_line(TermLine {
            body: body.into(),
            accent: true,
            kind: OutputKind::Error,
            node: None,
        });
    }

    pub fn push_html(&self, body: impl Into<String>) {
        self.push_line(TermLine {
            body: body.into(),
            accent: false,
            kind: OutputKind::Html,
            node: None,
        });
    }

    pub fn push_component(&self, node: yew::Html) {
        self.push_line(TermLine {
            body: String::new(),
            accent: false,
            kind: OutputKind::Component,
            node: Some(node),
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

    pub async fn process_command(&self, history: UseStateHandle<CommandHistory>, trimmed: String) {
        self.push_line(TermLine {
            body: trimmed.clone(),
            accent: false,
            kind: OutputKind::Text,
            node: None,
        });

        let mut next_history = (*history).clone();
        next_history.push(trimmed.clone());
        history.set(next_history);

        self.execute_command(&trimmed).await;
    }

    pub async fn execute_command(&self, input: &str) {
        let cache = self.cache.clone();
        let ctx = CommandContext {
            vfs: self.vfs.clone(),
            cache,
            terminal: self.clone(),
        };

        let mut specs = Vec::with_capacity(self.handlers.len());
        let mut map: HashMap<&str, &Box<dyn ExecutableCommand<CommandContext>>> = HashMap::new();
        for handler in self.handlers.iter() {
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

        if let Err(err) = handler.run(&command.args, &ctx) {
            ctx.terminal
                .push_error(format!("{}: {}", command.name, err));
        }
    }
}
