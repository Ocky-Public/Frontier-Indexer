CREATE TABLE IF NOT EXISTS owner_caps (
  id                  VARCHAR(66) PRIMARY KEY,
  object_id           VARCHAR(66) NOT NULL,
  owner_address       VARCHAR(66) NOT NULL,
  package_id          VARCHAR(66) NOT NULL,
  module_name         TEXT        NOT NULL,
  struct_name         TEXT        NOT NULL,
  checkpoint_updated  BIGINT      NOT NULL
);
