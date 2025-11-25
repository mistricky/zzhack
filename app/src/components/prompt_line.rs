use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PromptLineProps {
    pub prompt: String,
    pub value: String,
    pub on_input: Callback<String>,
    pub on_submit: Callback<()>,
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

    let on_submit = {
        let on_submit = props.on_submit.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            on_submit.emit(());
        })
    };

    html! {
        <form onsubmit={on_submit} class="flex items-center gap-3 font-mono text-sm text-slate-100">
            <span class="text-slate-500">{ props.prompt.clone() }</span>
            <span class="text-emerald-400">{ "❯" }</span>
            <input
                class="flex-1 bg-transparent text-slate-100 outline-none placeholder:text-slate-600"
                type="text"
                value={props.value.clone()}
                oninput={on_input}
                placeholder="type a command and press enter"
                autocomplete="off"
                spellcheck="false"
            />
        </form>
    }
}
