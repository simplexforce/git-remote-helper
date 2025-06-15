
use super::CommandHandler;
use crate::remote::Remote;

use log::debug;

pub struct PushHandler {}

impl CommandHandler for PushHandler {
    fn name(&self) -> &'static str {
        "push"
    }

    fn handle(&self, remote: &impl Remote, args: Vec<&str>) {
        debug!(r#"Write "\n""#);
        println!();
    }
}