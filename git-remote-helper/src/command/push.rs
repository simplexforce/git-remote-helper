
use super::CommandHandler;
use crate::{
    command::write_line, 
    remote::Remote,
};

use log::debug;

pub struct PushHandler {}

impl CommandHandler for PushHandler {
    fn name(&self) -> &'static str {
        "push"
    }

    fn handle(&self, remote: &impl Remote, args: Vec<&str>) {
        write_line("");
    }
}