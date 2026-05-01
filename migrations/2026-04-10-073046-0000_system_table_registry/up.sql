CREATE TABLE IF NOT EXISTS system_table_registry (
  table_id            VARCHAR(66) PRIMARY KEY,
  parent_id           VARCHAR(66) NOT NULL,
  package_id          VARCHAR(66) NOT NULL,
  module_name         TEXT        NOT NULL,
  struct_name         TEXT        NOT NULL,
  key_type            TEXT        NOT NULL,
  value_type          TEXT        NOT NULL,
  checkpoint_updated  BIGINT      NOT NULL
);
