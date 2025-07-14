use std::env;

use log::{error, info};

pub struct Config {
    
}

pub struct Env {
    pub git_proxy_helper: String,
    pub git_dir: String,
    pub git_proxy_capabilities: Option<Vec<String>>,
}

pub fn load_env() -> Env {
    // Get real helper name from environment variable
    let git_proxy_helper = env::var("GIT_PROXY_HELPER")
        .expect("GIT_PROXY_HELPER environment variable not set");
    info!("GIT_PROXY_HELPER={}", git_proxy_helper);

    let git_dir = env::var("GIT_DIR")
        .expect("GIT_DIR environment variable not set");
    info!("GIT_DIR={}", git_dir);

    let mut git_proxy_capabilities = None;
    if let Some(caps) = env::var("GIT_PROXY_CAPABILITIES").ok() {
        // Use configured shadow capabilities
        let capabilities: Vec<String> = caps.split(',')
            .map(|s| s.to_string())
            .collect();

        git_proxy_capabilities = Some(capabilities);
    };

    Env {
        git_proxy_capabilities,
        git_proxy_helper, 
        git_dir
    }
}

pub fn load_args() -> (String, String) {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        error!("Usage: git-remote-proxy <remote-name> <url>");
        std::process::exit(1);
    }

    let remote = &args[1];
    let url = &args[2];

    info!("Starting git-remote-proxy for {}: {}", remote, url);

    (remote.clone(), url.clone())
}