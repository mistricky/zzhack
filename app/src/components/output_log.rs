use crate::types::TermLine;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct OutputLogProps {
    pub lines: Vec<TermLine>,
}

#[function_component(OutputLog)]
pub fn output_log(props: &OutputLogProps) -> Html {
    html! {
        <div class="space-y-1 px-4 py-3 font-mono text-sm text-slate-100">
            { for props.lines.iter().enumerate().map(|(idx, line)| render_line(idx, line)) }
        </div>
    }
}

fn render_line(idx: usize, line: &TermLine) -> Html {
    html! {
        <div class="flex gap-3 leading-relaxed" key={idx.to_string()}>
            <span class="text-slate-500">{ &line.prompt }</span>
            <span class={classes!(
                if line.accent { "text-emerald-300" } else { "text-slate-100" },
                "break-words"
            )}>{ &line.body }</span>
        </div>
    }
}
