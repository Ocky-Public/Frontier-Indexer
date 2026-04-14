CREATE TABLE IF NOT EXISTS indexer.events_gate_extension_authorized (
  event_id          VARCHAR(100)  NOT NULL,
  occurred_at       TIMESTAMPTZ   NOT NULL,
  id                VARCHAR(66)   NOT NULL,
  item_id           VARCHAR(20)   NOT NULL,
  package_id        VARCHAR(66)   NOT NULL,
  module_name       TEXT          NOT NULL,
  struct_name       TEXT          NOT NULL,
  package_id_old    VARCHAR(66),
  module_name_old   TEXT,
  struct_name_old   TEXT,
  PRIMARY KEY (event_id, occurred_at)
);

SELECT public.create_hypertable('indexer.events_gate_extension_authorized', 'occurred_at');
