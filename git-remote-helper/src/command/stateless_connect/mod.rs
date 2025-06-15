// See https://git-scm.com/docs/gitprotocol-v2

// capabilities
// ls-refs
// fetch
// ...

use super::CommandHandler;
use crate::{command::write_line, remote::Remote};

use log::debug;

pub struct StatelessConnectHandler {}

impl CommandHandler for StatelessConnectHandler {
    fn name(&self) -> &'static str {
        "stateless-connect"
    }

    fn handle(&self, remote: &impl Remote, args: Vec<&str>) {
        if args.len() < 2 {
            panic!("Invalid number of arguments");
        }

        // Accept this command by a "\n"
        write_line("");

        let service  = args[1];
        debug!("service: {:?}", service);

        match service {
            "git-upload-pack" => {
                todo!()
            }
            "git-receive-pack" => {
                todo!()
            }
            _ => {
                panic!("invalid service")
            }
        }

    }
}