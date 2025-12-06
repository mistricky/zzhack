use tracing_subscriber::{filter::Targets, prelude::*};
use tracing_web::MakeWebConsoleWriter;
use zzhack_v6::app::App;

fn main() {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(true)
        .without_time()
        .with_writer(MakeWebConsoleWriter::new())
        .with_filter(
            Targets::new()
                .with_target("yew", tracing::Level::DEBUG)
                .with_default(tracing::Level::TRACE),
        );

    tracing_subscriber::registry().with(fmt_layer).init();

    tracing::info!("Starting Yew application");
    yew::Renderer::<App>::new().render();
}
