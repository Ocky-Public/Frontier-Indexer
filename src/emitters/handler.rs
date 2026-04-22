use async_trait::async_trait;
use std::sync::Arc;

use sui_indexer_alt_framework::pipeline::sequential::Handler;
use sui_indexer_alt_framework::pipeline::Processor;
use sui_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;
use sui_indexer_alt_framework_store_traits::Store;

use crate::emitters::PipelineEmitter;

pub struct EmittingHandler<H>
where
    H: Handler,
    H::Batch: Clone + Send + Sync + 'static,
{
    inner: H,
    emitter: Arc<dyn PipelineEmitter<H::Batch>>,
}

impl<H> EmittingHandler<H>
where
    H: Handler,
    H::Batch: Clone + Send + Sync + 'static,
{
    pub fn new(inner: H, emitter: Arc<dyn PipelineEmitter<H::Batch>>) -> Self {
        Self { inner, emitter }
    }
}

#[async_trait]
impl<H> Processor for EmittingHandler<H>
where
    H: Handler,
    H::Batch: Clone + Send + Sync + 'static,
{
    const NAME: &'static str = H::NAME;
    type Value = H::Value;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> anyhow::Result<Vec<Self::Value>> {
        self.inner.process(checkpoint).await
    }
}

#[async_trait]
impl<H> Handler for EmittingHandler<H>
where
    H: Handler + Send + Sync + 'static,
    H::Batch: Clone + Default + Send + Sync + 'static,
{
    type Store = H::Store;
    type Batch = H::Batch;

    fn batch(&self, batch: &mut Self::Batch, values: std::vec::IntoIter<Self::Value>) {
        self.inner.batch(batch, values);
    }

    async fn commit<'a>(
        &self,
        batch: &Self::Batch,
        conn: &mut <Self::Store as Store>::Connection<'a>,
    ) -> anyhow::Result<usize> {
        self.inner.commit(batch, conn).await
    }

    async fn post_commit(&self, batch: &Self::Batch) {
        let batch_clone = batch.clone();
        let emitter = Arc::clone(&self.emitter);
        tokio::spawn(async move {
            emitter.emit(H::NAME, &batch_clone).await;
        });
    }
}
