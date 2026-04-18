# Frontier Indexer

A custom [Sui](https://sui.io/) indexer for the EVE Frontier world contracts. It processes Sui blockchain checkpoints and projects on-chain state into a PostgreSQL database, making it queryable for front-end services, analytics, and other downstream systems.

> **Note:** This project is still under active development. Interfaces and configuration options may change between releases.

---

## Installation

The indexer is distributed as a Docker container. It requires a running [TimescaleDB](https://www.timescale.com/) database; database schema migrations are applied automatically on startup. The recommended container is `timescale/timescaledb-ha:pg17`:

```sh
docker run -d \
  --name timescaledb \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=postgres \
  -p 5432:5432 \
  timescale/timescaledb-ha:pg17
```

```sh
docker run ghcr.io/frontier/indexer:<version> \
  -e DB_HOST=your-db-host \
  -e DB_NAME=your-db-name \
  -e DB_USER=your-db-user \
  -e DB_PASSWORD=your-db-password \
  -e SUI_NETWORK=testnet
```

All behaviour is controlled through environment variables. See [Container Configuration](docs/users/configuration.md) for the full list of available options.

---

## Development

The indexer is designed to be extended with application-specific handlers for your own smart contracts. The `PACKAGES` environment variable controls which package groups are indexed (`world`, `app`, or both). Custom application logic goes inside the `App` pipeline.

For a full explanation of the handler system, package filtering, and how the indexer interacts with the world contracts, see the [Developer Documentation](docs/developers/):

- [Architecture Overview](docs/developers/architecture.md)
- [World Contracts Integration](docs/developers/world_contracts.md)
- [Database and Models](docs/developers/database.md)

---

## Contributing

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version specified in `Dockerfile`, currently `1.90.0`)
- [TimescaleDB](https://docs.timescale.com/self-hosted/latest/install/) (`timescale/timescaledb-ha:pg17` is the recommended container)
- [Diesel CLI](https://diesel.rs/guides/getting-started)

### Running Locally

1. Clone the repository:
   ```sh
   git clone https://github.com/frontier/indexer.git
   cd indexer
   ```

2. Copy your environment config (or export the variables directly):
   ```sh
   cp .env.example .env
   # edit .env with your database credentials and network settings
   ```

3. Run the indexer:
   ```sh
   cargo run
   ```

For information on adding database migrations, see [Database and Models](docs/developers/database.md#adding-a-new-migration).
