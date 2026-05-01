CREATE TABLE IF NOT EXISTS items(
  id          VARCHAR(66)   PRIMARY KEY,
  parent_id   VARCHAR(66)   NOT NULL,
  location    VARCHAR(66)   NOT NULL,
  item_id     VARCHAR(20)   NOT NULL,
  type_id     BIGINT        NOT NULL,
  volume      BIGINT        NOT NULL,
  quantity    BIGINT        NOT NULL
);
