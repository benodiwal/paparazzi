# GitHub Workflows Documentation

This repository contains several GitHub Actions workflows for CI/CD automation.

## Workflows Overview

### 1. CLI Tool CI/CD (`cli.yml`)
**Triggers:**
- Push to `main` or `develop` branches (when CLI code changes)
- Pull requests to `main` (when CLI code changes)

**Jobs:**
- **Test Suite**: Runs tests, formatting checks, and Clippy linting
- **Build**: Cross-platform builds for Linux and macOS (Intel & Apple Silicon)
- **Security Audit**: Runs `cargo audit` to check for security vulnerabilities

**Artifacts:** Pre-built binaries for all supported platforms

### 2. Landing Page CI/CD (`landing.yml`)
**Triggers:**
- Push to `main` or `develop` branches (when landing page changes)
- Pull requests to `main` (when landing page changes)

**Jobs:**
- **Lint and Test**: TypeScript checks, ESLint, and build verification
- **Build and Deploy**: Production build and deployment to Vercel
- **Lighthouse Audit**: Performance auditing on PRs

**Deployment:**
- Vercel production deployment on main branch pushes

### 3. Release (`release.yml`)
**Triggers:**
- Git tags matching `v*.*.*` pattern
- Manual dispatch with tag input

**Jobs:**
- **Create Release**: Creates GitHub release with Cargo and manual installation instructions
- **Build and Upload**: macOS and Linux binary builds and asset uploads

## Required Secrets

### For Vercel Deployment (Landing Page)
```
VERCEL_TOKEN=your_vercel_token
VERCEL_ORG_ID=your_org_id
VERCEL_PROJECT_ID=your_project_id
```

### For GitHub Releases
```
GITHUB_TOKEN=automatically_provided_by_github
```

## Setup Instructions

### 1. Vercel Deployment Setup
1. Install Vercel CLI: `npm i -g vercel`
2. Navigate to landing directory: `cd landing`
3. Link project: `vercel link`
4. Get org/project IDs: `vercel project ls`
5. Add secrets to GitHub repository settings

### 2. Cargo.toml Configuration
Ensure your `Cargo.toml` has proper metadata for publishing:
```toml
[package]
name = "paparazzi"
description = "A CLI tool for instant screenshots to Claude Code"
repository = "https://github.com/your-username/paparazzi"
license = "MIT OR Apache-2.0"
keywords = ["cli", "screenshot", "claude"]
categories = ["command-line-utilities"]
```

## Dependency Management

Dependabot is configured to automatically update:
- Rust dependencies (weekly)
- Node.js dependencies (weekly)
- GitHub Actions versions (weekly)

## Testing Workflows Locally

### CLI Workflow
```bash
# Install act (GitHub Actions local runner)
brew install act

# Run specific job
act -j test

# Run with secrets
act -j build --secret-file .secrets
```

### Landing Page Workflow
```bash
cd landing
npm ci
npm run lint
npm run build
```

## Publishing to Cargo

The CLI tool is designed to be published to crates.io. To publish:

```bash
# Login to crates.io (one time setup)
cargo login your_api_token

# Publish new version
cargo publish
```

**Note**: GitHub releases provide pre-built binaries for users who prefer not to compile from source.

## Workflow Customization

### Adding New Platforms
Edit the matrix in `cli.yml` and `release.yml`. Note that full functionality requires macOS:

```yaml
matrix:
  include:
    - os: ubuntu-latest
      target: aarch64-unknown-linux-gnu  # ARM Linux
      suffix: ""
```

**Note**: While you can add other platforms, screenshot integration only works on macOS due to the use of `screencapture` and AppleScript.

### Changing Vercel Settings
Update the Vercel action parameters in `landing.yml` or modify vercel.json configuration.

### Custom Release Notes
Modify the release body template in `release.yml` to match your changelog format.

## Troubleshooting

### Common Issues
1. **Build failures**: Check Rust version compatibility
2. **Missing secrets**: Verify Vercel secrets are set for landing page deployment
3. **Cargo publish**: Ensure you're logged in with `cargo login` and have proper permissions
4. **Cache issues**: Clear workflow caches in GitHub Actions tab

### Debugging
- Use `act` for local testing
- Enable debug logging with `ACTIONS_STEP_DEBUG=true`
- Check workflow logs in GitHub Actions tab