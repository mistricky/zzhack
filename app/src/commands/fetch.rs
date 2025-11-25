use crate::cache_service::CacheService;
use crate::commands::{parse_cli, CommandContext};
use micro_cli::Parser;
use shell_parser::integration::ExecutableCommand;
use shell_parser::CommandSpec;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};

#[derive(Parser, Debug, Default)]
#[command(about = "Fetch a remote resource")]
struct FetchCli {
    #[arg(positional, help = "URI to fetch")]
    uri: String,
}

pub struct FetchCommand;

pub async fn fetch_text_with_cache(uri: &str, cache: &Rc<CacheService>) -> Result<String, String> {
    let uri_refresh = uri.to_string();
    let cache_for_refresh = cache.clone();
    spawn_local(async move {
        if let Ok(bytes) = fetch_bytes(&uri_refresh).await {
            let _ = cache_for_refresh.put(&uri_refresh, bytes).await;
        }
    });

    if let Ok(Some(bytes)) = cache.get(uri).await {
        return Ok(bytes_to_text(&bytes));
    }

    match fetch_bytes(uri).await {
        Ok(bytes) => {
            let text = bytes_to_text(&bytes);
            let _ = cache.put(uri, bytes).await;
            Ok(text)
        }
        Err(_) => Err(format!("failed to fetch {uri}")),
    }
}

impl ExecutableCommand<CommandContext> for FetchCommand {
    fn name(&self) -> &'static str {
        "fetch"
    }

    fn description(&self) -> &'static str {
        "Fetch a remote resource"
    }

    fn spec(&self) -> CommandSpec {
        CommandSpec::new("fetch").with_min_args(1).with_max_args(1)
    }

    fn run(&self, args: &[String], ctx: &CommandContext) -> Result<(), String> {
        let Some(cli) = parse_cli::<FetchCli>(args, ctx, self.name()) else {
            return Ok(());
        };

        let Some(cache) = ctx.cache.clone() else {
            ctx.terminal
                .push_error("fetch: cache unavailable (OPFS init failed)");
            return Ok(());
        };

        let ctx = ctx.clone();
        spawn_local(async move {
            match fetch_text_with_cache(&cli.uri, &cache).await {
                Ok(text) => ctx.terminal.push_text(text),
                Err(err) => ctx.terminal.push_error(format!("fetch: {err}")),
            };
        });

        Ok(())
    }
}

pub(crate) async fn fetch_bytes(url: &str) -> Result<Vec<u8>, wasm_bindgen::JsValue> {
    let window = web_sys::window().ok_or_else(|| js_sys::Error::new("no window"))?;
    let resp_value = JsFuture::from(window.fetch_with_str(url)).await?;
    let resp: web_sys::Response = resp_value.dyn_into()?;

    if !resp.ok() {
        return Err(js_sys::Error::new(&format!("status {}", resp.status())).into());
    }

    let buffer = JsFuture::from(resp.array_buffer()?).await?;
    let array = js_sys::Uint8Array::new(&buffer);
    Ok(array.to_vec())
}

pub(crate) fn bytes_to_text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).to_string()
}
