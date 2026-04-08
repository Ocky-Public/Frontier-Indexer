CREATE TABLE IF NOT EXISTS indexer.characters (
  id                  VARCHAR(66) PRIMARY KEY,
  item_id             VARCHAR(12) NOT NULL,
  tenant              VARCHAR(12) NOT NULL,
  owner_cap_id        VARCHAR(66) NOT NULL,
  owner_address       VARCHAR(66) NOT NULL,
  tribe_id            BIGINT      NOT NULL,
  name                TEXT        NOT NULL,
  description         TEXT,
  url                 TEXT,
  checkpoint_updated  BIGINT      NOT NULL
);
