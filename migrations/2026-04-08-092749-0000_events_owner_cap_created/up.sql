-- Your SQL goes here
CREATE TABLE IF NOT EXISTS indexer.events_owner_cap_created (
  occurred_at TIMESTAMPTZ NOT NULL,
  id          VARCHAR(66) NOT NULL,
  object_id   VARCHAR(66) NOT NULL,
  PRIMARY KEY (id, occurred_at)
);

SELECT public.create_hypertable('indexer.events_owner_cap_created', 'occurred_at');
