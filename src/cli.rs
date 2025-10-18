use clap::{Parser, Subcommand};
use global_hotkey::hotkey::{Code, Modifiers};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "clipse")]
#[command(author = "Sachin Beniwal")]
#[command(version = "0.1.0")]
#[command(about = "A CLI tool for instant screenshots to Claude Code", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start the screenshot service
    Run {
        /// Run as background daemon
        #[arg(short, long)]
        background: bool,
    },
    /// Stop the background daemon
    Stop,
    /// Check daemon status
    Status,
    /// View daemon logs
    Logs,
    /// Attach to running daemon (bring to foreground)
    Attach {
        #[arg(short, long)]
        follow: bool,
    },
    /// Configure hotkeys for taking screenshots
    Hotkeys {
        /// Set the modifier keys (e.g., "ctrl+shift", "cmd+alt")
        #[arg(short, long)]
        modifiers: Option<String>,
        /// Set the key (e.g., "s", "p", "x")
        #[arg(short, long)]
        key: Option<String>,
        /// List current hotkey configuration
        #[arg(short, long)]
        list: bool,
    },
    /// Configure logging settings
    Logging {
        /// Set log level (info, success, error, warning, all, off)
        #[arg(short, long)]
        level: Option<String>,
        /// Show current logging configuration
        #[arg(short, long)]
        show: bool,
    },
    /// Display version information
    Version,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyConfig {
    pub modifiers: String,
    pub key: String,

    #[serde(skip)]
    pub modifiers_parsed: Option<Modifiers>,
    #[serde(skip)]
    pub key_parsed: Option<Code>,
}

impl Default for HotkeyConfig {
    fn default() -> Self {
        HotkeyConfig {
            modifiers: "ctrl+shift".to_string(),
            key: "s".to_string(),
            modifiers_parsed: Some(Modifiers::CONTROL | Modifiers::SHIFT),
            key_parsed: Some(Code::KeyS),
        }
    }
}

impl HotkeyConfig {
    pub fn from_strings(modifiers_str: &str, key_str: &str) -> Result<Self, String> {
        let modifiers_parsed = parse_modifiers(modifiers_str)?;
        let key_parsed = parse_key(key_str)?;

        Ok(HotkeyConfig {
            modifiers: modifiers_str.to_string(),
            key: key_str.to_string(),
            modifiers_parsed: Some(modifiers_parsed),
            key_parsed: Some(key_parsed),
        })
    }

    pub fn parse(&mut self) -> Result<(), String> {
        self.modifiers_parsed = Some(parse_modifiers(&self.modifiers)?);
        self.key_parsed = Some(parse_key(&self.key)?);
        Ok(())
    }

    pub fn to_string(&self) -> String {
        if let (Some(mods), Some(key)) = (&self.modifiers_parsed, &self.key_parsed) {
            let mut mod_names = Vec::new();
            if mods.contains(Modifiers::CONTROL) {
                mod_names.push("Ctrl");
            }
            if mods.contains(Modifiers::SHIFT) {
                mod_names.push("Shift");
            }
            if mods.contains(Modifiers::ALT) {
                mod_names.push("Alt");
            }
            if mods.contains(Modifiers::SUPER) {
                mod_names.push("Cmd/Super");
            }

            format!("{} + {}", mod_names.join(" + "), format_key(key))
        } else {
            format!("{} + {}", self.modifiers, self.key)
        }
    }

    pub fn modifiers(&self) -> Modifiers {
        self.modifiers_parsed
            .unwrap_or(Modifiers::CONTROL | Modifiers::SHIFT)
    }

    pub fn key(&self) -> Code {
        self.key_parsed.unwrap_or(Code::KeyS)
    }
}

fn parse_modifiers(input: &str) -> Result<Modifiers, String> {
    let parts: Vec<String> = input.split('+').map(|s| s.trim().to_lowercase()).collect();
    let mut modifiers = Modifiers::empty();

    for part in parts {
        match part.as_ref() {
            "ctrl" | "control" => modifiers |= Modifiers::CONTROL,
            "shift" => modifiers |= Modifiers::SHIFT,
            "alt" | "option" => modifiers |= Modifiers::ALT,
            "cmd" | "super" | "win" | "command" => modifiers |= Modifiers::SUPER,
            _ => return Err(format!("Unknown modifier: {}", part)),
        }
    }

    if modifiers.is_empty() {
        return Err("No valid modifiers specified".to_string());
    }

    Ok(modifiers)
}

