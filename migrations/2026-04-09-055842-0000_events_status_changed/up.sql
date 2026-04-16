CREATE TABLE IF NOT EXISTS indexer.events_status_changed (
  event_id      VARCHAR(100)  NOT NULL,
  occurred_at   TIMESTAMPTZ   NOT NULL,
  id            VARCHAR(66)   NOT NULL,
  item_id       VARCHAR(20)   NOT NULL,
  status        TEXT          NOT NULL,
  action        TEXT          NOT NULL,
  PRIMARY KEY (event_id, occurred_at)
);

SELECT public.create_hypertable('indexer.events_status_changed', 'occurred_at');