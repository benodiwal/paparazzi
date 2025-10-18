use anyhow::{Ok, Result};
use std::process::Command;

pub fn capture() -> Result<String> {
    let temp_dir = tempfile::tempdir()?;
    let path = temp_dir.path().join("screenshot.png");
    let path_str = path.to_string_lossy().to_string();

    // macOS screencapture command
    // -i: interactive mode (allows user to select area or window)
    // -o: no shadow for window capture
    let output = Command::new("screencapture")
        .arg("-i")  // Interactive mode - allows selection or window capture with space bar
        .arg("-o")  // No shadow
        .arg(&path_str)
        .output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("Screenshot capture failed"));
    }

    if !path.exists() {
        return Err(anyhow::anyhow!("Screenshot was cancelled"));
    }

    std::mem::forget(temp_dir);
    Ok(path_str)
}