CREATE TABLE IF NOT EXISTS events_fuel_deleted (
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

SELECT public.create_hypertable('events_fuel_deleted', 'occurred_at');
