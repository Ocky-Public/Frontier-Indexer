use serde::Serialize;

use sui_indexer_alt_framework::FieldCount;

#[derive(Serialize, Debug, Clone, FieldCount)]
pub struct StoredTransactionDigest {
    pub digest: String,
    pub checkpoint: i64,
}
