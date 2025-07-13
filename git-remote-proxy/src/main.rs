mod handlers;
mod utils;

use std::sync::{Arc, Mutex};
use std::{env, io, thread};

use log::{debug, error, info};

use handlers::*;
use utils::*;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        error!("Usage: git-remote-proxy <remote-name> <url>");
        std::process::exit(1);
    }
    let remote_name = &args[1];
    let url = &args[2];

    info!("Starting git-remote-proxy for {}: {}", remote_name, url);

    // Get real helper name from environment variable
    let helper_name = env::var("GIT_PROXY_HELPER")
        .expect("GIT_PROXY_HELPER environment variable not set");
    info!("Using helper name: {}", helper_name);

    // Transform URL by replacing proxy scheme with helper name
    let transformed_url = url.replace("proxy://", &format!("{}://", helper_name));
    info!("Transformed URL: {}", transformed_url);

    use_git_exec_path();

    // Create real helper command by prepending "git-remote-"
    let real_helper = format!("git-remote-{}", helper_name);
    debug!("Real helper: {}", real_helper);

    let (
        mut helper, 
        helper_stdin, 
        helper_stdout,
    ) = spawn_real_helper(&real_helper, &remote_name, &transformed_url);

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
