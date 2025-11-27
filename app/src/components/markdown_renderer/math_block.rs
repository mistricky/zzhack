use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MathBlockProps {
    pub formula: AttrValue,
}

#[function_component(MathBlock)]
pub fn math_block(props: &MathBlockProps) -> Html {
    html! {
        <div class="my-4 rounded-lg bg-black/30 p-4 font-mono text-slate-100">
            { props.formula.clone() }
        </div>
    }
}
