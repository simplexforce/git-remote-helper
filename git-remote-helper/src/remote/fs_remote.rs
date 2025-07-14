use super::Remote;
use crate::remote::RefUpdate;
use async_trait::async_trait;
use std::path::PathBuf;
use std::fs;

pub struct Config {
    pub git_dir: PathBuf,
}

pub struct FsRemote {
    config: Config,
}

impl FsRemote {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    fn is_valid_object_id(id: &str) -> bool {
        id.len() == 40 && id.chars().all(|c| c.is_ascii_hexdigit())
    }
}

#[async_trait]
impl Remote for FsRemote {
    async fn list_refs(&self) -> Vec<String> {
        let mut refs = Vec::new();
        let refs_path = self.config.git_dir.join("refs/heads");
        
        if let Ok(entries) = fs::read_dir(refs_path) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    refs.push(format!("refs/heads/{}", name));
                }
            }
        }
        
        // Add HEAD reference
        let head_path = self.config.git_dir.join("HEAD");
        if let Ok(head_content) = fs::read_to_string(head_path) {
            if let Some(head_ref) = head_content.strip_prefix("ref: ") {
                refs.push(head_ref.trim().to_string());
            }
        }
        
        refs
    }
    
    async fn list_push_refs(&self) -> Vec<String> {
        // For push operations, include both heads and tags
        let mut refs = self.list_refs().await;
        let tags_path = self.config.git_dir.join("refs/tags");
        
        if let Ok(entries) = fs::read_dir(tags_path) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    refs.push(format!("refs/tags/{}", name));
                }
            }
        }
        
        refs
    }
    
    async fn update_refs(&self, updates: Vec<RefUpdate>) -> Result<(), String> {
        for update in updates {
            let ref_path = self.config.git_dir.join(&update.dst);
            let parent = ref_path.parent().ok_or_else(|| "Invalid ref path".to_string())?;
            
            // Create parent directory if needed
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create ref directory: {}", e))?;
            }
            
            // Write the new reference
            fs::write(&ref_path, update.src.as_bytes())
                .map_err(|e| format!("Failed to update ref {}: {}", update.dst, e))?;
        }
        Ok(())
    }

    async fn fetch_object(&self, id: String) -> Result<Vec<u8>, String> {
        if !Self::is_valid_object_id(&id) {
            return Err(format!("Invalid object ID: {}", id));
        }

        let object_path = self.config.git_dir
            .join("objects")
            .join(&id[0..2])
            .join(&id[2..]);
            
        fs::read(&object_path)
            .map_err(|e| format!("Failed to read object {}: {}", id, e))
    }
    
    async fn exists_object(&self, id: String) -> Result<bool, String> {
        if !Self::is_valid_object_id(&id) {
            return Err(format!("Invalid object ID: {}", id));
        }

        let object_path = self.config.git_dir
            .join("objects")
            .join(&id[0..2])
            .join(&id[2..]);
            
        Ok(object_path.exists())
    }

    async fn push_object(&self, id: String, obj: Vec<u8>) -> Result<(), String> {
        if !Self::is_valid_object_id(&id) {
            return Err(format!("Invalid object ID: {}", id));
        }

        let dir_path = self.config.git_dir
            .join("objects")
            .join(&id[0..2]);
            
        fs::create_dir_all(&dir_path)
            .map_err(|e| format!("Failed to create object directory: {}", e))?;
            
        let object_path = dir_path.join(&id[2..]);
        
        fs::write(&object_path, &obj)
            .map_err(|e| format!("Failed to write object {}: {}", id, e))
    }
}
