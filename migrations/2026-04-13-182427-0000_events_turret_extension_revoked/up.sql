CREATE TABLE IF NOT EXISTS indexer.events_turret_extension_revoked (
  event_id    VARCHAR(100)  NOT NULL,
  occurred_at TIMESTAMPTZ   NOT NULL,
  id          VARCHAR(66)   NOT NULL,
  item_id     VARCHAR(20)   NOT NULL,
  package_id  VARCHAR(66)   NOT NULL,
  module_name TEXT          NOT NULL,
  struct_name TEXT          NOT NULL,
  PRIMARY KEY (event_id, occurred_at)
);

SELECT public.create_hypertable('indexer.events_turret_extension_revoked', 'occurred_at');
