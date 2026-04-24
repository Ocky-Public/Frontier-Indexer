use futures::future::join_all;
use serde::Serialize;
use std::sync::Arc;

use crate::transports::Transport;

pub struct Emitter<I: Serialize + Send + Sync + 'static> {
    routing_fn: Box<dyn Fn(&I) -> Option<String> + Send + Sync + 'static>,
    transports: Vec<Arc<dyn Transport<I>>>,
}

impl<I: Serialize + Send + Sync + 'static> Emitter<I> {
    pub fn new(
        routing_fn: impl Fn(&I) -> Option<String> + Send + Sync + 'static,
        transports: Vec<Arc<dyn Transport<I>>>,
    ) -> Self {
        Self {
            routing_fn: Box::new(routing_fn),
            transports,
        }
    }

    pub async fn dispatch(&self, pipeline: &'static str, item: &I) {
        if self.transports.is_empty() {
            return;
        }

        let Some(routing_key) = (self.routing_fn)(item) else {
            return;
        };

        let futs: Vec<_> = self
            .transports
            .iter()
            .map(|t| t.send(pipeline, &routing_key, item))
            .collect();


        let results = join_all(futs).await;

        for result in results {
            if let Err(e) = result {
                tracing::warn!(pipeline, routing_key, "Transport send failed: {:?}", e);
            }
        }
    }
}
