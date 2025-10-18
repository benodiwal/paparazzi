use chrono::{DateTime, Local};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum LogLevel {
    Info,
    Success,
    Error,
    Warning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    pub level: String,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: "all".to_string(),
        }
    }
}

fn get_log_config_path() -> PathBuf {
    home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".paparazzi")
        .join("log_config.json")
}

fn load_log_config() -> LogConfig {
    let config_path = get_log_config_path();
    if config_path.exists()
        && let Ok(content) = fs::read_to_string(&config_path)
        && let Ok(config) = serde_json::from_str::<LogConfig>(&content)
    {
        return config;
    }
    LogConfig::default()
}

fn should_log(level: &LogLevel) -> bool {
    let config = load_log_config();
    match config.level.as_str() {
        "off" => false,
        "info" => matches!(level, LogLevel::Info),
        "success" => matches!(level, LogLevel::Success),
        "error" => matches!(level, LogLevel::Error),
        "warning" => matches!(level, LogLevel::Warning),
        "all" => true,
        _ => true, // Default to showing all logs
    }
}

pub fn log(level: LogLevel, message: &str) {
    if !should_log(&level) {
        return;
    }

    let timestamp: DateTime<Local> = Local::now();
    let formatted_time = timestamp.format("%Y-%m-%d %H:%M:%S%.3f");

    let prefix = match level {
        LogLevel::Info => "INFO",
        LogLevel::Success => "SUCCESS",
        LogLevel::Error => "ERROR",
        LogLevel::Warning => "WARN",
    };

    println!("[{} - {}] {}", formatted_time, prefix, message);
}

pub fn info(message: &str) {
    log(LogLevel::Info, message);
}

pub fn success(message: &str) {
    log(LogLevel::Success, message);
}

#[allow(unused)]
pub fn error(message: &str) {
    log(LogLevel::Error, message);
}

#[allow(unused)]
pub fn warning(message: &str) {
    log(LogLevel::Warning, message);
}

pub fn save_log_config(level: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config = LogConfig {
        level: level.to_string(),
    };

    let config_path = get_log_config_path();

    // Create directory if it doesn't exist
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string_pretty(&config)?;
    fs::write(&config_path, json)?;

    Ok(())
}

pub fn get_current_log_level() -> String {
    load_log_config().level
}
