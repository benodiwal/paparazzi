use anyhow::{Ok, Result};
use std::process::Command;

pub fn capture() -> Result<String> {
    // Create temp file with .png extension directly
    let temp_dir = tempfile::tempdir()?;
    let path = temp_dir.path().join("screenshot.png");
    let path_str = path.to_string_lossy().to_string();

    // Use macOS screencapture command
    // -i: interactive mode (allows user to select area or window)
    // -o: no shadow for window capture
    // Without -w flag to avoid the "could not create image from window" error
    let output = Command::new("screencapture")
        .arg("-i")  // Interactive mode - allows selection or window capture with space bar
        .arg("-o")  // No shadow
        .arg(&path_str)
        .output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("Screenshot capture failed"));
    }

    // Check if file was created (user might have cancelled)
    if !path.exists() {
        return Err(anyhow::anyhow!("Screenshot was cancelled"));
    }

    // Keep the temp directory alive by forgetting it
    std::mem::forget(temp_dir);
    Ok(path_str)
}