use crate::components::{HeaderBar, OutputLog, PromptLine};
use crate::types::TermLine;
use yew::prelude::*;

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
