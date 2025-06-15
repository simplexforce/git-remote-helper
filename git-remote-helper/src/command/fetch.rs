
use super::{write_line, CommandHandler};
use crate::remote::Remote;

use log::debug;

pub struct FetchHandler {}

impl CommandHandler for FetchHandler {
    fn name(&self) -> &'static str {
        "fetch"
    }

    async fn handle(&self, remote: &impl Remote, args: Vec<&str>) {
        write_line("");
    }
}