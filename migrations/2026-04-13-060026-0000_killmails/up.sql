-- Your SQL goes here
CREATE TABLE IF NOT EXISTS indexer.killmails (
  id                VARCHAR(66) NOT NULL,
  kill_id           BIGINT      NOT NULL,
  tenant            TEXT        NOT NULL,
  occurred_at       TIMESTAMPTZ NOT NULL,
  solar_system_id   BIGINT      NOT NULL,
  loss_type         TEXT        NOT NULL,
  killer_id         BIGINT      NOT NULL,
  victim_id         BIGINT      NOT NULL,
  reporter_id       BIGINT      NOT NULL,
  PRIMARY KEY (id, occurred_at)
);

SELECT public.create_hypertable('indexer.killmails', 'occurred_at');
