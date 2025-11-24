use crate::types::{OutputKind, TermLine};
use yew::UseStateHandle;

#[derive(Clone)]
pub struct Terminal {
    lines: UseStateHandle<Vec<TermLine>>,
    cwd: UseStateHandle<Vec<String>>,
}

impl Terminal {
    pub fn new(lines: UseStateHandle<Vec<TermLine>>, cwd: UseStateHandle<Vec<String>>) -> Self {
        Self { lines, cwd }
    }

    pub fn snapshot(&self) -> Vec<TermLine> {
        (*self.lines).clone()
    }

    pub fn push_line(&self, line: TermLine) {
        let mut next = self.snapshot();
        next.push(line);
        self.lines.set(next);
    }

    pub fn push_text(&self, body: impl Into<String>) {
        self.push_line(TermLine {
            prompt: ">".into(),
            body: body.into(),
            accent: false,
            kind: OutputKind::Text,
        });
    }

    pub fn push_error(&self, body: impl Into<String>) {
        self.push_line(TermLine {
            prompt: "!".into(),
            body: body.into(),
            accent: true,
            kind: OutputKind::Error,
        });
    }

    pub fn push_html(&self, body: impl Into<String>) {
        self.push_line(TermLine {
            prompt: ">".into(),
            body: body.into(),
            accent: false,
            kind: OutputKind::Html,
        });
    }

    pub fn push_error_html(&self, body: impl Into<String>) {
        self.push_line(TermLine {
            prompt: "!".into(),
            body: body.into(),
            accent: true,
            kind: OutputKind::Error,
        });
    }

    #[allow(dead_code)]
    pub fn extend(&self, lines: impl IntoIterator<Item = TermLine>) {
        let mut next = self.snapshot();
        next.extend(lines);
        self.lines.set(next);
    }

    pub fn clear(&self) {
        self.lines.set(Vec::new());
    }

    pub fn cwd(&self) -> Vec<String> {
        (*self.cwd).clone()
    }

    pub fn set_cwd(&self, cwd: Vec<String>) {
        self.cwd.set(cwd);
    }
}
