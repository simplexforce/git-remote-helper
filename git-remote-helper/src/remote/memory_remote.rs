use super::Remote;
use crate::remote::RefUpdate; // Import RefUpdate from parent module
use async_trait::async_trait;
use std::collections::BTreeMap;
use std::sync::{RwLock, Arc};

pub struct MemoryRemote {
    pub head: String,
    pub refs: Arc<RwLock<Vec<String>>>,
    pub objects: RwLock<BTreeMap<String, Vec<u8>>>,
}

impl MemoryRemote {
    pub fn new() -> Self {
        Self {
            head: String::new(),
            refs: Arc::new(RwLock::new(Vec::new())),
            objects: RwLock::new(BTreeMap::new()),
        }
    }
}

#[async_trait]
impl Remote for MemoryRemote {
    async fn list_refs(&self) -> Vec<String> {
        let refs = self.refs.read().unwrap().clone();
        let mut all_refs = refs;

        if !self.head.is_empty() {
            all_refs.push(self.head.clone());
        }

        all_refs
    }
    
    async fn list_push_refs(&self) -> Vec<String> {
        // For push operations, include both heads and tags
        // In memory remote, we treat all refs as pushable
        self.list_refs().await
    }
    
    async fn update_refs(&self, updates: Vec<RefUpdate>) -> Result<(), String> {
        let mut refs = self.refs.write().unwrap();
        for update in updates {
            // Remove any existing ref with same destination
            refs.retain(|r| r != &update.dst);
            // Add the new ref
            refs.push(update.dst);
        }
        Ok(())
    }

    async fn fetch_object(&self, id: String) -> Result<Vec<u8>, String> {
        let objects = self.objects.read().map_err(|e| e.to_string())?;
        objects
            .get(&id)
            .cloned()
            .ok_or_else(|| format!("Object not found: {}", id))
    }
    
    async fn exists_object(&self, id: String) -> Result<bool, String> {
        let objects = self.objects.read().map_err(|e| e.to_string())?;
        Ok(objects.contains_key(&id))
    }

    async fn push_object(&self, id: String, obj: Vec<u8>) -> Result<(), String> {
        let mut objects = self.objects.write().map_err(|e| e.to_string())?;
        objects.insert(id, obj);
        Ok(())
    }
}
