CREATE EXTENSION IF NOT EXISTS timescaledb;

CREATE TABLE IF NOT EXISTS indexer.events_character_created (
  event_id            VARCHAR(100)  NOT NULL,
  occurred_at         TIMESTAMPTZ   NOT NULL,
  id                  VARCHAR(66)   NOT NULL,
  item_id             VARCHAR(20)   NOT NULL,
  tenant              TEXT          NOT NULL,
  owner_address       VARCHAR(66)   NOT NULL,
  tribe_id            VARCHAR(20)   NOT NULL,
  PRIMARY KEY (event_id, occurred_at)
);

SELECT public.create_hypertable('indexer.events_character_created', 'occurred_at');
