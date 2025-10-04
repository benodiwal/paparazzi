use clap::{Parser, Subcommand};
use global_hotkey::hotkey::{Code, Modifiers};

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
    Run {
        /// Run in background mode
        #[arg(short, long)]
        background: bool,
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
    /// Display version information
    Version,
}

#[derive(Debug, Clone)]
pub struct HotkeyConfig {
    pub modifiers: Modifiers,
    pub key: Code,
}

impl Default for HotkeyConfig {
    fn default() -> Self {
        HotkeyConfig {
            modifiers: Modifiers::CONTROL | Modifiers::SHIFT,
            key: Code::KeyS,
        }
    }
}

impl HotkeyConfig {
    pub fn from_strings(modifiers_str: &str, key_str: &str) -> Result<Self, String> {
        let modifiers = parse_modifiers(modifiers_str)?;
        let key = parse_key(key_str)?;
        Ok(HotkeyConfig { modifiers, key })
    }

    pub fn to_string(&self) -> String {
        let mut mods = Vec::new();
        if self.modifiers.contains(Modifiers::CONTROL) {
            mods.push("Ctrl");
        }
        if self.modifiers.contains(Modifiers::SHIFT) {
            mods.push("Shift");
        }
        if self.modifiers.contains(Modifiers::ALT) {
            mods.push("Alt");
        }
        if self.modifiers.contains(Modifiers::SUPER) {
            mods.push("Cmd/Super");
        }

        format!("{} + {}", mods.join(" + "), format_key(&self.key))
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
    }.to_string()
}

pub fn save_hotkey_config(config: &HotkeyConfig) -> Result<(), String> {
    // For now, we'll just print the config
    // In a real implementation, you'd save this to a config file
    println!("💾 Hotkey configuration would be saved: {}", config.to_string());
    Ok(())
}

pub fn load_hotkey_config() -> HotkeyConfig {
    // For now, return default config
    // In a real implementation, you'd load from a config file
    HotkeyConfig::default()
}