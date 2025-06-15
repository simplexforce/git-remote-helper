// See https://git-scm.com/docs/gitprotocol-pack

use super::CommandHandler;
use crate::{command::write_line, remote::Remote};

use log::debug;

pub struct ConnectHandler {}

impl CommandHandler for ConnectHandler {
    fn name(&self) -> &'static str {
        "connect"
    }

    async fn handle(&self, remote: &impl Remote, args: Vec<&str>) {
        // Accept this command by a "\n"
        write_line("");

        // Complete
        write_line("");
    }
}