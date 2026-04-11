use async_trait::async_trait;
use std::collections::HashSet;
use std::str::FromStr;
use std::sync::Arc;

use diesel_async::RunQueryDsl;

use sui_types::event::Event;

use move_core_types::account_address::AccountAddress;
use sui_indexer_alt_framework::pipeline::sequential::Handler;
use sui_indexer_alt_framework::pipeline::Processor;
use sui_indexer_alt_framework::postgres::{Connection, Db};
use sui_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;

use crate::handlers::is_indexed_tx;
use crate::handlers::EventMeta;
use crate::models::world::StoredCharacterCreated;

use crate::AppContext;

pub struct CharacterCreatedHandler {
    ctx: AppContext,
    package_set: HashSet<AccountAddress>,
}

impl CharacterCreatedHandler {
    pub fn new(ctx: &AppContext) -> Self {
        let package_set: HashSet<AccountAddress> = ctx
            .get_world_package_strings()
            .iter()
            .filter_map(|s| AccountAddress::from_str(s).ok())
            .collect();

        Self { ctx: ctx.clone(), package_set }
    }

    fn is_character_created(&self, event: &Event) -> bool {
        let module_name = "character";
        let event_name = "CharacterCreatedEvent";

        let tag = &event.type_;

        if !self.package_set.contains(&tag.address) {
            return false;
        }

        if tag.module.as_str() != module_name {
            return false;
        }

        if tag.name.as_str() != event_name {
            return false;
        }

        true
    }
}

#[async_trait]
impl Processor for CharacterCreatedHandler {
    const NAME: &'static str = "charater_created_handler";
    type Value = StoredCharacterCreated;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> anyhow::Result<Vec<Self::Value>> {
        let mut results = vec![];

        for tx in &checkpoint.transactions {
            if !is_indexed_tx(tx, &checkpoint.object_set, &self.ctx) {
                continue;
            }

            let Some(events) = &tx.events else { continue };

            let base_meta = EventMeta::from_checkpoint_tx(checkpoint, tx);

            for (index, ev) in events.data.iter().enumerate() {
                if self.is_character_created(ev) {
                    let meta = base_meta.with_index(index);
                    let event = StoredCharacterCreated::from_event(ev, &meta);
                    results.push(event);
                }
            }
        }

        Ok(results)
    }
}

#[async_trait]
impl Handler for CharacterCreatedHandler {
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
        use crate::schema::indexer::events_character_created::dsl::*;

        diesel::insert_into(events_character_created)
            .values(batch)
            .on_conflict((event_id, occurred_at))
            .do_nothing()
            .execute(conn)
            .await?;

        Ok(batch.len())
    }
}
