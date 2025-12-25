use crate::types::TermLine;
use std::rc::Rc;
use uuid::Uuid;
use yew::Reducible;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TerminalState {
    pub lines: Vec<TermLine>,
    pub cwd: Vec<String>,
}

#[derive(Clone, Debug)]
pub enum TerminalAction {
    PushLine(TermLine),
    ClearLines(Option<usize>),
    RemoveLine(Uuid),
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
            TerminalAction::ClearLines(clear_last_nth) => {
                let lines = match clear_last_nth {
                    Some(last_nth) => {
                        let mut lines = self.lines.clone();

                        remove_from_end(&mut lines, last_nth);
                        lines
                    }
                    None => vec![],
                };

                Rc::new(Self {
                    lines,
                    cwd: self.cwd.clone(),
                })
            }
            TerminalAction::RemoveLine(id) => {
                let mut lines = self.lines.clone();

                tracing::info!("{:?}", lines);

                lines.retain(|line| line.id != id);

                tracing::info!("{:?}", lines);

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

fn remove_from_end<T>(v: &mut Vec<T>, n: usize) -> Option<T> {
    if n == 0 || n > v.len() {
        return None;
    }
    let idx = v.len() - n;
    Some(v.remove(idx))
}
