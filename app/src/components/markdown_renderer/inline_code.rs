use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InlineCodeProps {
    pub code: AttrValue,
}

#[function_component(InlineCode)]
pub fn inline_code(props: &InlineCodeProps) -> Html {
    html! {
        <code class="rounded bg-white/10 px-1.5 py-0.5 text-sm font-mono text-white/90">
            { props.code.clone() }
        </code>
    }
}
