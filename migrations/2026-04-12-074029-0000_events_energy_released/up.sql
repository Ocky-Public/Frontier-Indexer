CREATE TABLE IF NOT EXISTS events_energy_released (
  event_id        VARCHAR(100)  NOT NULL,
  occurred_at     TIMESTAMPTZ   NOT NULL,
  id              VARCHAR(66)   NOT NULL,
  type_id         BIGINT        NOT NULL,
  released        BIGINT        NOT NULL,
  reserved_total  BIGINT        NOT NULL,
  PRIMARY KEY (event_id, occurred_at)
);

SELECT public.create_hypertable('events_energy_released', 'occurred_at');
