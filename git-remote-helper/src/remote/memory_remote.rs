use super::Remote;

use async_trait::async_trait;
use std::collections::BTreeMap;
use std::sync::RwLock;

pub struct MemoryRemote {
    pub head: String,
    pub refs: Vec<String>,
    pub objects: RwLock<BTreeMap<String, String>>,
}

impl MemoryRemote {
    pub fn new() -> Self {
        Self {
            head: String::new(),
            refs: Vec::new(),
            objects: RwLock::new(BTreeMap::new()),
        }
    }
}

#[async_trait]
impl Remote for MemoryRemote {
    async fn get_refs(&self) -> Vec<String> {
        let mut refs = self.refs.clone();

        if self.head != "" {
            refs.push(self.head.clone());
        }

        refs
    }

    async fn get_object(&self, id: String) -> Result<String, String> {
        let objects = self.objects.read().map_err(|e| e.to_string())?;
        objects
            .get(&id)
            .cloned()
            .ok_or_else(|| format!("Object not found: {}", id))
    }

    async fn push_object(&self, id: String, obj: String) -> Result<(), String> {
        let mut objects = self.objects.write().map_err(|e| e.to_string())?;
        objects.insert(id, obj);
        Ok(())
    }
}
