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
use sui_types::object::Owner;
use sui_types::storage::ObjectKey;

use sui_indexer_alt_framework::pipeline::sequential::Handler;
use sui_indexer_alt_framework::pipeline::Processor;
use sui_indexer_alt_framework::postgres::{Connection, Db};
use sui_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;

use crate::models::world::StoredExtensionFreeze;
use crate::models::world::StoredGate;

use crate::AppContext;

pub struct GateHandler {
    ctx: AppContext,
}

impl GateHandler {
    pub fn new(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    fn is_gate(&self, obj: &Object) -> bool {
        let module_name = "gate";
        let struct_name = "Gate";
        self.ctx.is_world_object(obj, module_name, struct_name)
    }

    fn is_gate_extension_freeze(
        &self,
        obj: &Object,
        gates: &HashMap<String, Arc<StoredGate>>,
    ) -> Option<Arc<StoredGate>> {
        let key_module = "extension_freeze";
        let key_struct = "ExtensionFrozenKey";

        let value_module = "extension_freeze";
        let value_struct = "ExtensionFrozen";

        let Some(move_type) = obj.type_() else {
            return None;
        };

        if !move_type.is_dynamic_field() || move_type.type_params().len() != 2 {
            return None;
        }

        if !self
            .ctx
            .is_world_struct(move_type.type_params()[0].as_ref(), key_module, key_struct)
        {
            return None;
        }

        if !self.ctx.is_world_struct(
            move_type.type_params()[1].as_ref(),
            value_module,
            value_struct,
        ) {
            return None;
        }

        let Owner::ObjectOwner(owner_str) = obj.owner else {
            return None;
        };

        let owner_id = owner_str.to_string();

        // CHeck the entry again gates in the same checkpoint.
        gates.get(&owner_id).cloned()
    }
}

#[derive(FieldCount)]
pub enum GateAction {
    Upsert(StoredGate),
    Freeze(StoredExtensionFreeze),
    Delete(String),
}

#[async_trait]
impl Processor for GateHandler {
    const NAME: &'static str = "gates";
    type Value = GateAction;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> anyhow::Result<Vec<Self::Value>> {
        let mut results = vec![];
        let checkpoint_updated = checkpoint.summary.sequence_number as i64;

        let mut gates = HashMap::new();

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

                        if self.is_gate(obj) {
                            let gate = StoredGate::from_object(obj, checkpoint_updated);
                            gates.insert(gate.id.clone(), Arc::new(gate.clone()));
                            results.push(GateAction::Upsert(gate));
                        }

                        if let Some(gate) = self.is_gate_extension_freeze(obj, &gates) {
                            let freeze = StoredExtensionFreeze::from_object(obj, gate);
                            results.push(GateAction::Freeze(freeze));
                        }
                    }
                    IDOperation::Deleted => {
                        results.push(GateAction::Delete(object_id.to_string()));
                    }
                }
            }
        }

        Ok(results)
    }
}

#[async_trait]
impl Handler for GateHandler {
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
        let mut upsert_map: HashMap<String, &StoredGate> = HashMap::new();
        let mut to_freeze = Vec::new();
        let mut to_delete = Vec::new();

        for action in batch {
            match action {
                GateAction::Upsert(gate) => {
                    let entry = upsert_map.entry(gate.id.clone());

                    match entry {
                        Entry::Occupied(mut entry) => {
                            if gate.checkpoint_updated > entry.get().checkpoint_updated {
                                entry.insert(gate);
                            }
                        }
                        Entry::Vacant(entry) => {
                            entry.insert(gate);
                        }
                    }
                }
                GateAction::Freeze(freeze) => to_freeze.push(freeze),
                GateAction::Delete(id_str) => to_delete.push(id_str.clone()),
            }
        }

        // Remove any updates for which deletions exist.
        upsert_map.retain(|obj_id, _| !to_delete.contains(obj_id));

        let final_values: Vec<&StoredGate> = upsert_map.into_values().collect();

        if !final_values.is_empty() {
            use crate::schema::indexer::gates::dsl::*;
            diesel::insert_into(gates)
                .values(final_values)
                .on_conflict(id)
                .do_update()
                .set((
                    item_id.eq(item_id),
                    tenant.eq(excluded(tenant)),
                    type_id.eq(excluded(type_id)),
                    owner_cap_id.eq(excluded(owner_cap_id)),
                    location.eq(excluded(location)),
                    status.eq(excluded(status)),
                    energy_source_id.eq(excluded(energy_source_id)),
                    linked_id.eq(excluded(linked_id)),
                    name.eq(excluded(name)),
                    description.eq(excluded(description)),
                    url.eq(excluded(url)),
                    package_id.eq(excluded(package_id)),
                    module_name.eq(excluded(module_name)),
                    struct_name.eq(excluded(struct_name)),
                    checkpoint_updated.eq(excluded(checkpoint_updated)),
                ))
                .filter(checkpoint_updated.lt(excluded(checkpoint_updated)))
                .execute(conn)
                .await?;
        }

        if !to_freeze.is_empty() {
            use crate::schema::indexer::extension_freezes::dsl::*;
            diesel::insert_into(extension_freezes)
                .values(to_freeze)
                .on_conflict(id)
                .do_nothing()
                .execute(conn)
                .await?;
        }

        if !to_delete.is_empty() {
            use crate::schema::indexer::{extension_freezes, gates};

            diesel::delete(gates::dsl::gates)
                .filter(gates::dsl::id.eq_any(to_delete.clone()))
                .execute(conn)
                .await?;

            diesel::delete(extension_freezes::dsl::extension_freezes)
                .filter(extension_freezes::dsl::id.eq_any(to_delete))
                .execute(conn)
                .await?;
        }

        Ok(batch.len())
    }
}