fn parse_key(input: &str) -> Result<Code, String> {
    let key = input.trim().to_lowercase();
    match key.as_str() {
        "a" => Ok(Code::KeyA),
        "b" => Ok(Code::KeyB),
        "c" => Ok(Code::KeyC),
        "d" => Ok(Code::KeyD),
        "e" => Ok(Code::KeyE),
        "f" => Ok(Code::KeyF),
        "g" => Ok(Code::KeyG),
        "h" => Ok(Code::KeyH),
        "i" => Ok(Code::KeyI),
        "j" => Ok(Code::KeyJ),
        "k" => Ok(Code::KeyK),
        "l" => Ok(Code::KeyL),
        "m" => Ok(Code::KeyM),
        "n" => Ok(Code::KeyN),
        "o" => Ok(Code::KeyO),
        "p" => Ok(Code::KeyP),
        "q" => Ok(Code::KeyQ),
        "r" => Ok(Code::KeyR),
        "s" => Ok(Code::KeyS),
        "t" => Ok(Code::KeyT),
        "u" => Ok(Code::KeyU),
        "v" => Ok(Code::KeyV),
        "w" => Ok(Code::KeyW),
        "x" => Ok(Code::KeyX),
        "y" => Ok(Code::KeyY),
        "z" => Ok(Code::KeyZ),
        "1" => Ok(Code::Digit1),
        "2" => Ok(Code::Digit2),
        "3" => Ok(Code::Digit3),
        "4" => Ok(Code::Digit4),
        "5" => Ok(Code::Digit5),
        "6" => Ok(Code::Digit6),
        "7" => Ok(Code::Digit7),
        "8" => Ok(Code::Digit8),
        "9" => Ok(Code::Digit9),
        "0" => Ok(Code::Digit0),
        "space" => Ok(Code::Space),
        "enter" | "return" => Ok(Code::Enter),
        "tab" => Ok(Code::Tab),
        "escape" | "esc" => Ok(Code::Escape),
        _ => Err(format!("Unknown key: {}", key)),
    }
}

fn format_key(code: &Code) -> String {
    match code {
        Code::KeyA => "A",
        Code::KeyB => "B",
        Code::KeyC => "C",
        Code::KeyD => "D",
        Code::KeyE => "E",
        Code::KeyF => "F",
        Code::KeyG => "G",
        Code::KeyH => "H",
        Code::KeyI => "I",
        Code::KeyJ => "J",
        Code::KeyK => "K",
        Code::KeyL => "L",
        Code::KeyM => "M",
        Code::KeyN => "N",
        Code::KeyO => "O",
        Code::KeyP => "P",
        Code::KeyQ => "Q",
        Code::KeyR => "R",
        Code::KeyS => "S",
        Code::KeyT => "T",
        Code::KeyU => "U",
        Code::KeyV => "V",
        Code::KeyW => "W",
        Code::KeyX => "X",
        Code::KeyY => "Y",
        Code::KeyZ => "Z",
        Code::Digit1 => "1",
        Code::Digit2 => "2",
        Code::Digit3 => "3",
        Code::Digit4 => "4",
        Code::Digit5 => "5",
        Code::Digit6 => "6",
        Code::Digit7 => "7",
        Code::Digit8 => "8",
        Code::Digit9 => "9",
        Code::Digit0 => "0",
        Code::Space => "Space",
        Code::Enter => "Enter",
        Code::Tab => "Tab",
        Code::Escape => "Escape",
        _ => "Unknown",
    }
    .to_string()
}

fn get_config_path() -> PathBuf {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("paparazzi");

    fs::create_dir_all(&config_dir).ok();
    config_dir.join("config.json")
}

pub fn save_hotkey_config(config: &HotkeyConfig) -> Result<(), String> {
    let config_path = get_config_path();
    let json = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;

    fs::write(&config_path, json).map_err(|e| format!("Failed to write config file: {}", e))?;

    Ok(())
}

pub fn load_hotkey_config() -> HotkeyConfig {
    let config_path = get_config_path();

    if config_path.exists() {
        if let Ok(contents) = fs::read_to_string(&config_path) {
            if let Ok(mut config) = serde_json::from_str::<HotkeyConfig>(&contents) {
                if config.parse().is_ok() {
                    return config;
                }
            }
        }
    }

    HotkeyConfig::default()
}
