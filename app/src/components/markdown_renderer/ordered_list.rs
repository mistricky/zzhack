use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct OrderedListProps {
    pub items: Vec<Html>,
}

#[function_component(OrderedList)]
pub fn ordered_list(props: &OrderedListProps) -> Html {
    html! {
        <ol class="ml-5 list-decimal space-y-2 text-white">
            { for props.items.iter().map(|item| html! { <li>{ item.clone() }</li> }) }
        </ol>
    }
}
