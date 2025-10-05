import { Link } from 'react-router-dom'
import './Blogs.css'

function Blogs() {
  const blogPosts = [
    {
      id: 1,
      title: "Building Paparazzi: Solving the Daemon Problem",
      date: "2025-10-05",
      author: "Sachin Beniwal",
      excerpt: "How I built a screenshot tool that runs in the background and learned to tame daemon mode along the way. A deep dive into process management, signal handling, and creating reusable abstractions for CLI tools.",
      tags: ["rust", "daemon", "cli", "unix"],
      readTime: "12 min read"
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

      </div>
    </div>
  )
}

export default Blogs