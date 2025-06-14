// See https://git-scm.com/docs/gitprotocol-pack

use super::CommandHandler;
use crate::remote::Remote;

use log::debug;

pub struct ConnectHandler {}

impl CommandHandler for ConnectHandler {
    fn name(&self) -> &'static str {
        "connect"
    }

    fn handle(&self, remote: &impl Remote, args: Vec<&str>) {
        // Accept this command by a "\n"
        debug!(r#"Write "\n""#);
        println!();

        // Complete
        debug!(r#"Write "\n""#);
        println!();
    }
}