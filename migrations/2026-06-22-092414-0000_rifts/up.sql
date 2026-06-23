CREATE TABLE IF NOT EXISTS rifts (
  id                  VARCHAR(66)   PRIMARY KEY,
  item_id             VARCHAR(20)   NOT NULL,
  tenant              TEXT          NOT NULL,
  location            VARCHAR(66)   NOT NULL,
  checkpoint_updated  BIGINT        NOT NULL
);
