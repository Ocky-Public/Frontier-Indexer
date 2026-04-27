CREATE TABLE IF NOT EXISTS inventory_entries (
  inventory_id        VARCHAR(66)   NOT NULL,
  type_id             BIGINT        NOT NULL,
  item_id             VARCHAR(20)   NOT NULL,
  volume              BIGINT        NOT NULL,
  quantity            BIGINT        NOT NULL,
  checkpoint_updated  BIGINT        NOT NULL,
  PRIMARY KEY(inventory_id, type_id)
);
