use crate::command::{Command, OptionKind, ParsedOptions};
use crate::error::CliError;

#[derive(Debug)]
pub(crate) struct CommandMatch<'a> {
    pub command: &'a Command,
    pub options: ParsedOptions,
    pub args: Vec<String>,
    pub path: Vec<String>,
}

pub(crate) fn parse_root<'a>(
    app: &'a crate::command::CliApp,
    args: &[String],
) -> Result<CommandMatch<'a>, CliError> {
    let name = &args[0];
    let Some(cmd) = app.commands.iter().find(|c| c.name == name) else {
        return Err(CliError::UnknownCommand(name.clone()));
    };
    parse_command(cmd, &args[1..], &[app.name.to_string(), name.clone()])
}

pub(crate) fn parse_command<'a>(
    command: &'a Command,
    args: &[String],
    path: &[String],
) -> Result<CommandMatch<'a>, CliError> {
    let mut options = ParsedOptions::default();
    let mut positionals = Vec::new();
    let mut idx = 0;

    while idx < args.len() {
        let arg = &args[idx];
        if arg == "--help" || arg == "-h" {
            return Err(CliError::Help(command.build_help(path)));
        }

        if let Some(sub) = command.find_subcommand(arg) {
            let mut new_path = path.to_vec();
            new_path.push(arg.clone());
            return parse_command(sub, &args[idx + 1..], &new_path);
        }

        if arg.starts_with("--") {
            handle_long_option(command, arg, &args, &mut idx, &mut options)?;
        } else if arg.starts_with('-') && arg.len() > 1 {
            handle_short_option(command, arg, &mut options, &args, &mut idx)?;
        } else {
            positionals.push(arg.clone());
        }
        idx += 1;
    }

    Ok(CommandMatch {
        command,
        options,
        args: positionals,
        path: path.to_vec(),
    })
}

fn handle_long_option(
    command: &Command,
    token: &str,
    args: &[String],
    idx: &mut usize,
    options: &mut ParsedOptions,
) -> Result<(), CliError> {
    let mut parts = token.splitn(2, '=');
    let name_part = parts.next().unwrap_or_default().trim_start_matches("--");
    let value_part = parts.next();

    let Some(spec) = command.options.iter().find(|o| o.long == Some(name_part)) else {
        return Err(CliError::UnknownOption(token.to_string()));
    };

    match spec.kind {
        OptionKind::Flag => options.insert_flag(spec.name.to_string()),
        OptionKind::Value => {
            if let Some(value) = value_part {
                options.insert_value(spec.name.to_string(), value.to_string());
            } else {
                let next = args
                    .get(*idx + 1)
                    .ok_or_else(|| CliError::MissingOptionValue(format!("--{}", name_part)))?;
                *idx += 1;
                options.insert_value(spec.name.to_string(), next.clone());
            }
        }
    }
    Ok(())
}

fn handle_short_option(
    command: &Command,
    token: &str,
    options: &mut ParsedOptions,
    args: &[String],
    idx: &mut usize,
) -> Result<(), CliError> {
    let mut chars = token.chars();
    chars.next(); // skip leading '-'
    let short = chars.next().unwrap();

    let Some(spec) = command.options.iter().find(|o| o.short == Some(short)) else {
        return Err(CliError::UnknownOption(token.to_string()));
    };

    match spec.kind {
        OptionKind::Flag => options.insert_flag(spec.name.to_string()),
        OptionKind::Value => {
            if chars.as_str().is_empty() {
                let next = args
                    .get(*idx + 1)
                    .ok_or_else(|| CliError::MissingOptionValue(format!("-{}", short)))?;
                *idx += 1;
                options.insert_value(spec.name.to_string(), next.clone());
            } else {
                options.insert_value(spec.name.to_string(), chars.as_str().to_string());
            }
        }
    }
    Ok(())
}
