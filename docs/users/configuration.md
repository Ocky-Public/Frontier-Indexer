# Container Configuration

This guide describes how to configure the indexer container using environment variables. The indexer requires a [TimescaleDB](https://www.timescale.com/) database (`timescale/timescaledb-ha:pg17` is the recommended container).

For the repository's local compose workflow, copy `.env.sample` to `.env` and start the stack with `./scripts/compose.sh up -d`. If `.env` is missing, the helper script will create it from `.env.sample` automatically. The compose file passes the same environment variables documented here and overrides `DB_HOST` to `db` so the indexer connects to the database service over the compose network.

## Database Configuration

These variables are used to connect the indexer to your TimescaleDB database.

| Variable                   | Description                                                                                                                              | Default     |
| -------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- | ----------- |
| `DB_USER`                  | Database username                                                                                                                        | `postgres`  |
| `DB_PASSWORD`              | Database password                                                                                                                        | `postgres`  |
| `DB_HOST`                  | Database host address                                                                                                                    | `localhost` |
| `DB_PORT`                  | Database port                                                                                                                            | `5432`      |
| `DB_NAME`                  | Database name                                                                                                                            | `postgres`  |
| `DB_SCHEMA`                | Database schema to use                                                                                                                   | `indexer`   |
| `DB_CONNECTION_POOL_SIZE`  | Maximum number of concurrent database connections                                                                                        | `100`       |
| `DB_CONNECTION_TIMEOUT_MS` | How long to wait when acquiring a connection from the pool before giving up, in milliseconds                                             | `60000`     |
| `DB_STATEMENT_TIMEOUT_MS`  | Maximum time a single database statement is allowed to run before being cancelled, in milliseconds. Useful for catching runaway queries. | (None)      |
| `DB_TLS_VERIFY_CERT`       | Whether to verify the database server's TLS certificate                                                                                  | `false`     |
| `DB_TLS_CA_CERT_PATH`      | Path to a custom TLS CA certificate file. Required when connecting to a database with a self-signed or private CA certificate.           | (None)      |

## General Indexer Settings

Control which network and data the indexer targets.

| Variable           | Description                                                                                                                                    | Default                                                        |
| ------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------- |
| `SUI_NETWORK`      | The Sui network to index (`mainnet` or `testnet`)                                                                                              | `testnet`                                                      |
| `PACKAGES`         | Comma-separated list of data groups to index. `world` indexes the EVE Frontier world contracts. `app` is reserved for custom application data. | `app,world`                                                    |
| `FIRST_CHECKPOINT` | Start indexing from this checkpoint sequence number. Useful for backfilling historical data.                                                   | (None — starts from the beginning or last committed watermark) |
| `LAST_CHECKPOINT`  | Stop indexing after reaching this checkpoint sequence number.                                                                                  | (None — runs continuously)                                     |
| `PIPELINES`        | Comma-separated list of pipeline names to run. When set, only the named pipelines are active and all others are skipped.                       | (None — all pipelines run)                                     |

## Performance Tuning

These settings control how the indexer batches and commits data. The defaults are a reasonable starting point; adjust them if you are seeing lag or high database load.

### Sequential Pipeline

Controls how checkpoints are gathered and written to the database.

| Variable                       | Description                                                                                                                                               | Default |
| ------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------- | ------- |
| `CHECKPOINT_LAG`               | Number of checkpoints to stay behind the chain tip. Setting this above `0` adds a buffer that helps avoid reprocessing in the event of short chain forks. | `0`     |
| `WRITE_CONCURRENCY`            | Number of pipeline committers that can write to the database simultaneously.                                                                              | `5`     |
| `COLLECT_INTERVAL_MS`          | How frequently the pipeline collects processed results into a batch, in milliseconds.                                                                     | `500`   |
| `WATERMARK_INTERVAL_MS`        | How frequently the pipeline updates its progress watermark in the database, in milliseconds.                                                              | `500`   |
| `WATERMARK_INTERVAL_JITTER_MS` | Random jitter added to the watermark interval to spread out writes when running multiple indexer instances.                                               | `0`     |
| `MIN_EAGER_ROWS`               | Minimum number of rows in a batch before the pipeline will commit early without waiting for `COLLECT_INTERVAL_MS`.                                        | (None)  |
| `MAX_BATCH_CHECKPOINTS`        | Maximum number of checkpoints that can be grouped into a single commit batch.                                                                             | (None)  |
| `PROCESSOR_CHANNEL_SIZE`       | Size of the internal channel between the processor and committer stages. Increasing this allows more checkpoints to be processed ahead of commits.        | (None)  |

### Ingestion

Controls how checkpoint data is fetched from the network.

| Variable                               | Description                                                                       | Default |
| -------------------------------------- | --------------------------------------------------------------------------------- | ------- |
| `CHECKPOINT_BUFFER_SIZE`               | Number of checkpoints to buffer in memory after fetching but before processing.   | `50`    |
| `RETRY_INTERVAL_MS`                    | How long to wait before retrying a failed checkpoint fetch, in milliseconds.      | `200`   |
| `STREAMING_BACKOFF_INITIAL_BATCH_SIZE` | Starting batch size for the streaming backoff strategy when fetching checkpoints. | `10`    |
| `STREAMING_BACKOFF_MAX_BATCH_SIZE`     | Maximum batch size the streaming backoff strategy will grow to.                   | `10000` |
| `STREAMING_CONNECTION_TIMEOUT_MS`      | Timeout for establishing a connection to the checkpoint stream, in milliseconds.  | `5000`  |
| `STREAMING_STATEMENT_TIMEOUT_MS`       | Timeout for individual requests made to the checkpoint stream, in milliseconds.   | `5000`  |

## Sandbox Mode

Sandbox mode is for testing and development. It allows the indexer to run against custom package IDs instead of the hardcoded network addresses, and can source checkpoint data locally rather than from the network.

| Variable                 | Description                                                                                                                        | Default    |
| ------------------------ | ---------------------------------------------------------------------------------------------------------------------------------- | ---------- |
| `SANDBOX`                | Enable sandbox mode. Requires `SANDBOX_APP_PACKAGES` to also be set.                                                               | `false`    |
| `SANDBOX_NETWORK`        | The network environment for sandbox mode (`localnet` or `testnet`).                                                                | `localnet` |
| `SANDBOX_APP_PACKAGES`   | Comma-separated list of App package IDs to track. Required when `SANDBOX=true`.                                                    | (None)     |
| `SANDBOX_WORLD_PACKAGES` | Comma-separated list of World package IDs to use instead of the hardcoded addresses. When omitted, the World pipeline is disabled. | (None)     |
| `SANDBOX_INGESTION_PATH` | Path to a local directory of checkpoint files. Required when `SANDBOX_NETWORK=localnet`.                                           | (None)     |

## Monitoring

| Variable          | Description                                            | Default        |
| ----------------- | ------------------------------------------------------ | -------------- |
| `METRICS_ADDRESS` | Address and port where Prometheus metrics are exposed. | `0.0.0.0:9184` |
