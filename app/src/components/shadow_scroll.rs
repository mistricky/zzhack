use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ShadowScrollProps {
    #[prop_or_default]
    pub children: Children,
    /// Additional classes applied to the outer wrapper.
    #[prop_or_default]
    pub class: Classes,
    /// Extra classes applied to the scrollable content area.
    #[prop_or_default]
    pub content_class: Classes,
}

#[function_component(ShadowScroll)]
pub fn shadow_scroll(props: &ShadowScrollProps) -> Html {
    let show_top_shadow = use_state(|| false);
    let show_bottom_shadow = use_state(|| false);
    let scroll_ref = use_node_ref();

    {
        let scroll_ref = scroll_ref.clone();
        let show_top_shadow = show_top_shadow.clone();
        let show_bottom_shadow = show_bottom_shadow.clone();
        let child_count = props.children.len();

        use_effect_with(child_count, move |_| {
            if let Some(element) = scroll_ref.cast::<HtmlElement>() {
                sync_shadow_flags(&element, &show_top_shadow, &show_bottom_shadow);
            }
            || ()
        });
    }

    let on_scroll = {
        let show_top_shadow = show_top_shadow.clone();
        let show_bottom_shadow = show_bottom_shadow.clone();
        Callback::from(move |event: Event| {
            if let Some(target) = event
                .target()
                .and_then(|t| t.dyn_into::<HtmlElement>().ok())
            {
                sync_shadow_flags(&target, &show_top_shadow, &show_bottom_shadow);
            }
        })
    };

    let container_classes = classes!("relative", "overflow-hidden", props.class.clone());
    let scroll_classes = classes!(
        "relative",
        "h-full",
        "overflow-y-auto",
        "overscroll-contain",
        props.content_class.clone()
    );

    let top_shadow_classes = classes!(
        "pointer-events-none",
        "absolute",
        "z-10",
        "inset-x-0",
        "top-0",
        "h-10",
        "bg-gradient-to-b",
        "from-black/50",
        "via-black/10",
        "to-transparent",
        "transition-opacity",
        "duration-200",
        if *show_top_shadow {
            "opacity-100"
        } else {
            "opacity-0"
        }
    );

    let bottom_shadow_classes = classes!(
        "pointer-events-none",
        "absolute",
        "z-10",
        "inset-x-0",
        "bottom-0",
        "h-10",
        "bg-gradient-to-t",
        "from-black/50",
        "via-black/10",
        "to-transparent",
        "transition-opacity",
        "duration-200",
        if *show_bottom_shadow {
            "opacity-100"
        } else {
            "opacity-0"
        }
    );

    html! {
        <div class={container_classes}>
            <div class={top_shadow_classes} />
            <div
                ref={scroll_ref}
                onscroll={on_scroll}
                class={scroll_classes}
            >
                { for props.children.iter() }
            </div>
            <div class={bottom_shadow_classes} />
        </div>
    }
}

fn sync_shadow_flags(
    element: &HtmlElement,
    show_top_shadow: &UseStateHandle<bool>,
    show_bottom_shadow: &UseStateHandle<bool>,
) {
    let scroll_top = element.scroll_top() as f64;
    let scroll_height = element.scroll_height() as f64;
    let client_height = element.client_height() as f64;
    let max_scroll = (scroll_height - client_height).max(0.0);

    let should_show_top = scroll_top > 1.0;
    let should_show_bottom = scroll_top < max_scroll - 1.0;

    if **show_top_shadow != should_show_top {
        show_top_shadow.set(should_show_top);
    }

    if **show_bottom_shadow != should_show_bottom {
        show_bottom_shadow.set(should_show_bottom);
    }
}
