use crate::cache_service::CacheService;
use crate::commands::{command_handlers, execute_command, CommandContext, CommandHandler};
use crate::components::TerminalWindow;
use crate::terminal::Terminal;
use crate::types::{OutputKind, TermLine};
use crate::vfs_data::{load_vfs, VfsNode};
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Clone)]
struct SubmitState {
    terminal: Terminal,
    input: UseStateHandle<String>,
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

    let ctx = CommandContext {
        vfs: state.vfs.clone(),
        cache: cache_handle,
        terminal: state.terminal.clone(),
    };

    execute_command(&trimmed, ctx, state.handlers.as_ref()).await;

    // restore cleared input (kept empty)
    state.input.set(String::new());
}

fn demo_lines() -> Vec<TermLine> {
    let mut lines = vec![
        TermLine {
            prompt: "$".into(),
            body: "trunk serve --open --release".into(),
            accent: false,
            kind: OutputKind::Text,
        },
        TermLine {
            prompt: ">".into(),
            body: "watching for file changes...".into(),
            accent: true,
            kind: OutputKind::Text,
        },
        TermLine {
            prompt: ">".into(),
            body: "compiling to wasm32-unknown-unknown".into(),
            accent: false,
            kind: OutputKind::Text,
        },
        TermLine {
            prompt: ">".into(),
            body: "build finished in 1.2s; output -> dist/".into(),
            accent: true,
            kind: OutputKind::Text,
        },
        TermLine {
            prompt: "$".into(),
            body: "open http://127.0.0.1:8080 to view".into(),
            accent: false,
            kind: OutputKind::Text,
        },
    ];

    lines.push(TermLine {
        prompt: ">".into(),
        body: r#"<strong class="text-amber-300">HTML output enabled</strong>"#.into(),
        accent: false,
        kind: OutputKind::Html,
    });
    lines.push(TermLine {
        prompt: "!".into(),
        body: r#"<em class="text-rose-300">errors can be styled too</em>"#.into(),
        accent: true,
        kind: OutputKind::Error,
    });

    lines
}
