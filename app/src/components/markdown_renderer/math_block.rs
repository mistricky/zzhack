use crate::utils::formula::parse_formula_to_html;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MathBlockProps {
    pub formula: AttrValue,
}

#[function_component(MathBlock)]
pub fn math_block(props: &MathBlockProps) -> Html {
    let formula = props.formula.to_string();

    html! {
        <div class="my-4 rounded-lg bg-black/30 p-4 font-mono text-slate-100">
            { parse_formula_to_html(&formula) }
        </div>
    }
}
