use crate::commands::{line_error, line_out, CommandContext, CommandHandler, CommandOutcome};
use async_trait::async_trait;
use shell_parser::CommandSpec;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};

pub struct FetchCommand;

#[async_trait(?Send)]
impl CommandHandler for FetchCommand {
    fn name(&self) -> &'static str {
        "fetch"
    }

    fn spec(&self) -> CommandSpec {
        CommandSpec::new("fetch").with_min_args(1).with_max_args(1)
    }

    async fn run(&self, args: &[String], ctx: &CommandContext) -> CommandOutcome {
        let Some(uri) = args.get(0) else {
            return CommandOutcome {
                lines: vec![line_error("fetch: missing uri".into())],
                new_cwd: None,
            };
        };

        let Some(cache) = ctx.cache.clone() else {
            return CommandOutcome {
                lines: vec![line_error(
                    "fetch: cache unavailable (OPFS init failed)".into(),
                )],
                new_cwd: None,
            };
        };

        let uri_refresh = uri.clone();
        let cache_for_refresh = cache.clone();
        spawn_local(async move {
            if let Ok(bytes) = fetch_bytes(&uri_refresh).await {
                let _ = cache_for_refresh.put(&uri_refresh, bytes).await;
            }
        });

        if let Ok(Some(bytes)) = cache.get(uri).await {
            let text = bytes_to_text(&bytes);
            return CommandOutcome {
                lines: vec![line_out(text)],
                new_cwd: None,
            };
        }

        match fetch_bytes(uri).await {
            Ok(bytes) => {
                let text = bytes_to_text(&bytes);
                let _ = cache.put(uri, bytes).await;
                CommandOutcome {
                    lines: vec![line_out(text)],
                    new_cwd: None,
                }
            }
            Err(_) => CommandOutcome {
                lines: vec![line_error(format!("fetch: failed to fetch {uri}"))],
                new_cwd: None,
            },
        }
    }
}

async fn fetch_bytes(url: &str) -> Result<Vec<u8>, wasm_bindgen::JsValue> {
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

fn bytes_to_text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).to_string()
}
