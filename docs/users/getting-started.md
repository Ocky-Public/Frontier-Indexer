Todo: Describe a very simple docker-compose setup that can be used for the deployment of the indexer and related services.

Here is a very simple example of such as docker-compose deployment:

```
services:
  postgresql:
    image: timescale/timescaledb-ha:pg17
    volumes:
      - ./postgresql:/home/postgres/pgdata/data
    environment:
      - POSTGRES_PASSWORD=postgres
    user: '$UID:$GID'
    restart: on-failure

  pgadmin:
    image: dpage/pgadmin4:latest
    ports:
      - 8080:80
    volumes:
      - ./pgadmin:/var/lib/pgadmin
    environment:
      - PGADMIN_DEFAULT_EMAIL=admin@domain.com
      - PGADMIN_DEFAULT_PASSWORD=admin
    user: '$UID:$GID'
    restart: on-failure

  indexer:
    image: ghcr.io/ocky-public/frontier-indexer:latest
    ports:
      - 9184:9184
    environment:
      SUI_NETWORK: testnet
      FIRST_CHECKPOINT: 308265845

      DB_USER: postgres
      DB_PASSWORD: postgres
      DB_HOST: postgresql
      DB_PORT: 5432
      DB_NAME: postgres
      DB_SCHEMA: indexer
      DB_CONNECTION_POOL_SIZE: 10

      INGESTION_SOURCE: fullnode
      INGEST_CONCURRENCY_MAX: 1

    volumes:
      - ./indexer/pipelines.toml:/opt/indexer/bin/pipelines.toml
    restart: on-failure

  prometheus:
    image: prom/prometheus:latest
    ports:
      - 9090:9090
    volumes:
      - ./prometheus/prometheus.yml:/etc/prometheus/prometheus.yml
      - ./prometheus/data/:/prometheus
    command:
      - '--config.file=/etc/prometheus/prometheus.yml'
      - '--storage.tsdb.path=/prometheus'

  grafana:
    image: grafana/grafana:latest
    ports:
      - 3004:3000
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=Grafan@321
    depends_on:
      - prometheus
```