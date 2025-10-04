use anyhow::{Ok, Result};
use screenshots::Screen;
use tempfile::NamedTempFile;

pub fn capture() -> Result<String> {
    let screens = Screen::all()?;
    let screen = screens
        .first()
        .ok_or_else(|| anyhow::anyhow!("No screens found"))?;
    let image = screen.capture()?;

    let temp_file = NamedTempFile::new()?.into_temp_path();
    let path = temp_file.to_path_buf().with_extension("png");
    image.save(&path)?;

    let path_str = path.to_string_lossy().to_string();
    std::mem::forget(temp_file);

    Ok(path_str)
}