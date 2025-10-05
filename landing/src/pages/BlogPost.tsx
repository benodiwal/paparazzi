import { Link, useParams, Navigate } from 'react-router-dom'
import './BlogPost.css'

function BlogPost() {
  const { id } = useParams<{ id: string }>()

  if (id !== '1') {
    return <Navigate to="/blogs" replace />
  }

  const content = `
*How I built a screenshot tool that runs in the background and learned to tame daemon mode along the way*

---

## What I'm Building

Paparazzi is a command-line tool with a simple purpose: press a keyboard shortcut anywhere on your system, take a screenshot, and send it directly to Claude Code. No opening apps, no dragging files, no context switching.

The core challenge? To listen for that global keyboard shortcut, Paparazzi needs to be running all the time. But nobody wants to keep a terminal window open 24/7 just to use their screenshot tool.

This is where daemon mode comes in.

---

## Understanding Daemon Mode

A daemon is a program that runs in the background, independent of any terminal session. Think of system services you never see but are always running: your web server, your database, your clipboard manager.

When you daemonize a process, you're essentially telling it: "Go run somewhere I can't see you, and keep running even if I close this terminal."

For Paparazzi, this means you can start it once with \`paparazzi run --background\`, close your terminal, and the global hotkey keeps working. Open a new terminal later and run \`paparazzi stop\` to shut it down cleanly.

Simple in concept. Not so simple in practice.

---

## The First Problem: How Do You Actually Make Something a Daemon?

Here's what needs to happen when you daemonize a process:

**The Double Fork Dance**

You can't just tell a program "go to the background." You need to fork the process twice. The first fork detaches from the parent process. The second fork ensures the process can never acquire a controlling terminal. Between the forks, you call \`setsid()\` to create a new session.

**PID File Management**

When your daemon starts, it needs to write its process ID to a file (usually \`/tmp/yourapp.pid\`). This lets other commands know the daemon is running and how to talk to it.

When you run \`paparazzi stop\`, it needs to:
- Read the PID from that file
- Send a SIGTERM signal to that process
- Wait for clean shutdown
- Remove the PID file

If the PID file exists but the process is dead, that's a stale PID file. You need to detect and clean this up.

**Log File Setup**

Where do \`println!\` statements go when there's no terminal? You need to redirect stdout and stderr to log files. But should you overwrite old logs or append to them? If you append, how do you tell when one session ended and another began?

**Signal Handling**

When someone runs \`paparazzi stop\`, you can't just kill the process. You need to send SIGTERM, which the daemon catches, allowing it to clean up resources, unregister the global hotkey, and exit gracefully.

**Status Checking**

How do you implement \`paparazzi status\`? Read the PID file, check if that process actually exists, show resource usage, display log locations.

All of this adds up to about 150 lines of careful, error-prone code that has nothing to do with taking screenshots.

---

## The Solution: Getting It Right

After understanding all these requirements, I implemented proper daemon management for Paparazzi. The key was being methodical about each step.

Here's how the daemon management works in Paparazzi:

**Step 1: Safety Checks**
Before starting, check if a daemon is already running by reading the PID file and verifying the process exists. If it's already running, bail out with a helpful error.

**Step 2: Log Configuration**
Open log files in append mode. This means every time you restart the daemon, old logs are preserved. I add timestamp separators so you can see exactly when each session started.

**Step 3: Daemonization**
Fork the process twice, call \`setsid()\`, set the working directory, redirect stdout/stderr to the log files, and write the PID file. All the Unix magic happens here.

**Step 4: Signal Setup**
Register handlers for SIGTERM and SIGINT. When these signals arrive, the daemon can clean up resources before exiting.

**Step 5: Run The Service**
Once daemonized, register the global hotkey and enter the event loop.

The result is clean daemon management that handles all edge cases properly.

---

## The Attach Command: My Favorite Feature

One problem with daemons is you can't see what they're doing. Logs go to a file, and you have to manually \`tail -f\` them.

I added an \`attach\` command that lets you peek at a running daemon:

\`\`\`bash
# Terminal 1: Start the daemon
paparazzi run --background

# Terminal 2: Watch what it's doing
paparazzi attach --follow

# See live logs as the daemon works
# Press Ctrl+C to detach (daemon keeps running)
\`\`\`

It's like plugging a monitor into a background process. Perfect for debugging without stopping the service.

---

## Log Management: The Append vs Overwrite Debate

This turned into an interesting design decision. When you restart a daemon, what should happen to old logs?

**Option 1: Overwrite**
Start fresh every time. Clean logs, but you lose history.

**Option 2: Append**
Keep all logs forever. Full history, but files grow unbounded.

**Option 3: Rotate**
Keep the last N sessions, automatically deleting old ones.

I chose append with session separators. Here's what the logs look like:

\`\`\`
============================================================
Session started at 2024-01-15 10:00:00
============================================================
Paparazzi Screenshot Service
Hotkey registered: Ctrl+Shift+S
Service running...

Hotkey pressed!
Screenshot saved to /tmp/screenshot_1234.png
Sent to Claude Code successfully

Shutdown signal received
Cleaning up...
Goodbye

============================================================
Session started at 2024-01-15 14:30:00
============================================================
Paparazzi Screenshot Service
Hotkey registered: Ctrl+Shift+S
Service running...
\`\`\`

The separators make it immediately clear when restarts happened. You get full history without confusion.

---

## Signal Handling: The Right Way to Stop

When you run \`paparazzi stop\`, you're sending SIGTERM to the daemon. This is different from SIGKILL (the dreaded \`kill -9\`).

SIGTERM is polite. It says: "Please shut down when you're ready." The daemon can catch this signal, clean up resources, and exit gracefully.

SIGKILL is brutal. It says: "Die immediately." The process has no chance to cleanup. PID files stay around, global hotkeys stay registered, resources leak.

My signal handler does this:

\`\`\`rust
fn setup_signal_handler(shutdown_flag: Arc<AtomicBool>) {
    let mut signals = Signals::new(&[SIGTERM, SIGINT])?;

    thread::spawn(move || {
        for signal in signals.forever() {
            match signal {
                SIGTERM | SIGINT => {
                    println!("Shutdown signal received");
                    shutdown_flag.store(true, Ordering::Relaxed);
                    cleanup_and_exit();
                }
                _ => {}
            }
        }
    });
}
\`\`\`

When SIGTERM arrives, we set a flag that the event loop checks. It exits cleanly, unregisters the hotkey, removes the PID file, and terminates.

No orphaned processes. No leaked resources. Clean shutdown every time.

---

## The Results

After implementing proper daemon management, Paparazzi has:
- Clean background mode that actually works
- Proper signal handling and graceful shutdown
- Reliable PID file management
- Clear log separation between sessions
- No orphaned processes or resource leaks

The daemon implementation adds about 150 lines of careful code, but it's worth it. Users can now run \`paparazzi run --background\` and forget about it - the tool just works in the background, ready whenever they need it.

---

## What I Learned

**Daemons are harder than they look.** The naive approach (just fork and run) leads to zombie processes, leaked resources, and debugging nightmares. Proper daemonization requires understanding Unix process management at a deeper level than most applications need.

**Every detail matters.** From PID file handling to signal management to log rotation - each piece needs careful attention. Miss one detail and you get stale PID files, orphaned processes, or confused users.

**Good logging saves hours of debugging.** Append mode with session separators means you can see exactly what happened across multiple daemon restarts. This has saved me countless times when tracking down intermittent issues.

**The attach command changes everything.** Being able to peek into a running daemon without stopping it makes debugging so much easier. It's like having a window into a black box.

---

## The Bigger Picture

Paparazzi is a screenshot tool, but the daemon problem taught me about the deeper challenges of system programming. Understanding process lifecycle, signal handling, and Unix conventions made me a better developer.

More importantly, solving the daemon problem correctly means users can rely on Paparazzi. It starts cleanly, runs reliably, and shuts down gracefully. That reliability is what makes the difference between a tool people try once and a tool they use every day.

---

**Paparazzi** will be open source and available on GitHub when released. Stay tuned if you're building CLI tools that need to run in the background, or if you're just curious about how daemon mode works under the hood.

I built it to scratch my own itch. Turns out, a lot of other people had the same itch.
`

  return (
    <div className="blog-post-container">
      <nav className="blog-nav">
        <Link to="/blogs" className="nav-link">← Back to Blogs</Link>
        <Link to="/" className="nav-link">Home</Link>
      </nav>

      <article className="blog-post-content">
        <header className="blog-post-header">
          <h1>Building Paparazzi: Solving the Daemon Problem</h1>
          <div className="blog-post-meta">
            <span className="blog-post-date">October 5, 2025</span>
            <span className="blog-post-author">By Sachin Beniwal</span>
            <span className="blog-post-read-time">12 min read</span>
          </div>
          <div className="blog-post-tags">
            <span className="blog-tag">rust</span>
            <span className="blog-tag">daemon</span>
            <span className="blog-tag">cli</span>
            <span className="blog-tag">unix</span>
          </div>
        </header>

        <div className="blog-post-body" dangerouslySetInnerHTML={{ __html: formatMarkdown(content) }} />
      </article>
    </div>
  )
}

