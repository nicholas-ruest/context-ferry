use cf_types::ContextEvent;
use ruvector_core::types::{DbOptions, HnswConfig};
use ruvector_core::{DistanceMetric, SearchQuery, VectorDB, VectorEntry};
use std::collections::HashMap;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum MemoryError {
    #[error("ruvector: {0}")]
    Database(String),
}
pub struct ContextMemory {
    db: VectorDB,
}
fn embedding(e: &ContextEvent) -> Vec<f32> {
    vec![
        e.importance as f32,
        f32::from(e.sensitivity.level()) / 2.0,
        e.tokens as f32 / 100.0,
        (e.text.len() % 101) as f32 / 100.0,
    ]
}
impl ContextMemory {
    pub fn open(path: &str) -> Result<Self, MemoryError> {
        let o = DbOptions {
            dimensions: 4,
            distance_metric: DistanceMetric::Cosine,
            storage_path: path.into(),
            hnsw_config: Some(HnswConfig::default()),
            quantization: None,
        };
        VectorDB::new(o)
            .map(|db| Self { db })
            .map_err(|e| MemoryError::Database(e.to_string()))
    }
    pub fn remember(&self, e: &ContextEvent) -> Result<String, MemoryError> {
        let mut m = HashMap::new();
        m.insert("kind".into(), serde_json::json!(e.kind));
        self.db
            .insert(VectorEntry {
                id: Some(e.id.to_string()),
                vector: embedding(e),
                metadata: Some(m),
            })
            .map_err(|e| MemoryError::Database(e.to_string()))
    }
    pub fn recall(&self, e: &ContextEvent, k: usize) -> Result<usize, MemoryError> {
        self.db
            .search(SearchQuery {
                vector: embedding(e),
                k,
                filter: None,
                ef_search: Some(32),
            })
            .map(|v| v.len())
            .map_err(|e| MemoryError::Database(e.to_string()))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use cf_types::Sensitivity;
    #[test]
    fn persistent_roundtrip() {
        let d = tempfile::tempdir().unwrap();
        let path = d.path().join("memory.db");
        let m = ContextMemory::open(path.to_str().unwrap()).unwrap();
        let e = ContextEvent::new("a", "objective", "ship", Sensitivity::Public, 0.9).unwrap();
        m.remember(&e).unwrap();
        assert_eq!(m.recall(&e, 1).unwrap(), 1);
    }
}
