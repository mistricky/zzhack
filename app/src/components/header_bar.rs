use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HeaderBarProps {}

#[function_component(HeaderBar)]
pub fn header_bar(_props: &HeaderBarProps) -> Html {
    html! {
        <div class="flex items-center justify-between px-4 py-2 sm:py-3 bg-card backdrop-blur-xl">
            <div class="flex items-center gap-3">
                <div class="flex items-center gap-2">
                    <span class="inline-block h-3 w-3 rounded-full bg-rose-500 shadow-inner shadow-rose-500/30" />
                    <span class="inline-block h-3 w-3 rounded-full bg-amber-400 shadow-inner shadow-amber-400/30" />
                    <span class="inline-block h-3 w-3 rounded-full bg-emerald-500 shadow-inner shadow-emerald-500/30" />
                </div>
            </div>
        </div>
    }
}
