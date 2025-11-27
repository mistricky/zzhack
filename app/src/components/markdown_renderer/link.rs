use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LinkProps {
    pub href: AttrValue,
    #[prop_or_default]
    pub title: AttrValue,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Link)]
pub fn link(props: &LinkProps) -> Html {
    html! {
        <a
            href={props.href.clone()}
            title={props.title.clone()}
            class="text-blue-300 underline decoration-blue-200 hover:text-blue-100 hover:decoration-blue-100"
            target="_blank"
            rel="noreferrer noopener"
        >
            { for props.children.iter() }
        </a>
    }
}
