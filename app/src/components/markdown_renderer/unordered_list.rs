use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct UnorderedListProps {
    pub items: Vec<Html>,
}

#[function_component(UnorderedList)]
pub fn unordered_list(props: &UnorderedListProps) -> Html {
    html! {
        <ul class="ml-5 list-disc space-y-2 text-white">
            { for props.items.iter().map(|item| html! { <li>{ item.clone() }</li> }) }
        </ul>
    }
}
