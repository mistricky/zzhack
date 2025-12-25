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
        <div class="space-y-1 font-mono text-base sm:text-sm text-slate-100">
            { for props.lines.iter().map(render_line) }
        </div>
    }
}

fn render_line(line: &TermLine) -> Html {
    let content = match line.kind {
        OutputKind::Text => html! { &line.body },
        OutputKind::Html => Html::from_html_unchecked(AttrValue::from(line.body.clone())),
        OutputKind::Error => Html::from_html_unchecked(AttrValue::from(line.body.clone())),
        OutputKind::Component => line.node.clone().unwrap_or_else(|| html! {}),
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
        <div class="output-log leading-relaxed flex gap-2 w-full" key={line.id.to_string()}>
            <span class={classes!(text_class, "break-words", "w-full")}>{ content }</span>
        </div>
    }
}
