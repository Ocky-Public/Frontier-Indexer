CREATE EXTENSION IF NOT EXISTS timescaledb;

CREATE TABLE IF NOT EXISTS indexer.events_character_created (
  occurred_at         TIMESTAMPTZ NOT NULL,
  item_id             VARCHAR(12) NOT NULL,
  tenant              VARCHAR(12) NOT NULL,
  character_id        VARCHAR(66) NOT NULL,
  owner_address       VARCHAR(66) NOT NULL,
  tribe_id            BIGINT      NOT NULL,
  PRIMARY KEY (character_id, occurred_at)
);

SELECT public.create_hypertable('indexer.events_character_created', 'occurred_at');
