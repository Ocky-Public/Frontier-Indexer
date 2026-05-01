use anyhow::Context;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct PipelineConfig {
    pub pipelines: HashMap<String, bool>,
}

impl PipelineConfig {
    pub fn from_file(path: &Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read pipeline config: {}", path.display()))?;
        toml::from_str(&content)
            .with_context(|| format!("Failed to parse pipeline config: {}", path.display()))
    }

    pub fn enabled_pipelines(&self) -> Option<Vec<String>> {
        let enabled: Vec<String> = self
            .pipelines
            .iter()
            .filter(|(_, &v)| v)
            .map(|(k, _)| k.clone())
            .collect();

        if enabled.len() == self.pipelines.len() {
            // All entries are enabled — no need to pass an allowlist.
            None
        } else {
            Some(enabled)
        }
    }
}
