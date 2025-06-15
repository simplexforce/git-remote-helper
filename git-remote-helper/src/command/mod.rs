
mod capabilities;
mod list;
mod push;
mod fetch;
mod connect;
mod stateless_connect;

pub use capabilities::*;
pub use list::*;
pub use push::*;
pub use fetch::*;
pub use connect::*;
pub use stateless_connect::*;

use crate::remote::Remote;

use log::debug;
use std::io;

pub fn write_line(s: &str) {
    debug!(r#"Write "{}\n" "#, s);
    println!("{}", s);
}

pub trait CommandHandler {
    fn name(&self) -> &'static str;
    fn handle(&self, remote: &impl Remote, args: Vec<&str>);
}

pub struct Handler<T: Remote> {
    pub remote: T,

    // Command handlers
    pub capabilities_handler: CapabilitiesHandler,
    pub list_handler: ListHandler,
    pub push_handler: PushHandler,
    pub fetch_handler: FetchHandler,
    pub connect_handler: ConnectHandler,
    pub stateless_connect_handler: StatelessConnectHandler,
}

impl <T : Remote>Handler<T> {
    pub fn run(&self) {
        let mut buf: String = String::new();

        loop {
            buf.clear();

            io::stdin()
                .read_line(&mut buf)
                .expect("Failed to read from stdin");

            debug!(r#"Read: {:?}"#, buf);

            if buf.as_str() == "" {
                return
            }

            if buf.as_str() == "\n" {
                // Do nothing
                continue;
            }

            let args: Vec<&str> = buf.trim().split(" ").collect();
            if args.len() == 0 {
                panic!(r#"Invalid command: {:?}"#, buf);
            }

            // NOTE: Use a map (command name -> command handler) instead of a match statement？
            // The CommandHandler trait is not dyn compatible. See https://github.com/dtolnay/async-trait
            // 
            // Not worth importing async-trait for this case
            let cmd = args[0];
            match cmd {
                "capabilities" => {
                    self.capabilities_handler.handle(&self.remote, args);
                }
                "list" => {
                    self.list_handler.handle(&self.remote, args);
                }
                "fetch" => {
                    self.fetch_handler.handle(&self.remote, args);
                }
                "push" => {
                    self.push_handler.handle(&self.remote, args);
                }
                "connect" => {
                    self.connect_handler.handle(&self.remote, args);
                }
                "stateless-connect" => {
                    self.stateless_connect_handler.handle(&self.remote, args);
                }
                _ => {
                    panic!("Unknown command")
                }
            }
        }
    }
}
