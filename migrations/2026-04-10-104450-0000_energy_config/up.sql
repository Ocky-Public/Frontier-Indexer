CREATE TABLE IF NOT EXISTS indexer.energy_config (
  package_id          VARCHAR(66) NOT NULL,
  assembly_id         VARCHAR(20) NOT NULL,
  energy_cost         BIGINT      NOT NULL,
  entry_object_id     VARCHAR(66) NOT NULL,
  checkpoint_updated  BIGINT      NOT NULL,
  PRIMARY KEY (assembly_id, package_id)
);
