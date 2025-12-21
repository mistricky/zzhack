use crate::components::{HistoryDirection, TerminalWindow};
use crate::config_service::ConfigService;
use crate::router::{start_router, RouterHandle};
use crate::terminal::Terminal;
use crate::terminal_state::TerminalState;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::{prelude::*, use_effect_with, use_mut_ref, use_reducer};

const SHRC_CONTENT: &str = include_str!("../../data/.shrc");
const LOADING_LOGO: &str = include_str!("icons/loading_logo.svg");

#[derive(Clone)]
struct SubmitState {
    terminal: Rc<RefCell<Option<Terminal>>>,
    terminal_ready: UseStateHandle<bool>,
    input: UseStateHandle<String>,
}

#[function_component(App)]
pub fn app() -> Html {
    let terminal_state = use_reducer(TerminalState::default);
    let input = use_state(String::new);
    let terminal = use_mut_ref(|| Option::<Terminal>::None);
    let terminal_ready = use_state(|| false);
    let router_handle = use_mut_ref(|| Option::<RouterHandle>::None);

    {
        let terminal = terminal.clone();
        let terminal_ready = terminal_ready.clone();
        let terminal_state = terminal_state.clone();
        let router_handle = router_handle.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let built = Terminal::new(terminal_state).await;
                *terminal.borrow_mut() = Some(built.clone());

                if router_handle.borrow().is_none() {
                    if let Some(handle) = start_router(built.clone()) {
                        *router_handle.borrow_mut() = Some(handle);
                    }
                }

                terminal_ready.set(true);
                built.execute_command(SHRC_CONTENT);
            });
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
    };

    let on_submit = {
        let submit_state = submit_state.clone();
        Callback::from(move |_| handle_submit(submit_state.clone()))
    };

    let displayed_lines = (*terminal_state).lines.clone();

    let on_history_nav = {
        let terminal = terminal.clone();
        let terminal_ready = terminal_ready.clone();
        let input = input.clone();
        Callback::from(move |dir: HistoryDirection| {
            if !*terminal_ready {
                return;
            }

            let history_handle = {
                let term_ref = terminal.borrow();
                let Some(term) = term_ref.as_ref() else {
                    return;
                };
                term.history()
            };

            let replacement = {
                let mut hist = history_handle.borrow_mut();
                match dir {
                    HistoryDirection::Previous => hist.previous(),
                    HistoryDirection::Next => hist.next(),
                }
            };

            if let Some(val) = replacement {
                input.set(val);
            }
        })
    };

    let config = ConfigService::get();
    let show_window = config.app.terminal_window;

    let loading_logo = Html::from_html_unchecked(AttrValue::from(LOADING_LOGO));

    html! {
        <>
            <TerminalWindow
                lines={displayed_lines}
                input={(*input).clone()}
                on_input={on_input}
                on_submit={on_submit}
                on_history_nav={on_history_nav}
                show_window={show_window}
            />
        </>
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

    terminal.process_command(trimmed);
}
