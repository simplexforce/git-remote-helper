mod fs_remote;
mod memory_remote;

pub use fs_remote::*;
pub use memory_remote::*;

use async_trait::async_trait;

#[async_trait]
pub trait Remote {
    /// List all references (for fetch operations)
    async fn list_refs(&self) -> Vec<String>;
    
    /// List references specifically for push operations
    async fn list_push_refs(&self) -> Vec<String>;
    
    /// Batch update references
    async fn update_refs(&self, updates: Vec<RefUpdate>) -> Result<(), String>;
    
    /// Get an object by its ID
    async fn fetch_object(&self, id: String) -> Result<Vec<u8>, String>;
    
    /// Push an object to the remote
    async fn push_object(&self, id: String, obj: Vec<u8>) -> Result<(), String>;
    
    /// Check if an object exists on the remote
    async fn exists_object(&self, id: String) -> Result<bool, String>;
}

/// Represents a reference update operation
pub struct RefUpdate {
    /// Source reference name
    pub src: String,
    /// Destination reference name
    pub dst: String,
    /// Whether to force the update
    pub force: bool,
}
