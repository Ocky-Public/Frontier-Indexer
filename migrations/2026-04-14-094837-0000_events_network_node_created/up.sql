CREATE TABLE IF NOT EXISTS indexer.events_network_node_created (
  event_id      VARCHAR(100)  NOT NULL,
  occurred_at   TIMESTAMPTZ   NOT NULL,
  id            VARCHAR(66)   NOT NULL,
  item_id       VARCHAR(20)   NOT NULL,
  tenant        TEXT          NOT NULL,
  type_id       BIGINT        NOT NULL,
  owner_cap_id  VARCHAR(66)   NOT NULL,
  PRIMARY KEY (event_id, occurred_at)
);

SELECT public.create_hypertable('indexer.events_network_node_created', 'occurred_at');
