use crate::constants::{PID_FILE, STDERR_FILE, STDOUT_FILE};
use anyhow::Result;
use daemonize::Daemonize;
use std::fs::{self, File};
use std::path::PathBuf;
use sysinfo::{Pid, System};

pub struct Daemon;

impl Daemon {
    pub fn new() -> Self {
        Daemon
    }

    pub fn start(&self) -> Result<()> {
        if self.is_running()? {
            anyhow::bail!("Clipse daemon is already running. Use 'clipse stop' to stop it first.");
        }

        let stdout = File::create(STDOUT_FILE)?;
        let stderr = File::create(STDERR_FILE)?;

        let daemonize = Daemonize::new()
            .pid_file(PID_FILE)
            .working_directory("/tmp")
            .stdout(stdout)
            .stderr(stderr);

        match daemonize.start() {
            Ok(_) => Ok(()),
            Err(e) => {
                anyhow::bail!("Failed to daemonize: {}", e);
            }
        }
    }

    pub fn stop(&self) -> Result<()> {
        let pid = read_pid_file()?;

        #[cfg(unix)]
        {
            use nix::sys::signal::{Signal, kill};
            use nix::unistd::Pid as NixPid;

            kill(NixPid::from_raw(pid), Signal::SIGTERM)?;
        }

        std::thread::sleep(std::time::Duration::from_millis(500));

        if !self.is_running()? {
            let _ = fs::remove_file(PID_FILE);
            Ok(())
        } else {
            anyhow::bail!("Failed to stop the daemon. It may require manual intervention.");
        }
    }

    pub fn status(&self) -> Result<()> {
        match self.is_running()? {
            true => {
                let pid = read_pid_file()?;
                println!("Clipse daemon is running");
                println!("   PID: {}", pid);
                println!("   PID file: {}", PID_FILE);
                println!("   Logs: {}", STDOUT_FILE);

                let mut sys = System::new_all();
                sys.refresh_all();

                if let Some(process) = sys.process(Pid::from(pid as usize)) {
                    println!("   Memory: {} KB", process.memory() / 1024);
                    println!("   CPU: {:.2}%", process.cpu_usage());
                }
            }
            false => {
                println!("Clipse daemon is not running");
                println!("\nStart it with: clipse run --background");
            }
        }

        Ok(())
    }

    pub fn is_running(&self) -> Result<bool> {
        let pid = match read_pid_file() {
            Ok(pid) => pid,
            Err(_) => return Ok(false),
        };

        let mut sys = System::new_all();
        sys.refresh_all();

        Ok(sys.process(Pid::from(pid as usize)).is_some())
    }
}

fn read_pid_file() -> Result<i32> {
    let pid_str = fs::read_to_string(PID_FILE)?;
    let pid = pid_str.trim().parse::<i32>()?;
    Ok(pid)
}

pub fn get_log_file_path() -> PathBuf {
    PathBuf::from(STDOUT_FILE)
}

pub fn show_logs() -> Result<()> {
    let log_path = get_log_file_path();
    if !log_path.exists() {
        println!("No log file found at {}", log_path.display());
        println!("\nThe daemon might not be running or hasn't been started yet.");
        return Ok(());
    }

    let logs = std::fs::read_to_string(&log_path)?;
    println!("{}", logs);

    println!("{}", "=".repeat(60));
    println!(
        "\nTip: Use 'tail -f {}' to follow logs in real-time",
        log_path.display()
    );

    Ok(())
}
