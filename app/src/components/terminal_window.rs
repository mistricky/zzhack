use crate::components::{HeaderBar, OutputLog, PromptLine};
use crate::types::TermLine;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TerminalWindowProps {
    pub lines: Vec<TermLine>,
    pub input: String,
    pub prompt: String,
    pub on_input: Callback<String>,
    pub on_submit: Callback<()>,
}

#[function_component(TerminalWindow)]
pub fn terminal_window(props: &TerminalWindowProps) -> Html {
    html! {
        <div class="min-h-screen bg-gradient-to-br from-slate-950 via-slate-900 to-slate-950 text-slate-50 flex items-center justify-center p-6">
            <div class="w-full max-w-4xl overflow-hidden rounded-2xl border border-slate-800/70 bg-slate-950/70 shadow-[0_20px_60px_-25px_rgba(0,0,0,0.85)] backdrop-blur-xl ring-1 ring-slate-800/50">
                <HeaderBar />
                <div class="bg-gradient-to-b from-slate-950/60 via-slate-950/70 to-slate-950/80 px-5 py-4 font-mono text-sm text-slate-100 space-y-3">
                    <OutputLog lines={props.lines.clone()} />
                    <PromptLine
                        prompt={props.prompt.clone()}
                        value={props.input.clone()}
                        on_input={props.on_input.clone()}
                        on_submit={props.on_submit.clone()}
                    />
                </div>
            </div>
        </div>
    }
}
