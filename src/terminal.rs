use anyhow::Result;

#[cfg(target_os = "macos")]
use crate::logger;
#[cfg(target_os = "macos")]
use std::fs;
#[cfg(target_os = "macos")]
use std::process::Command;
#[cfg(target_os = "macos")]
use tempfile::NamedTempFile;

// Embedded AppleScript content
#[cfg(target_os = "macos")]
const ITERM2_SCRIPT: &str = r#"on run argv
    set messageText to item 1 of argv

    tell application "iTerm"
        set foundSession to false

        -- Loop through all windows
        repeat with w in windows
            -- Loop through all tabs in the window
            repeat with t in tabs of w
                -- Loop through all sessions in the tab
                repeat with s in sessions of t
                    -- Get the session content
                    set sessionText to contents of s

                    -- Check if this session contains Claude Code indicators
                    if sessionText contains "claude" or sessionText contains "Claude" then
                        -- Switch to this window
                        select w
                        -- Switch to this tab
                        select t
                        -- Switch to this session
                        select s
                        -- Send the text
                        tell s to write text messageText
                        set foundSession to true
                        exit repeat
                    end if
                end repeat
                if foundSession then exit repeat
            end repeat
            if foundSession then exit repeat
        end repeat

        if foundSession then
            activate
            return "true"
        else
            return "false"
        end if
    end tell
end run"#;

#[cfg(target_os = "macos")]
const TERMINAL_SCRIPT: &str = r#"on run argv
    set messageText to item 1 of argv

    tell application "Terminal"
        set foundTab to false

        -- Loop through all windows
        repeat with w in windows
            -- Loop through all tabs in the window
            repeat with t in tabs of w
                -- Get the tab's processes
                set tabProcesses to processes of t

                -- Check if any process contains "claude"
                repeat with p in tabProcesses
                    if p contains "claude" or p contains "Claude" then
                        -- Switch to this window and tab
                        set frontmost of w to true
                        set selected of t to true
                        activate

                        -- Send the text
                        do script messageText in t
                        set foundTab to true
                        exit repeat
                    end if
                end repeat
                if foundTab then exit repeat
            end repeat
            if foundTab then exit repeat
        end repeat

        return foundTab
    end tell
end run"#;

#[cfg(target_os = "macos")]
const GHOSTTY_SCRIPT: &str = r#"on run argv
    set messageText to item 1 of argv

    -- First, copy the message to clipboard using pbcopy
    set the clipboard to messageText

    tell application "System Events"
        tell process "Ghostty"
            set frontmost to true

            -- Look for windows
            set windowList to windows
            if (count of windowList) > 0 then
                -- Focus the first/current window (assuming it's the active one)
                set targetWindow to item 1 of windowList
                perform action "AXRaise" of targetWindow
                set focused of targetWindow to true

                -- Wait a moment for focus
                delay 0.3

                -- Paste the content using Cmd+V
                key code 9 using command down

                -- Press return
                delay 0.1
                key code 36
                return "true"
            end if
        end tell
    end tell
    return "false"
end run"#;

#[cfg(target_os = "macos")]
const TERMINAL_TTY_SCRIPT: &str = r#"on run argv
    set ttyPath to item 1 of argv
    set messageText to item 2 of argv

    tell application "Terminal"
        set foundTab to false

        -- Loop through all windows
        repeat with w in windows
            -- Loop through all tabs in the window
            repeat with t in tabs of w
                -- Get the tab's tty
                set tabTty to tty of t

                -- Check if this matches our target tty
                if tabTty is equal to ttyPath then
                    -- Switch to this window and tab
                    set frontmost of w to true
                    set selected of t to true
                    activate

                    -- Send the text
                    do script messageText in t
                    set foundTab to true
                    exit repeat
                end if
            end repeat
            if foundTab then exit repeat
        end repeat

        return foundTab
    end tell
end run"#;

// Helper function to create temporary AppleScript file and return its path
#[cfg(target_os = "macos")]
fn create_temp_script(script_content: &str) -> Result<NamedTempFile> {
    let temp_file = NamedTempFile::new()?;
    fs::write(temp_file.path(), script_content)?;
    Ok(temp_file)
}

#[cfg(target_os = "macos")]
pub fn send_to_claude_code_terminal(message: &str) -> Result<()> {
    let pids = find_claude_code_processes()?;
    logger::info(&format!(
        "Found {} Claude Code processes: {:?}",
        pids.len(),
        pids
    ));

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

    Err(anyhow::anyhow!(
        "Could not send to Claude Code terminal. Make sure Claude Code is running in a terminal."
    ))
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
        return Err(anyhow::anyhow!(
            "No Claude Code processes found. Please make sure Claude Code is running."
        ));
    }

    Ok(pids)
}

#[cfg(target_os = "macos")]
fn find_terminal_for_process(pid: i32) -> Result<String> {
    let output = Command::new("ps")
        .args(["-p", &pid.to_string(), "-o", "tty="])
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
    let script_file = create_temp_script(ITERM2_SCRIPT)?;

    let output = Command::new("osascript")
        .arg(script_file.path())
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
    let script_file = create_temp_script(TERMINAL_SCRIPT)?;

    let output = Command::new("osascript")
        .arg(script_file.path())
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
        if let Ok(terminal_name) = get_terminal_name_for_process(*pid)
            && terminal_name.to_lowercase().contains("ghostty")
        {
            // Found Claude running in Ghostty, proceed with the script
            let script_file = create_temp_script(GHOSTTY_SCRIPT)?;

            let output = Command::new("osascript")
                .arg(script_file.path())
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

    Err(anyhow::anyhow!("Claude Code not found running in Ghostty"))
}

// Helper function to get terminal name for a process
#[cfg(target_os = "macos")]
fn get_terminal_name_for_process(pid: i32) -> Result<String> {
    // Get the parent process ID to find the terminal
    let output = Command::new("ps")
        .args(["-p", &pid.to_string(), "-o", "ppid="])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let ppid_str = stdout.trim();
    let ppid: i32 = ppid_str.parse()?;

    // Get the command name of the parent process (should be the terminal)
    let output = Command::new("ps")
        .args(["-p", &ppid.to_string(), "-o", "comm="])
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
    let script_file = create_temp_script(TERMINAL_TTY_SCRIPT)?;

    let output = Command::new("osascript")
        .arg(script_file.path())
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

#[cfg(not(target_os = "macos"))]
pub fn send_to_claude_code_terminal(_message: &str) -> Result<()> {
    Err(anyhow::anyhow!(
        "Terminal integration is only supported on macOS"
    ))
}
