use async_trait::async_trait;
use serde::Serialize;

#[async_trait]
pub trait Transport<I>: Send + Sync + 'static
where
    I: Serialize + Send + Sync,
{
    async fn send(&self, pipeline: &'static str, routing_key: &str, item: &I)
        -> anyhow::Result<()>;
}
