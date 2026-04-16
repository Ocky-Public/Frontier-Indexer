-- Your SQL goes here
CREATE TABLE IF NOT EXISTS indexer.killmails (
  id                VARCHAR(66) NOT NULL,
  kill_id           VARCHAR(20) NOT NULL,
  tenant            TEXT        NOT NULL,
  occurred_at       TIMESTAMPTZ NOT NULL,
  solar_system_id   VARCHAR(20) NOT NULL,
  loss_type         TEXT        NOT NULL,
  killer_id         VARCHAR(20) NOT NULL,
  victim_id         VARCHAR(20) NOT NULL,
  reporter_id       VARCHAR(20) NOT NULL,
  PRIMARY KEY (id, occurred_at)
);

SELECT public.create_hypertable('indexer.killmails', 'occurred_at');
