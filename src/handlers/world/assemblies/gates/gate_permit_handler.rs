use async_trait::async_trait;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use std::sync::Arc;

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use sui_pg_db::FieldCount;
use sui_types::effects::{IDOperation, TransactionEffectsAPI};
use sui_types::object::Object;
use sui_types::storage::ObjectKey;

use sui_indexer_alt_framework::pipeline::sequential::Handler;
use sui_indexer_alt_framework::pipeline::Processor;
use sui_indexer_alt_framework::postgres::{Connection, Db};
use sui_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;

use crate::models::world::StoredGatePermit;

use crate::AppContext;

pub struct GatePermitHandler {
    ctx: AppContext,
}

impl GatePermitHandler {
    pub fn new(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    fn is_gate_permit(&self, obj: &Object) -> bool {
        let module_name = "gate";
        let struct_name = "JumpPermit";
        self.ctx.is_world_object(obj, module_name, struct_name)
    }
}

#[derive(FieldCount)]
pub enum GatePermitAction {
    Upsert(StoredGatePermit),
    Delete(String),
}

#[async_trait]
impl Processor for GatePermitHandler {
    const NAME: &'static str = "gate_permits";
    type Value = GatePermitAction;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> anyhow::Result<Vec<Self::Value>> {
        let mut results = vec![];

        for tx in &checkpoint.transactions {
            if !self.ctx.is_indexed_tx(tx, &checkpoint.object_set) {
                continue;
            }

            for change in &tx.effects.object_changes() {
                let object_id = change.id;

                match change.id_operation {
                    IDOperation::Created => {
                        let Some(version) = change.output_version else {
                            continue;
                        };

                        let key = ObjectKey(object_id, version);

                        let Some(obj) = checkpoint.object_set.get(&key) else {
                            continue;
                        };

                        if self.is_gate_permit(obj) {
                            let permit = StoredGatePermit::from_object(obj);
                            results.push(GatePermitAction::Upsert(permit));
                        }
                    }
                    IDOperation::None => {} // Permits are immutable, no need to handle updates.
                    IDOperation::Deleted => {
                        results.push(GatePermitAction::Delete(object_id.to_string()));
                    }
                }
            }
        }

        Ok(results)
    }
}

#[async_trait]
impl Handler for GatePermitHandler {
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
        use crate::schema::indexer::gate_permits::dsl::*;

        let mut to_upsert: HashMap<String, &StoredGatePermit> = HashMap::new();
        let mut to_delete: HashSet<String> = HashSet::new();

        for action in batch {
            match action {
                GatePermitAction::Upsert(permit) => {
                    let entry = to_upsert.entry(permit.id.clone());

                    match entry {
                        Entry::Occupied(mut _entry) => {}
                        Entry::Vacant(entry) => {
                            entry.insert(permit);
                        }
                    }
                }
                GatePermitAction::Delete(id_str) => {
                    to_delete.insert(id_str.clone());
                }
            }
        }

        // Remove any updates for which deletions exist.
        to_upsert.retain(|obj_id, _| !to_delete.contains(obj_id));

        let final_values: Vec<&StoredGatePermit> = to_upsert.into_values().collect();

        if !final_values.is_empty() {
            diesel::insert_into(gate_permits)
                .values(final_values)
                .on_conflict(id)
                .do_nothing()
                .execute(conn)
                .await?;
        }

        if !to_delete.is_empty() {
            diesel::delete(gate_permits)
                .filter(id.eq_any(to_delete))
                .execute(conn)
                .await?;
        }

        Ok(batch.len())
    }
}
