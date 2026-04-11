use serde::Deserialize;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use sui_pg_db::Connection;
use sui_sdk_types::Address;
use sui_types::collection_types::Table;

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::schema::indexer::system_table_registry;

#[derive(Deserialize)]
pub struct MoveEnergyConfig {
    pub id: Address,
    pub assembly_energy: Table,
}

#[derive(Queryable, Insertable, Selectable, Clone, Debug)]
#[diesel(table_name = system_table_registry)]
pub struct StoredTableRecord {
    pub table_id: String,
    pub parent_id: String,
    pub package_id: String,
    pub module_name: String,
    pub struct_name: String,
    pub key_type: String,
    pub value_type: String,
    pub checkpoint_updated: i64,
}

pub struct TableRegistry {
    cache: RwLock<HashMap<String, Arc<StoredTableRecord>>>,
}

impl TableRegistry {
    pub async fn load_from_db(conn: &mut Connection<'_>) -> Self {
        use crate::schema::indexer::system_table_registry::dsl::*;

        // Inside your async function
        let records = system_table_registry
            .load::<StoredTableRecord>(conn)
            .await
            .expect("Failed to fetch records from table registry");

        let mut cache = HashMap::new();

        for record in records {
            cache.insert(record.table_id.clone(), Arc::new(record));
        }

        Self {
            cache: RwLock::new(cache),
        }
    }

    pub async fn add_table(
        &self,
        conn: &mut Connection<'_>,
        record: &StoredTableRecord,
    ) -> QueryResult<()> {
        use crate::schema::indexer::system_table_registry::dsl::*;

        diesel::insert_into(system_table_registry)
            .values(record)
            .execute(conn)
            .await?;

        let shared = Arc::new(record.clone());

        self.cache
            .write()
            .unwrap()
            .insert(shared.table_id.clone(), shared);

        Ok(())
    }

    pub fn get_record(&self, entry_owner_id: &str) -> Option<Arc<StoredTableRecord>> {
        let cache = self.cache.read().unwrap();
        cache.get(entry_owner_id).cloned()
    }

    pub fn contains(&self, entry_owner_id: &str) -> bool {
        let cache = self.cache.read().unwrap();
        cache.contains_key(entry_owner_id)
    }

    pub fn belongs_to_type(
        &self,
        entry_owner_id: &str,
        package_id: &str,
        module_name: &str,
        struct_name: &str,
    ) -> bool {
        let cache = self.cache.read().unwrap();

        if let Some(record) = cache.get(entry_owner_id) {
            return record.package_id == package_id
                && record.module_name == module_name
                && record.struct_name == struct_name;
        }

        false
    }

    pub fn belongs_to_parent(&self, entry_owner_id: &str, parent_id: &str) -> bool {
        let cache = self.cache.read().unwrap();

        if let Some(record) = cache.get(entry_owner_id) {
            return record.parent_id == parent_id;
        }

        false
    }
}
