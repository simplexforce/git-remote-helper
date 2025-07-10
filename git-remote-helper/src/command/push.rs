use super::{CommandHandler, Context, write_line};

use async_trait::async_trait;
use log::debug;

pub struct PushHandler {}

#[async_trait]
impl CommandHandler for PushHandler {
    fn name(&self) -> &'static str {
        "push"
    }

    async fn handle(&self, context: &Context, args: Vec<&str>) {
        write_line("");
    }
}
