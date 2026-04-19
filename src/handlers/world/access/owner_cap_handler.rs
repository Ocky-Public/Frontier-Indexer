use async_trait::async_trait;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::Arc;

use diesel::prelude::*;
use diesel::query_dsl::methods::FilterDsl;
use diesel::upsert::excluded;
use diesel_async::RunQueryDsl;

use sui_pg_db::FieldCount;
use sui_types::effects::{IDOperation, TransactionEffectsAPI};
use sui_types::object::Object;
use sui_types::storage::ObjectKey;

use sui_indexer_alt_framework::pipeline::sequential::Handler;
use sui_indexer_alt_framework::pipeline::Processor;
use sui_indexer_alt_framework::postgres::{Connection, Db};
use sui_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;

use crate::models::world::StoredOwnerCap;

use crate::AppContext;

pub struct OwnerCapHandler {
    ctx: AppContext,
}

impl OwnerCapHandler {
    pub fn new(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    fn is_owner_cap(&self, obj: &Object) -> bool {
        let module_name = "access";
        let struct_name = "OwnerCap";
        self.ctx.is_world_object(obj, module_name, struct_name)
    }
}

#[derive(FieldCount)]
pub enum OwnerCapAction {
    Upsert(StoredOwnerCap),
    Delete(String),
}

#[async_trait]
impl Processor for OwnerCapHandler {
    const NAME: &'static str = "owner_caps";
    type Value = OwnerCapAction;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> anyhow::Result<Vec<Self::Value>> {
        let mut results = vec![];
        let checkpoint_updated = checkpoint.summary.sequence_number as i64;

        for tx in &checkpoint.transactions {
            if !self.ctx.is_indexed_tx(tx, &checkpoint.object_set) {
                continue;
            }

            for change in &tx.effects.object_changes() {
                let object_id = change.id;

                match change.id_operation {
                    IDOperation::Created | IDOperation::None => {
                        let Some(version) = change.output_version else {
                            continue;
                        };

                        let key = ObjectKey(object_id, version);

                        let Some(obj) = checkpoint.object_set.get(&key) else {
                            continue;
                        };

                        if self.is_owner_cap(obj) {
                            let owner_cap = StoredOwnerCap::from_object(obj, checkpoint_updated);
                            results.push(OwnerCapAction::Upsert(owner_cap));
                        }
                    }
                    IDOperation::Deleted => {
                        results.push(OwnerCapAction::Delete(object_id.to_string()));
                    }
                }
            }
        }

        Ok(results)
    }
}

#[async_trait]
impl Handler for OwnerCapHandler {
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
        use crate::schema::indexer::owner_caps::dsl::*;

        let mut upsert_map: HashMap<String, &StoredOwnerCap> = HashMap::new();
        let mut to_delete = Vec::new();

        for action in batch {
            match action {
                OwnerCapAction::Upsert(owner_cap) => {
                    let entry = upsert_map.entry(owner_cap.id.clone());
                    match entry {
                        Entry::Occupied(mut entry) => {
                            if owner_cap.checkpoint_updated > entry.get().checkpoint_updated {
                                entry.insert(owner_cap);
                            }
                        }
                        Entry::Vacant(entry) => {
                            entry.insert(owner_cap);
                        }
                    }
                }
                OwnerCapAction::Delete(id_str) => to_delete.push(id_str.clone()),
            }
        }

        // Remove any updates for which deletions exist.
        upsert_map.retain(|obj_id, _| !to_delete.contains(obj_id));

        let final_values: Vec<&StoredOwnerCap> = upsert_map.into_values().collect();

        if !final_values.is_empty() {
            diesel::insert_into(owner_caps)
                .values(final_values)
                .on_conflict(id)
                .do_update()
                .set((
                    object_id.eq(excluded(object_id)),
                    owner_address.eq(excluded(owner_address)),
                    package_id.eq(excluded(package_id)),
                    module_name.eq(excluded(module_name)),
                    struct_name.eq(excluded(struct_name)),
                    checkpoint_updated.eq(excluded(checkpoint_updated)),
                ))
                .filter(checkpoint_updated.lt(excluded(checkpoint_updated)))
                .execute(conn)
                .await?;
        }

        // Deletions happen last in case an object was updated before deletion.
        if !to_delete.is_empty() {
            diesel::delete(owner_caps)
                .filter(id.eq_any(to_delete))
                .execute(conn)
                .await?;
        }

        Ok(batch.len())
    }
}
