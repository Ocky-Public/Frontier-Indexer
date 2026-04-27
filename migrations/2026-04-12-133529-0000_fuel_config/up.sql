CREATE TABLE IF NOT EXISTS fuel_config (
  table_id            VARCHAR(66)   NOT NULL,
  type_id             BIGINT        NOT NULL,
  efficiency          BIGINT        NOT NULL,
  entry_object_id     VARCHAR(66)   NOT NULL,
  checkpoint_updated  BIGINT        NOT NULL,
  PRIMARY KEY(type_id, table_id)
);
