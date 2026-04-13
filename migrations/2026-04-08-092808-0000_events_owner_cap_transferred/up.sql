CREATE TABLE IF NOT EXISTS indexer.events_owner_cap_transferred (
  event_id        VARCHAR(100)  NOT NULL,
  occurred_at     TIMESTAMPTZ   NOT NULL,
  id              VARCHAR(66)   NOT NULL,
  object_id       VARCHAR(66)   NOT NULL,
  owner_previous  VARCHAR(66)   NOT NULL,
  owner_new       VARCHAR(66)   NOT NULL,
  PRIMARY KEY (event_id, occurred_at)
);

SELECT public.create_hypertable('indexer.events_owner_cap_transferred', 'occurred_at');
