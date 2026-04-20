# Frontier Indexer

A custom [Sui](https://sui.io/) indexer for the EVE Frontier world contracts. It processes Sui blockchain checkpoints and projects on-chain state into a PostgreSQL database, making it queryable for front-end services, analytics, and other downstream systems.

> [!NOTE]
> This project is still under active development. Interfaces and configuration options may change between releases.

---

## Installation

The indexer is distributed as a Docker container. It requires a running [TimescaleDB](https://www.timescale.com/) database; database schema migrations are applied automatically on startup. The recommended container is `docker.io/timescale/timescaledb-ha:pg17`:

```sh
docker network create frontier
```

```sh
docker run -d --network frontier \
  --name timescaledb \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=postgres \
  -p 5432:5432 \
  docker.io/timescale/timescaledb-ha:pg17
```

> [!IMPORTANT]
> You **must** wait a few seconds for the database to initialize before starting the indexer, otherwise it will fail to connect and exit.

Then, start the indexer container:

```sh
INDEXER_VERSION=$(curl -s https://api.github.com/repos/Ocky-Public/Frontier-Indexer/releases/latest | grep '"tag_name"' | sed 's/.*"tag_name": *"\(.*\)".*/\1/')

docker run --rm --network frontier \
  --name frontier-indexer \
  -e DB_HOST=timescaledb \
  -e DB_NAME=postgres \
  -e DB_USER=postgres \
  -e DB_PASSWORD=postgres \
  -e DB_SCHEMA=indexer \
  -e SUI_NETWORK=testnet \
  ghcr.io/ocky-public/frontier-indexer:$INDEXER_VERSION
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
- [libpq-dev](https://www.postgresql.org/download/) (PostgreSQL client library, required to compile the `diesel` postgres driver — install via `apt-get install libpq-dev` or equivalent)
- [TimescaleDB](https://docs.timescale.com/self-hosted/latest/install/) (`timescale/timescaledb-ha:pg17` is the recommended container)
- [Diesel CLI](https://diesel.rs/guides/getting-started)

### Running Locally

1. Clone the repository:

   ```sh
   git clone https://github.com/Ocky-Public/Frontier-Indexer.git
   cd Frontier-Indexer
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
