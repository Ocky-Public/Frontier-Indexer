use futures::future::join_all;
use serde::Serialize;
use std::sync::Arc;

use crate::transports::Transport;

pub struct Emitter<I: Serialize + Send + Sync + 'static> {
    transports: Vec<Arc<dyn Transport<I>>>,
}

impl<I: Serialize + Send + Sync + 'static> Emitter<I> {
    pub fn new(transports: Vec<Arc<dyn Transport<I>>>) -> Self {
        Self { transports }
    }

    pub async fn dispatch(&self, pipeline: &'static str, item: &I) {
        if self.transports.is_empty() {
            return;
        }

        let futs: Vec<_> = self
            .transports
            .iter()
            .map(|t| t.send(pipeline, item))
            .collect();

        let results = join_all(futs).await;

        for result in results {
            if let Err(e) = result {
                tracing::warn!(pipeline, "Transport send failed: {:?}", e);
            }
        }
    }
}
