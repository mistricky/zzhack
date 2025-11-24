#[derive(Clone, PartialEq)]
pub enum OutputKind {
    Text,
    Html,
    Error,
}

#[derive(Clone, PartialEq)]
pub struct TermLine {
    pub prompt: String,
    pub body: String,
    pub accent: bool,
    pub kind: OutputKind,
}
