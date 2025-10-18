import { Link } from 'react-router-dom'
import { useState, useEffect } from 'react'
import './Docs.css'

interface SearchResult {
  title: string
  content: string
  section: string
  href: string
}

interface SearchModalProps {
  isOpen: boolean
  onClose: () => void
  searchableContent: SearchResult[]
}

const SearchModal = ({ isOpen, onClose, searchableContent }: SearchModalProps) => {
  const [searchTerm, setSearchTerm] = useState('')
  const [results, setResults] = useState<SearchResult[]>([])

  useEffect(() => {
    if (!isOpen) {
      setSearchTerm('')
      setResults([])
    }
  }, [isOpen])

  useEffect(() => {
    if (searchTerm.trim()) {
      const filtered = searchableContent.filter(item =>
        item.title.toLowerCase().includes(searchTerm.toLowerCase()) ||
        item.content.toLowerCase().includes(searchTerm.toLowerCase()) ||
        item.section.toLowerCase().includes(searchTerm.toLowerCase())
      )
      setResults(filtered)
    } else {
      setResults([])
    }
  }, [searchTerm, searchableContent])

  const handleResultClick = (href: string) => {
    onClose()
    const element = document.querySelector(href)
    if (element) {
      element.scrollIntoView({ behavior: 'smooth', block: 'start' })
    }
  }

  if (!isOpen) return null

  return (
    <div className="search-overlay" onClick={onClose}>
      <div className="search-modal" onClick={(e) => e.stopPropagation()}>
        <div className="search-header">
          <input
            type="text"
            className="search-input"
            placeholder="Search documentation..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            autoFocus
          />
          <button className="search-close" onClick={onClose}>
            <span>ESC</span>
          </button>
        </div>
        <div className="search-results">
          {searchTerm && results.length === 0 && (
            <div className="no-results">No results found for "{searchTerm}"</div>
          )}
          {results.map((result, index) => (
            <div
              key={index}
              className="search-result-item"
              onClick={() => handleResultClick(result.href)}
            >
              <div className="search-result-section">{result.section}</div>
              <div className="search-result-title">{result.title}</div>
              <div className="search-result-content">{result.content}</div>
            </div>
          ))}
        </div>
      </div>
    </div>
  )
}

