CREATE TABLE IF NOT EXISTS gate_permits (
  id            VARCHAR(66)   PRIMARY KEY,
  character_id  VARCHAR(66)   NOT NULL,
  link_hash     VARCHAR(66)   NOT NULL,
  expires_at    TIMESTAMPTZ   NOT NULL
);