function formatMarkdown(text: string): string {
  // Split into sections by code blocks first
  const codeBlockRegex = /```(\w*)\n([\s\S]*?)```/g
  const parts: Array<{type: 'text' | 'code', content: string, language?: string}> = []
  let lastIndex = 0
  let match

  while ((match = codeBlockRegex.exec(text)) !== null) {
    // Add text before code block
    if (match.index > lastIndex) {
      parts.push({
        type: 'text',
        content: text.slice(lastIndex, match.index)
      })
    }

    // Add code block
    parts.push({
      type: 'code',
      content: match[2],
      language: match[1] || ''
    })

    lastIndex = match.index + match[0].length
  }

  // Add remaining text
  if (lastIndex < text.length) {
    parts.push({
      type: 'text',
      content: text.slice(lastIndex)
    })
  }

  // Process each part
  return parts.map(part => {
    if (part.type === 'code') {
      return `<pre><code class="language-${part.language}">${part.content}</code></pre>`
    } else {
      return processTextContent(part.content)
    }
  }).join('')
}

function processTextContent(text: string): string {
  const lines = text.split('\n')
  const processedLines: string[] = []
  let inList = false
  let listItems: string[] = []

  for (const line of lines) {
    if (line.startsWith('## ')) {
      if (inList && listItems.length > 0) {
        processedLines.push('<ul>' + listItems.join('') + '</ul>')
        listItems = []
        inList = false
      }
      processedLines.push('<h2>' + processInlineFormatting(line.substring(3)) + '</h2>')
    }
    else if (line.startsWith('# ')) {
      if (inList && listItems.length > 0) {
        processedLines.push('<ul>' + listItems.join('') + '</ul>')
        listItems = []
        inList = false
      }
      processedLines.push('<h1>' + processInlineFormatting(line.substring(2)) + '</h1>')
    }
    else if (line === '---') {
      if (inList && listItems.length > 0) {
        processedLines.push('<ul>' + listItems.join('') + '</ul>')
        listItems = []
        inList = false
      }
      processedLines.push('<hr />')
    }
    else if (line.startsWith('- ')) {
      inList = true
      const listContent = processInlineFormatting(line.substring(2))
      listItems.push('<li>' + listContent + '</li>')
    }
    else if (line.trim() === '') {
      if (inList && listItems.length > 0) {
        processedLines.push('<ul>' + listItems.join('') + '</ul>')
        listItems = []
        inList = false
      }
      processedLines.push('')
    }
    else if (line.trim() !== '') {
      if (inList && listItems.length > 0) {
        processedLines.push('<ul>' + listItems.join('') + '</ul>')
        listItems = []
        inList = false
      }
      const processed = processInlineFormatting(line)
      processedLines.push('<p>' + processed + '</p>')
    }
  }

  // Close any remaining list
  if (inList && listItems.length > 0) {
    processedLines.push('<ul>' + listItems.join('') + '</ul>')
  }

  return processedLines.join('\n')
}

function processInlineFormatting(text: string): string {
  return text
    .replace(/\*\*([^*]+)\*\*/g, '<strong>$1</strong>')
    .replace(/\*([^*]+)\*/g, '<em>$1</em>')
    .replace(/`([^`]+)`/g, '<code>$1</code>')
}

export default BlogPost