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
    pub show_window: bool,
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
    let body = html! {
        <>
            <OutputLog lines={props.lines.clone()} />
            <PromptLine
                value={props.input.clone()}
                on_input={props.on_input.clone()}
                on_submit={props.on_submit.clone()}
                on_history_nav={props.on_history_nav.clone()}
            />
        </>
    };

    html! {
        <div class="min-h-[100svh] bg-page text-text flex items-start sm:items-center justify-center px-4 pt-16 pb-6 sm:p-6 overflow-y-auto">
            <button
                type="button"
                class="fixed top-4 right-4 sm:top-6 sm:right-6 h-9 w-9 sm:h-10 sm:w-10 flex items-center justify-center rounded-full text-border hover:bg-hover transition-colors"
                aria-label="Toggle theme"
                onclick={toggle_icon}
            >
                { icon }
            </button>

            {
                if !props.show_window {
                    html! {
                        <div class="w-full sm:max-w-prose">
                            {body}
                        </div>
                    }
                } else {
                    html! {
                        <div class="w-full sm:max-w-prose min-h-[70vh] sm:min-h-[400px] overflow-hidden rounded-none sm:rounded-2xl border-0 sm:border sm:border-[0.5px] border-border bg-card shadow-none sm:shadow-[0_20px_60px_-25px_rgba(0,0,0,0.85)] backdrop-blur-xl ring-0 sm:ring-[0.5px] ring-border">
                            <HeaderBar />
                            <div class="bg-card backdrop-blur-xl px-4 sm:px-5 py-4 font-mono text-base sm:text-sm text-text space-y-3">
                                {body}
                            </div>
                        </div>
                    }
                }
            }
        </div>
    }
}
