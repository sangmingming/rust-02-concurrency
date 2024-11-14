use anyhow::Result;
use core::fmt;
use dashmap::DashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<DashMap<String, i64>>,
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for entry in self.data.iter() {
            write!(f, "{}:{}", entry.key(), entry.value())?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }

    pub fn dec(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter -= 1;
        Ok(())
    }
}
