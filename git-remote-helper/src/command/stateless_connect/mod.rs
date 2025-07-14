// See https://git-scm.com/docs/gitprotocol-v2

// capabilities
// ls-refs
// fetch
// ...

use std::{io::{self, stdout, Read, Write}, thread::sleep, time};

use crate::{command::write, pktline::{wrap_pktline, FLUSH_PKT}};

use super::{CommandHandler, Context, write_line};

use async_trait::async_trait;
use log::debug;

pub struct StatelessConnectHandler {}

impl StatelessConnectHandler {
    async fn handle_upload_pack(&self, context: &Context) {
        // Stateless fetch would use:
        // - remote.list_refs() to list references
        // - remote.exists_object() for object negotiation
        // - remote.get_object() to retrieve objects
        // todo!();

        write(&wrap_pktline(b"version 2\n"));
        write(&wrap_pktline(b"agent=git/unknown-server\n"));
        write(&wrap_pktline(b"ls-refs=unborn\n"));
        write(&wrap_pktline(b"fetch=shallow wait-for-done filter\n"));
        write(&wrap_pktline(b"server-option\n"));
        write(&wrap_pktline(b"object-format=sha1\n"));
        write(FLUSH_PKT);
        stdout().flush()
            .expect("Failed to flush output stream");

        // TODO
        let mut buf = [0; 1024];
        loop {
            let n = io::stdin()
                .read(&mut buf)
                .expect("Failed to read from stdin");
            
            sleep(time::Duration::from_millis(200));
            debug!("Read: {:?}", String::from_utf8_lossy(&buf[..n]));
        }
    }

    async fn handle_receive_pack(&self, context: &Context) {
        // Stateless push would use:
        // - remote.list_push_refs() for push references
        // - remote.push_object() for object transfer
        // - remote.update_refs() to update references

        todo!()
    }
}

#[async_trait]
impl CommandHandler for StatelessConnectHandler {
    fn name(&self) -> &'static str {
        "stateless-connect"
    }

    async fn handle(&self, context: &Context, args: Vec<&str>) {
        if args.len() < 2 {
            panic!("Invalid number of arguments");
        }

        // Accept this command by a "\n"
        write_line("");

        let service = args[1];
        debug!("service: {:?}", service);

        match service {
            "git-upload-pack" => {
                self.handle_upload_pack(context).await
            }
            "git-receive-pack" => {
                self.handle_receive_pack(context).await
            }
            _ => {
                panic!("invalid service")
            }
        }
    }
}
