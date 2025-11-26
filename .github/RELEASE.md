# Release Workflow Documentation

This document explains how to publish new releases of `droidpi` to crates.io using our automated GitHub Actions workflow.

## Overview

The release workflow automatically:
- ✅ Validates that the git tag matches the version in `Cargo.toml`
- 🧪 Runs all tests and checks (cargo test, clippy, integration tests)
- 📦 Publishes to [crates.io](https://crates.io/crates/droidpi)
- 🏷️ Creates a GitHub release with auto-generated changelog
- 📝 Notifies about successful publication

## Prerequisites

Before you can publish releases, you need to set up a crates.io API token:

### 1. Generate a crates.io API Token

1. Go to [crates.io](https://crates.io/)
2. Sign in with your GitHub account
3. Navigate to [Account Settings](https://crates.io/settings/tokens)
4. Click "New Token"
5. Give it a descriptive name (e.g., "droidpi-github-actions")
6. Select the scope: **"publish-update"** (recommended for security)
7. Copy the generated token (you won't be able to see it again!)

### 2. Add Token to GitHub Repository Secrets

1. Go to your GitHub repository: https://github.com/davidgaspardev/droidpi
2. Navigate to **Settings** → **Secrets and variables** → **Actions**
3. Click **"New repository secret"**
4. Name: `CARGO_REGISTRY_TOKEN`
5. Value: Paste the token from crates.io
6. Click **"Add secret"**

## Release Process

### Standard Release (Recommended)

Follow these steps for a standard release:

#### 1. Update Version Number

Edit `Cargo.toml` and update the version:

```toml
[package]
version = "0.2.0"  # Update this
```

#### 2. Update CHANGELOG (Optional but Recommended)

Create or update a `CHANGELOG.md` file documenting what changed:

```markdown
## [0.2.0] - 2024-01-15

### Added
- New feature X
- New feature Y

### Fixed
- Bug fix Z

### Changed
- Improved performance
```

#### 3. Commit Changes

```bash
git add Cargo.toml CHANGELOG.md
git commit -m "chore: bump version to 0.2.0"
```

#### 4. Create and Push Tag

```bash
# Create annotated tag (recommended)
git tag -a v0.2.0 -m "Release version 0.2.0"

# Push the commit and tag
git push origin main
git push origin v0.2.0
```

#### 5. Wait for Workflow

The GitHub Actions workflow will automatically:
- Run on tag push
- Validate version matches
- Run all tests
- Publish to crates.io
- Create GitHub release

Monitor progress at: https://github.com/davidgaspardev/droidpi/actions

### Quick Release (Development)

For rapid iterations during development:

1. Update version in `Cargo.toml`
2. Commit and push to `main` branch
3. The workflow will publish automatically (no GitHub release created)

**Note:** Use this sparingly, as it bypasses version validation.

## Version Numbering (Semantic Versioning)

Follow [Semantic Versioning](https://semver.org/):

- **MAJOR** version (1.0.0): Breaking changes
- **MINOR** version (0.1.0): New features, backward compatible
- **PATCH** version (0.0.1): Bug fixes, backward compatible

Examples:
- `0.1.0` → `0.1.1`: Bug fix
- `0.1.1` → `0.2.0`: New feature
- `0.9.0` → `1.0.0`: Stable release with breaking changes

## Workflow Triggers

The release workflow runs on:

1. **Tag push**: `v*.*.*` (e.g., `v0.2.0`, `v1.0.0-beta.1`)
   - Full validation and GitHub release creation
   - Recommended for production releases

2. **Main branch push**: Any commit to `main`
   - Publishes to crates.io without creating GitHub release
   - Use with caution

3. **Manual trigger**: Via GitHub Actions UI
   - For emergency deployments or retries

## Troubleshooting

### Version Mismatch Error

**Error:** `❌ Version mismatch! Cargo.toml has X but tag is vY`

**Solution:** Ensure `Cargo.toml` version matches the git tag:
```bash
# If tag is v0.2.0, Cargo.toml should have:
version = "0.2.0"
```

### Publish Failed: Already Exists

**Error:** `error: crate version 0.2.0 is already uploaded`

**Solution:** 
- You cannot republish the same version to crates.io
- Increment the version number and create a new tag
- Versions are immutable on crates.io for security

### Token Not Found

**Error:** `❌ CARGO_REGISTRY_TOKEN secret is not set`

**Solution:** Follow the "Prerequisites" section above to add the token.

### Tests Failed

**Error:** Workflow fails during test phase

**Solution:**
- Check the test logs in GitHub Actions
- Fix the failing tests locally
- Run tests before creating a tag:
  ```bash
  cargo test --all-features
  cargo clippy --all-features
  cargo build --release
  ```

## Best Practices

### ✅ DO

- Always test locally before releasing
- Write meaningful commit messages
- Update documentation for new features
- Follow semantic versioning
- Use annotated tags (`git tag -a`)
- Keep a CHANGELOG.md
- Review the workflow run before announcing

### ❌ DON'T

- Don't skip tests
- Don't force push tags
- Don't delete published versions (they're immutable)
- Don't commit the `CARGO_REGISTRY_TOKEN` to the repository
- Don't publish breaking changes as patch versions

## Testing Before Release

Run these commands locally to ensure everything works:

```bash
# Run all checks
cargo check --all-features
cargo test --all-features
cargo clippy --all-features -- -D warnings

# Build release binary
cargo build --release

# Test the CLI
./target/release/droidpi --help

# Verify package can be published (dry run)
cargo publish --dry-run
```

## Rollback Procedure

If you need to rollback a release:

1. **crates.io**: You cannot unpublish versions (they're immutable)
   - Publish a new fixed version (e.g., 0.2.1)
   - Use `cargo yank` to prevent new projects from using the broken version:
     ```bash
     cargo yank --version 0.2.0
     ```

2. **GitHub Release**: You can delete or edit releases
   - Go to Releases page
   - Click "Delete" or "Edit" on the problematic release

3. **Git Tag**: Delete and recreate if needed (use with caution)
   ```bash
   git tag -d v0.2.0
   git push origin :refs/tags/v0.2.0
   ```

## Manual Publishing

If the workflow fails and you need to publish manually:

```bash
# Ensure you're on the correct commit
git checkout main

# Publish to crates.io
cargo publish --token YOUR_CRATES_IO_TOKEN
```

## GitHub Release Features

The automated GitHub release includes:

- 📋 Auto-generated changelog from commit messages
- 📦 Installation instructions
- 🔗 Links to crates.io
- 📝 Comparison with previous version
- 🏷️ Proper version tagging

## Monitoring

After releasing, monitor:

- [crates.io downloads](https://crates.io/crates/droidpi)
- [GitHub releases](https://github.com/davidgaspardev/droidpi/releases)
- [GitHub Actions runs](https://github.com/davidgaspardev/droidpi/actions)
- User issues and feedback

## Security Notes

- ✅ The `CARGO_REGISTRY_TOKEN` is stored securely in GitHub Secrets
- ✅ The token is not logged in workflow output
- ✅ Use "publish-update" scope for minimal permissions
- ✅ Rotate tokens periodically (every 6-12 months)
- ❌ Never commit tokens to the repository
- ❌ Never share tokens in issues or pull requests

## Support

If you encounter issues:

1. Check the [GitHub Actions logs](https://github.com/davidgaspardev/droidpi/actions)
2. Review this documentation
3. Check [crates.io status](https://status.crates.io/)
4. Open an issue on GitHub if needed

## Example Release Workflow

Complete example of releasing version 0.2.0:

```bash
# 1. Make sure you're on main and up to date
git checkout main
git pull origin main

# 2. Update version
vim Cargo.toml  # Change version to 0.2.0

# 3. Update changelog (optional)
vim CHANGELOG.md

# 4. Commit changes
git add Cargo.toml CHANGELOG.md
git commit -m "chore: bump version to 0.2.0"

# 5. Create tag
git tag -a v0.2.0 -m "Release version 0.2.0"

# 6. Push everything
git push origin main
git push origin v0.2.0

# 7. Watch the workflow
# Visit: https://github.com/davidgaspardev/droidpi/actions

# 8. Verify publication
# Visit: https://crates.io/crates/droidpi
```

---

**Happy Releasing! 🚀**