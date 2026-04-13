CREATE TABLE IF NOT EXISTS indexer.events_fuel_burning_started (
  event_id      VARCHAR(100)  NOT NULL,
  occurred_at   TIMESTAMPTZ   NOT NULL,
  id            VARCHAR(66)   NOT NULL,
  item_id       VARCHAR(20)   NOT NULL,
  type_id       BIGINT        NOT NULL,
  quantity      BIGINT        NOT NULL,
  quantity_old  BIGINT        NOT NULL,
  burning       BOOLEAN       NOT NULL,
  PRIMARY KEY (event_id, occurred_at)
);

SELECT public.create_hypertable('indexer.events_fuel_burning_started', 'occurred_at');
