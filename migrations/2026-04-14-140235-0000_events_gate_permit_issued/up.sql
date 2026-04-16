CREATE TABLE IF NOT EXISTS indexer.events_gate_permit_issued (
  event_id            VARCHAR(100) NOT NULL,
  occurred_at         TIMESTAMPTZ NOT NULL,
  id                  VARCHAR(66) NOT NULL,
  character_id        VARCHAR(66) NOT NULL,
  character_item_id   VARCHAR(20) NOT NULL,
  departure_id        VARCHAR(66) NOT NULL,
  departure_item_id   VARCHAR(20) NOT NULL,
  destination_id      VARCHAR(66) NOT NULL,
  destination_item_id VARCHAR(20) NOT NULL,
  link_hash           VARCHAR(66) NOT NULL,
  package_id          VARCHAR(66) NOT NULL,
  module_name         TEXT        NOT NULL,
  struct_name         TEXT        NOT NULL,
  expires_at          TIMESTAMPTZ NOT NULL,
  PRIMARY KEY (event_id, occurred_at)
);

SELECT public.create_hypertable('indexer.events_gate_permit_issued', 'occurred_at');
