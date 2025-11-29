use crate::cache_service::CacheService;
use crate::commands::{command_handlers, CommandContext};
use crate::commands_history_service::CommandHistory;
use crate::config_service::ConfigService;
use crate::terminal_state::{TerminalAction, TerminalState};
use crate::types::{OutputKind, TermLine};
use crate::vfs_data::{load_vfs, VfsNode};
use shell_parser::integration::ExecutableCommand;
use shell_parser::{with_cli, ShellParseError};
use std::cell::RefCell;
use std::rc::Rc;
use yew::UseReducerHandle;

#[derive(Clone)]
pub struct Terminal {
    state: UseReducerHandle<TerminalState>,
    vfs: Rc<VfsNode>,
    cache: Option<Rc<CacheService>>,
    history: Rc<RefCell<CommandHistory>>,
    cwd: Rc<RefCell<Vec<String>>>,
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

        let history = Rc::new(RefCell::new(CommandHistory::new(cache.clone()).await));

        Self {
            state,
            vfs: Rc::new(load_vfs()),
            cache,
            history,
            cwd: Rc::new(RefCell::new(Vec::new())),
        }
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
        self.cwd.borrow().clone()
    }

    pub fn set_cwd(&self, cwd: Vec<String>) {
        *self.cwd.borrow_mut() = cwd.clone();
        self.state.dispatch(TerminalAction::SetCwd(cwd));
    }

    pub fn cache(&self) -> Option<Rc<CacheService>> {
        self.cache.clone()
    }

    pub fn history(&self) -> Rc<RefCell<CommandHistory>> {
        self.history.clone()
    }

    pub async fn process_command(&self, trimmed: String) {
        self.push_line(TermLine {
            body: format!(
                r#"<div class="text-sm mt-4 text-gray-600">{}</div>"#,
                trimmed.clone()
            ),
            accent: false,
            kind: OutputKind::Html,
            node: None,
        });

        self.history.borrow_mut().push(trimmed.clone());

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

        let runner = with_cli(ctx.clone(), command_handlers());

        if let Err(err) = runner.run_script(input) {
            let message = match err {
                shell_parser::integration::ShellCliError::Parse(parse_err) => match parse_err {
                    ShellParseError::UnknownCommand { name, .. } => {
                        format!("Unknown command {name}")
                    }
                    other => format!("parse error: {other}"),
                },
                shell_parser::integration::ShellCliError::Execution { command, message } => {
                    format!("{command}: {message}")
                }
            };
            self.push_error(message);
        }
    }
}
