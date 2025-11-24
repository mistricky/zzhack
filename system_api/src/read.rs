use std::io;

/// Fetch a resource over HTTP(S) and return its bytes.
#[cfg(target_arch = "wasm32")]
pub async fn read(path: impl AsRef<str>) -> io::Result<Vec<u8>> {
    use gloo_net::http::Request;

    let url = path.as_ref();
    let resp = Request::get(url)
        .send()
        .await
        .map_err(|err| to_io_error("request failed", err))?;

    if !resp.ok() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("request failed with status {}", resp.status()),
        ));
    }

    resp.binary()
        .await
        .map_err(|err| to_io_error("read body failed", err))
}

/// Non-wasm fallback: not supported.
#[cfg(not(target_arch = "wasm32"))]
pub async fn read(path: impl AsRef<str>) -> io::Result<Vec<u8>> {
    let _ = path;
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "read is only available in wasm/browser targets",
    ))
}

#[cfg(target_arch = "wasm32")]
fn to_io_error(context: &str, err: impl std::fmt::Display) -> io::Error {
    io::Error::new(io::ErrorKind::Other, format!("{context}: {err}"))
}
