use crate::cache_service::CacheService;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;

const HISTORY_CACHE_KEY: &str = "command_history.json";
const HISTORY_LIMIT: usize = 1000;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CommandHistory {
    entries: Vec<String>,
    cursor: Option<usize>,
    #[serde(skip, default)]
    cache: Option<Rc<CacheService>>,
}

impl PartialEq for CommandHistory {
    fn eq(&self, other: &Self) -> bool {
        self.entries == other.entries && self.cursor == other.cursor
    }
}

impl CommandHistory {
    pub async fn new(cache: Option<Rc<CacheService>>) -> Self {
        if let Some(cache_handle) = cache.clone() {
            if let Some(mut loaded) = Self::load_from_cache(cache_handle.clone()).await {
                loaded.cache = Some(cache_handle);
                loaded.cursor = None;
                return loaded;
            }
        }

        Self {
            entries: Vec::new(),
            cursor: None,
            cache,
        }
    }

    pub fn push(&mut self, entry: String) {
        if entry.is_empty() {
            return;
        }
        self.entries.push(entry);
        if self.entries.len() > HISTORY_LIMIT {
            let overflow = self.entries.len() - HISTORY_LIMIT;
            self.entries.drain(0..overflow);
        }
        self.cursor = None;
        self.persist_async();
    }

    pub fn previous(&mut self) -> Option<String> {
        if self.entries.is_empty() {
            return None;
        }
        let next_cursor = match self.cursor {
            Some(0) => 0,
            Some(idx) => idx.saturating_sub(1),
            None => self.entries.len().saturating_sub(1),
        };
        self.cursor = Some(next_cursor);
        self.entries.get(next_cursor).cloned()
    }

    pub fn next(&mut self) -> Option<String> {
        let Some(current) = self.cursor else {
            return None;
        };
        let next_cursor = current + 1;
        if next_cursor >= self.entries.len() {
            self.cursor = None;
            return Some(String::new());
        }
        self.cursor = Some(next_cursor);
        self.entries.get(next_cursor).cloned()
    }

    fn persist_async(&self) {
        let Some(cache) = self.cache.clone() else {
            return;
        };

        let snapshot = self.clone();
        spawn_local(async move {
            if let Ok(bytes) = serde_json::to_vec(&snapshot) {
                let _ = cache.put(HISTORY_CACHE_KEY, bytes).await;
            }
        });
    }

    async fn load_from_cache(cache: Rc<CacheService>) -> Option<Self> {
        match cache.get(HISTORY_CACHE_KEY).await {
            Ok(Some(bytes)) => serde_json::from_slice(&bytes).ok(),
            _ => None,
        }
    }
}
