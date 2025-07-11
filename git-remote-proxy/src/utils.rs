use log::{error, info};
use std::{
    env, io,
    path::PathBuf,
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
};

pub fn get_git_exec_path() -> anyhow::Result<String> {
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

    Ok(git_exec_path)
}

pub fn use_git_exec_path() {
    // Get current PATH
    let old_path = env::var_os("PATH").unwrap_or_default();

    // Get git exec path by running `git --exec-path` command
    let git_exec_path = get_git_exec_path().expect("Failed to get git exec path");
    info!("Git exec path: {}", git_exec_path);
    let new_path_buf = PathBuf::from(git_exec_path);

    // Construct new PATH
    let mut paths = env::split_paths(&old_path).collect::<Vec<_>>();
    paths.insert(0, new_path_buf.clone());

    // Update PATH
    let new_path_env = env::join_paths(paths).expect("Failed to join paths");
    unsafe {
        env::set_var("PATH", new_path_env);
    }
}

pub fn spawn_real_helper(
    name: &str,
    remote: &str,
    url: &str,
) -> anyhow::Result<(Child, ChildStdin, ChildStdout)> {
    // Spawn real helper process
    let mut helper = Command::new(&name)
        .arg(remote)
        .arg(url)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;
    info!("Spawned real helper command: {}", name);

    let helper_stdin = helper.stdin.take().ok_or_else(|| {
        let err_msg = "Failed to take helper stdin";
        error!("{}", err_msg);
        io::Error::new(io::ErrorKind::Other, err_msg)
    })?;

    let helper_stdout = helper.stdout.take().ok_or_else(|| {
        let err_msg = "Failed to take helper stdout";
        error!("{}", err_msg);
        io::Error::new(io::ErrorKind::Other, err_msg)
    })?;

    Ok((helper, helper_stdin, helper_stdout))
}
