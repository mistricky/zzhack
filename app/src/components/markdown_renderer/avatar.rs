use yew::prelude::*;

use crate::{config_service::ConfigService, utils::path::parse_data_url};

#[derive(Properties, PartialEq)]
pub struct AvatarProps {
    pub name: AttrValue,
    pub email: AttrValue,
}

#[function_component(Avatar)]
pub fn avatar(props: &AvatarProps) -> Html {
    let author = &ConfigService::get().author;
    let avatar_url = parse_data_url(&author.avatar);
    let mailto = format!("mailto:{}", props.email);

    html! {
        <a
            href={mailto}
            class="inline-block cursor-pointer"
            aria-label={format!("Email {}", props.name)}
        >
            <img
                src={avatar_url}
                alt={format!("{}'s avatar", props.name)}
                class="mt-6 w-10 h-10 rounded-full object-cover"
            />
        </a>
    }
}
