/// Separators that can appear between commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Separator {
    Newline,
    Semicolon,
    And,
    Pipe,
}
