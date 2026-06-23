CREATE TABLE IF NOT EXISTS events_rift_location_broadcasted (
  event_id          VARCHAR(100)  NOT NULL,
  occurred_at       TIMESTAMPTZ   NOT NULL,
  id                VARCHAR(66)   NOT NULL,
  item_id           VARCHAR(20)   NOT NULL,
  location_hash     VARCHAR(66)   NOT NULL,
  solar_system_id   VARCHAR(20)   NOT NULL,
  x                 TEXT          NOT NULL,
  y                 TEXT          NOT NULL,
  z                 TEXT          NOT NULL,
  PRIMARY KEY (event_id, occurred_at)
);

SELECT public.create_hypertable('events_rift_location_broadcasted', 'occurred_at');
