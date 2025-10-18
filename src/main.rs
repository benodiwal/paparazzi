use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands, HotkeyConfig};
use daemon::show_logs;
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, hotkey::HotKey};
use winit::application::ApplicationHandler;
use winit::event_loop::{ControlFlow, EventLoop};

mod cli;
mod constants;
mod daemon;
mod logger;
mod screenshot;
mod terminal;

struct App {
    receiver: crossbeam_channel::Receiver<GlobalHotKeyEvent>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {}

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        _event: winit::event::WindowEvent,
    ) {
    }

    fn new_events(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _cause: winit::event::StartCause,
    ) {
        event_loop.set_control_flow(ControlFlow::Wait);

        if let Ok(event) = self.receiver.try_recv() {
            if event.state == global_hotkey::HotKeyState::Pressed {
                logger::info("Hotkey pressed! Taking screenshot...");
                if let Err(err) = handle_screenshot() {
                    eprintln!("Error: {}", err);
                }
            }
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let daemon = daemon::Daemon::new();

    match cli.command {
        Some(Commands::Run { background }) => {
            if background {
                daemon.start()?;
                // After daemonization, run the service
                run_service_internal()?;
            } else {
                // Foreground mode
                run_service_internal()?;
            }
        }
        Some(Commands::Stop) => {
            daemon.stop()?;
        }
        Some(Commands::Status) => {
            daemon.status()?;
        }
        Some(Commands::Logs) => {
            show_logs()?;
        }
        Some(Commands::Attach { follow }) => daemon.attach(follow)?,
        Some(Commands::Hotkeys {
            modifiers,
            key,
            list,
        }) => {
            handle_hotkeys_command(modifiers, key, list, &daemon)?;
        }
        Some(Commands::Logging { level, show }) => {
            handle_logging_command(level, show)?;
        }
        Some(Commands::Version) => {
            print_version();
        }
        None => {
            print_intro();
        }
    }

    Ok(())
}

fn run_service_internal() -> Result<()> {
    let mut config = cli::load_hotkey_config();
    config
        .parse()
        .map_err(|e| anyhow::anyhow!("Failed to parse hotkey config: {}", e))?;

    #[cfg(unix)]
    setup_signal_handler();

    println!("Using hotkey: {}", config.to_string());
    println!("Press {} to take a screenshot", config.to_string());
    println!("Press Ctrl+C to exit\n");

    let event_loop = EventLoop::new()?;
    let manager = GlobalHotKeyManager::new()?;
    let hotkey = HotKey::new(Some(config.modifiers()), config.key());

    manager.register(hotkey)?;
    logger::success("Hotkey registered successfully");

    let receiver = GlobalHotKeyEvent::receiver().to_owned();
    let mut app = App { receiver };

    logger::info("Service is running...");
    event_loop.run_app(&mut app)?;

    // Cleanup
    manager.unregister(hotkey)?;
    logger::info("Service stopped");

    Ok(())
}

#[cfg(unix)]
fn setup_signal_handler() {
    use signal_hook::{consts::SIGTERM, iterator::Signals};
    use std::thread;

    let mut signals = Signals::new(&[SIGTERM]).unwrap();

    thread::spawn(move || {
        for sig in signals.forever() {
            match sig {
                SIGTERM => {
                    println!("\nReceived SIGTERM, shutting down gracefully...");
                    std::process::exit(0);
                }
                _ => {}
            }
        }
    });
}

fn handle_hotkeys_command(
    modifiers: Option<String>,
    key: Option<String>,
    list: bool,
    daemon: &daemon::Daemon,
) -> Result<()> {
    if list {
        let config = cli::load_hotkey_config();
        println!("Current hotkey configuration:");
        println!("   {}", config.to_string());
        return Ok(());
    }

    if let (Some(mod_str), Some(key_str)) = (modifiers, key) {
        match HotkeyConfig::from_strings(&mod_str, &key_str) {
            Ok(config) => {
                cli::save_hotkey_config(&config).map_err(|e| anyhow::anyhow!(e))?;
                logger::success("Hotkey configuration updated!");
                println!("   New hotkey: {}", config.to_string());

                if daemon.is_running()? {
                    println!("\nDaemon is currently running with old hotkey");
                    println!("   Restart it to apply changes:");
                    println!("   $ clipse stop");
                    println!("   $ clipse run --background");
                } else {
                    println!("\nRun 'clipse run --background' to start with the new hotkey");
                }
            }
            Err(e) => {
                eprintln!("Invalid hotkey configuration: {}", e);
                eprintln!("\nExample usage:");
                eprintln!("  clipse hotkeys --modifiers \"ctrl+shift\" --key s");
                eprintln!("  clipse hotkeys --modifiers \"cmd+alt\" --key p");
            }
        }
    } else {
        println!("Hotkey Configuration");
        println!();
        println!("Current hotkey: {}", cli::load_hotkey_config().to_string());
        println!();
        println!("To change the hotkey:");
        println!("  clipse hotkeys --modifiers <mods> --key <key>");
        println!();
        println!("Examples:");
        println!("  clipse hotkeys --modifiers \"ctrl+shift\" --key s");
        println!("  clipse hotkeys --modifiers \"cmd+alt\" --key p");
        println!("  clipse hotkeys --modifiers \"ctrl+alt\" --key x");
        println!();
        println!("Available modifiers:");
        println!("  ctrl, shift, alt, cmd (or super/win)");
        println!();
        println!("Available keys:");
        println!("  a-z, 0-9, space, enter, tab, escape");
        println!("\nBye\n");
    }

    Ok(())
}

fn handle_logging_command(level: Option<String>, show: bool) -> Result<()> {
    if show {
        let current_level = logger::get_current_log_level();
        println!("Current logging configuration:");
        println!("   Level: {}", current_level);
        return Ok(());
    }

    if let Some(level_str) = level {
        let valid_levels = ["off", "info", "success", "error", "warning", "all"];

        if !valid_levels.contains(&level_str.as_str()) {
            eprintln!("Invalid log level: {}", level_str);
            eprintln!("\nValid levels: {}", valid_levels.join(", "));
            return Ok(());
        }

        logger::save_log_config(&level_str)
            .map_err(|e| anyhow::anyhow!("Failed to save log config: {}", e))?;
        println!("Log level updated to: {}", level_str);

        match level_str.as_str() {
            "off" => println!("   All logging is now disabled"),
            "info" => println!("   Only INFO messages will be shown"),
            "success" => println!("   Only SUCCESS messages will be shown"),
            "error" => println!("   Only ERROR messages will be shown"),
            "warning" => println!("   Only WARNING messages will be shown"),
            "all" => println!("   All log messages will be shown"),
            _ => {}
        }
    } else {
        println!("Logging Configuration");
        println!();
        println!("Current level: {}", logger::get_current_log_level());
        println!();
        println!("To change the log level:");
        println!("  clipse logging --level <level>");
        println!();
        println!("Available levels:");
        println!("  off      - No logging");
        println!("  info     - Show only info messages");
        println!("  success  - Show only success messages");
        println!("  error    - Show only error messages");
        println!("  warning  - Show only warning messages");
        println!("  all      - Show all messages (default)");
        println!();
        println!("Examples:");
        println!("  clipse logging --level all");
        println!("  clipse logging --level off");
        println!("  clipse logging --show");
    }

    Ok(())
}

fn print_version() {
    println!("clipse {}", env!("CARGO_PKG_VERSION"));
    println!("A CLI tool for instant screenshots to Claude Code");
    println!();
    println!("Built with Rust\n");
}

fn print_intro() {
    println!("\nClipse - Instant Screenshots to Claude Code");
    println!();
    println!("USAGE:");
    println!("  clipse [COMMAND]");
    println!();
    println!("COMMANDS:");
    println!("  run       Start the screenshot service");
    println!("    --background, -b    Run as background daemon");
    println!("  stop      Stop the background daemon");
    println!("  status    Check daemon status");
    println!("  logs      View daemon logs");
    println!("  attach    Attach to running daemon (bring to foreground)");
    println!("    --follow, -f        Follow logs in real-time");
    println!("  hotkeys   Configure keyboard shortcuts");
    println!("  logging   Configure logging settings");
    println!("    --level, -l         Set log level (info, success, error, warning, all, off)");
    println!("    --show, -s          Show current logging configuration");
    println!("  version   Display version information");
    println!("  help      Display this help message");
    println!();
    println!("QUICK START:");
    println!("  1. Run 'clipse run --background' to start the daemon");
    println!("  2. Press the configured hotkey to take a screenshot");
    println!("  3. The screenshot will be sent to Claude Code");
    println!();
    println!("EXAMPLES:");
    println!("  clipse run                                    # Run in foreground");
    println!("  clipse run --background                       # Run as daemon");
    println!("  clipse stop                                   # Stop daemon");
    println!("  clipse status                                 # Check daemon status");
    println!("  clipse logs                                   # View daemon logs");
    println!("  clipse hotkeys --list                         # Show current hotkey");
    println!("  clipse hotkeys --modifiers \"ctrl+shift\" --key s  # Set new hotkey");
    println!("  clipse logging --show                         # Show log level");
    println!("  clipse logging --level off                    # Disable logging");
    println!();
    println!("For more information, visit: https://github.com/benodiwal/paparazzi");
    println!("\n Bye\n");
}

fn handle_screenshot() -> Result<()> {
    let screenshot_path = screenshot::capture()?;
    logger::info(&format!("Screenshot saved to: {}", screenshot_path));

    let message = format!("{} Analyze this image", screenshot_path);
    terminal::send_to_claude_code_terminal(&message)?;

    println!("Sent to Claude Code!");

    Ok(())
}
