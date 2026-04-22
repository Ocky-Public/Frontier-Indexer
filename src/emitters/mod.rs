use async_trait::async_trait;
use futures::future::join_all;
use serde::Serialize;
use std::sync::Arc;

use crate::transports::Transport;

pub mod handler;

pub trait Routing<I>: Send + Sync + 'static {
    fn route(&self, item: &I) -> Option<String>;
}

#[async_trait]
pub trait PipelineEmitter<B: Send + Sync + 'static>: Send + Sync + 'static {
    async fn emit(&self, pipeline: &'static str, batch: &B);
}

#[async_trait]
pub trait Dispatching<I>: Send + Sync + 'static {
    async fn dispatch(&self, pipeline: &'static str, item: &I);
}

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
}

impl<I: Serialize + Send + Sync + 'static> Routing<I> for Emitter<I> {
    fn route(&self, item: &I) -> Option<String> {
        (self.routing_fn)(item)
    }
}

#[async_trait]
impl<I: Serialize + Send + Sync + 'static> PipelineEmitter<Vec<I>> for Emitter<I> {
    async fn emit(&self, pipeline: &'static str, batch: &Vec<I>) {
        let futs = batch.iter().map(|item| self.dispatch(pipeline, item));
        join_all(futs).await;
    }
}

#[async_trait]
impl<I: Serialize + Send + Sync + 'static> Dispatching<I> for Emitter<I> {
    async fn dispatch(&self, pipeline: &'static str, item: &I) {
        let Some(routing_key) = self.route(item) else {
            return;
        };

        let futs = self
            .transports
            .iter()
            .map(|t| t.send(pipeline, &routing_key, item));

        let results = join_all(futs).await;

        for result in results {
            if let Err(e) = result {
                tracing::warn!(pipeline, routing_key, "Transport send failed: {:?}", e);
            }
        }
    }
}
