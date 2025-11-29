use crate::commands::{parse_cli, CommandContext};
use micro_cli::Parser;
use shell_parser::integration::{CommandInfo, ExecutableCommand};
use wasm_bindgen::JsValue;
use web_sys::window;

#[derive(Parser, Debug, Default)]
#[command(name = "history", about = "Control the browser history API")]
pub struct HistoryCommand {
    #[arg(
        short = 'p',
        long = "push",
        help = "Push a new entry onto the history stack"
    )]
    push: Option<String>,
    #[arg(
        short = 'r',
        long = "replace",
        help = "Replace the current history entry with the given path"
    )]
    replace: Option<String>,
    #[arg(short = 'b', long = "back", help = "Navigate back one entry")]
    back: bool,
    #[arg(short = 'f', long = "forward", help = "Navigate forward one entry")]
    forward: bool,
}

enum HistoryAction {
    Push(String),
    Replace(String),
    Back,
    Forward,
}

impl HistoryCommand {
    fn action(&self) -> Result<HistoryAction, String> {
        let mut action: Option<HistoryAction> = None;

        let mut set_action = |next: HistoryAction| -> Result<(), String> {
            if action.is_some() {
                return Err(
                    "history: specify only one of --push, --replace, --back, or --forward"
                        .to_string(),
                );
            }
            action = Some(next);
            Ok(())
        };

        if let Some(path) = self.push.as_deref() {
            let trimmed = path.trim();
            if trimmed.is_empty() {
                return Err("history: --push requires a non-empty path".to_string());
            }
            set_action(HistoryAction::Push(trimmed.to_string()))?;
        }

        if let Some(path) = self.replace.as_deref() {
            let trimmed = path.trim();
            if trimmed.is_empty() {
                return Err("history: --replace requires a non-empty path".to_string());
            }
            set_action(HistoryAction::Replace(trimmed.to_string()))?;
        }

        if self.back {
            set_action(HistoryAction::Back)?;
        }

        if self.forward {
            set_action(HistoryAction::Forward)?;
        }

        action.ok_or_else(|| "history: specify --push, --replace, --back, or --forward".to_string())
    }
}

impl ExecutableCommand<CommandContext> for HistoryCommand {
    fn run(&self, args: &[String], ctx: &CommandContext) -> Result<(), String> {
        let Some(cli) = parse_cli::<HistoryCommand>(args, ctx, self.command_name()) else {
            return Ok(());
        };

        let action = match cli.action() {
            Ok(action) => action,
            Err(err) => {
                ctx.terminal.push_error(err);
                return Ok(());
            }
        };

        let result = match action {
            HistoryAction::Push(path) => push_state(&path).map(|_| format!("pushed {path}")),
            HistoryAction::Replace(path) => {
                replace_state(&path).map(|_| format!("replaced with {path}"))
            }
            HistoryAction::Back => navigate_back().map(|_| "navigating back".to_string()),
            HistoryAction::Forward => navigate_forward().map(|_| "navigating forward".to_string()),
        };

        match result {
            Ok(message) => ctx.terminal.push_text(format!("history: {message}")),
            Err(err) => ctx.terminal.push_error(err),
        }

        Ok(())
    }
}

fn browser_history() -> Result<web_sys::History, String> {
    window()
        .ok_or_else(|| "history: window unavailable".to_string())?
        .history()
        .map_err(|_| "history: failed to access browser history".to_string())
}

fn push_state(path: &str) -> Result<(), String> {
    let history = browser_history()?;
    history
        .push_state_with_url(&JsValue::NULL, "", Some(path))
        .map_err(|err| history_error("push state", err))
}

fn replace_state(path: &str) -> Result<(), String> {
    let history = browser_history()?;
    history
        .replace_state_with_url(&JsValue::NULL, "", Some(path))
        .map_err(|err| history_error("replace state", err))
}

fn navigate_back() -> Result<(), String> {
    let history = browser_history()?;
    history
        .back()
        .map_err(|err| history_error("navigate back", err))
}

fn navigate_forward() -> Result<(), String> {
    let history = browser_history()?;
    history
        .forward()
        .map_err(|err| history_error("navigate forward", err))
}

fn history_error(action: &str, err: JsValue) -> String {
    let detail = err
        .as_string()
        .unwrap_or_else(|| "unknown error".to_string());
    format!("history: failed to {action}: {detail}")
}
