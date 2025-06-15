
use git_remote_helper::{
    command::{
        Handler,
        CapabilitiesHandler,
        ListHandler,
        PushHandler, FetchHandler,
        ConnectHandler, StatelessConnectHandler,
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

    let commander = Handler {
        remote,
        capabilities_handler: CapabilitiesHandler {
            capabilities: vec![
                "capabilities",
                "list",
                "fetch",
                "push",
                "connect",
                "*stateless-connect",
            ]
        },
        list_handler: ListHandler {},
        push_handler: PushHandler {},
        fetch_handler: FetchHandler {},
        connect_handler: ConnectHandler {},
        stateless_connect_handler: StatelessConnectHandler {},
    };

    commander.run().await;

    info!("Communication done.")
}
