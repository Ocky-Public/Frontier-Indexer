CREATE TABLE IF NOT EXISTS inventories (
  id                  VARCHAR(66) PRIMARY KEY,
  parent_id           VARCHAR(66) NOT NULL,
  capacity_used       BIGINT      NOT NULL,
  capacity_max        BIGINT      NOT NULL,
  checkpoint_updated  BIGINT      NOT NULL
);
