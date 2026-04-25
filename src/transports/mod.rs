use async_trait::async_trait;
use serde::Serialize;

#[async_trait]
pub trait Transport<I>: Send + Sync + 'static
where
    I: Serialize + Send + Sync,
{
    fn id(&self) -> String;

    async fn send(&self, pipeline: &'static str, item: &I) -> anyhow::Result<()>;
}
