CREATE TABLE IF NOT EXISTS events_energy_production_stopped (
  event_id    VARCHAR(100)  NOT NULL,
  occurred_at TIMESTAMPTZ   NOT NULL,
  id          VARCHAR(66)   NOT NULL,
  PRIMARY KEY (event_id, occurred_at)
);

SELECT public.create_hypertable('events_energy_production_stopped', 'occurred_at');
