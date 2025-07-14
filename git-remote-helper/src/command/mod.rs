mod capabilities;
mod connect;
mod fetch;
mod list;
mod option;
mod push;
mod stateless_connect;

pub use capabilities::*;
pub use connect::*;
pub use fetch::*;
pub use list::*;
pub use option::*;
pub use push::*;
pub use stateless_connect::*;

use crate::remote::Remote;

use async_trait::async_trait;
use log::debug;
use std::{collections::BTreeMap, io};

pub fn write_line(s: &str) {
    debug!(r#"Write "{}\n" "#, s);
    println!("{}", s);
}

pub struct Context {
    pub remote: Box<dyn Remote>,
    // pub reader: Box<dyn io::Read>,
    // pub writer: Box<dyn io::Write>,
}

unsafe impl Sync for Context {}

#[async_trait]
pub trait CommandHandler {
    fn name(&self) -> &'static str;
    async fn handle(&self, remote: &Context, args: Vec<&str>);
}

pub struct Handler {
    context: Context,
    handlers: BTreeMap<&'static str, Box<dyn CommandHandler>>,
}

impl Handler {
    pub fn new(
        context: Context,
        handlers: BTreeMap<&'static str, Box<dyn CommandHandler>>,
    ) -> Self {
        return Self { context, handlers };
    }

    pub async fn run(&self) {
        let mut buf: String = String::new();

        loop {
            buf.clear();

            io::stdin()
                .read_line(&mut buf)
                .expect("Failed to read from stdin");

            debug!(r#"Read: {:?}"#, buf);

            if buf.as_str() == "\n" {
                // Do nothing
                return;
            }

            let args: Vec<&str> = buf.trim().split(" ").collect();
            if args.len() == 0 {
                panic!(r#"Invalid command: {:?}"#, buf);
            }

            // NOTE: Use a map (command name -> command handler) instead of a match statement？
            // The CommandHandler trait is not dyn compatible. See https://github.com/dtolnay/async-trait
            let cmd = args[0];

            if let Some(handler) = self.handlers.get(cmd) {
                handler.handle(&self.context, args).await;
            }

            panic!("Unknown command");
        }
    }
}

pub struct HandlerMapBuilder<'a> {
    handlers: BTreeMap<&'static str, Box<dyn CommandHandler + 'a>>,
}

impl<'a> HandlerMapBuilder<'a> {
    pub fn new() -> Self {
        Self {
            handlers: BTreeMap::new(),
        }
    }

    pub fn cmd_handler(mut self, handler: impl CommandHandler + 'a) -> Self {
        self.handlers.insert(handler.name(), Box::new(handler));
        self
    }

    pub fn build(self) -> BTreeMap<&'static str, Box<dyn CommandHandler + 'a>> {
        self.handlers
    }
}
