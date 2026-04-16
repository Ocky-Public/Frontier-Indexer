CREATE TABLE IF NOT EXISTS indexer.events_gate_jumped (
  event_id            VARCHAR(100) NOT NULL,
  occurred_at         TIMESTAMPTZ NOT NULL,
  character_id        VARCHAR(66) NOT NULL,
  character_item_id   VARCHAR(20) NOT NULL,
  departure_id        VARCHAR(66) NOT NULL,
  departure_item_id   VARCHAR(20) NOT NULL,
  destination_id      VARCHAR(66) NOT NULL,
  destination_item_id VARCHAR(20) NOT NULL,
  PRIMARY KEY (event_id, occurred_at)
);

SELECT public.create_hypertable('indexer.events_gate_jumped', 'occurred_at');
