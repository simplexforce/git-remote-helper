mod memory_remote;

pub use memory_remote::*;

// use std::future::Future;

pub trait Remote {
    fn get_refs(&self) -> Vec<String>;
    fn get_object(&self, id: String) -> Result<String, String>;
    fn push_object(&self, id: String, obj: String) -> Result<(), String>;

    // TODO: Find a more generic interface design

    // TODO: async
    
    // fn get_refs(&self) -> impl Future<Output = Vec<String>> + Send;
    // fn get_object(&self, id: String) -> impl Future<Output = Result<String, String>> + Send;
    // fn push_object(&self, id: String, obj: String) -> impl Future<Output = Result<(), String>> + Send;

    // TODO get_pack
    // TDDO push_pack
}