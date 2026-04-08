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
use crate::models::StoredCharacter;
use crate::AppEnv;

pub struct CharacterHandler {
    env: AppEnv,
    package_set: HashSet<AccountAddress>,
}

impl CharacterHandler {
    pub fn new(env: AppEnv) -> Self {
        let package_set: HashSet<AccountAddress> = env
            .get_world_package_strings()
            .iter()
            .filter_map(|s| AccountAddress::from_str(s).ok())
            .collect();

        Self { env, package_set }
    }

    fn is_character(&self, obj: &Object) -> bool {
        let module_name = "character";
        let struct_name = "Character";

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
pub enum CharacterAction {
    Upsert(StoredCharacter),
    Delete(String),
}

#[async_trait]
impl Processor for CharacterHandler {
    const NAME: &'static str = "character_handler";
    type Value = CharacterAction;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> anyhow::Result<Vec<Self::Value>> {
        let mut results = vec![];
        let cp_sequence = checkpoint.summary.sequence_number as i64;

        for tx in &checkpoint.transactions {
            if !is_indexed_tx(tx, &checkpoint.object_set, self.env) {
                continue;
            }

            for change in &tx.effects.object_changes() {
                let object_id = change.id;

                match change.id_operation {
                    IDOperation::Created | IDOperation::None => {
                        if let Some(version) = change.output_version {
                            let key = ObjectKey(object_id, version);

                            if let Some(obj) = checkpoint.object_set.get(&key) {
                                if self.is_character(obj) {
                                    let character = StoredCharacter::from_object(obj, cp_sequence);
                                    results.push(CharacterAction::Upsert(character));
                                }
                            }
                        }
                    }
                    IDOperation::Deleted => {
                        results.push(CharacterAction::Delete(object_id.to_string()));
                    }
                }
            }
        }

        Ok(results)
    }
}

#[async_trait]
impl Handler for CharacterHandler {
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
        use crate::schema::indexer::characters::dsl::*;

        let mut upsert_map: HashMap<String, &StoredCharacter> = HashMap::new();
        let mut to_delete = Vec::new();

        for action in batch {
            match action {
                CharacterAction::Upsert(character) => {
                    let entry = upsert_map.entry(character.id.clone());
                    match entry {
                        Entry::Occupied(mut entry) => {
                            if character.checkpoint_updated > entry.get().checkpoint_updated {
                                entry.insert(character);
                            }
                        }
                        Entry::Vacant(e) => {
                            e.insert(character);
                        }
                    }
                }
                CharacterAction::Delete(id_str) => to_delete.push(id_str.clone()),
            }
        }

        // Remove any updates for which deletions exist.
        upsert_map.retain(|obj_id, _| !to_delete.contains(obj_id));

        let final_values: Vec<&StoredCharacter> = upsert_map.into_values().collect();

        if !final_values.is_empty() {
            diesel::insert_into(characters)
                .values(final_values)
                .on_conflict(id)
                .do_update()
                .set((
                    item_id.eq(excluded(item_id)),
                    tenant.eq(excluded(tenant)),
                    owner_cap_id.eq(excluded(owner_cap_id)),
                    owner_address.eq(excluded(owner_address)),
                    tribe_id.eq(excluded(tribe_id)),
                    name.eq(excluded(name)),
                    description.eq(excluded(description)),
                    url.eq(excluded(url)),
                    checkpoint_updated.eq(excluded(checkpoint_updated)),
                ))
                .filter(checkpoint_updated.lt(excluded(checkpoint_updated)))
                .execute(conn)
                .await?;
        }

        // Deletions happen last incase an object was updated before deletion.
        if !to_delete.is_empty() {
            diesel::delete(characters)
                .filter(id.eq_any(to_delete))
                .execute(conn)
                .await?;
        }

        // Todo: Broadcast events for all of the above entry changes via websocket.

        Ok(batch.len())
    }
}
