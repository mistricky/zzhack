use yew::prelude::*;
use yew::AttrValue;

#[derive(Properties, PartialEq)]
pub struct SvgIconProps {
    pub src: &'static str,
}

#[function_component(SVGIcon)]
pub fn svg_icon(props: &SvgIconProps) -> Html {
    Html::from_html_unchecked(AttrValue::from(props.src))
}
