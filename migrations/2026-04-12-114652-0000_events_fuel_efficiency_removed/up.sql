CREATE TABLE IF NOT EXISTS indexer.events_fuel_efficiency_removed (
  event_id    VARCHAR(100)  NOT NULL,
  occurred_at TIMESTAMPTZ   NOT NULL,
  type_id     BIGINT        NOT NULL,
  PRIMARY KEY (event_id, occurred_at)
);

SELECT public.create_hypertable('indexer.events_fuel_efficiency_removed', 'occurred_at');
