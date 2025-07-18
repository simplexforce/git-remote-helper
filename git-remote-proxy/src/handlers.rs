use std::env;
use std::io::{self, Read, Stdin, Stdout, Write};
use std::process::{ChildStdin, ChildStdout};
use std::sync::{Arc, Mutex};

use log::{debug, error, info};

pub struct Context {
    pub proxy_stdout: Stdout,
    pub helper_stdin: ChildStdin,

    pub current_command: String,
}

impl Context {
    pub fn is_connect_cmd(&self) -> bool {
        match self.current_command.as_str() {
            "connect" => true,
            "stateless-connect" => true,
            _ => false,
        }
    }
}

pub struct GitHandler {
    pub proxy_stdin: Stdin,

    pub context: Arc<Mutex<Context>>,
}

impl GitHandler {
    pub fn read_from_git(&mut self) -> io::Result<()> {
        // let mut buf = String::new();
        let mut buf = [0; 65536];
        loop {
            // buf.clear();

            let n = self.proxy_stdin.read(&mut buf)?;
            // let n = self.proxy_stdin.read_line(&mut buf)?;
            if n == 0 {
                info!("Git stdin closed");
                break;
            }

            let str = String::from_utf8_lossy(&buf[..n]);

            debug!(r#"Read from git: {:?}"#, str);

            if str == "\n" {
                let mut context = self.context.lock().unwrap();
                // Forward blank line to helper
                context.helper_stdin.write_all(b"\n")?;
                context.helper_stdin.flush()?;
                break;
            }

            self.handle_commands(&str)?;
        }
        Ok(())
    }

    fn handle_commands(&mut self, line: &str) -> io::Result<()> {
        let args: Vec<&str> = line.trim().split(" ").collect();
        if args.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Empty command"));
        }

        let command = args[0];
        debug!("Processing command: {}", command);

        self.context.lock().unwrap().current_command = command.to_string();

        // Log based on command type
        match command {
            // TODO: For connect/stateless-connect commands, log in pktline format
            "capabilities" => {
                // Handle capabilities command separately
                return self.handle_capabilities();
            }
            _ => {
                info!("[GIT -> HELPER] \"{}\\n\"", line.trim_end());
            }
        }

        // Forward to real helper (line already includes newline)
        let mut context = self.context.lock().unwrap();
        context.helper_stdin.write_all(line.as_bytes())?;
        context.helper_stdin.flush()?;
        Ok(())
    }

    fn handle_capabilities(&mut self) -> io::Result<()> {
        let mut context =  self.context.lock().unwrap();

        // Check if shadow capabilities are configured
        if let Ok(shadow_caps) = env::var("GIT_PROXY_CAPABILITIES") {
            // Use configured shadow capabilities
            let capabilities: Vec<&str> = shadow_caps.split(',').collect();

            // Write capabilities to git stdout
            for cap in capabilities {
                writeln!(context.proxy_stdout, "{}", cap)?;
                info!("[PROXY -> GIT] \"{}\\n\"", cap)
            }

            // Write empty line to terminate
            writeln!(context.proxy_stdout)?;
            info!("Shadowed capabilities: {}", shadow_caps);
            return Ok(());
        }

        // Forward to real helper when no shadowing
        info!("Forwarding capabilities command to real helper");
        context.helper_stdin.write_all(b"capabilities\n")?;
        context.helper_stdin.flush()?;
        Ok(())
    }
}

pub struct HelperHandler {
    pub helper_stdout: ChildStdout,

    pub context: Arc<Mutex<Context>>,
}

impl HelperHandler {
    pub fn read_from_helper(&mut self) -> io::Result<()> {
        // let mut buf = String::new();
        let mut buf = [0 as u8; 65536];
        loop {
            // buf.clear();

            let n = self.helper_stdout.read(&mut buf)?;

            // Read line from helper
            // let n = self.helper_stdout.read_line(&mut buf)?;
            if n == 0 {
                error!("Helper stdout closed");
                break;
            }

            let str = String::from_utf8_lossy(&buf[..n]);

            debug!(r#"Read from helper: {:?}"#, str);

            let mut context = self.context.lock().unwrap();
            // Check if we're in a connect/stateless-connect command
            let is_connect_cmd = context.is_connect_cmd();

            // Log and forward
            if is_connect_cmd {
                // Check for termination packet
                if str.ends_with("0000") {
                    info!("Termination packet received, ending {} state", context.current_command);
                    context.current_command = "".to_string();

                    // For connect commands, log in pktline format
                    info!("[HELPER -> GIT] \"{}\"", str.trim_end());
                } else {
                    // For connect commands, log in pktline format
                    info!("[HELPER -> GIT] \"{}\\n\"", str.trim_end());
                }
            } else {
                info!("[HELPER -> GIT] \"{}\\n\"", str.trim_end());
            }

            context.proxy_stdout.write_all(&buf[..n])?;
            context.proxy_stdout.flush()?;
        }
        Ok(())
    }
}
