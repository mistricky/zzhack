use crate::types::TermLine;
use std::rc::Rc;
use yew::Reducible;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TerminalState {
    pub lines: Vec<TermLine>,
    pub cwd: Vec<String>,
}

#[derive(Clone, Debug)]
pub enum TerminalAction {
    PushLine(TermLine),
    ClearLines(bool),
    SetCwd(Vec<String>),
}

impl Reducible for TerminalState {
    type Action = TerminalAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            TerminalAction::PushLine(line) => {
                let mut lines = self.lines.clone();
                lines.push(line);
                Rc::new(Self {
                    lines,
                    cwd: self.cwd.clone(),
                })
            }
            TerminalAction::ClearLines(clear_last_line) => {
                let lines = if clear_last_line {
                    let mut lines = self.lines.clone();

                    lines.truncate(lines.len() - 2);
                    lines
                } else {
                    vec![]
                };

                Rc::new(Self {
                    lines,
                    cwd: self.cwd.clone(),
                })
            }
            TerminalAction::SetCwd(cwd) => Rc::new(Self {
                lines: self.lines.clone(),
                cwd,
            }),
        }
    }
}
