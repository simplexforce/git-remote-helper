
use super::CommandHandler;
use crate::remote::Remote;

use log::debug;

pub struct ListHandler {}

impl CommandHandler for ListHandler {
    fn name(&self) -> &'static str {
        "list"
    }

    fn handle(&self, remote: &impl Remote, args: Vec<&str>) {
        // TODO check args
        // list for-push

        let refs = remote.get_refs();

        for git_ref in refs.iter() {
            debug!(r#"Write "{}\n""#, git_ref);
            println!("{}", git_ref)
        }

        debug!(r#"Write "\n""#);
        println!()
    }
}