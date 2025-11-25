use crate::types::{OutputKind, TermLine};
use yew::prelude::*;
use yew::AttrValue;

#[derive(Properties, PartialEq)]
pub struct OutputLogProps {
    pub lines: Vec<TermLine>,
}

#[function_component(OutputLog)]
pub fn output_log(props: &OutputLogProps) -> Html {
    html! {
        <div class="space-y-1 font-mono text-sm text-slate-100">
            { for props.lines.iter().enumerate().map(|(idx, line)| render_line(idx, line)) }
        </div>
    }
}

fn render_line(idx: usize, line: &TermLine) -> Html {
    let content = match line.kind {
        OutputKind::Text => html! { &line.body },
        OutputKind::Html => Html::from_html_unchecked(AttrValue::from(line.body.clone())),
        OutputKind::Error => Html::from_html_unchecked(AttrValue::from(line.body.clone())),
    };

    let text_class = match line.kind {
        OutputKind::Error => "text-rose-300",
        _ => {
            if line.accent {
                "text-emerald-300"
            } else {
                "text-slate-100"
            }
        }
    };

    html! {
        <div class="leading-relaxed flex gap-2" key={idx.to_string()}>
            {
                if !line.prompt.is_empty() {
                    html! { <span class="text-slate-500">{ &line.prompt }</span> }
                } else {
                    html! {}
                }
            }
            <span class={classes!(text_class, "break-words")}>{ content }</span>
        </div>
    }
}
