use crate::{commands::CommandContext, components::SVGIcon};
use std::future::Future;
use wasm_bindgen_futures::spawn_local;
use yew::html;

pub fn run_async<Fut>(ctx: CommandContext, fut: Fut)
where
    Fut: Future<Output = ()> + 'static,
{
    let terminal = ctx.terminal.clone();
    spawn_local(async move {
        let id = terminal.push_component(html! {
            <div>
                <SVGIcon src={include_str!("../icons/loading.svg")}></SVGIcon>
            </div>
        });
        fut.await;
        terminal.remove(id);
    });
}
