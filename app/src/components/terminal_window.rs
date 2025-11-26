use crate::components::{HeaderBar, OutputLog, PromptLine};
use crate::types::TermLine;
use web_sys::window;
use yew::prelude::*;

const DARK_MODE_ICON: &str = include_str!("../icons/dark_mode.svg");
const LIGHT_MODE_ICON: &str = include_str!("../icons/light_mode.svg");

#[derive(Properties, PartialEq)]
pub struct TerminalWindowProps {
    pub lines: Vec<TermLine>,
    pub input: String,
    pub on_input: Callback<String>,
    pub on_submit: Callback<()>,
    pub on_history_nav: Callback<crate::components::HistoryDirection>,
}

#[function_component(TerminalWindow)]
pub fn terminal_window(props: &TerminalWindowProps) -> Html {
    let is_dark_icon = use_state(|| true);

    let toggle_icon = {
        let is_dark_icon = is_dark_icon.clone();
        Callback::from(move |_| is_dark_icon.set(!*is_dark_icon))
    };

    {
        let is_dark_icon = is_dark_icon.clone();
        use_effect_with(is_dark_icon, move |is_dark| {
            if let Some(document) = window().and_then(|w| w.document()) {
                if let Some(root) = document.document_element() {
                    let _ = if **is_dark {
                        root.class_list().add_1("theme-dark")
                    } else {
                        root.class_list().remove_1("theme-dark")
                    };
                }
            }
            || ()
        });
    }

    let icon_svg = if *is_dark_icon {
        DARK_MODE_ICON
    } else {
        LIGHT_MODE_ICON
    };
    let icon = Html::from_html_unchecked(AttrValue::from(icon_svg));

    html! {
            <div class="min-h-screen bg-page text-text flex items-center justify-center p-6">
            <button
                type="button"
                class="fixed top-6 right-6 h-10 w-10 flex items-center justify-center rounded-full text-border hover:bg-hover transition-colors"
                aria-label="Toggle theme"
                onclick={toggle_icon}
            >
                { icon }
            </button>
            <div class="w-full max-w-4xl overflow-hidden rounded-2xl border border-[0.5px] border-border bg-card shadow-[0_20px_60px_-25px_rgba(0,0,0,0.85)] backdrop-blur-xl ring-[0.5px] ring-border">
                <HeaderBar />
                <div class="bg-card backdrop-blur-xl px-5 py-4 font-mono text-sm text-text space-y-3">
                    <OutputLog lines={props.lines.clone()} />
                    <PromptLine
                        value={props.input.clone()}
                        on_input={props.on_input.clone()}
                        on_submit={props.on_submit.clone()}
                        on_history_nav={props.on_history_nav.clone()}
                    />
                </div>
            </div>
        </div>
    }
}
