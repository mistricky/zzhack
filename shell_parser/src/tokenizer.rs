use crate::error::ShellParseError;
use crate::separator::Separator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Token {
    pub value: String,
    pub position: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CommandTokens {
    pub tokens: Vec<Token>,
    pub separator: Option<Separator>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Mode {
    Normal,
    SingleQuote,
    DoubleQuote,
}

pub(crate) fn tokenize(input: &str) -> Result<Vec<CommandTokens>, ShellParseError> {
    let mut commands: Vec<CommandTokens> = Vec::new();
    let mut current_command: Vec<Token> = Vec::new();
    let mut current_token = String::new();
    let mut token_start: Option<usize> = None;
    let mut mode = Mode::Normal;
    let mut quote_start = 0;

    let mut iter = input.char_indices().peekable();
    while let Some((idx, ch)) = iter.next() {
        match mode {
            Mode::Normal => match ch {
                '\\' => {
                    let Some((_, escaped)) = iter.next() else {
                        return Err(ShellParseError::TrailingEscape { position: idx });
                    };
                    if token_start.is_none() {
                        token_start = Some(idx);
                    }
                    current_token.push(escaped);
                }
                '\'' => {
                    mode = Mode::SingleQuote;
                    quote_start = idx;
                    if token_start.is_none() {
                        token_start = Some(idx);
                    }
                }
                '"' => {
                    mode = Mode::DoubleQuote;
                    quote_start = idx;
                    if token_start.is_none() {
                        token_start = Some(idx);
                    }
                }
                '#' => {
                    while let Some((_, next)) = iter.peek() {
                        if *next == '\n' {
                            break;
                        }
                        iter.next();
                    }
                }
                ';' | '\n' | '|' => {
                    push_token(&mut current_command, &mut current_token, &mut token_start);
                    let separator = match ch {
                        ';' => Some(Separator::Semicolon),
                        '\n' => Some(Separator::Newline),
                        '|' => Some(Separator::Pipe),
                        _ => None,
                    };
                    push_command(&mut commands, &mut current_command, separator);
                }
                c if c.is_whitespace() => {
                    push_token(&mut current_command, &mut current_token, &mut token_start);
                }
                _ => {
                    if token_start.is_none() {
                        token_start = Some(idx);
                    }
                    current_token.push(ch);
                }
            },
            Mode::SingleQuote => match ch {
                '\'' => {
                    mode = Mode::Normal;
                }
                _ => {
                    current_token.push(ch);
                }
            },
            Mode::DoubleQuote => match ch {
                '"' => {
                    mode = Mode::Normal;
                }
                '\\' => {
                    let Some((_, escaped)) = iter.next() else {
                        return Err(ShellParseError::TrailingEscape { position: idx });
                    };
                    current_token.push(escaped);
                }
                _ => {
                    current_token.push(ch);
                }
            },
        }
    }

    match mode {
        Mode::Normal => {
            push_token(&mut current_command, &mut current_token, &mut token_start);
            push_command(&mut commands, &mut current_command, None);
        }
        Mode::SingleQuote | Mode::DoubleQuote => {
            return Err(ShellParseError::UnterminatedQuote {
                quote: if mode == Mode::SingleQuote { '\'' } else { '"' },
                position: quote_start,
            });
        }
    }

    Ok(commands)
}

fn push_token(
    current_command: &mut Vec<Token>,
    current_token: &mut String,
    token_start: &mut Option<usize>,
) {
    if !current_token.is_empty() {
        let position = token_start.take().unwrap_or(0);
        current_command.push(Token {
            value: std::mem::take(current_token),
            position,
        });
    } else {
        *token_start = None;
    }
}

fn push_command(
    commands: &mut Vec<CommandTokens>,
    current_command: &mut Vec<Token>,
    separator: Option<Separator>,
) {
    if !current_command.is_empty() {
        commands.push(CommandTokens {
            tokens: std::mem::take(current_command),
            separator,
        });
    }
}
