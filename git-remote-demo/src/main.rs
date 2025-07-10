
use git_remote_helper::{
    command::{
        CapabilitiesHandler, ConnectHandler, Context, FetchHandler, Handler, HandlerMapBuilder, ListHandler, PushHandler, StatelessConnectHandler
    },
    remote::MemoryRemote,
};

use log::info;

#[tokio::main]
async fn main() {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    info!("Command invoked with args: {:?}", args);

    let mut remote = MemoryRemote::new();
    remote.head = "@refs/heads/main HEAD".to_string();
    remote.refs = vec![
        "8f2cea9673ed3d08ced6aa62281d86e5a6c344b9 refs/heads/main".to_string(),
    ];

    let commands = HandlerMapBuilder::new()
        .cmd_handler(CapabilitiesHandler {
            capabilities: vec![
                "capabilities",
                "list",
                "fetch",
                "push",
                "connect",
                "*stateless-connect",
            ]
        })
        .cmd_handler(ListHandler {})
        .cmd_handler(PushHandler {})
        .cmd_handler(FetchHandler {})
        .cmd_handler(ConnectHandler {})
        .cmd_handler(StatelessConnectHandler {})
        .build();
        
    let context = Context {
        remote: Box::new(remote),
    };
    
    let handler = Handler::new(context, commands);

    handler.run().await;

    info!("Communication done.")
}
