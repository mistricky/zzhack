use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BlockquoteProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Blockquote)]
pub fn blockquote(props: &BlockquoteProps) -> Html {
    html! {
        <blockquote class="my-4 border-l-4 border-white/40 bg-black/20 px-4 py-3 text-white/90">
            { for props.children.iter() }
        </blockquote>
    }
}
