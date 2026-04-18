# Database and Models

The indexer uses PostgreSQL for storage and Diesel as the ORM/Query builder.

## Migrations
Database schema evolutions are handled via Diesel migrations.
- **Location**: All migration scripts are located in the `/migrations` directory.
- **Application**: Migrations are automatically run at startup in `main.rs` using `embed_migrations!`.

Each migration consists of an `up.sql` (to apply changes) and a `down.sql` (to revert changes).

### Adding a New Migration

The project uses the [Diesel CLI](https://diesel.rs/guides/getting-started) to manage migrations. Make sure it is installed before proceeding.

1. Set the database URL:
   ```sh
   PSQL_URL=postgres://username@localhost:5432/sui_indexer
   ```

2. Verify the connection works:
   ```sh
   psql $PSQL_URL -c "SELECT 'Connected';"
   ```
   Expected output:
   ```
    ?column?
   -----------
    Connected
   (1 row)
   ```

3. Configure the Diesel CLI:
   ```sh
   diesel setup --database-url $PSQL_URL
   ```

4. Generate the migration (choose a descriptive name):
   ```sh
   diesel migration generate <name_here>
   ```
   This creates a new directory under `/migrations` with blank `up.sql` and `down.sql` files for you to fill in.

5. Apply the migration:
   ```sh
   diesel migration run --database-url $PSQL_URL
   ```

6. If `src/schema.rs` does not update automatically after running the migration, regenerate it manually:
   ```sh
   diesel print-schema --database-url $PSQL_URL > src/schema.rs
   ```

## Models
The data models are defined in `src/models/`. They are split into:
- **World Models**: Represent the state of the world contracts (e.g., `src/models/world.rs`).
- **System Models**: Represent internal indexer state or registry information (e.g., `src/models/system.rs`).

## Registries
The system uses "Registries" to load configuration from the database at startup:
- `TableRegistry`: Tracks which tables are currently registered and should be indexed.
- `FuelRegistry`: Manages fuel-related configuration stored in the database.

These are loaded in `main.rs` and passed into the `AppContext`, allowing handlers to make decisions based on database-stored configuration.
