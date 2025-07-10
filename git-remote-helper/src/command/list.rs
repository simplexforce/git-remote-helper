use super::{CommandHandler, Context, write_line};

use async_trait::async_trait;
use log::debug;

pub struct ListHandler {}

#[async_trait]
impl CommandHandler for ListHandler {
    fn name(&self) -> &'static str {
        "list"
    }

    async fn handle(&self, context: &Context, args: Vec<&str>) {
        // TODO check args
        // list for-push

        let refs = context.remote.get_refs().await;

        for git_ref in refs.iter() {
            write_line(git_ref);
        }

        write_line("");
    }
}
