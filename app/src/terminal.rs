use crate::cache_service::CacheService;
use crate::commands::{command_handlers, CommandContext};
use crate::commands_history_service::CommandHistory;
use crate::config_service::ConfigService;
use crate::terminal_state::{TerminalAction, TerminalState};
use crate::types::{OutputKind, TermLine};
use crate::vfs_data::{load_vfs, VfsNode};
use shell_parser::integration::ExecutableCommand;
use shell_parser::{ShellParseError, ShellParser};
use std::collections::HashMap;
use std::rc::Rc;
use yew::{UseReducerHandle, UseStateHandle};

#[derive(Clone)]
pub struct Terminal {
    state: UseReducerHandle<TerminalState>,
    vfs: Rc<VfsNode>,
    cache: Option<Rc<CacheService>>,
    handlers: Rc<Vec<Box<dyn ExecutableCommand<CommandContext>>>>,
}

impl Terminal {
    pub async fn new(state: UseReducerHandle<TerminalState>) -> Self {
        let cache = match CacheService::new().await {
            Ok(service) => Some(Rc::new(service)),
            Err(err) => {
                web_sys::console::error_1(&err);
                None
            }
        };

        Self {
            state,
            vfs: Rc::new(load_vfs()),
            cache,
            handlers: Rc::new(command_handlers()),
        }
    }

    pub fn snapshot(&self) -> Vec<TermLine> {
        (*self.state).lines.clone()
    }

    pub fn push_line(&self, line: TermLine) {
        self.state.dispatch(TerminalAction::PushLine(line));
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

    pub fn clear(&self) {
        self.state.dispatch(TerminalAction::ClearLines);
    }

    pub fn cwd(&self) -> Vec<String> {
        (*self.state).cwd.clone()
    }

    pub fn set_cwd(&self, cwd: Vec<String>) {
        self.state.dispatch(TerminalAction::SetCwd(cwd));
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
        let config = ConfigService::get();
        let ctx = CommandContext {
            vfs: self.vfs.clone(),
            cache,
            terminal: self.clone(),
            config,
        };

        let mut specs = Vec::with_capacity(self.handlers.len());
        let mut map: HashMap<String, &Box<dyn ExecutableCommand<CommandContext>>> = HashMap::new();
        for handler in self.handlers.iter() {
            let spec = handler.spec();
            map.insert(spec.name.clone(), handler);
            specs.push(spec);
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
