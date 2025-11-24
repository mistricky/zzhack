use js_sys::Uint8Array;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

/// Simple OPFS-backed cache for fetched resources.
pub struct CacheService {
    root: web_sys::FileSystemDirectoryHandle,
}

impl CacheService {
    pub async fn new() -> Result<Self, JsValue> {
        let window = web_sys::window().ok_or_else(|| js_sys::Error::new("no window"))?;
        let navigator = window.navigator();
        let storage = navigator.storage();
        let dir = JsFuture::from(storage.get_directory()).await?;
        let handle: web_sys::FileSystemDirectoryHandle = dir.dyn_into()?;
        Ok(Self { root: handle })
    }

    /// Return cached bytes for the given key if present.
    pub async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, JsValue> {
        match self.file_handle(key, false).await {
            Ok(handle) => {
                let file = JsFuture::from(handle.get_file()).await?;
                let file: web_sys::File = file.dyn_into()?;
                let buf = JsFuture::from(file.array_buffer()).await?;
                let array = Uint8Array::new(&buf);
                Ok(Some(array.to_vec()))
            }
            Err(_) => Ok(None),
        }
    }

    /// Write bytes to cache.
    pub async fn put(&self, key: &str, bytes: Vec<u8>) -> Result<(), JsValue> {
        let handle = self.file_handle(key, true).await?;
        let writable = JsFuture::from(handle.create_writable()).await?;
        let stream: web_sys::FileSystemWritableFileStream = writable.dyn_into()?;
        JsFuture::from(stream.write_with_u8_array(&bytes)?).await?;
        JsFuture::from(stream.close()).await?;
        Ok(())
    }

    async fn file_handle(
        &self,
        key: &str,
        create: bool,
    ) -> Result<web_sys::FileSystemFileHandle, JsValue> {
        if create {
            let opts = web_sys::FileSystemGetFileOptions::new();
            opts.set_create(true);
            JsFuture::from(self.root.get_file_handle_with_options(key, &opts))
                .await?
                .dyn_into()
        } else {
            JsFuture::from(self.root.get_file_handle(key))
                .await?
                .dyn_into()
        }
    }
}
