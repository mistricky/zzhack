#[derive(Clone, Debug, Default, PartialEq)]
pub struct CommandHistory {
    entries: Vec<String>,
    cursor: Option<usize>,
}

impl CommandHistory {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            cursor: None,
        }
    }

    pub fn push(&mut self, entry: String) {
        if entry.is_empty() {
            return;
        }
        self.entries.push(entry);
        self.cursor = None;
    }

    pub fn previous(&mut self) -> Option<String> {
        if self.entries.is_empty() {
            return None;
        }
        let next_cursor = match self.cursor {
            Some(0) => 0,
            Some(idx) => idx.saturating_sub(1),
            None => self.entries.len().saturating_sub(1),
        };
        self.cursor = Some(next_cursor);
        self.entries.get(next_cursor).cloned()
    }

    pub fn next(&mut self) -> Option<String> {
        let Some(current) = self.cursor else {
            return None;
        };
        let next_cursor = current + 1;
        if next_cursor >= self.entries.len() {
            self.cursor = None;
            return Some(String::new());
        }
        self.cursor = Some(next_cursor);
        self.entries.get(next_cursor).cloned()
    }
}
