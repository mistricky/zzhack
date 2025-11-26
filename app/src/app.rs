use crate::commands_history_service::CommandHistory;
use crate::components::{HistoryDirection, TerminalWindow};
use crate::config_service::ConfigService;
use crate::terminal::Terminal;
use crate::types::TermLine;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::{prelude::*, use_effect_with, use_mut_ref};

#[derive(Clone)]
struct SubmitState {
    terminal: Rc<RefCell<Option<Terminal>>>,
    terminal_ready: UseStateHandle<bool>,
    input: UseStateHandle<String>,
    history: UseStateHandle<CommandHistory>,
}

#[function_component(App)]
pub fn app() -> Html {
    let lines = use_state(Vec::<TermLine>::new);
    let input = use_state(String::new);
    let cwd = use_state(Vec::<String>::new);
    let history = use_state(CommandHistory::new);
    let terminal = use_mut_ref(|| Option::<Terminal>::None);
    let terminal_ready = use_state(|| false);

    {
        let terminal = terminal.clone();
        let terminal_ready = terminal_ready.clone();
        let lines = lines.clone();
        let cwd = cwd.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let built = Terminal::new(lines, cwd).await;
                *terminal.borrow_mut() = Some(built);
                terminal_ready.set(true);
            });
            || ()
        });
    }

    {
        let terminal = terminal.clone();
        let lines = lines.clone();
        let cwd = cwd.clone();
        use_effect_with((lines.clone(), cwd.clone()), move |(lines, cwd)| {
            if let Some(term) = terminal.borrow_mut().as_mut() {
                term.update_state_handles(lines.clone(), cwd.clone());
            }
            || ()
        });
    }

    let on_input = {
        let input = input.clone();
        Callback::from(move |value: String| input.set(value))
    };

    let submit_state = SubmitState {
        terminal: terminal.clone(),
        terminal_ready: terminal_ready.clone(),
        input: input.clone(),
        history: history.clone(),
    };

    let on_submit = {
        let submit_state = submit_state.clone();
        Callback::from(move |_| handle_submit(submit_state.clone()))
    };

    let displayed_lines = (*lines).clone();

    let on_history_nav = {
        let history = history.clone();
        let input = input.clone();
        Callback::from(move |dir: HistoryDirection| {
            let mut next = (*history).clone();
            let replacement = match dir {
                HistoryDirection::Previous => next.previous(),
                HistoryDirection::Next => next.next(),
            };
            if let Some(val) = replacement {
                input.set(val);
            }
            history.set(next);
        })
    };

    let config = ConfigService::get();
    let show_window = config.app.terminal_window;

    html! {
        <TerminalWindow
            lines={displayed_lines}
            input={(*input).clone()}
            on_input={on_input}
            on_submit={on_submit}
            on_history_nav={on_history_nav}
            show_window={show_window}
        />
    }
}

fn handle_submit(state: SubmitState) {
    let trimmed = (*state.input).trim().to_string();
    state.input.set(String::new());

    if trimmed.is_empty() || !*state.terminal_ready {
        return;
    }

    let Some(terminal) = state.terminal.borrow().clone() else {
        return;
    };

    let history = state.history.clone();
    spawn_local(async move {
        terminal.process_command(history, trimmed).await;
    });
}
