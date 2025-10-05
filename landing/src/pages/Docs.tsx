import { Link } from 'react-router-dom'
import './Docs.css'

function Docs() {
  return (
    <div className="docs-container">
      <nav className="docs-nav">
        <Link to="/" className="nav-link">← Back to Home</Link>
        <Link to="/blogs" className="nav-link">Blogs</Link>
      </nav>

      <div className="docs-content">
        <header className="docs-header">
          <h1>Paparazzi Documentation</h1>
          <p>Complete guide to using Paparazzi for instant screenshots to Claude Code</p>
        </header>

        <div className="docs-sections">
          <div className="coming-soon-docs">
            <h2>Coming Soon</h2>
            <p>Full documentation will be available when the CLI tool is released.</p>
          </div>

          {/*
          <section className="docs-section">
            <h2 id="getting-started">Getting Started</h2>
            <p>Paparazzi is a lightweight CLI tool that captures screenshots and sends them directly to Claude Code with zero friction.</p>

            <h3>Prerequisites</h3>
            <ul>
              <li>macOS (currently supported)</li>
              <li>Claude Code installed and running</li>
              <li>Terminal access</li>
            </ul>

            <h3>Quick Installation</h3>
            <div className="code-block">
              <h4>Via Homebrew (Recommended)</h4>
              <code>brew install paparazzi</code>
            </div>

            <div className="code-block">
              <h4>From Source</h4>
              <code>git clone https://github.com/benodiwal/paparazzi.git<br/>cd paparazzi<br/>./install.sh</code>
            </div>
          </section>

          <section className="docs-section">
            <h2 id="usage">Usage</h2>

            <h3>Basic Commands</h3>
            <div className="command-grid">
              <div className="command-item">
                <h4>Start Service</h4>
                <code>paparazzi run</code>
                <p>Starts the hotkey listener service</p>
              </div>

              <div className="command-item">
                <h4>Background Mode</h4>
                <code>paparazzi run --background</code>
                <p>Runs the service in background mode</p>
              </div>

              <div className="command-item">
                <h4>Configure Hotkeys</h4>
                <code>paparazzi hotkeys --modifiers "ctrl+shift" --key s</code>
                <p>Set custom keyboard shortcuts</p>
              </div>

              <div className="command-item">
                <h4>View Configuration</h4>
                <code>paparazzi hotkeys --list</code>
                <p>Show current hotkey settings</p>
              </div>
            </div>
          </section>

          <section className="docs-section">
            <h2 id="configuration">Configuration</h2>

            <h3>Hotkey Customization</h3>
            <p>Paparazzi supports flexible hotkey configuration with multiple modifier combinations:</p>

            <h4>Available Modifiers</h4>
            <ul>
              <li><code>ctrl</code> - Control key</li>
              <li><code>shift</code> - Shift key</li>
              <li><code>alt</code> or <code>option</code> - Alt/Option key</li>
              <li><code>cmd</code> or <code>super</code> - Command/Super key</li>
            </ul>

            <h4>Available Keys</h4>
            <ul>
              <li>Letters: <code>a-z</code></li>
              <li>Numbers: <code>0-9</code></li>
              <li>Special: <code>space</code>, <code>enter</code>, <code>tab</code>, <code>escape</code></li>
            </ul>

            <h4>Example Configurations</h4>
            <div className="code-block">
              <code>paparazzi hotkeys --modifiers "ctrl+shift" --key s</code>
              <code>paparazzi hotkeys --modifiers "cmd+alt" --key p</code>
              <code>paparazzi hotkeys --modifiers "ctrl+alt" --key space</code>
            </div>
          </section>

          <section className="docs-section">
            <h2 id="how-it-works">How It Works</h2>

            <h3>Architecture</h3>
            <p>Paparazzi is built with Rust and integrates deeply with macOS native APIs:</p>

            <ul>
              <li><strong>Core Graphics Integration</strong> - Uses native macOS screenshot APIs for optimal performance</li>
              <li><strong>Global Hotkey Manager</strong> - Registers system-wide keyboard shortcuts</li>
              <li><strong>IPC Communication</strong> - Communicates directly with Claude Code's stdin</li>
              <li><strong>Zero Dependencies</strong> - No external screenshot tools required</li>
            </ul>

            <h3>Workflow</h3>
            <ol>
              <li>User presses configured hotkey</li>
              <li>Paparazzi captures screenshot using Core Graphics</li>
              <li>Image is temporarily saved to secure location</li>
              <li>Path is sent to Claude Code with analysis prompt</li>
              <li>Claude Code receives and processes the image</li>
            </ol>
          </section>

          <section className="docs-section">
            <h2 id="troubleshooting">Troubleshooting</h2>

            <h3>Common Issues</h3>

            <div className="troubleshoot-item">
              <h4>Permission Issues</h4>
              <p>If screenshots aren't working, ensure Paparazzi has screen recording permissions:</p>
              <ol>
                <li>Go to System Preferences → Security & Privacy → Privacy</li>
                <li>Select "Screen Recording" from the left sidebar</li>
                <li>Add your terminal app (Terminal, iTerm2, etc.)</li>
                <li>Restart Paparazzi</li>
              </ol>
            </div>

            <div className="troubleshoot-item">
              <h4>Hotkey Not Working</h4>
              <p>If hotkeys aren't responding:</p>
              <ul>
                <li>Check if another app is using the same hotkey</li>
                <li>Verify the hotkey configuration with <code>paparazzi hotkeys --list</code></li>
                <li>Try a different key combination</li>
                <li>Restart the service with <code>paparazzi run</code></li>
              </ul>
            </div>

            <div className="troubleshoot-item">
              <h4>Claude Code Not Receiving Images</h4>
              <p>If Claude Code isn't receiving screenshots:</p>
              <ul>
                <li>Ensure Claude Code is running and active</li>
                <li>Check that Paparazzi has the correct Claude Code session</li>
                <li>Verify terminal permissions</li>
              </ul>
            </div>
          </section>

          <section className="docs-section">
            <h2 id="api-reference">API Reference</h2>

            <h3>Command Line Interface</h3>

            <div className="api-item">
              <h4><code>paparazzi run [OPTIONS]</code></h4>
              <p>Start the screenshot service</p>
              <h5>Options:</h5>
              <ul>
                <li><code>-b, --background</code> - Run in background mode</li>
              </ul>
            </div>

            <div className="api-item">
              <h4><code>paparazzi hotkeys [OPTIONS]</code></h4>
              <p>Configure keyboard shortcuts</p>
              <h5>Options:</h5>
              <ul>
                <li><code>-m, --modifiers &lt;MODS&gt;</code> - Set modifier keys</li>
                <li><code>-k, --key &lt;KEY&gt;</code> - Set the trigger key</li>
                <li><code>-l, --list</code> - Show current configuration</li>
              </ul>
            </div>

            <div className="api-item">
              <h4><code>paparazzi version</code></h4>
              <p>Display version information</p>
            </div>

            <div className="api-item">
              <h4><code>paparazzi help</code></h4>
              <p>Show help information</p>
            </div>
          </section>
          */}
        </div>
      </div>
    </div>
  )
}

export default Docs