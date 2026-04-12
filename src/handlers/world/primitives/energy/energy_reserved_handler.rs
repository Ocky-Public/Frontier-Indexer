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
use crate::models::world::StoredEnergyReserved;

use crate::AppContext;

pub struct EnergyReservedHandler {
    ctx: AppContext,
    package_set: HashSet<AccountAddress>,
}

impl EnergyReservedHandler {
    pub fn new(ctx: &AppContext) -> Self {
        let package_set: HashSet<AccountAddress> = ctx
            .get_world_package_strings()
            .iter()
            .filter_map(|s| AccountAddress::from_str(s).ok())
            .collect();

        Self {
            ctx: ctx.clone(),
            package_set,
        }
    }

    fn is_energy_reserved(&self, event: &Event) -> bool {
        let module_name = "energy";
        let event_name = "EnergyReservedEvent";

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
impl Processor for EnergyReservedHandler {
    const NAME: &'static str = "energy_reserved_handler";
    type Value = StoredEnergyReserved;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> anyhow::Result<Vec<Self::Value>> {
        let mut results = vec![];

        for tx in &checkpoint.transactions {
            if !is_indexed_tx(tx, &checkpoint.object_set, &self.ctx) {
                continue;
            }

            let Some(events) = &tx.events else { continue };

            let base_meta = EventMeta::from_checkpoint_tx(checkpoint, tx);

            for (index, ev) in events.data.iter().enumerate() {
                if self.is_energy_reserved(ev) {
                    let meta = base_meta.with_index(index);
                    let event = StoredEnergyReserved::from_event(ev, &meta);
                    results.push(event);
                }
            }
        }

        Ok(results)
    }
}

#[async_trait]
impl Handler for EnergyReservedHandler {
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
        use crate::schema::indexer::events_energy_reserved::dsl::*;

        diesel::insert_into(events_energy_reserved)
            .values(batch)
            .on_conflict((event_id, occurred_at))
            .do_nothing()
            .execute(conn)
            .await?;

        Ok(batch.len())
    }
}
