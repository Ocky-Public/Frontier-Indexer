CREATE TABLE IF NOT EXISTS energy_config (
  table_id            VARCHAR(66) NOT NULL,
  type_id             BIGINT      NOT NULL,
  energy_cost         BIGINT      NOT NULL,
  entry_object_id     VARCHAR(66) NOT NULL,
  checkpoint_updated  BIGINT      NOT NULL,
  PRIMARY KEY (type_id, table_id)
);
