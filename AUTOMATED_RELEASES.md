# Automated Releases for DroiDPI

This document provides a comprehensive overview of the automated release system for DroiDPI.

## 📋 Table of Contents

- [Overview](#overview)
- [What Was Set Up](#what-was-set-up)
- [How It Works](#how-it-works)
- [Quick Start Guide](#quick-start-guide)
- [Workflow Features](#workflow-features)
- [Best Practices](#best-practices)
- [Documentation Reference](#documentation-reference)

## Overview

DroiDPI now has a fully automated release pipeline that publishes to crates.io and creates GitHub releases. The system follows industry best practices for security, testing, and version management.

## What Was Set Up

### 1. GitHub Actions Workflow (`.github/workflows/release.yml`)

A comprehensive workflow that:
- ✅ Validates version consistency between git tags and `Cargo.toml`
- ✅ Runs complete test suite (unit tests, clippy, integration tests)
- ✅ Publishes to crates.io automatically
- ✅ Creates GitHub releases with auto-generated changelogs
- ✅ Provides clear status notifications

### 2. Enhanced Cargo.toml Metadata

Added important metadata for better crates.io presentation:
- `homepage`: Links to GitHub repository
- `documentation`: Links to README
- `readme`: Includes README.md in package
- `keywords`: For discoverability (cli, image, android, flutter, icon)
- `categories`: Proper categorization (command-line-utilities, multimedia::images)

### 3. Comprehensive Documentation

Four documentation files to guide you:

- **`.github/SETUP_GUIDE.md`**: First-time setup instructions
- **`.github/RELEASE.md`**: Detailed release process documentation
- **`.github/RELEASE_CHECKLIST.md`**: Quick reference checklist
- **`README.md`**: Updated with installation and contributing sections

## How It Works

### Trigger Mechanisms

The workflow can be triggered in three ways:

#### 1. Git Tag (Recommended for Production)
```bash
git tag -a v0.2.0 -m "Release version 0.2.0"
git push origin v0.2.0
```
- Runs full validation
- Creates GitHub release
- Publishes to crates.io

#### 2. Push to Main Branch
```bash
git push origin main
```
- Publishes to crates.io
- No GitHub release created
- Use sparingly

#### 3. Manual Trigger
Via GitHub Actions UI:
- Go to Actions → Release and Publish → Run workflow
- For emergency deployments or retries

### Workflow Jobs

```
┌─────────────┐
│  validate   │  Ensures tag matches Cargo.toml version
└──────┬──────┘
       │
       v
┌─────────────┐
│    test     │  Runs all tests and checks
└──────┬──────┘
       │
       v
┌─────────────┐
│publish-     │  Publishes to crates.io
│crates       │
└──────┬──────┘
       │
       v
┌─────────────┐
│create-      │  Creates GitHub release with changelog
│github-      │
│release      │
└─────────────┘
```

## Quick Start Guide

### First Time Setup (One-time only)

**Step 1: Get crates.io Token**
1. Go to https://crates.io/settings/tokens
2. Create new token with "publish-update" scope
3. Copy the token (starts with `cio_`)

**Step 2: Add to GitHub Secrets**
1. Go to repository Settings → Secrets and variables → Actions
2. Create secret named `CARGO_REGISTRY_TOKEN`
3. Paste the token value

**Step 3: Test Locally**
```bash
cargo publish --dry-run
```

### Making a Release

**Standard Release Process:**

```bash
# 1. Update version in Cargo.toml
version = "0.2.0"

# 2. Commit the change
git add Cargo.toml
git commit -m "chore: bump version to 0.2.0"

# 3. Create annotated tag
git tag -a v0.2.0 -m "Release version 0.2.0"

# 4. Push both commit and tag
git push origin main
git push origin v0.2.0

# 5. Watch the workflow
# Visit: https://github.com/davidgaspardev/droidpi/actions
```

**That's it!** The workflow handles everything else automatically.

## Workflow Features

### Security Features

✅ **Token Security**
- Token stored in GitHub Secrets (encrypted)
- Never logged in workflow output
- Uses minimal "publish-update" scope

✅ **Permission Controls**
- Workflow has write access only to releases
- Token rotation recommended every 6-12 months

### Quality Assurance

✅ **Version Validation**
```yaml
- Ensures git tag matches Cargo.toml version
- Prevents accidental version mismatches
```

✅ **Comprehensive Testing**
```yaml
- cargo check --all-features
- cargo test --all-features
- cargo clippy -- -D warnings
- Integration tests (Flutter & Android)
```

✅ **Dry Run Before Publishing**
```yaml
- Verifies package structure
- Catches issues before actual publish
```

### Release Management

✅ **Auto-generated Changelog**
- Lists all commits since last release
- Includes commit hashes for traceability
- Shows installation instructions

✅ **GitHub Release Creation**
- Automatic release notes
- Links to crates.io
- Version comparison with previous release

✅ **Status Notifications**
- Clear success/failure messages
- Direct links to published versions

## Best Practices

### Version Numbering (Semantic Versioning)

Follow [SemVer](https://semver.org/):

```
MAJOR.MINOR.PATCH

Examples:
- 0.1.0 → 0.1.1  (Bug fixes)
- 0.1.1 → 0.2.0  (New features, backward compatible)
- 0.9.0 → 1.0.0  (Breaking changes, stable API)
```

### Release Workflow

**✅ DO:**
- Test locally before releasing
- Use annotated tags: `git tag -a v1.0.0 -m "message"`
- Follow semantic versioning
- Keep CHANGELOG.md updated
- Review workflow logs after release
- Write meaningful commit messages

**❌ DON'T:**
- Skip tests
- Force push tags
- Delete published versions (immutable)
- Commit tokens to repository
- Publish breaking changes as patch versions

### Testing Before Release

Always run these commands locally:

```bash
# Check compilation
cargo check --all-features

# Run tests
cargo test --all-features

# Check for issues
cargo clippy --all-features -- -D warnings

# Build release binary
cargo build --release

# Verify package
cargo publish --dry-run
```

## Documentation Reference

Detailed documentation is available in these files:

| Document | Purpose | When to Use |
|----------|---------|-------------|
| [`.github/SETUP_GUIDE.md`](.github/SETUP_GUIDE.md) | First-time setup | Setting up the release system |
| [`.github/RELEASE.md`](.github/RELEASE.md) | Complete documentation | Detailed release process |
| [`.github/RELEASE_CHECKLIST.md`](.github/RELEASE_CHECKLIST.md) | Quick reference | Before each release |
| `README.md` | Project overview | General information |

## Common Scenarios

### Scenario 1: Regular Feature Release

```bash
# Update version (0.1.0 → 0.2.0)
vim Cargo.toml

# Commit and tag
git add Cargo.toml
git commit -m "chore: bump version to 0.2.0"
git tag -a v0.2.0 -m "Release version 0.2.0"
git push origin main v0.2.0
```

### Scenario 2: Hotfix Release

```bash
# Update version (0.2.0 → 0.2.1)
vim Cargo.toml

# Commit and tag
git add Cargo.toml
git commit -m "fix: critical bug"
git tag -a v0.2.1 -m "Hotfix: critical bug"
git push origin main v0.2.1
```

### Scenario 3: Pre-release (Beta/RC)

```bash
# Update version (0.2.0 → 0.3.0-beta.1)
vim Cargo.toml

# Commit and tag
git add Cargo.toml
git commit -m "chore: bump version to 0.3.0-beta.1"
git tag -a v0.3.0-beta.1 -m "Beta release"
git push origin main v0.3.0-beta.1
```

### Scenario 4: Rollback (Yank Version)

If you need to prevent new installations of a broken version:

```bash
# Yank the broken version (doesn't delete it)
cargo yank --version 0.2.0

# Or un-yank if it was a mistake
cargo yank --undo --version 0.2.0

# Then publish a fixed version
# Update to 0.2.1 and follow normal release process
```

## Monitoring & Verification

After releasing, verify:

### 1. crates.io Publication
- Visit: https://crates.io/crates/droidpi
- Check version appears
- Verify metadata is correct

### 2. GitHub Release
- Visit: https://github.com/davidgaspardev/droidpi/releases
- Check release notes
- Verify tag is correct

### 3. Installation Test
```bash
cargo install droidpi@0.2.0
droidpi --help
```

### 4. Workflow Status
- Visit: https://github.com/davidgaspardev/droidpi/actions
- Check all jobs passed
- Review logs for any warnings

## Troubleshooting

### Version Mismatch Error
```
❌ Version mismatch! Cargo.toml has 0.1.0 but tag is v0.2.0
```
**Fix:** Ensure `Cargo.toml` version matches git tag (without 'v' prefix)

### Token Not Found Error
```
❌ CARGO_REGISTRY_TOKEN secret is not set
```
**Fix:** Add token to GitHub repository secrets (Settings → Secrets → Actions)

### Already Published Error
```
error: crate version 0.2.0 is already uploaded
```
**Fix:** Increment version number (can't republish same version)

### Test Failures
```
error: test failed, to rerun pass '--lib'
```
**Fix:** Run tests locally, fix issues, then retry release

## Support & Resources

- **GitHub Actions Logs**: https://github.com/davidgaspardev/droidpi/actions
- **crates.io Package**: https://crates.io/crates/droidpi
- **GitHub Releases**: https://github.com/davidgaspardev/droidpi/releases
- **Rust Docs**: https://doc.rust-lang.org/cargo/
- **SemVer Guide**: https://semver.org/

## Summary

You now have a professional, automated release system that:

1. **Validates** versions automatically
2. **Tests** thoroughly before publishing
3. **Publishes** to crates.io securely
4. **Creates** GitHub releases with changelogs
5. **Notifies** you of success or failure

The system follows industry best practices for security, testing, and version management. Simply update your version, create a tag, and push—the automation handles the rest!

---

**Need help?** See [.github/SETUP_GUIDE.md](.github/SETUP_GUIDE.md) for setup or [.github/RELEASE.md](.github/RELEASE.md) for detailed release instructions.

**Happy Releasing! 🚀**