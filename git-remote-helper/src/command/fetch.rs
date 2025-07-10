use super::{CommandHandler, Context, write_line};

use async_trait::async_trait;

pub struct FetchHandler {}

#[async_trait]
impl CommandHandler for FetchHandler {
    fn name(&self) -> &'static str {
        "fetch"
    }

    async fn handle(&self, remote: &Context, args: Vec<&str>) {
        write_line("");
    }
}
