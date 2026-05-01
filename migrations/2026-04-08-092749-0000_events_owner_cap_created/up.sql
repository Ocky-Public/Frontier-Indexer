-- Your SQL goes here
CREATE TABLE IF NOT EXISTS events_owner_cap_created (
  event_id    VARCHAR(100)  NOT NULL,
  occurred_at TIMESTAMPTZ   NOT NULL,
  id          VARCHAR(66)   NOT NULL,
  object_id   VARCHAR(66)   NOT NULL,
  PRIMARY KEY (event_id, occurred_at)
);

SELECT public.create_hypertable('events_owner_cap_created', 'occurred_at');
