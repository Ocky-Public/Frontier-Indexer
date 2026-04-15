CREATE TABLE IF NOT EXISTS indexer.inventory_entries (
  parent_id           VARCHAR(66)   NOT NULL,
  inventory_id        VARCHAR(66)   NOT NULL,
  type_id             BIGINT        NOT NULL,
  item_id             BIGINT        NOT NULL,
  volume              BIGINT        NOT NULL,
  quantity            BIGINT        NOT NULL,
  checkpoint_updated  BIGINT        NOT NULL,
  PRIMARY KEY(inventory_id, type_id)
);
