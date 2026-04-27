CREATE TABLE IF NOT EXISTS extension_freezes (
  id          VARCHAR(66)   PRIMARY KEY,
  owner_id    VARCHAR(66)   NOT NULL,
  package_id  VARCHAR(66)   NOT NULL,
  module_name TEXT          NOT NULL,
  struct_name TEXT          NOT NULL
);
