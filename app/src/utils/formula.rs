use yew::prelude::*;

pub fn parse_formula_to_html(formula: &str) -> VNode {
    let parsed_formula_html = katex::render(formula).unwrap();

    Html::from_html_unchecked(AttrValue::from(parsed_formula_html))
}
