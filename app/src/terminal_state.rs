use crate::types::TermLine;
use std::rc::Rc;
use yew::Reducible;

#[derive(Clone, Default, PartialEq)]
pub struct TerminalState {
    pub lines: Vec<TermLine>,
    pub cwd: Vec<String>,
}

#[derive(Clone)]
pub enum TerminalAction {
    PushLine(TermLine),
    ClearLines,
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
            TerminalAction::ClearLines => Rc::new(Self {
                lines: Vec::new(),
                cwd: self.cwd.clone(),
            }),
            TerminalAction::SetCwd(cwd) => Rc::new(Self {
                lines: self.lines.clone(),
                cwd,
            }),
        }
    }
}
