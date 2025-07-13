use log::{debug, info};
use std::{
    env,
    path::PathBuf,
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
};

pub fn use_git_exec_path() {
    // Get current PATH
    let old_path = env::var_os("PATH").unwrap_or_default();

    let git_exec_path = get_git_exec_path();

    debug!("Git exec path: {}", git_exec_path);

    // Construct new PATH
    let mut paths = env::split_paths(&old_path)
        .collect::<Vec<_>>();
    paths.insert(0, PathBuf::from(git_exec_path));

    // Update PATH
    let new_path_env = env::join_paths(paths)
        .expect("Failed to join paths");
    unsafe {
        env::set_var("PATH", new_path_env);
    }
}

// Get git exec path by running `git --exec-path` command
pub fn get_git_exec_path() -> String {
    let git_exec_path = Command::new("git")
        .arg("--exec-path")
        .output()
        .expect("Failed to get output of `Git --exec-path`")
        .stdout
        .into_iter()
        .take_while(|&c| c != b'\n')
        .collect::<Vec<_>>();

    let git_exec_path = String::from_utf8(git_exec_path)
        .expect("Failed to get Git exec path");

    git_exec_path
}

pub fn spawn_real_helper(
    name: &str,
    remote: &str,
    url: &str,
) -> (Child, ChildStdin, ChildStdout) {
    info!("Spawned real helper command: {}", name);

    // Spawn real helper process
    let mut helper = Command::new(&name)
        .arg(remote)
        .arg(url)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to spawn real helper");

    let helper_stdin = helper.stdin.take()
        .expect("Failed to take helper stdin");

    let helper_stdout = helper.stdout.take()
        .expect("Failed to take helper stdout");

    (helper, helper_stdin, helper_stdout)
}
