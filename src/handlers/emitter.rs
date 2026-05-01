use futures::future::join_all;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;

use crate::transports::Transport;

pub trait Dispatchable<I> {
    fn dispatch_iter(&self) -> Box<dyn Iterator<Item = &I> + Send + '_>;
    fn is_empty(&self) -> bool;
}

impl<I: Send + Sync> Dispatchable<I> for Vec<I> {
    fn dispatch_iter(&self) -> Box<dyn Iterator<Item = &I> + Send + '_> {
        Box::new(self.iter())
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<I: Send + Sync> Dispatchable<I> for HashMap<String, I> {
    fn dispatch_iter(&self) -> Box<dyn Iterator<Item = &I> + Send + '_> {
        Box::new(self.values())
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

pub struct Emitter<I: Serialize + Send + Sync + 'static> {
    transports: Vec<Arc<dyn Transport<I>>>,
}

impl<I: Serialize + Send + Sync + 'static> Emitter<I> {
    pub fn new(transports: Vec<Arc<dyn Transport<I>>>) -> Self {
        Self { transports }
    }

    pub fn dispatch<B>(self: &Arc<Self>, pipeline: &'static str, batch: &B)
    where
        B: Dispatchable<I> + Clone + Send + 'static,
    {
        if batch.is_empty() {
            return;
        }

        let batch = batch.clone();
        let emitter = Arc::clone(self);

        tokio::spawn(async move {
            for entry in batch.dispatch_iter() {
                emitter.dispatch_entry(pipeline, entry).await;
            }
        });
    }

    async fn dispatch_entry(&self, pipeline: &'static str, item: &I) {
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
