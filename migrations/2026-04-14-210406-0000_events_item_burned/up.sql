CREATE TABLE IF NOT EXISTS indexer.events_item_burned (
  event_id          VARCHAR(66)   NOT NULL,
  occurred_at       TIMESTAMPTZ   NOT NULL,
  item_id           VARCHAR(20)   NOT NULL,
  type_id           BIGINT        NOT NULL,
  quantity          BIGINT        NOT NULL,
  assembly_id       VARCHAR(66)   NOT NULL,
  assembly_item_id  VARCHAR(20)   NOT NULL,
  character_id      VARCHAR(66)   NOT NULL,
  character_item_id VARCHAR(20)   NOT NULL,
  PRIMARY KEY (event_id, occurred_at)
);

SELECT public.create_hypertable('indexer.events_item_burned', 'occurred_at');
