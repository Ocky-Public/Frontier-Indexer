CREATE TABLE IF NOT EXISTS indexer.events_extension_frozen (
  event_id    VARCHAR(66) NOT NULL,
  occurred_at TIMESTAMPTZ NOT NULL,
  id          VARCHAR(66) NOT NULL,
  PRIMARY KEY (event_id, occurred_at, id)
);

SELECT public.create_hypertable('indexer.events_extension_frozen', 'occurred_at');
