CREATE TABLE IF NOT EXISTS indexer.events_fuel_efficiency_set (
  event_id    VARCHAR(100)  NOT NULL,
  occurred_at TIMESTAMPTZ   NOT NULL,
  type_id     BIGINT        NOT NULL,
  efficiency  BIGINT        NOT NULL,
  PRIMARY KEY (event_id, occurred_at)
);

SELECT public.create_hypertable('indexer.events_fuel_efficiency_set', 'occurred_at');
