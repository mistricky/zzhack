use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PromptLineProps {
    pub value: String,
    pub on_input: Callback<String>,
    pub on_submit: Callback<()>,
    pub on_history_nav: Callback<HistoryDirection>,
}

#[derive(Clone, PartialEq)]
pub enum HistoryDirection {
    Previous,
    Next,
}

#[function_component(PromptLine)]
pub fn prompt_line(props: &PromptLineProps) -> Html {
    let on_input = {
        let on_input = props.on_input.clone();
        Callback::from(move |e: InputEvent| {
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            on_input.emit(value);
        })
    };

    let on_keydown = {
        let on_history_nav = props.on_history_nav.clone();
        Callback::from(move |e: KeyboardEvent| match e.key().as_str() {
            "ArrowUp" => {
                e.prevent_default();
                on_history_nav.emit(HistoryDirection::Previous);
            }
            "ArrowDown" => {
                e.prevent_default();
                on_history_nav.emit(HistoryDirection::Next);
            }
            _ => {}
        })
    };

    let on_submit = {
        let on_submit = props.on_submit.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            on_submit.emit(());
        })
    };

    html! {
        <form onsubmit={on_submit} class="flex items-center gap-3 font-mono text-sm text-slate-100">
            <span class="text-emerald-400">{ "❯" }</span>
            <input
                class="flex-1 bg-transparent text-slate-100 outline-none placeholder:text-slate-600"
                type="text"
                value={props.value.clone()}
                oninput={on_input}
                onkeydown={on_keydown}
                placeholder="type a command and press enter"
                autocomplete="off"
                spellcheck="false"
            />
        </form>
    }
}
