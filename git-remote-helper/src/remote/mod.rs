mod memory_remote;

pub use memory_remote::*;

use async_trait::async_trait;

#[async_trait]
pub trait Remote {
    async fn get_refs(&self) -> Vec<String>;
    async fn get_object(&self, id: String) -> Result<String, String>;
    async fn push_object(&self, id: String, obj: String) -> Result<(), String>;
}
