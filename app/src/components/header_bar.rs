use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HeaderBarProps {
    pub title: String,
    pub status: String,
}

#[function_component(HeaderBar)]
pub fn header_bar(props: &HeaderBarProps) -> Html {
    html! {
        <div class="flex items-center justify-between border-b border-slate-800 px-4 py-3 bg-slate-900/80">
            <div class="flex items-center gap-2 text-slate-100 font-semibold">
                <span class="inline-block h-3 w-3 rounded-full bg-emerald-500 shadow-inner" />
                <span class="tracking-tight">{ props.title.clone() }</span>
            </div>
            <div class="text-xs uppercase tracking-[0.2em] text-emerald-400 font-mono">
                { props.status.clone() }
            </div>
        </div>
    }
}
