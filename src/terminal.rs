use anyhow::{Context, Result};
use std::process::Command;

#[cfg(target_os = "macos")]
pub fn send_to_claude_code_terminal(message: &str) -> Result<()> {
    // ALL processes running claude
    let pids = find_all_claude_processes()?;
    println!("Found {} Claude processes: {:?}", pids.len(), pids);

    // Trying each PID until we find one that works
    for pid in pids {
        println!("\nTrying PID: {}", pid);

        if let Ok(tty) = find_terminal_for_process(pid) {
            println!("TTY: {}", tty);

            // Try different terminal emulators
            if send_to_ghostty(&tty, message).is_ok() {
                println!("✅ Sent via Ghostty");
                return Ok(());
            }

            if send_to_terminal_app(&tty, message).is_ok() {
                println!("✅ Sent via Terminal.app");
                return Ok(());
            }

            if send_to_iterm2(&tty, message).is_ok() {
                println!("✅ Sent via iTerm2");
                return Ok(());
            }
        }
    }

    Err(anyhow::anyhow!("Could not send to any terminal"))
}

#[cfg(target_os = "macos")]
fn find_all_claude_processes() -> Result<Vec<i32>> {
    let output = Command::new("pgrep").arg("-f").arg("claude").output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    let pids: Vec<i32> = stdout
        .lines()
        .filter_map(|line| line.trim().parse::<i32>().ok())
        .collect();

    if pids.is_empty() {
        return Err(anyhow::anyhow!("No Claude Code processes found"));
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

// Ghostty
#[cfg(target_os = "macos")]
fn send_to_ghostty(tty: &str, message: &str) -> Result<()> {
    let escaped = message.replace("\\", "\\\\").replace("\"", "\\\"");

    let script = format!(
        r#"
        tell application "Ghostty"
            activate
        end tell
        
        tell application "System Events"
            tell process "Ghostty"
                keystroke "{}"
                keystroke return
            end tell
        end tell
        "#,
        escaped
    );

    let output = Command::new("osascript").arg("-e").arg(&script).output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("Ghostty AppleScript failed"));
    }

    Ok(())
}

// Terminal.app
#[cfg(target_os = "macos")]
fn send_to_terminal_app(tty: &str, message: &str) -> Result<()> {
    let escaped = message.replace("\\", "\\\\").replace("\"", "\\\"");

    let script = format!(
        r#"
        tell application "Terminal"
            set foundTab to false
            
            repeat with w in windows
                repeat with t in tabs of w
                    set currentTTY to tty of t
                    
                    if currentTTY is equal to "{}" then
                        set selected of t to true
                        set frontmost of w to true
                        activate
                        
                        do script "{}" in t
                        set foundTab to true
                        exit repeat
                    end if
                end repeat
                
                if foundTab then exit repeat
            end repeat
            
            return foundTab
        end tell
        "#,
        tty, escaped
    );

    let output = Command::new("osascript").arg("-e").arg(&script).output()?;

    let result = String::from_utf8_lossy(&output.stdout);
    let result = result.trim();
    if result == "false" {
        return Err(anyhow::anyhow!("TTY not found in Terminal.app"));
    }

    Ok(())
}

// iTerm2
#[cfg(target_os = "macos")]
fn send_to_iterm2(tty: &str, message: &str) -> Result<()> {
    let escaped = message.replace("\\", "\\\\").replace("\"", "\\\"");

    let script = format!(
        r#"
        tell application "iTerm"
            tell current session of current window
                write text "{}"
            end tell
            activate
        end tell
        "#,
        escaped
    );

    let output = Command::new("osascript").arg("-e").arg(&script).output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("iTerm2 not available"));
    }

    Ok(())
}
