import { useState, useEffect } from 'react'
import { Link } from 'react-router-dom'
import './App.css'

interface NavLinkProps {
  href: string
  children: React.ReactNode
}

const NavLink = ({ href, children }: NavLinkProps) => {
  const handleClick = (e: React.MouseEvent<HTMLAnchorElement>) => {
    if (href.startsWith('#')) {
      e.preventDefault()
      const element = document.querySelector(href)
      if (element) {
        element.scrollIntoView({ behavior: 'smooth', block: 'start' })
      }
    }
  }

  return (
    <a href={href} className="nav-link" onClick={handleClick}>
      {children}
    </a>
  )
}

interface ItemProps {
  title?: string
  href?: string
  meta?: string
  description: React.ReactNode
}

const Item = ({ title, href, meta, description }: ItemProps) => (
  <div className="item">
    {title && (
      <h3>
        {href ? <a href={href}>{title}</a> : title}
      </h3>
    )}
    {meta && <div className="item-meta">{meta}</div>}
    <div className="item-description">{description}</div>
  </div>
)

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
            placeholder="Search documentation, features, commands..."
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

function App() {
  const [status] = useState('checking status...')
  const [isSearchOpen, setIsSearchOpen] = useState(false)

  const asciiArt = `
................. ...  ..............................  ....  ..........
...................::................................:-+++=-. .........
.............. .::.-:.::........................... -*%@@@%%+. ........
.............. .:..-:.::........................... =%%@@@@%*: ........
......  ........:::::::..............................=*%%%%+-...  .....
......:-----------====--------------------------------=+++==-----. ....
.... :+:.......::::::::........::::::::::.........:-------.....:=+.....
.... -+........:.    .:....:-------::------::.....-%@@@@%*:.....:*. ...
.... -+........::.....:.:-=-:..............:-=-:..-+*****+:.....:*. ...
.... -+...............:==:. ..:..........:.. .-=-...............:*. ...
.... -+..............-=:  ::....:::::::....::. .-=:........:::..:*. ...
.... -+.............==. .:...:-::::..::::::..::  :+:.......:::..:*. ...
.... -+............-+. .:. .::::::::--....:-. .:  -+.......:::..:*. ...
.... -+............+-  :. .-:::::::+*:.....--  :. .+-......:::..:*. ...
.... -+............+:  -. :-.::::::-*+-:...:-. ::  +-......:::..:*. ...
.... -+............+-  -. .-.......=%+:....:-  :.  +-......:::..:*. ...
.... -+............-+. .:. :-:....-=:.....:-. .:  -+.......:::..:*. ...
.... -+.............==. .:...::::::....::::..::  :+:.......:::..:*. ...
.... -+..............-=: .::....::::::::....:. .-=:........:::..:*. ...
.... -+...............:=-:  ..:..........:.. .:=-...............:*. ...
.... :+-::::::::::::::::-==-:..............-===:::::::::::::::::==.....
..... .--------------------==-----::::-----==-------------------:. ....
......                       .....:::.....                       ......
................................        ...............................`

  const features = [
    {
      title: 'instant capture',
      href: '#instant',
      description: 'screenshot and paste to claude code in one command. stay in flow state.'
    },
    {
      title: 'native macos',
      href: '#native',
      description: 'built with swift and core graphics. minimal resource footprint, maximum performance.'
    },
    {
      title: 'privacy focused',
      href: '#privacy',
      description: 'all processing happens locally. your screenshots never leave your machine.'
    },
    {
      title: 'keyboard driven',
      href: '#keyboard',
      description: 'configurable shortcuts. never leave your terminal.'
    }
  ]

  const roadmapItems = [
    'custom keyboard shortcuts',
    'annotation tools before sending',
    'multi-monitor improvements',
    'video capture support',
    'clipboard history integration'
  ]

  const links = [
    {
      title: 'github',
      href: 'https://github.com/benodiwal/clipse',
      description: 'source code, issues, pull requests welcome'
    },
    {
      title: 'claude code',
      href: 'https://www.claude.com/product/claude-code',
      description: 'learn more about claude code'
    }
  ]

  // Prepare searchable content
  const searchableContent: SearchResult[] = [
    {
      title: 'Home',
      content: 'paparazzi - instant screenshots to claude code, zero friction',
      section: 'Navigation',
      href: '#home'
    },
    {
      title: 'Installation - Homebrew',
      content: 'brew install paparazzi',
      section: 'Installation',
      href: '#install'
    },
    {
      title: 'Installation - From Source',
      content: 'git clone https://github.com/benodiwal/paparazzi.git',
      section: 'Installation',
      href: '#install'
    },
    {
      title: 'Usage - Start Service',
      content: 'paparazzi run - starts the hotkey listener service',
      section: 'Usage',
      href: '#docs'
    },
    {
      title: 'Usage - Background Mode',
      content: 'paparazzi run --background - runs the service in background mode',
      section: 'Usage',
      href: '#docs'
    },
    {
      title: 'Usage - Configure Hotkeys',
      content: 'paparazzi hotkeys --modifiers "ctrl+shift" --key s - set custom keyboard shortcuts',
      section: 'Usage',
      href: '#docs'
    },
    {
      title: 'Usage - View Configuration',
      content: 'paparazzi hotkeys --list - show current hotkey settings',
      section: 'Usage',
      href: '#docs'
    },
    {
      title: 'Usage - Version',
      content: 'paparazzi version - display version information',
      section: 'Usage',
      href: '#docs'
    },
    {
      title: 'Usage - Help',
      content: 'paparazzi help - show help information',
      section: 'Usage',
      href: '#docs'
    },
    {
      title: 'How it Works',
      content: 'paparazzi hooks into macos native screenshot apis and communicates directly with claude code stdin',
      section: 'Documentation',
      href: '#docs'
    },
    {
      title: 'Native macOS Integration',
      content: 'native macos integration via core graphics',
      section: 'How it Works',
      href: '#docs'
    },
    {
      title: 'IPC Communication',
      content: 'ipc communication with claude code',
      section: 'How it Works',
      href: '#docs'
    },
    {
      title: 'Zero Dependencies',
      content: 'zero external dependencies',
      section: 'How it Works',
      href: '#docs'
    },
    {
      title: 'Privacy First',
      content: 'privacy-first, everything stays local',
      section: 'How it Works',
      href: '#docs'
    },
    ...features.map(f => ({
      title: f.title,
      content: f.description as string,
      section: 'Features',
      href: '#features'
    })),
    ...roadmapItems.map(item => ({
      title: item,
      content: 'Upcoming feature in development',
      section: 'Roadmap',
      href: '#roadmap'
    }))
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
    <div className="container">
      <nav>
        <NavLink href="#home">home</NavLink>
        <Link to="/docs" className="nav-link">docs</Link>
        <Link to="/blogs" className="nav-link">blogs</Link>
        <NavLink href="#install">install</NavLink>
        <NavLink href="https://github.com/benodiwal/paparazzi">github</NavLink>
        <div className="shortcuts" onClick={() => setIsSearchOpen(true)}>
          <span className="shortcut">⌘</span>
          <span className="shortcut">K</span>
        </div>
      </nav>

      <div id="home" className="hero">
        <pre className="ascii-art">{asciiArt}</pre>

        <div className="intro">
          <h1>paparazzi</h1>
          <p>macos</p>
          <p>cli tool for developers</p>
          <div className="status">$ {status}</div>
          <div className="description">
            screenshot directly to claude code. no manual pasting, no context switching,
            no friction. built for developers who value speed.
          </div>
        </div>
      </div>

      <div className="content">
        <div className="left-column">
          <h1 id="install">installation</h1>

          <div className="section">
            <Item
              title="homebrew"
              description={<code>brew install paparazzi</code>}
            />

            <Item
              title="from source"
              description={
                <code>
                  git clone https://github.com/benodiwal/paparazzi.git
                </code>
              }
            />
          </div>

          <h1 id="docs">usage</h1>

          <div className="section">
            <Item
              title="start service"
              description={
                <>
                  <code>paparazzi run</code>
                  <div style={{ marginTop: '0.5rem', color: '#666', fontSize: '0.9rem' }}>
                    starts the hotkey listener service
                  </div>
                </>
              }
            />

            <Item
              title="background mode"
              description={
                <>
                  <code>paparazzi run --background</code>
                  <div style={{ marginTop: '0.5rem', color: '#666', fontSize: '0.9rem' }}>
                    runs the service in background mode
                  </div>
                </>
              }
            />

            <Item
              title="configure hotkeys"
              description={
                <>
                  <code>paparazzi hotkeys --modifiers "ctrl+shift" --key s</code>
                  <div style={{ marginTop: '0.5rem', color: '#666', fontSize: '0.9rem' }}>
                    set custom keyboard shortcuts
                  </div>
                </>
              }
            />

            <Item
              title="view configuration"
              description={
                <>
                  <code>paparazzi hotkeys --list</code>
                  <div style={{ marginTop: '0.5rem', color: '#666', fontSize: '0.9rem' }}>
                    show current hotkey settings
                  </div>
                </>
              }
            />
          </div>

          <h1>how it works</h1>

          <div className="section">
            <div className="item-description">
              paparazzi hooks into macos native screenshot apis and communicates directly
              with claude code's stdin. when you capture, the image is encoded and piped
              instantly to your active claude code session.
            </div>
            <ul>
              <li>native macos integration via core graphics</li>
              <li>ipc communication with claude code</li>
              <li>zero external dependencies</li>
              <li>privacy-first, everything stays local</li>
            </ul>
          </div>
        </div>

        <div className="right-column">
          <h1 id="features">features</h1>

          <div className="section">
            {features.map((feature, index) => (
              <Item
                key={index}
                title={feature.title}
                href={feature.href}
                description={feature.description}
              />
            ))}
          </div>

          <h1 id="roadmap">roadmap</h1>

          <div className="section">
            <ul>
              {roadmapItems.map((item, index) => (
                <li key={index}>{item}</li>
              ))}
            </ul>
          </div>

          <h1 id="links">links</h1>

          <div className="section">
            {links.map((link, index) => (
              <Item
                key={index}
                title={link.title}
                href={link.href}
                description={link.description}
              />
            ))}
          </div>
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

export default App
