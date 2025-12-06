use gloo_worker::{oneshot::OneshotBridge, Spawnable};
use std::cell::RefCell;

use worker::{HighlightRequest, HighlightWorker};

pub struct HighlightService;

const WORKER_ENTRYPOINT: &str = "/highlight_worker.js";

thread_local! {
    static HIGHLIGHT_WORKER: RefCell<Option<OneshotBridge<HighlightWorker>>> = RefCell::new(None);
}

impl HighlightService {
    pub async fn highlight_html(code: &str, language: Option<&str>) -> String {
        Self::highlight_lines_html(code, language).await.join("\n")
    }

    pub async fn highlight_lines_html(code: &str, language: Option<&str>) -> Vec<String> {
        let request = HighlightRequest {
            code: code.to_owned(),
            language: language.map(str::to_owned),
        };

        run_worker(request).await
    }
}

async fn run_worker(request: HighlightRequest) -> Vec<String> {
    let mut bridge = HIGHLIGHT_WORKER.with(|cell| {
        let mut worker = cell.borrow_mut();
        if worker.is_none() {
            let base = HighlightWorker::spawner().spawn(WORKER_ENTRYPOINT);
            *worker = Some(base);
        }

        worker.as_ref().expect("worker initialized above").fork()
    });

    bridge.run(request).await
}

pub mod worker {
    use gloo_worker::{oneshot::oneshot, Registrable};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct HighlightRequest {
        pub code: String,
        pub language: Option<String>,
    }

    #[oneshot(HighlightWorker)]
    pub(crate) async fn highlight_worker(request: HighlightRequest) -> Vec<String> {
        crate::highlight_engine::HighlightEngine::highlight_lines(
            &request.code,
            request.language.as_deref(),
        )
    }

    pub fn register() {
        HighlightWorker::registrar().register();
    }
}
