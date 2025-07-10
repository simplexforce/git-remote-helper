// See https://git-scm.com/docs/gitprotocol-pack

use super::{CommandHandler, Context};
use crate::command::write_line;

use async_trait::async_trait;
use log::debug;

pub struct ConnectHandler {}

#[async_trait]
impl CommandHandler for ConnectHandler {
    fn name(&self) -> &'static str {
        "connect"
    }

    async fn handle(&self, _remote: &Context, _args: Vec<&str>) {
        // Accept this command by a "\n"
        write_line("");

        // Complete
        write_line("");
    }
}
