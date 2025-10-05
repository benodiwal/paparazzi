import { Link } from 'react-router-dom'
import './Blogs.css'

function Blogs() {
  const blogPosts = [
    {
      id: 1,
      title: "Building Paparazzi: From Idea to CLI Tool",
      date: "2025-01-15",
      author: "Sachin Beniwal",
      excerpt: "The journey of creating a Rust-based screenshot tool that integrates seamlessly with Claude Code. From initial conception to the first working prototype.",
      tags: ["rust", "cli", "development"],
      readTime: "5 min read"
    },
    {
      id: 2,
      title: "Why Global Hotkeys Matter for Developer Productivity",
      date: "2025-01-10",
      author: "Sachin Beniwal",
      excerpt: "Exploring the importance of frictionless screenshot capture in modern development workflows and how global hotkeys eliminate context switching.",
      tags: ["productivity", "ux", "developer-tools"],
      readTime: "3 min read"
    },
    {
      id: 3,
      title: "Deep Dive: macOS Core Graphics and Screenshot APIs",
      date: "2025-01-05",
      author: "Sachin Beniwal",
      excerpt: "Technical exploration of macOS native screenshot capabilities and how Paparazzi leverages Core Graphics for optimal performance.",
      tags: ["macos", "core-graphics", "api"],
      readTime: "8 min read"
    },
    {
      id: 4,
      title: "The Art of Zero-Friction Developer Tools",
      date: "2024-12-28",
      author: "Sachin Beniwal",
      excerpt: "What makes a developer tool truly frictionless? Lessons learned from building tools that get out of your way and let you focus on what matters.",
      tags: ["developer-experience", "tools", "design"],
      readTime: "6 min read"
    },
    {
      id: 5,
      title: "Rust + Claude Code: A Perfect Match",
      date: "2024-12-20",
      author: "Sachin Beniwal",
      excerpt: "Why Rust was the perfect choice for building Paparazzi and how it integrates beautifully with Claude Code's workflow.",
      tags: ["rust", "claude-code", "integration"],
      readTime: "4 min read"
    }
  ]

  return (
    <div className="blogs-container">
      <nav className="blogs-nav">
        <Link to="/" className="nav-link">← Back to Home</Link>
        <Link to="/docs" className="nav-link">Documentation</Link>
      </nav>

      <div className="blogs-content">
        <header className="blogs-header">
          <h1>Paparazzi Blog</h1>
          <p>Insights, tutorials, and stories from building the ultimate screenshot tool</p>
        </header>

        <div className="blog-grid">
          {blogPosts.map((post) => (
            <article key={post.id} className="blog-card">
              <div className="blog-meta">
                <span className="blog-date">{post.date}</span>
                <span className="blog-read-time">{post.readTime}</span>
              </div>

              <h2 className="blog-title">
                <a href={`/blog/${post.id}`} className="blog-link">
                  {post.title}
                </a>
              </h2>

              <p className="blog-excerpt">{post.excerpt}</p>

              <div className="blog-footer">
                <div className="blog-author">By {post.author}</div>
                <div className="blog-tags">
                  {post.tags.map((tag) => (
                    <span key={tag} className="blog-tag">
                      {tag}
                    </span>
                  ))}
                </div>
              </div>
            </article>
          ))}
        </div>

        <section className="newsletter-section">
          <h2>Stay Updated</h2>
          <p>Get notified about new blog posts, features, and updates</p>
          <div className="newsletter-form">
            <input
              type="email"
              placeholder="your.email@example.com"
              className="newsletter-input"
            />
            <button className="newsletter-button">Subscribe</button>
          </div>
          <p className="newsletter-note">
            No spam, unsubscribe anytime. Updates about Paparazzi development and related topics.
          </p>
        </section>

        <section className="featured-topics">
          <h2>Explore Topics</h2>
          <div className="topics-grid">
            <div className="topic-card">
              <h3>Rust Development</h3>
              <p>Deep dives into Rust programming, system integration, and performance optimization</p>
              <span className="topic-count">3 articles</span>
            </div>

            <div className="topic-card">
              <h3>Developer Productivity</h3>
              <p>Tools, workflows, and techniques for maximizing developer efficiency</p>
              <span className="topic-count">2 articles</span>
            </div>

            <div className="topic-card">
              <h3>macOS Integration</h3>
              <p>Native macOS development, APIs, and system-level programming</p>
              <span className="topic-count">1 article</span>
            </div>

            <div className="topic-card">
              <h3>CLI Design</h3>
              <p>Building intuitive command-line interfaces and developer experiences</p>
              <span className="topic-count">2 articles</span>
            </div>
          </div>
        </section>
      </div>
    </div>
  )
}

export default Blogs