function Docs() {
  const [isSearchOpen, setIsSearchOpen] = useState(false)

  // Prepare searchable content for docs
  const searchableContent: SearchResult[] = [
    // Getting Started
    {
      title: 'Prerequisites',
      content: 'macOS, Claude Code installed and running, Terminal access',
      section: 'Getting Started',
      href: '#getting-started'
    },
    {
      title: 'Install via Cargo',
      content: 'cargo install paparazzi - recommended installation method',
      section: 'Getting Started',
      href: '#getting-started'
    },
    {
      title: 'Install from GitHub Releases',
      content: 'curl download for macOS Apple Silicon and Intel platforms',
      section: 'Getting Started',
      href: '#getting-started'
    },
    // Usage Commands
    {
      title: 'Start Service',
      content: 'paparazzi run - starts the hotkey listener service',
      section: 'Usage',
      href: '#usage'
    },
    {
      title: 'Background Mode',
      content: 'paparazzi run --background - runs the service in background mode',
      section: 'Usage',
      href: '#usage'
    },
    {
      title: 'Configure Hotkeys',
      content: 'paparazzi hotkeys --modifiers ctrl+shift --key s - set custom keyboard shortcuts',
      section: 'Usage',
      href: '#usage'
    },
    {
      title: 'View Configuration',
      content: 'paparazzi hotkeys --list - show current hotkey settings',
      section: 'Usage',
      href: '#usage'
    },
    {
      title: 'Stop Service',
      content: 'paparazzi stop - stop the background daemon',
      section: 'Usage',
      href: '#usage'
    },
    {
      title: 'Check Status',
      content: 'paparazzi status - check daemon status and system info',
      section: 'Usage',
      href: '#usage'
    },
    {
      title: 'View Logs',
      content: 'paparazzi logs - view daemon logs and activity',
      section: 'Usage',
      href: '#usage'
    },
    {
      title: 'Configure Logging',
      content: 'paparazzi logging --level all - set logging verbosity level',
      section: 'Usage',
      href: '#usage'
    },
    // Configuration
    {
      title: 'Hotkey Modifiers',
      content: 'ctrl, shift, alt, option, cmd, super - available modifier keys',
      section: 'Configuration',
      href: '#configuration'
    },
    {
      title: 'Available Keys',
      content: 'letters a-z, numbers 0-9, special keys space, enter, tab, escape',
      section: 'Configuration',
      href: '#configuration'
    },
    {
      title: 'Log Levels',
      content: 'off, info, success, error, warning, all - available logging levels',
      section: 'Configuration',
      href: '#configuration'
    },
    // How it Works
    {
      title: 'Core Graphics Integration',
      content: 'uses native macOS screenshot APIs for optimal performance',
      section: 'How it Works',
      href: '#how-it-works'
    },
    {
      title: 'Global Hotkey Manager',
      content: 'registers system-wide keyboard shortcuts',
      section: 'How it Works',
      href: '#how-it-works'
    },
    {
      title: 'IPC Communication',
      content: 'communicates directly with Claude Code stdin',
      section: 'How it Works',
      href: '#how-it-works'
    },
    // Troubleshooting
    {
      title: 'Permission Issues',
      content: 'screen recording permissions, System Preferences Security & Privacy',
      section: 'Troubleshooting',
      href: '#troubleshooting'
    },
    {
      title: 'Hotkey Not Working',
      content: 'check for conflicts, verify configuration, try different key combination',
      section: 'Troubleshooting',
      href: '#troubleshooting'
    },
    {
      title: 'Claude Code Not Receiving Images',
      content: 'ensure Claude Code is running, check terminal permissions',
      section: 'Troubleshooting',
      href: '#troubleshooting'
    },
    // API Reference
    {
      title: 'paparazzi run',
      content: 'start the screenshot service with optional background mode',
      section: 'API Reference',
      href: '#api-reference'
    },
    {
      title: 'paparazzi hotkeys',
      content: 'configure keyboard shortcuts with modifiers and keys',
      section: 'API Reference',
      href: '#api-reference'
    },
    {
      title: 'paparazzi version',
      content: 'display version information',
      section: 'API Reference',
      href: '#api-reference'
    },
    {
      title: 'paparazzi help',
      content: 'show help information',
      section: 'API Reference',
      href: '#api-reference'
    }
  ]

  // Keyboard shortcut handler
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
        e.preventDefault()
        setIsSearchOpen(true)
      }
      if (e.key === 'Escape') {
        setIsSearchOpen(false)
      }
    }

    window.addEventListener('keydown', handleKeyDown)
    return () => window.removeEventListener('keydown', handleKeyDown)
  }, [])
  return (
    <div className="docs-container">
      <nav className="docs-nav">
        <Link to="/" className="nav-link">← Back to Home</Link>
        <Link to="/blogs" className="nav-link">Blogs</Link>
        <div className="nav-controls">
          <div className="shortcuts" onClick={() => setIsSearchOpen(true)}>
            <span className="shortcut">⌘</span>
            <span className="shortcut">K</span>
          </div>
        </div>
      </nav>

      <div className="docs-content">
        <header className="docs-header">
          <h1>Paparazzi Documentation</h1>
          <p>Complete guide to using Paparazzi for instant screenshots to Claude Code</p>
        </header>

        <div className="docs-sections">
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
              <h4>Via Cargo (Recommended)</h4>
              <code>cargo install paparazzi</code>
            </div>

            <div className="code-block">
              <h4>From GitHub Releases</h4>
              <code>curl -L https://github.com/benodiwal/paparazzi/releases/latest/download/paparazzi-aarch64-apple-darwin -o paparazzi<br/>chmod +x paparazzi<br/>sudo mv paparazzi /usr/local/bin/</code>
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

              <div className="command-item">
                <h4>Stop Service</h4>
                <code>paparazzi stop</code>
                <p>Stop the background daemon</p>
              </div>

              <div className="command-item">
                <h4>Check Status</h4>
                <code>paparazzi status</code>
                <p>Check daemon status and system info</p>
              </div>

              <div className="command-item">
                <h4>View Logs</h4>
                <code>paparazzi logs</code>
                <p>View daemon logs and activity</p>
              </div>

              <div className="command-item">
                <h4>Configure Logging</h4>
                <code>paparazzi logging --level all</code>
                <p>Set logging verbosity level</p>
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

            <h3>Logging Configuration</h3>
            <p>Paparazzi provides configurable logging levels to control output verbosity:</p>

            <h4>Available Log Levels</h4>
            <ul>
              <li><code>off</code> - No logging output</li>
              <li><code>info</code> - Show only informational messages</li>
              <li><code>success</code> - Show only success messages</li>
              <li><code>error</code> - Show only error messages</li>
              <li><code>warning</code> - Show only warning messages</li>
              <li><code>all</code> - Show all log messages (default)</li>
            </ul>

            <h4>Logging Commands</h4>
            <div className="code-block">
              <code>paparazzi logging --level off       # Disable all logging</code>
              <code>paparazzi logging --level success   # Only show success messages</code>
              <code>paparazzi logging --show            # Show current log level</code>
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
        </div>
      </div>
      <SearchModal
        isOpen={isSearchOpen}
        onClose={() => setIsSearchOpen(false)}
        searchableContent={searchableContent}
      />
    </div>
  )
}

export default Docs