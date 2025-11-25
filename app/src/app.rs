use crate::cache_service::CacheService;
use crate::commands::{command_handlers, CommandContext};
use crate::components::TerminalWindow;
use crate::terminal::Terminal;
use crate::types::{OutputKind, TermLine};
use crate::vfs_data::{load_vfs, VfsNode};
use shell_parser::integration::ExecutableCommand;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Clone)]
struct SubmitState {
    terminal: Terminal,
    input: UseStateHandle<String>,
    cache: UseStateHandle<Option<Rc<CacheService>>>,
    vfs: Rc<VfsNode>,
    handlers: Rc<Vec<Box<dyn ExecutableCommand<CommandContext>>>>,
}

#[function_component(App)]
pub fn app() -> Html {
    let vfs = use_memo((), |_| load_vfs());
    let lines = use_state(Vec::<TermLine>::new);
    let input = use_state(String::new);
    let cwd = use_state(Vec::<String>::new);
    let cache = use_state(|| Option::<Rc<CacheService>>::None);
    let handlers = use_memo((), |_| command_handlers());

    let on_input = {
        let input = input.clone();
        Callback::from(move |value: String| input.set(value))
    };

    let submit_state = SubmitState {
        terminal: Terminal::new(lines.clone(), cwd.clone()),
        input: input.clone(),
        cache: cache.clone(),
        vfs: vfs.clone(),
        handlers: handlers.clone(),
    };

    let on_submit = {
        let submit_state = submit_state.clone();
        Callback::from(move |_| handle_submit(submit_state.clone()))
    };

    let prompt = submit_state.terminal.prompt();

    html! {
        <TerminalWindow
            lines={submit_state.terminal.snapshot()}
            input={(*input).clone()}
            prompt={prompt}
            on_input={on_input}
            on_submit={on_submit}
        />
    }
}

fn handle_submit(state: SubmitState) {
    let trimmed = (*state.input).trim().to_string();
    state.input.set(String::new());

    if trimmed.is_empty() {
        return;
    }

    spawn_local(process_command(state, trimmed));
}

async fn process_command(state: SubmitState, trimmed: String) {
    state.terminal.push_line(TermLine {
        prompt: state.terminal.prompt(),
        body: trimmed.clone(),
        accent: false,
        kind: OutputKind::Text,
    });

    let cache_handle = if let Some(existing) = state.cache.as_ref() {
        Some(existing.clone())
    } else {
        match CacheService::new().await {
            Ok(service) => {
                let rc: Rc<CacheService> = Rc::new(service);
                state.cache.set(Some(rc.clone()));
                Some(rc)
            }
            Err(err) => {
                web_sys::console::error_1(&err);
                None
            }
        }
    };

    state
        .terminal
        .execute_command(
            &trimmed,
            state.vfs.clone(),
            cache_handle,
            state.handlers.as_ref(),
        )
        .await;

    // restore cleared input (kept empty)
    state.input.set(String::new());
}
