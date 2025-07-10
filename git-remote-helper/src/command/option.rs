use super::{CommandHandler, Context};

use async_trait::async_trait;
use log::debug;

pub struct OptionHandler {}

#[async_trait]
impl CommandHandler for OptionHandler {
    fn name(&self) -> &'static str {
        "option"
    }

    async fn handle(&self, _: &Context, args: Vec<&str>) {
        // Shouldn't write anything.
    }
}
