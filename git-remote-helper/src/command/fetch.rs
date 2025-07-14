use super::{CommandHandler, Context, write_line};

use async_trait::async_trait;

pub struct FetchHandler {}

#[async_trait]
impl CommandHandler for FetchHandler {
    fn name(&self) -> &'static str {
        "fetch"
    }

    async fn handle(&self, remote: &Context, args: Vec<&str>) {
        // Fetch implementation would use:
        // - remote.exists_object() to check for missing objects
        // - remote.get_object() to retrieve missing objects
        // For now, just output empty line as placeholder
        write_line("");
    }
}
