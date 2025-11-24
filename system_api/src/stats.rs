use std::io;

/// Remote resource metadata derived from HTTP headers.
#[derive(Debug, Clone)]
pub struct FileStats {
    pub url: String,
    pub kind: FileKind,
    pub size: Option<u64>,
    pub readonly: bool,
    pub accessed: Option<String>,
    pub modified: Option<String>,
    pub created: Option<String>,
    pub extension: Option<String>,
    pub content_type: Option<String>,
    pub status: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileKind {
    File,
    Other,
}

/// Retrieve resource stats via HTTP HEAD.
#[cfg(target_arch = "wasm32")]
pub async fn stats(path: impl AsRef<str>) -> io::Result<FileStats> {
    use gloo_net::http::Request;

    let url = path.as_ref().to_string();
    let resp = Request::head(&url)
        .send()
        .await
        .map_err(|err| to_io_error("request failed", err))?;

    let status = resp.status();
    if !resp.ok() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("request failed with status {status}"),
        ));
    }

    let headers = resp.headers();
    let size = headers
        .get("content-length")
        .and_then(|val| val.parse::<u64>().ok());
    let content_type = headers.get("content-type");
    let modified = headers.get("last-modified");

    Ok(FileStats {
        url: url.clone(),
        kind: FileKind::File,
        size,
        readonly: true,
        accessed: None,
        modified,
        created: None,
        extension: guess_extension(&url),
        content_type,
        status,
    })
}

/// Non-wasm fallback: not supported.
#[cfg(not(target_arch = "wasm32"))]
pub async fn stats(path: impl AsRef<str>) -> io::Result<FileStats> {
    let _ = path;
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "stats is only available in wasm/browser targets",
    ))
}

#[cfg(target_arch = "wasm32")]
fn guess_extension(url: &str) -> Option<String> {
    // Strip query/fragment, then pull extension after last dot.
    let trimmed = url.split(&['?', '#'][..]).next().unwrap_or(url);
    trimmed
        .rsplit_once('.')
        .map(|(_, ext)| ext.to_string())
        .filter(|ext| !ext.is_empty())
}

#[cfg(target_arch = "wasm32")]
fn to_io_error(context: &str, err: impl std::fmt::Display) -> io::Error {
    io::Error::new(io::ErrorKind::Other, format!("{context}: {err}"))
}
