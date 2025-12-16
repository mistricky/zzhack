use crate::utils::{estimate_reading_minutes, format_timestamp_short};
use crate::vfs_data::VfsNode;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PostItemProps {
    /// Metadata for the post. For post directories, pass the `index.md` metadata.
    pub metadata: VfsNode,
    pub on_click: Callback<VfsNode>,
}

#[function_component(PostItem)]
pub fn post_item(props: &PostItemProps) -> Html {
    let title = props.metadata.title.clone().unwrap_or_else(|| {
        if let Some(ext) = &props.metadata.extension {
            if let Some(stripped) = props.metadata.name.strip_suffix(&format!(".{ext}")) {
                return stripped.to_string();
            }
        }
        props.metadata.name.clone()
    });

    let formatted_meta = props
        .metadata
        .modified
        .as_deref()
        .and_then(format_timestamp_short);

    let meta_text = match (
        formatted_meta,
        props
            .metadata
            .size
            .map(|size| estimate_reading_minutes(size as usize)),
    ) {
        (Some(meta), Some(minutes)) => format!("{meta} · {}min", minutes),
        (Some(meta), None) => meta,
        (None, Some(minutes)) => format!("{}min", minutes),
        _ => String::new(),
    };

    let on_click = {
        let on_click = props.on_click.clone();
        let metadata = props.metadata.clone();
        Callback::from(move |_| {
            on_click.emit(metadata.clone());
        })
    };

    html! {
        <div
            class="flex flex-col gap-1 text-post hover:text-post-hover hover:cursor-pointer transition-colors duration-150 text-base"
            onclick={on_click}
        >
            <div class="flex items-center gap-3">
                <span>{ title }</span>
                if !meta_text.is_empty() {
                    <span class="text-xs">{ meta_text }</span>
                }
            </div>
        </div>
    }
}
