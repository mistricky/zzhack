use crate::commands::{parse_cli, CommandContext};
use micro_cli::Parser;
use shell_parser::integration::{CommandInfo, ExecutableCommand};
use std::str::FromStr;
use web_sys::window;

const DARK_CLASS: &str = "theme-dark";

#[derive(Parser, Debug, Default)]
#[command(name = "theme", about = "Get or set the UI theme")]
pub struct ThemeCommand {
    #[arg(
        short = 's',
        long = "set",
        help = "Set the theme to dark or light (dark|light)"
    )]
    set: Option<ThemeMode>,
}

impl ThemeCommand {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ThemeMode {
    Dark,
    Light,
}

impl ThemeMode {
    fn as_str(&self) -> &'static str {
        match self {
            ThemeMode::Dark => "dark",
            ThemeMode::Light => "light",
        }
    }
}

impl FromStr for ThemeMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "dark" => Ok(ThemeMode::Dark),
            "light" => Ok(ThemeMode::Light),
            _ => Err(()),
        }
    }
}

impl ExecutableCommand<CommandContext> for ThemeCommand {
    fn run(&self, args: &[String], ctx: &CommandContext) -> Result<(), String> {
        let Some(cli) = parse_cli::<ThemeCommand>(args, ctx, self.command_name()) else {
            return Ok(());
        };

        if let Some(mode) = cli.set {
            set_theme(mode)?;
            ctx.terminal
                .push_text(format!("theme set to {}", mode.as_str()));
            return Ok(());
        }

        let mode = current_theme().unwrap_or(ThemeMode::Light);
        ctx.terminal
            .push_text(format!("current theme: {}", mode.as_str()));

        Ok(())
    }
}

fn document_root() -> Option<web_sys::Element> {
    window()
        .and_then(|w| w.document())
        .and_then(|d| d.document_element())
}

fn current_theme() -> Option<ThemeMode> {
    let root = document_root()?;
    match root.class_list().contains(DARK_CLASS) {
        true => Some(ThemeMode::Dark),
        false => Some(ThemeMode::Light),
    }
}

fn set_theme(mode: ThemeMode) -> Result<(), String> {
    let root = document_root().ok_or_else(|| "theme: no document root".to_string())?;
    match mode {
        ThemeMode::Dark => root
            .class_list()
            .add_1(DARK_CLASS)
            .map_err(|_| "theme: failed to set dark mode".to_string()),
        ThemeMode::Light => root
            .class_list()
            .remove_1(DARK_CLASS)
            .map_err(|_| "theme: failed to set light mode".to_string()),
    }
}
