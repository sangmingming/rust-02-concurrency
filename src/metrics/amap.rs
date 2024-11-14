use std::{
    collections::HashMap,
    fmt::Display,
    sync::{atomic::AtomicI64, Arc},
};

use anyhow::anyhow;
use anyhow::Result;

#[derive(Debug)]
pub struct AmapMetrics {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl Clone for AmapMetrics {
    fn clone(&self) -> Self {
        AmapMetrics {
            data: Arc::clone(&self.data),
        }
    }
}

impl Display for AmapMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for (key, value) in self.data.iter() {
            write!(
                f,
                "{}:{}",
                key,
                value.load(std::sync::atomic::Ordering::Relaxed)
            )?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl AmapMetrics {
    pub fn new(metric_names: &[&'static str]) -> Self {
        let map = metric_names
            .iter()
            .map(|&name| (name, AtomicI64::new(0)))
            .collect();
        AmapMetrics {
            data: Arc::new(map),
        }
    }

    pub fn inc(&self, key: impl AsRef<str>) -> Result<()> {
        let item = self
            .data
            .get(key.as_ref())
            .ok_or_else(|| anyhow!("key {} not found", key.as_ref()))?;
        item.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    }

    pub fn dec(&self, key: impl AsRef<str>) -> Result<()> {
        let item = self
            .data
            .get(key.as_ref())
            .ok_or_else(|| anyhow!("key {} not found", key.as_ref()))?;
        item.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    }
}
