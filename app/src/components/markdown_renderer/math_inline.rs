use crate::utils::formula::parse_formula_to_html;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MathInlineProps {
    pub formula: AttrValue,
}

#[function_component(MathInline)]
pub fn math_inline(props: &MathInlineProps) -> Html {
    let formula = props.formula.to_string();

    html! {
        <span class="rounded bg-white/10 px-1.5 py-0.5 font-mono text-sm text-slate-100">
            { parse_formula_to_html(&formula) }
        </span>
    }
}
