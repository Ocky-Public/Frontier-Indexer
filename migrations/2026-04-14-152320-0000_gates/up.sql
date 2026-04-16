CREATE TABLE IF NOT EXISTS indexer.gates (
  id                  VARCHAR(66) PRIMARY KEY,
  item_id             VARCHAR(20) NOT NULL,
  tenant              TEXT        NOT NULL,
  type_id             BIGINT      NOT NULL,
  owner_cap_id        VARCHAR(66) NOT NULL,
  location            VARCHAR(66) NOT NULL,
  status              TEXT        NOT NULL,
  energy_source_id    VARCHAR(66),
  linked_id           VARCHAR(66),
  name                TEXT,
  description         TEXT,
  url                 TEXT,
  package_id          VARCHAR(66),
  module_name         TEXT,
  struct_name         TEXT,
  checkpoint_updated  BIGINT      NOT NULL
);
