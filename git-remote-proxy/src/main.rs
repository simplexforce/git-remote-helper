mod handlers;
mod utils;
mod config;

use std::sync::{Arc, Mutex};
use std::{io, thread};

use log::{debug, info};

use handlers::*;
use utils::*;
use config::*;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let (remote, url) = load_args();

    let config = load_env();

    update_git_config_file(&format!("{}/config", config.git_dir), &config.git_proxy_helper);

    // Transform URL by replacing proxy scheme with helper name
    let transformed_url = url.replace("proxy://", &format!("{}://", &config.git_proxy_helper));
    debug!("Transformed URL: {}", transformed_url);

    use_git_exec_path();

    // Create real helper command by prepending "git-remote-"
    let real_helper = format!("git-remote-{}", &config.git_proxy_helper);
    debug!("Real helper: {}", real_helper);

    let (
        mut helper, 
        helper_stdin, 
        helper_stdout,
    ) = spawn_real_helper(&real_helper, &remote, &transformed_url);

    let proxy_stdin = io::stdin();
    let proxy_stdout = io::stdout();

    let (mut git_handler, mut helper_handler) = {
        let context = Arc::new(Mutex::new(Context {
            proxy_stdout,
            helper_stdin,
            current_command: String::new(),
        }));

        let git_handler = GitHandler {
            proxy_stdin,
            context: context.clone(),
        };

        let helper_handler = HelperHandler {
            helper_stdout,
            context: context.clone(),
        };

        (git_handler, helper_handler)
    };

    info!("Starting bidirectional forwarding");

    // Thread to forward Git -> Proxy -> Helper
    let t1 = thread::spawn(move || -> io::Result<()> { 
        git_handler.read_from_git()
    });

    // Thread to forward Helper -> Proxy -> Git
    let t2 = thread::spawn(move || -> io::Result<()> {
        helper_handler.read_from_helper()
    });

    // Wait for threads to finish
    t1.join().unwrap()?;
    t2.join().unwrap()?;

    info!("Forwarding threads completed");

    // Wait for helper process to exit
    let status = helper.wait()?;
    let exit_code = status.code().unwrap_or(0);
    info!("Helper process exited with code: {}", exit_code);
    std::process::exit(exit_code);
}
