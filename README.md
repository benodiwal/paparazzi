# Paparazzi

A lightweight CLI tool for instant screenshots to Claude Code with zero friction.

## Overview

Paparazzi is a macOS-native command-line tool that captures screenshots and sends them directly to Claude Code with minimal setup. Built with Rust for optimal performance, it integrates deeply with macOS APIs to provide seamless screenshot capture and communication with Claude Code terminals.

## Features

- **Instant Capture**: Screenshot and paste to Claude Code in one command
- **Native macOS Integration**: Built with Core Graphics for optimal performance
- **Privacy Focused**: All processing happens locally, screenshots never leave your machine
- **Keyboard Driven**: Configurable shortcuts, never leave your terminal
- **Background Service**: Run as a daemon with hotkey support
- **Flexible Configuration**: Customizable hotkeys and logging levels
- **Zero Dependencies**: No external screenshot tools required

## Installation

### Via Cargo (Recommended)

```bash
cargo install paparazzi
```

### From GitHub Releases

#### macOS Apple Silicon

```bash
curl -L https://github.com/benodiwal/paparazzi/releases/latest/download/paparazzi-aarch64-apple-darwin -o paparazzi
chmod +x paparazzi
sudo mv paparazzi /usr/local/bin/
```

#### macOS Intel

```bash
curl -L https://github.com/benodiwal/paparazzi/releases/latest/download/paparazzi-x86_64-apple-darwin -o paparazzi
chmod +x paparazzi
sudo mv paparazzi /usr/local/bin/
```

### From Source

```bash
git clone https://github.com/benodiwal/paparazzi.git
cd paparazzi
cargo build --release
```

The binary will be available at `target/release/paparazzi`.

## Prerequisites

- macOS (currently supported platform)
- Claude Code installed and running
- Terminal access
- Screen recording permissions (macOS will prompt automatically)

## Quick Start

1. Install Paparazzi using one of the methods above
2. Start the service:
   ```bash
   paparazzi run
   ```
3. Use the default hotkey (Ctrl+Shift+S) to capture screenshots
4. Screenshots will be sent directly to your active Claude Code session

## Usage

### Basic Commands

#### Start Service
```bash
paparazzi run
```
Starts the hotkey listener service in foreground mode.

#### Background Mode
```bash
paparazzi run --background
```
Runs the service in background mode as a daemon.

#### Stop Service
```bash
paparazzi stop
```
Stops the background daemon service.

#### Check Status
```bash
paparazzi status
```
Displays daemon status and system information.

#### View Logs
```bash
paparazzi logs
```
Shows daemon logs and activity.

### Hotkey Configuration

#### Configure Custom Hotkeys
```bash
paparazzi hotkeys --modifiers "ctrl+shift" --key s
```

#### View Current Configuration
```bash
paparazzi hotkeys --list
```

#### Available Modifiers
- `ctrl` - Control key
- `shift` - Shift key
- `alt` or `option` - Alt/Option key
- `cmd` or `super` - Command/Super key

#### Available Keys
- Letters: `a-z`
- Numbers: `0-9`
- Special keys: `space`, `enter`, `tab`, `escape`

### Logging Configuration

#### Set Logging Level
```bash
paparazzi logging --level all
```

#### Available Log Levels
- `off` - No logging output
- `info` - Show only informational messages
- `success` - Show only success messages
- `error` - Show only error messages
- `warning` - Show only warning messages
- `all` - Show all log messages (default)

#### View Current Log Level
```bash
paparazzi logging --show
```

### Other Commands

#### Version Information
```bash
paparazzi version
```

#### Help
```bash
paparazzi help
```

## How It Works

Paparazzi is built with Rust and integrates deeply with macOS native APIs:

1. **Core Graphics Integration** - Uses native macOS screenshot APIs for optimal performance
2. **Global Hotkey Manager** - Registers system-wide keyboard shortcuts
3. **IPC Communication** - Communicates directly with Claude Code's stdin
4. **Terminal Detection** - Automatically finds and connects to Claude Code sessions

### Workflow

1. User presses configured hotkey
2. Paparazzi captures screenshot using Core Graphics
3. Image is temporarily saved to secure location
4. Path is sent to Claude Code with analysis prompt
5. Claude Code receives and processes the image

## Configuration

Configuration files are stored in `~/.paparazzi/`:
- `hotkey_config.json` - Hotkey settings
- `log_config.json` - Logging configuration
- `daemon.pid` - Daemon process ID (when running)

## Troubleshooting

### Permission Issues

If screenshots aren't working, ensure Paparazzi has screen recording permissions:

1. Go to System Preferences → Security & Privacy → Privacy
2. Select "Screen Recording" from the left sidebar
3. Add your terminal app (Terminal, iTerm2, etc.)
4. Restart Paparazzi

### Hotkey Not Working

- Check if another app is using the same hotkey
- Verify the hotkey configuration with `paparazzi hotkeys --list`
- Try a different key combination
- Restart the service with `paparazzi run`

### Claude Code Not Receiving Images

- Ensure Claude Code is running and active
- Check that Paparazzi has the correct Claude Code session
- Verify terminal permissions
- Check logs with `paparazzi logs`

## Development

### Prerequisites

- Rust 1.70+ (latest stable recommended)
- macOS development environment
- Xcode command line tools

### Building from Source

```bash
git clone https://github.com/benodiwal/paparazzi.git
cd paparazzi
cargo build
```

### Running Tests

```bash
cargo test
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Workflow

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`cargo test`)
5. Run formatting (`cargo fmt`)
6. Run linting (`cargo clippy`)
7. Commit your changes (`git commit -m 'Add some amazing feature'`)
8. Push to the branch (`git push origin feature/amazing-feature`)
9. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built for the Claude Code community
- Inspired by the need for frictionless screenshot sharing
- Thanks to all contributors and users providing feedback

## Support

If you encounter any issues or have questions:

1. Check the [troubleshooting section](#troubleshooting)
2. Search existing [GitHub issues](https://github.com/benodiwal/paparazzi/issues)
3. Create a new issue with detailed information about your problem

## Roadmap

- Custom keyboard shortcuts
- Annotation tools before sending
- Multi-monitor improvements
- Video capture support
- Clipboard history integration

---

For more detailed documentation, visit our [documentation site](https://www.paparazziforclaude.com/docs).
