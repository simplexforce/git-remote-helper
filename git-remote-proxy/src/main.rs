use std::env;
use std::io::{self, Read, Write};
use std::process::{Command, Stdio};
use std::thread;
use log::{info, error, debug};

fn main() -> io::Result<()> {
    // Initialize logger
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
    let helper_name = env::var("GIT_REMOTE_PROXY_HELPER").map_err(|_| {
        let err_msg = "GIT_REMOTE_PROXY_HELPER environment variable not set";
        error!("{}", err_msg);
        io::Error::new(io::ErrorKind::Other, err_msg)
    })?;
    info!("Using helper name: {}", helper_name);

    // Transform URL by replacing proxy scheme with helper name
    let transformed_url = url.replace("proxy://", &format!("{}://", helper_name));
    info!("Transformed URL: {}", transformed_url);

    // Get git exec path by running `git --exec-path` command
    let git_exec_path = Command::new("git")
        .arg("--exec-path")
        .output()?
        .stdout
        .into_iter()
        .take_while(|&c| c != b'\n')
        .collect::<Vec<_>>();
    let git_exec_path = String::from_utf8(git_exec_path).map_err(|e| {
        let err_msg = format!("Failed to parse git exec path: {}", e);
        error!("{}", err_msg);
        io::Error::new(io::ErrorKind::Other, err_msg)
    })?;
    info!("Git exec path: {}", git_exec_path);

    // Create real helper command by prepending "git-remote-"
    let real_helper = format!("{}/git-remote-{}", git_exec_path, helper_name);
    debug!("Real helper: {}", real_helper);

    // Spawn real helper process
    let mut child = Command::new(&real_helper)
        .arg(remote_name)
        .arg(&transformed_url)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;
    info!("Spawned real helper command: {}", real_helper);

    let mut helper_stdin = child.stdin.take().ok_or_else(|| {
        let err_msg = "Failed to take helper stdin";
        error!("{}", err_msg);
        io::Error::new(io::ErrorKind::Other, err_msg)
    })?;
    
    let mut helper_stdout = child.stdout.take().ok_or_else(|| {
        let err_msg = "Failed to take helper stdout";
        error!("{}", err_msg);
        io::Error::new(io::ErrorKind::Other, err_msg)
    })?;

    let mut git_stdin = io::stdin();
    let mut git_stdout = io::stdout();
    
    info!("Starting bidirectional forwarding");

    // Thread to forward Git -> Proxy -> Helper
    let t1 = thread::spawn(move || -> io::Result<()> {
        let mut buffer = [0; 4096];
        loop {
            let n = git_stdin.read(&mut buffer)?;
            if n == 0 {
                info!("Git stdin closed");
                break;
            }
            let chunk = &buffer[..n];
            info!("[GIT -> PROXY] {}", String::from_utf8_lossy(chunk));
            helper_stdin.write_all(chunk)?;
        }
        Ok(())
    });

    // Thread to forward Helper -> Proxy -> Git
    let t2 = thread::spawn(move || -> io::Result<()> {
        let mut buffer = [0; 4096];
        loop {
            let n = helper_stdout.read(&mut buffer)?;
            if n == 0 {
                info!("Helper stdout closed");
                break;
            }
            let chunk = &buffer[..n];
            info!("[HELPER -> PROXY] {}", String::from_utf8_lossy(chunk));
            git_stdout.write_all(chunk)?;
        }
        Ok(())
    });

    // Wait for threads to finish
    t1.join().unwrap()?;
    t2.join().unwrap()?;
    
    info!("Forwarding threads completed");

    // Wait for helper process to exit
    let status = child.wait()?;
    let exit_code = status.code().unwrap_or(0);
    info!("Helper process exited with code: {}", exit_code);
    std::process::exit(exit_code);
}
