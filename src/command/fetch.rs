
use super::CommandHandler;
use crate::remote::Remote;

use log::debug;

pub struct FetchHandler {}

impl CommandHandler for FetchHandler {
    fn name(&self) -> &'static str {
        "fetch"
    }

    fn handle(&self, remote: &impl Remote, args: Vec<&str>) {
        debug!(r#"Write "\n""#);
        println!();
    }
}