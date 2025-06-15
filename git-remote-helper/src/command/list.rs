
use super::{write_line, CommandHandler};
use crate::remote::Remote;

use log::debug;

pub struct ListHandler {}

impl CommandHandler for ListHandler {
    fn name(&self) -> &'static str {
        "list"
    }

    async fn handle(&self, remote: &impl Remote, args: Vec<&str>) {
        // TODO check args
        // list for-push

        let refs = remote.get_refs().await;

        for git_ref in refs.iter() {
            write_line(git_ref);
        }

        write_line("");
    }
}