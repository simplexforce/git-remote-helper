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
        // Validate arguments: either no arguments or "for-push"
        if args.len() > 1 {
            return write_line("error: too many arguments for list command");
        }

        let for_push =  match args.get(0) {
            Some(arg) => *arg == "for-push",
            _ => false
        };

        // If for-push is requested, list references suitable for push
        if for_push {
            let refs = context.remote.list_push_refs().await;

            // For push operations, list refs that should be pushed
            for git_ref in refs.iter() {
                write_line(git_ref);
            }
        } else {
            let refs = context.remote.list_refs().await;

            // Normal listing of all refs
            for git_ref in refs.iter() {
                write_line(git_ref);
            }
        }

        write_line("");
    }
}
