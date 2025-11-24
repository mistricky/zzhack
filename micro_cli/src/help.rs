use crate::command::{Command, OptionKind};

pub struct HelpBuilder<'a> {
    command: &'a Command,
    path: Vec<String>,
}

impl<'a> HelpBuilder<'a> {
    pub fn new(command: &'a Command, path: &[String]) -> Self {
        Self {
            command,
            path: path.to_vec(),
        }
    }

    pub fn render(&mut self) -> String {
        let mut output = String::new();
        let path = self.path.join(" ");
        output.push_str(&format!("Usage: {} [OPTIONS]", path));
        if !self.command.subcommands.is_empty() {
            output.push_str(" <SUBCOMMAND>");
        }
        if !self.command.subcommands.is_empty() || !self.command.options.is_empty() {
            output.push('\n');
        }
        output.push_str(&format!("\n{}\n", self.command.about));

        if !self.command.options.is_empty() {
            output.push_str("Options:\n");
            for opt in &self.command.options {
                let mut flags = Vec::new();
                if let Some(short) = opt.short {
                    flags.push(format!("-{}", short));
                }
                if let Some(long) = opt.long {
                    flags.push(format!("--{}", long));
                }
                if opt.kind == OptionKind::Value {
                    flags = flags.into_iter().map(|f| format!("{f} <value>")).collect();
                }
                output.push_str(&format!("  {:<18} {}\n", flags.join(", "), opt.help));
            }
            output.push('\n');
        }

        if !self.command.subcommands.is_empty() {
            output.push_str("Subcommands:\n");
            for sub in &self.command.subcommands {
                output.push_str(&format!("  {:<18} {}\n", sub.name, sub.about));
            }
        }

        output
    }
}
