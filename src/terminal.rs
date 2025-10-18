use anyhow::Result;
use std::process::Command;
use std::path::{Path, PathBuf};
use crate::logger;

// Helper function to resolve script paths
fn get_script_path(script_name: &str) -> PathBuf {
    let common_paths = [
        "/usr/local/share/clipse/macos/applescripts",       // Installed location
        "/opt/clipse/macos/applescripts",                   // Alternative install
    ];

    for base in &common_paths {
        let path = Path::new(base).join(script_name);
        if path.exists() {
            return path;
        }
    }

    if let Ok(exe_path) = std::env::current_exe() {
        // For daemon mode, the executable might be in target/debug or target/release
        // We need to go up to find the project root
        let mut current = exe_path.as_path();

        for _ in 0..5 {
            let script_path = current.join("macos/applescripts").join(script_name);
            if script_path.exists() {
                return script_path;
            }

            if let Some(parent) = current.parent() {
                current = parent;
            } else {
                break;
            }
        }
    }

    let fallback = Path::new("macos/applescripts").join(script_name);

    if !fallback.exists() {
        Path::new("/Users/sachin/personal/clipse/macos/applescripts").join(script_name)
    } else {
        fallback
    }
}

#[cfg(target_os = "macos")]
pub fn send_to_claude_code_terminal(message: &str) -> Result<()> {
    let pids = find_claude_code_processes()?;
    logger::info(&format!("Found {} Claude Code processes: {:?}", pids.len(), pids));

    if send_to_iterm2_claude_tab(message).is_ok() {
        logger::success("Sent to Claude Code via iTerm2");
        return Ok(());
    }

    if send_to_terminal_app_claude_tab(message).is_ok() {
        logger::success("Sent to Claude Code via Terminal.app");
        return Ok(());
    }

    if send_to_ghostty_claude_tab(message).is_ok() {
        logger::success("Sent to Claude Code via Ghostty");
        return Ok(());
    }

    for pid in pids {
        println!("\nTrying PID: {}", pid);

        if let Ok(tty) = find_terminal_for_process(pid) {
            println!("TTY: {}", tty);

            if send_to_terminal_app_by_tty(&tty, message).is_ok() {
                logger::success("Sent via Terminal.app (TTY method)");
                return Ok(());
            }
        }
    }

    Err(anyhow::anyhow!("Could not send to Claude Code terminal. Make sure Claude Code is running in a terminal."))
}

#[cfg(target_os = "macos")]
fn find_claude_code_processes() -> Result<Vec<i32>> {
    // Look for processes with "claude" in the command line
    // This should catch both "claude" and "claude-code" commands
    let output = Command::new("sh")
        .arg("-c")
        .arg("ps aux | grep -E '(claude|claude-code)' | grep -v grep | awk '{print $2}'")
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    let pids: Vec<i32> = stdout
        .lines()
        .filter_map(|line| line.trim().parse::<i32>().ok())
        .collect();

    if pids.is_empty() {
        return Err(anyhow::anyhow!("No Claude Code processes found. Please make sure Claude Code is running."));
    }

    Ok(pids)
}

#[cfg(target_os = "macos")]
fn find_terminal_for_process(pid: i32) -> Result<String> {
    let output = Command::new("ps")
        .args(&["-p", &pid.to_string(), "-o", "tty="])
        .output()?;

    let tty = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if tty.is_empty() || tty == "??" {
        return Err(anyhow::anyhow!("No TTY found for process"));
    }

    let full_tty = if tty.starts_with("/dev/") {
        tty
    } else {
        format!("/dev/{}", tty)
    };

    Ok(full_tty)
}

// iTerm2
#[cfg(target_os = "macos")]
fn send_to_iterm2_claude_tab(message: &str) -> Result<()> {
    let script_path = get_script_path("iterm2_send.applescript");

    let output = Command::new("osascript")
        .arg(&script_path)
        .arg(message)
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("iTerm2 not available or error: {}", error));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let result = stdout.trim();

    if result == "false" {
        return Err(anyhow::anyhow!("Claude Code tab not found in iTerm2"));
    }

    Ok(())
}

// Terminal.app
#[cfg(target_os = "macos")]
fn send_to_terminal_app_claude_tab(message: &str) -> Result<()> {
    let script_path = get_script_path("terminal_send.applescript");

    let output = Command::new("osascript")
        .arg(&script_path)
        .arg(message)
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Terminal.app error: {}", error));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let result = stdout.trim();

    if result == "false" {
        return Err(anyhow::anyhow!("Claude Code tab not found in Terminal.app"));
    }

    Ok(())
}

// Ghostty
#[cfg(target_os = "macos")]
fn send_to_ghostty_claude_tab(message: &str) -> Result<()> {
    // Get Claude Code PIDs to identify the right terminal
    let claude_pids = find_claude_code_processes()?;

    // Check if any of the Claude processes are running in Ghostty
    for pid in &claude_pids {
        if let Ok(terminal_name) = get_terminal_name_for_process(*pid) {
            if terminal_name.to_lowercase().contains("ghostty") {
                // Found Claude running in Ghostty, proceed with the script
                let script_path = get_script_path("ghostty_send.applescript");

                let output = Command::new("osascript")
                    .arg(&script_path)
                    .arg(message)
                    .output()?;

                if !output.status.success() {
                    let error = String::from_utf8_lossy(&output.stderr);
                    return Err(anyhow::anyhow!("Ghostty automation error: {}", error));
                }

                let stdout = String::from_utf8_lossy(&output.stdout);
                let result = stdout.trim();

                if result == "true" {
                    return Ok(());
                }
            }
        }
    }

    Err(anyhow::anyhow!("Claude Code not found running in Ghostty"))
}

// Helper function to get terminal name for a process
#[cfg(target_os = "macos")]
fn get_terminal_name_for_process(pid: i32) -> Result<String> {
    // Get the parent process ID to find the terminal
    let output = Command::new("ps")
        .args(&["-p", &pid.to_string(), "-o", "ppid="])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let ppid_str = stdout.trim();
    let ppid: i32 = ppid_str.parse()?;

    // Get the command name of the parent process (should be the terminal)
    let output = Command::new("ps")
        .args(&["-p", &ppid.to_string(), "-o", "comm="])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let terminal_name = stdout.trim().to_string();

    if terminal_name.is_empty() {
        return Err(anyhow::anyhow!("Could not determine terminal name"));
    }

    Ok(terminal_name)
}

// Terminal.app
#[cfg(target_os = "macos")]
fn send_to_terminal_app_by_tty(tty: &str, message: &str) -> Result<()> {
    let script_path = get_script_path("terminal_tty_send.applescript");

    let output = Command::new("osascript")
        .arg(&script_path)
        .arg(tty)
        .arg(message)
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Terminal.app TTY error: {}", error));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let result = stdout.trim();

    if result == "false" {
        return Err(anyhow::anyhow!("TTY not found in Terminal.app"));
    }

    Ok(())
}