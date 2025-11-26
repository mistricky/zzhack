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
        output.push_str("Usage: ");
        output.push_str(&path);
        if !self.command.options.is_empty() {
            output.push_str(" [OPTIONS]");
        }
        output.push('\n');
        output.push_str(&format!("{}\n", self.command.about));

        if !self.command.options.is_empty() {
            output.push('\n');
            output.push_str("Global options:\n");
            for opt in &self.command.options {
                let mut parts = Vec::new();
                if let Some(short) = opt.short {
                    parts.push(format!("-{}", short));
                }
                if let Some(long) = opt.long {
                    parts.push(format!("--{}", long));
                }
                let flag_text = if opt.kind == OptionKind::Value {
                    format!("{} <value>", parts.join(", "))
                } else {
                    parts.join(", ")
                };
                output.push_str(&format!("  {:<18} {}\n", flag_text, opt.help));
            }
            output.push_str("  -h, --help         Show help\n");
        }

        if !self.command.subcommands.is_empty() {
            output.push('\n');
            output.push_str("Subcommands:\n");
            for sub in &self.command.subcommands {
                output.push_str(&format!("  {:<18} {}\n", sub.name, sub.about));
            }
        }

        output
    }
}
