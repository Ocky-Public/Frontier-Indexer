CREATE TABLE IF NOT EXISTS events_rift_mining_started (
  event_id          VARCHAR(100)  NOT NULL,
  occurred_at       TIMESTAMPTZ   NOT NULL,
  item_id           VARCHAR(20)   NOT NULL,
  character_id      VARCHAR(66)   NOT NULL,
  solar_system_id   VARCHAR(20)   NOT NULL,
  x                 TEXT          NOT NULL,
  y                 TEXT          NOT NULL,
  z                 TEXT          NOT NULL,
  PRIMARY KEY (event_id, occurred_at)
);

SELECT public.create_hypertable('events_rift_mining_started', 'occurred_at');
