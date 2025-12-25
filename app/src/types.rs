use uuid::Uuid;
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
    pub id: Uuid,
    pub body: String,
    pub accent: bool,
    pub kind: OutputKind,
    pub node: Option<Html>,
}

impl PartialEq for TermLine {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.body == other.body
            && self.accent == other.accent
            && self.kind == other.kind
    }
}

impl TermLine {
    pub fn new(
        body: impl Into<String>,
        accent: bool,
        kind: OutputKind,
        node: Option<Html>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            body: body.into(),
            accent,
            kind,
            node,
        }
    }
}
