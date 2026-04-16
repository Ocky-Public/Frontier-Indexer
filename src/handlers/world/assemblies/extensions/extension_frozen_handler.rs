use async_trait::async_trait;
use std::sync::Arc;

use diesel_async::RunQueryDsl;

use sui_types::event::Event;

use sui_indexer_alt_framework::pipeline::sequential::Handler;
use sui_indexer_alt_framework::pipeline::Processor;
use sui_indexer_alt_framework::postgres::{Connection, Db};
use sui_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;

use crate::handlers::EventMeta;
use crate::models::world::StoredExtensionFrozen;

use crate::AppContext;

pub struct ExtensionFrozenHandler {
    ctx: AppContext,
}

impl ExtensionFrozenHandler {
      pub fn new(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    fn is_extension_frozen(&self, event: &Event) -> bool {
        let module_name = "extension_freeze";
        let event_name = "ExtensionConfigFrozenEvent";
        self.ctx.is_world_event(event, module_name, event_name)
    }
}

#[async_trait]
impl Processor for ExtensionFrozenHandler {
    const NAME: &'static str = "extension_frozen";
    type Value = StoredExtensionFrozen;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> anyhow::Result<Vec<Self::Value>> {
        let mut results = vec![];

        for tx in &checkpoint.transactions {
            if !self.ctx.is_indexed_tx(tx, &checkpoint.object_set) {
                continue;
            }

            let Some(events) = &tx.events else { continue };

            let base_meta = EventMeta::from_checkpoint_tx(checkpoint, tx);

            for (index, ev) in events.data.iter().enumerate() {
                if self.is_extension_frozen(ev) {
                    let meta = base_meta.with_index(index);
                    let event = StoredExtensionFrozen::from_event(ev, &meta);
                    results.push(event);
                }
            }
        }

        Ok(results)
    }
}

#[async_trait]
impl Handler for ExtensionFrozenHandler {
    type Store = Db;
    type Batch = Vec<Self::Value>;

    fn batch(&self, batch: &mut Self::Batch, values: std::vec::IntoIter<Self::Value>) {
        batch.extend(values);
    }

    async fn commit<'a>(
        &self,
        batch: &Self::Batch,
        conn: &mut Connection<'a>,
    ) -> anyhow::Result<usize> {
        use crate::schema::indexer::events_extension_frozen::dsl::*;

        diesel::insert_into(events_extension_frozen)
            .values(batch)
            .on_conflict((event_id, occurred_at))
            .do_nothing()
            .execute(conn)
            .await?;

        Ok(batch.len())
    }
}
