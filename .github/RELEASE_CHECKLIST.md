# Release Checklist

Quick reference checklist for releasing a new version of droidpi.

## Pre-Release Checklist

- [ ] All changes are merged to `main` branch
- [ ] All tests pass locally:
  ```bash
  cargo test --all-features
  cargo clippy --all-features -- -D warnings
  cargo build --release
  ```
- [ ] Documentation is up to date
- [ ] README.md reflects current features
- [ ] CHANGELOG.md is updated (if exists)
- [ ] Version number follows [Semantic Versioning](https://semver.org/)

## Release Steps

### 1. Update Version
- [ ] Update version in `Cargo.toml`
  ```toml
  version = "X.Y.Z"
  ```

### 2. Commit Version Bump
```bash
git add Cargo.toml CHANGELOG.md  # Add CHANGELOG if it exists
git commit -m "chore: bump version to X.Y.Z"
```

### 3. Create and Push Tag
```bash
# Create annotated tag
git tag -a vX.Y.Z -m "Release version X.Y.Z"

# Push commit and tag
git push origin main
git push origin vX.Y.Z
```

### 4. Monitor Workflow
- [ ] Check GitHub Actions: https://github.com/davidgaspardev/droidpi/actions
- [ ] Verify all jobs pass (validate, test, publish-crates, create-github-release)

### 5. Verify Publication
- [ ] Check crates.io: https://crates.io/crates/droidpi
- [ ] Check GitHub Releases: https://github.com/davidgaspardev/droidpi/releases
- [ ] Test installation: `cargo install droidpi@X.Y.Z`

## Post-Release

- [ ] Announce release (Twitter, Discord, Reddit, etc.)
- [ ] Monitor for issues
- [ ] Respond to user feedback

## Emergency Rollback

If something goes wrong:

```bash
# Yank the version (prevents new installs)
cargo yank --version X.Y.Z

# Or un-yank if it was a mistake
cargo yank --undo --version X.Y.Z
```

## Quick Commands

```bash
# Check current version
grep '^version = ' Cargo.toml | head -1

# List all tags
git tag -l

# Delete a local tag (if needed)
git tag -d vX.Y.Z

# Delete a remote tag (use with caution!)
git push origin :refs/tags/vX.Y.Z

# Dry run publish
cargo publish --dry-run
```

## Version Examples

- Bug fix: `0.1.0` → `0.1.1`
- New feature: `0.1.1` → `0.2.0`
- Breaking change: `0.9.0` → `1.0.0`
- Pre-release: `1.0.0-beta.1`, `1.0.0-rc.1`

## Need Help?

See [RELEASE.md](.github/RELEASE.md) for detailed documentation.