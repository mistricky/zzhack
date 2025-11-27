use yew::prelude::*;

use crate::{
    utils::{format_timestamp_short, get_cover_path, path::parse_data_url},
    vfs_data::VfsNode,
};

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub metadata: VfsNode,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let cover_url = parse_data_url(&get_cover_path(&props.metadata.path));
    let modified = props.metadata.modified.clone().unwrap_or_default();
    let formatted_time_str = format_timestamp_short(&modified);

    html! {
        <header class="group relative rounded-xl overflow-hidden">
            <img src={cover_url} alt="Cover" />
            <div class="absolute bottom-0 left-0 w-full transform-gpu transition-transform duration-300 ease-out group-hover:translate-y-full">
                <div class="relative overflow-hidden">
                    <div class="blur-fade-overlay"></div>
                    <div class="relative p-4 pt-6 flex flex-col gap-2 text-white">
                        <h1 class="mb-1">{ &props.metadata.title }</h1>
                        <div class="flex justify-between w-full text-sm">
                            <div class="text-post">{ &props.metadata.description }</div>
                            <div class="text-post">{ &formatted_time_str }</div>
                        </div>
                    </div>
                </div>
            </div>
        </header>
    }
}
