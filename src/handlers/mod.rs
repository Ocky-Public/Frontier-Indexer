use crate::AppEnv;
use std::sync::Arc;
use sui_indexer_alt_framework::types::full_checkpoint_content::{
    Checkpoint, ExecutedTransaction, ObjectSet,
};
use sui_types::effects::TransactionEffectsAPI;
use sui_types::transaction::{Command, TransactionDataAPI};

/// Captures common transaction metadata for event processing.
pub struct EventMeta {
    digest: Arc<str>,
    sender: Arc<str>,
    checkpoint: i64,
    checkpoint_timestamp_ms: i64,
    package: Arc<str>,
    event_index: usize,
}

impl EventMeta {
    pub fn from_checkpoint_tx(checkpoint: &Checkpoint, tx: &ExecutedTransaction) -> Self {
        Self {
            digest: tx.effects.transaction_digest().to_string().into(),
            sender: tx.transaction.sender().to_string().into(),
            checkpoint: checkpoint.summary.sequence_number as i64,
            checkpoint_timestamp_ms: checkpoint.summary.timestamp_ms as i64,
            package: try_extract_move_call_package(tx).unwrap_or_default().into(),
            event_index: 0,
        }
    }

    pub fn with_index(&self, index: usize) -> Self {
        Self {
            digest: Arc::clone(&self.digest),
            sender: Arc::clone(&self.sender),
            checkpoint: self.checkpoint,
            checkpoint_timestamp_ms: self.checkpoint_timestamp_ms,
            package: Arc::clone(&self.package),
            event_index: index,
        }
    }

    pub fn event_digest(&self) -> String {
        format!("{}{}", self.digest, self.event_index)
    }

    pub fn digest(&self) -> String {
        self.digest.to_string()
    }

    pub fn sender(&self) -> String {
        self.sender.to_string()
    }

    pub fn checkpoint(&self) -> i64 {
        self.checkpoint
    }

    pub fn checkpoint_timestamp_ms(&self) -> i64 {
        self.checkpoint_timestamp_ms
    }

    pub fn package(&self) -> String {
        self.package.to_string()
    }
}

// List of all handlers
pub mod world;

pub use world::access::owner_cap_handler::*;
pub use world::characters::character_created_handler::*;
pub use world::characters::character_handler::*;

pub(crate) fn is_indexed_tx(
    tx: &ExecutedTransaction,
    checkpoint_objects: &ObjectSet,
    env: AppEnv,
) -> bool {
    let app_addresses = env.package_addresses();
    let app_packages = env.package_ids();

    // Check input object against all known package versions
    let has_app_input = tx.input_objects(checkpoint_objects).any(|obj| {
        obj.data
            .type_()
            .map(|t| app_addresses.iter().any(|addr| t.address() == *addr))
            .unwrap_or_default()
    });

    if has_app_input {
        return true;
    }

    // Check if transaction has application events from any version
    if let Some(events) = &tx.events {
        let has_app_event = events.data.iter().any(|event| {
            app_addresses
                .iter()
                .any(|addr| event.type_.address == *addr)
        });

        if has_app_event {
            return true;
        }
    }

    // Check if transaction calls a application function from any version
    let txn_kind = tx.transaction.kind();

    let has_app_call = txn_kind.iter_commands().any(|cmd| {
        if let Command::MoveCall(move_call) = cmd {
            app_packages.iter().any(|pkg| *pkg == move_call.package)
        } else {
            false
        }
    });

    has_app_call
}

pub(crate) fn try_extract_move_call_package(tx: &ExecutedTransaction) -> Option<String> {
    let txn_kind = tx.transaction.kind();
    let first_command = txn_kind.iter_commands().next()?;
    if let Command::MoveCall(move_call) = first_command {
        Some(move_call.package.to_string())
    } else {
        None
    }
}
