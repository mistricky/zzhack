use yew::Html;

#[derive(Clone, Debug, PartialEq)]
pub enum OutputKind {
    Text,
    Html,
    Error,
    Component,
}

#[derive(Clone, Debug)]
pub struct TermLine {
    pub body: String,
    pub accent: bool,
    pub kind: OutputKind,
    pub node: Option<Html>,
}

impl PartialEq for TermLine {
    fn eq(&self, other: &Self) -> bool {
        self.body == other.body && self.accent == other.accent && self.kind == other.kind
    }
}
