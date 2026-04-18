# System Architecture

The indexer is designed to process Sui blockchain checkpoints and project the state into a relational PostgreSQL database.

## Framework
The project is built on top of [`sui-indexer-alt-framework`](https://github.com/MystenLabs/sui), which provides the core logic for:
- **Ingestion**: Fetching full checkpoint data (transactions, effects, events, object changes) from a remote Sui checkpoint store.
- **Sequential Pipelines**: An ordered processing system that guarantees checkpoints are committed in sequence. This ensures the database always reflects a consistent, monotonically advancing state.
- **Checkpointing**: Tracking the last committed checkpoint so the indexer can resume exactly where it left off after a restart.

## Data Flow

```
Sui Checkpoint Store
        |
        v
  Ingestion Client        fetches full checkpoint content
        |
        v
  Sequential Pipeline     distributes to registered Handlers
        |
        v
    Handlers              filter and transform data
        |
        v
    PostgreSQL            final state stored via Diesel
```

1. **Ingestion Client**: Pulls checkpoint bundles from a remote store (e.g. `https://checkpoints.testnet.sui.io`) or a local path in sandbox mode.
2. **Sequential Pipeline**: Receives each checkpoint and fans it out to all registered handlers. Batches results and manages the commit cycle. Configured via the `Sequential` and `Ingestion` settings.
3. **Handlers**: Each handler implements a `Processor` (filtering and transformation) and a `Handler` (database commit) trait. See [World Contracts Integration](./world_contracts.md) for details on how handlers are structured.
4. **Database**: PostgreSQL, managed through Diesel. Schema is defined by migrations that run automatically at startup. See [Database and Models](./database.md).

## Context

The `AppContext` struct (`src/lib.rs`) is constructed once at startup and shared (cloned) across all handlers. It holds:
- The current network environment (`Mainnet` / `Testnet`).
- The set of known world and app package addresses.
- The `TableRegistry` — an in-memory cache of Move `Table` object IDs mapped to their parent structs, used to index table entries.
- The `FuelRegistry` — a cached lookup of fuel efficiency values used during processing.

## Monitoring
The system exposes Prometheus metrics via a dedicated `MetricsService` on `0.0.0.0:9184` by default, including database connection pool statistics.
