use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ImageProps {
    pub src: AttrValue,
    #[prop_or_default]
    pub alt: AttrValue,
    #[prop_or_default]
    pub title: AttrValue,
}

#[function_component(Image)]
pub fn image(props: &ImageProps) -> Html {
    let is_open = use_state(|| false);
    let alt_text = props.alt.as_ref();
    let aria_label = if alt_text.is_empty() {
        "Open image".to_string()
    } else {
        format!("Open image: {}", alt_text)
    };

    let on_open = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(true))
    };
    let on_close = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(false))
    };
    let stop_click = Callback::from(|event: MouseEvent| event.stop_propagation());

    html! {
        <>
            <button
                type="button"
                class="group my-4 w-full text-left"
                onclick={on_open}
                aria-label={aria_label}
            >
                <div class="relative w-full overflow-hidden rounded-xl bg-black/40 aspect-video">
                    <img
                        src={props.src.clone()}
                        alt={props.alt.clone()}
                        title={props.title.clone()}
                        class="h-full w-full object-cover transition duration-300 group-hover:scale-[1.02]"
                    />
                </div>
            </button>
            if *is_open {
                <div
                    class="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-md p-4"
                    onclick={on_close}
                    role="dialog"
                    aria-modal="true"
                >
                    <div class="max-h-full max-w-full" onclick={stop_click}>
                        <img
                            src={props.src.clone()}
                            alt={props.alt.clone()}
                            title={props.title.clone()}
                            class="max-h-full max-w-full rounded-xl object-contain shadow-2xl"
                        />
                    </div>
                </div>
            }
        </>
    }
}
