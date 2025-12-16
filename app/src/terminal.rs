use crate::cache_service::CacheService;
use crate::commands::{command_handlers, CommandContext};
use crate::commands_history_service::CommandHistory;
use crate::config_service::ConfigService;
use crate::terminal_state::{TerminalAction, TerminalState};
use crate::types::{OutputKind, TermLine};
use crate::vfs_data::{load_vfs, VfsNode};
use shell_parser::{with_cli, CliRunner, ShellParseError};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};
use yew::UseReducerHandle;

struct TerminalCore {
    state: UseReducerHandle<TerminalState>,
    vfs: Rc<VfsNode>,
    cache: Option<Rc<CacheService>>,
    history: Rc<RefCell<CommandHistory>>,
    cwd: Rc<RefCell<Vec<String>>>,
    runner: RefCell<Option<Weak<CliRunner<CommandContext>>>>,
}

#[derive(Clone)]
pub struct TerminalHandle {
    inner: Rc<TerminalCore>,
}

#[derive(Clone)]
pub struct Terminal {
    handle: TerminalHandle,
    _runner: Rc<CliRunner<CommandContext>>,
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
        let vfs = Rc::new(load_vfs());
        let history = Rc::new(RefCell::new(CommandHistory::new(cache.clone()).await));
        let handle = TerminalHandle::new(state, vfs, cache, history);
        let runner = Rc::new(with_cli(handle.command_context(), command_handlers()));
        handle.set_runner(&runner);

        Self {
            handle,
            _runner: runner,
        }
    }

    pub fn process_command(&self, trimmed: String) {
        self.push_line(TermLine {
            body: format!(
                r#"<div class="text-sm mt-4 text-gray-600">{}</div>"#,
                trimmed.clone()
            ),
            accent: false,
            kind: OutputKind::Html,
            node: None,
        });

        self.history().borrow_mut().push(trimmed.clone());

        self.execute_command(&trimmed);
    }
}

impl Deref for Terminal {
    type Target = TerminalHandle;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl TerminalHandle {
    fn new(
        state: UseReducerHandle<TerminalState>,
        vfs: Rc<VfsNode>,
        cache: Option<Rc<CacheService>>,
        history: Rc<RefCell<CommandHistory>>,
    ) -> Self {
        Self {
            inner: Rc::new(TerminalCore {
                state,
                vfs,
                cache,
                history,
                cwd: Rc::new(RefCell::new(Vec::new())),
                runner: RefCell::new(None),
            }),
        }
    }

    fn command_context(&self) -> CommandContext {
        CommandContext {
            vfs: self.inner.vfs.clone(),
            cache: self.inner.cache.clone(),
            terminal: self.clone(),
            config: ConfigService::get(),
        }
    }

    fn set_runner(&self, runner: &Rc<CliRunner<CommandContext>>) {
        *self.inner.runner.borrow_mut() = Some(Rc::downgrade(runner));
    }

    fn runner(&self) -> Option<Rc<CliRunner<CommandContext>>> {
        self.inner
            .runner
            .borrow()
            .as_ref()
            .and_then(|weak| weak.upgrade())
    }

    fn runner_else(
        &self,
    ) -> Result<Rc<CliRunner<CommandContext>>, shell_parser::integration::ShellCliError> {
        let Some(runner) = self.runner() else {
            return Err(shell_parser::integration::ShellCliError::Execution {
                command: "None".to_string(),
                message: "runner unavailable".into(),
            });
        };

        Ok(runner)
    }

    pub fn push_line(&self, line: TermLine) {
        self.inner.state.dispatch(TerminalAction::PushLine(line));
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
        self.inner.state.dispatch(TerminalAction::ClearLines);
    }

    pub fn cwd(&self) -> Vec<String> {
        self.inner.cwd.borrow().clone()
    }

    pub fn set_cwd(&self, cwd: Vec<String>) {
        *self.inner.cwd.borrow_mut() = cwd.clone();
        self.inner.state.dispatch(TerminalAction::SetCwd(cwd));
    }

    pub fn cache(&self) -> Option<Rc<CacheService>> {
        self.inner.cache.clone()
    }

    pub fn history(&self) -> Rc<RefCell<CommandHistory>> {
        self.inner.history.clone()
    }

    pub fn help(&self) -> Result<String, shell_parser::integration::ShellCliError> {
        Ok(self.runner_else()?.help())
    }

    pub fn execute_command(&self, input: &str) {
        if let Err(err) = self.run_script(input) {
            tracing::error!("{:?}", &err);
            self.push_error(format_cli_error(err));
        }
    }

    fn run_script(&self, input: &str) -> Result<(), shell_parser::integration::ShellCliError> {
        self.runner_else()?.run_script(input)
    }

    pub fn to_terminal(&self) -> Option<Terminal> {
        self.runner().map(|runner| Terminal {
            handle: self.clone(),
            _runner: runner,
        })
    }
}

fn format_cli_error(err: shell_parser::integration::ShellCliError) -> String {
    match err {
        shell_parser::integration::ShellCliError::Parse(parse_err) => match parse_err {
            ShellParseError::UnknownCommand { name, .. } => format!("Unknown command {name}"),
            other => format!("parse error: {other}"),
        },
        shell_parser::integration::ShellCliError::Execution { command, message } => {
            format!("{command}: {message}")
        }
    }
}
