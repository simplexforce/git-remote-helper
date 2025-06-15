mod memory_remote;

pub use memory_remote::*;

pub trait Remote {
    fn get_refs(&self) -> impl Future<Output = Vec<String>> + Send;
    fn get_object(&self, id: String) -> impl Future<Output = Result<String, String>> + Send;
    fn push_object(&self, id: String, obj: String) -> impl Future<Output = Result<(), String>> + Send;
}