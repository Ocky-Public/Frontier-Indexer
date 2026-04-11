use async_trait::async_trait;
use move_core_types::account_address::AccountAddress;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
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

use crate::handlers::is_indexed_tx;
use crate::models::world::StoredAssembly;

use crate::AppContext;

pub struct AssemblyHandler {
    ctx: AppContext,
    package_set: HashSet<AccountAddress>,
}

impl AssemblyHandler {
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

    fn is_assembly(&self, obj: &Object) -> bool {
        let module_name = "assembly";
        let struct_name = "Assembly";

        if let Some(move_type) = obj.type_() {
            if let Some(tag) = move_type.other() {
                if !self.package_set.contains(&tag.address) {
                    return false;
                }

                if tag.module.as_str() != module_name {
                    return false;
                }

                if tag.name.as_str() != struct_name {
                    return false;
                }

                return true;
            }
        }

        false
    }
}
#[derive(FieldCount)]
pub enum AssemblyAction {
    Upsert(StoredAssembly),
    Delete(String),
}

#[async_trait]
impl Processor for AssemblyHandler {
    const NAME: &'static str = "assembly_handler";
    type Value = AssemblyAction;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> anyhow::Result<Vec<Self::Value>> {
        let mut results = vec![];
        let cp_sequence = checkpoint.summary.sequence_number as i64;

        for tx in &checkpoint.transactions {
            if !is_indexed_tx(tx, &checkpoint.object_set, &self.ctx) {
                continue;
            }

            for change in &tx.effects.object_changes() {
                let object_id = change.id;

                match change.id_operation {
                    IDOperation::Created | IDOperation::None => {
                        if let Some(version) = change.output_version {
                            let key = ObjectKey(object_id, version);

                            if let Some(obj) = checkpoint.object_set.get(&key) {
                                if self.is_assembly(obj) {
                                    let assembly = StoredAssembly::from_object(obj, cp_sequence);
                                    results.push(AssemblyAction::Upsert(assembly));
                                }
                            }
                        }
                    }
                    IDOperation::Deleted => {
                        results.push(AssemblyAction::Delete(object_id.to_string()));
                    }
                }
            }
        }

        Ok(results)
    }
}

#[async_trait]
impl Handler for AssemblyHandler {
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
        use crate::schema::indexer::assemblies::dsl::*;

        let mut upsert_map: HashMap<String, &StoredAssembly> = HashMap::new();
        let mut to_delete = Vec::new();

        for action in batch {
            match action {
                AssemblyAction::Upsert(assembly) => {
                    let entry = upsert_map.entry(assembly.id.clone());

                    match entry {
                        Entry::Occupied(mut entry) => {
                            if assembly.checkpoint_updated > entry.get().checkpoint_updated {
                                entry.insert(assembly);
                            }
                        }
                        Entry::Vacant(entry) => {
                            entry.insert(assembly);
                        }
                    }
                }
                AssemblyAction::Delete(id_str) => to_delete.push(id_str.clone()),
            }
        }

        // Remove any updates for which deletions exist.
        upsert_map.retain(|obj_id, _| !to_delete.contains(obj_id));

        let final_values: Vec<&StoredAssembly> = upsert_map.into_values().collect();

        if !final_values.is_empty() {
            diesel::insert_into(assemblies)
                .values(final_values)
                .on_conflict(id)
                .do_update()
                .set((
                    item_id.eq(excluded(item_id)),
                    tenant.eq(excluded(tenant)),
                    type_id.eq(excluded(type_id)),
                    owner_cap_id.eq(excluded(owner_cap_id)),
                    location.eq(excluded(location)),
                    status.eq(excluded(status)),
                    energy_source_id.eq(energy_source_id),
                    name.eq(excluded(name)),
                    description.eq(excluded(description)),
                    url.eq(excluded(url)),
                    checkpoint_updated.eq(excluded(checkpoint_updated)),
                ))
                .filter(checkpoint_updated.lt(excluded(checkpoint_updated)))
                .execute(conn)
                .await?;
        }

        Ok(batch.len())
    }
}
