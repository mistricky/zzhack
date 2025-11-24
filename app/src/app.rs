use crate::cache_service::CacheService;
use crate::commands::{command_handlers, execute_command, CommandContext, CommandHandler};
use crate::components::TerminalWindow;
use crate::types::TermLine;
use crate::vfs_data::{format_path, load_vfs, VfsNode};
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Clone)]
struct SubmitState {
    lines: UseStateHandle<Vec<TermLine>>,
    input: UseStateHandle<String>,
    cwd: UseStateHandle<Vec<String>>,
    cache: UseStateHandle<Option<Rc<CacheService>>>,
    vfs: Rc<VfsNode>,
    handlers: Rc<Vec<Box<dyn CommandHandler>>>,
}

#[function_component(App)]
pub fn app() -> Html {
    let vfs = use_memo((), |_| load_vfs());
    let lines = use_state(demo_lines);
    let input = use_state(String::new);
    let cwd = use_state(Vec::<String>::new);
    let cache = use_state(|| Option::<Rc<CacheService>>::None);
    let handlers = use_memo((), |_| command_handlers());

    let on_input = {
        let input = input.clone();
        Callback::from(move |value: String| input.set(value))
    };

    let submit_state = SubmitState {
        lines: lines.clone(),
        input: input.clone(),
        cwd: cwd.clone(),
        cache: cache.clone(),
        vfs: vfs.clone(),
        handlers: handlers.clone(),
    };

    let on_submit = {
        let submit_state = submit_state.clone();
        Callback::from(move |_| handle_submit(submit_state.clone()))
    };

    html! {
        <TerminalWindow
            title={"zzhack-v6 terminal".to_string()}
            status={"live".to_string()}
            lines={(*lines).clone()}
            input={(*input).clone()}
            prompt={format!(
                "guest@zzhack-v6:{}",
                if cwd.is_empty() { "/".into() } else { format_path(&cwd) }
            )}
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
    let mut next = (*state.lines).clone();
    next.push(TermLine {
        prompt: format!("{}$", format_path(&state.cwd)),
        body: trimmed.clone(),
        accent: false,
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

    let ctx = CommandContext {
        cwd: (*state.cwd).clone(),
        vfs: state.vfs.clone(),
        cache: cache_handle,
    };

    let result = execute_command(&trimmed, ctx, state.handlers.as_ref()).await;
    next.extend(result.lines);
    state.lines.set(next);

    if let Some(new_cwd) = result.new_cwd {
        state.cwd.set(new_cwd);
    }

    // restore cleared input (kept empty)
    state.input.set(String::new());
}

fn demo_lines() -> Vec<TermLine> {
    vec![
        TermLine {
            prompt: "$".into(),
            body: "trunk serve --open --release".into(),
            accent: false,
        },
        TermLine {
            prompt: ">".into(),
            body: "watching for file changes...".into(),
            accent: true,
        },
        TermLine {
            prompt: ">".into(),
            body: "compiling to wasm32-unknown-unknown".into(),
            accent: false,
        },
        TermLine {
            prompt: ">".into(),
            body: "build finished in 1.2s; output -> dist/".into(),
            accent: true,
        },
        TermLine {
            prompt: "$".into(),
            body: "open http://127.0.0.1:8080 to view".into(),
            accent: false,
        },
    ]
}
