# Setup Guide for Automated crates.io Publishing

This guide will walk you through the one-time setup required to enable automated publishing to crates.io for the droidpi project.

## Table of Contents

1. [Overview](#overview)
2. [Step-by-Step Setup](#step-by-step-setup)
3. [Verification](#verification)
4. [First Release](#first-release)

## Overview

The automated release workflow uses GitHub Actions to:
- Validate your release
- Run all tests
- Publish to crates.io
- Create GitHub releases

This requires a one-time setup to securely store your crates.io API token.

## Step-by-Step Setup

### Step 1: Create a crates.io Account

1. Visit [crates.io](https://crates.io/)
2. Click "Log in with GitHub" in the top right
3. Authorize crates.io to access your GitHub account
4. Your account is now created!

### Step 2: Generate a crates.io API Token

1. Once logged in to crates.io, click your profile icon (top right)
2. Select **"Account Settings"**
3. In the left sidebar, click **"API Tokens"**
4. Click the **"New Token"** button
5. Fill in the token details:
   - **Name**: `droidpi-github-actions` (or any descriptive name)
   - **Expiration**: Choose "No expiration" or set a date (you'll need to renew it)
   - **Scopes**: Select **"publish-update"** (this is the most secure option)
     - ✅ `publish-update`: Allows publishing new versions and updating existing crates
     - ❌ Avoid `publish-new` unless this is your first crate
6. Click **"Create"**
7. **IMPORTANT**: Copy the token immediately! You won't be able to see it again.
   - The token looks like: `cio_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx`

### Step 3: Add the Token to GitHub Secrets

1. Go to your GitHub repository:
   ```
   https://github.com/davidgaspardev/droidpi
   ```

2. Click on **"Settings"** (top menu, requires admin access)

3. In the left sidebar, expand **"Secrets and variables"** → Click **"Actions"**

4. Click the **"New repository secret"** button (green button)

5. Fill in the secret details:
   - **Name**: `CARGO_REGISTRY_TOKEN` (must be exactly this)
   - **Secret**: Paste the token you copied from crates.io
   - **Important**: Make sure there are no extra spaces or newlines

6. Click **"Add secret"**

7. You should now see `CARGO_REGISTRY_TOKEN` in your list of secrets

### Step 4: Verify Workflow File

The workflow file should already exist at `.github/workflows/release.yml`. You can verify it's there:

```bash
ls -la .github/workflows/release.yml
```

If it doesn't exist, the file has been created for you in this session.

### Step 5: Set up Repository Permissions

Ensure the workflow has permission to create releases:

1. Go to repository **Settings** → **Actions** → **General**
2. Scroll to **"Workflow permissions"**
3. Select **"Read and write permissions"**
4. Check ✅ **"Allow GitHub Actions to create and approve pull requests"**
5. Click **"Save"**

## Verification

Let's verify everything is set up correctly:

### Check 1: Secret is Set

1. Go to **Settings** → **Secrets and variables** → **Actions**
2. You should see `CARGO_REGISTRY_TOKEN` listed
3. ✅ If you see it, you're good!

### Check 2: Workflow Exists

```bash
cat .github/workflows/release.yml
```

You should see a workflow file with jobs like `validate`, `test`, `publish-crates`, etc.

### Check 3: Cargo.toml is Ready

Verify your `Cargo.toml` has the necessary metadata:

```bash
grep -E 'name|version|description|license|repository' Cargo.toml
```

Should show:
- `name = "droidpi"`
- `version = "0.1.0"` (or your current version)
- `description = "..."`
- `license = "MIT"`
- `repository = "https://github.com/davidgaspardev/droidpi"`

### Check 4: Test Locally (Dry Run)

Before actually publishing, test that your package can be published:

```bash
cargo publish --dry-run
```

Expected output:
```
    Updating crates.io index
   Packaging droidpi v0.1.0 (...)
   Verifying droidpi v0.1.0 (...)
   Compiling droidpi v0.1.0 (...)
    Finished dev [unoptimized + debuginfo] target(s)
   Packaged X files, Y (Z compressed)
```

If you see errors, fix them before proceeding.

## First Release

Now you're ready to publish your first release! Follow these steps:

### Option A: Using Git Tags (Recommended)

This triggers the full release workflow including GitHub release creation.

```bash
# 1. Make sure everything is committed
git status

# 2. Update version in Cargo.toml (if needed)
# Current version: 0.1.0

# 3. Commit any changes
git add Cargo.toml
git commit -m "chore: prepare for initial release"

# 4. Create an annotated tag
git tag -a v0.1.0 -m "Initial release of droidpi"

# 5. Push the commit and tag
git push origin main
git push origin v0.1.0
```

### Option B: Manual Trigger

You can also trigger the workflow manually:

1. Go to **Actions** tab in GitHub
2. Click on **"Release and Publish"** workflow
3. Click **"Run workflow"** button
4. Select branch: `main`
5. Click **"Run workflow"**

### Watch the Workflow

1. Go to the **Actions** tab: https://github.com/davidgaspardev/droidpi/actions
2. You should see a new workflow run named "Release and Publish"
3. Click on it to see the progress
4. The workflow has 4 main jobs:
   - ✅ **validate**: Checks version matches tag
   - ✅ **test**: Runs all tests
   - ✅ **publish-crates**: Publishes to crates.io
   - ✅ **create-github-release**: Creates GitHub release

### Verify Publication

After the workflow completes successfully:

1. **Check crates.io**: https://crates.io/crates/droidpi
   - Your crate should appear within a few minutes

2. **Check GitHub Releases**: https://github.com/davidgaspardev/droidpi/releases
   - You should see a new release with changelog

3. **Test Installation**:
   ```bash
   cargo install droidpi
   droidpi --help
   ```

## Troubleshooting

### "CARGO_REGISTRY_TOKEN secret is not set"

**Problem**: The workflow can't find your token.

**Solution**: 
1. Double-check the secret name is exactly `CARGO_REGISTRY_TOKEN`
2. Make sure you added it to the correct repository
3. Re-add the secret if necessary

### "Version mismatch"

**Problem**: Tag version doesn't match `Cargo.toml` version.

**Solution**:
```bash
# If tag is v0.1.0, ensure Cargo.toml has:
version = "0.1.0"

# Then commit and update the tag:
git add Cargo.toml
git commit -m "fix: sync version"
git tag -d v0.1.0  # Delete old tag
git tag -a v0.1.0 -m "Initial release"
git push origin main --force-with-lease
git push origin v0.1.0 --force
```

### "Crate already exists"

**Problem**: You're trying to publish a version that already exists.

**Solution**: You cannot republish the same version. Increment the version:
```bash
# Update Cargo.toml version to 0.1.1 (or 0.2.0)
git add Cargo.toml
git commit -m "chore: bump version to 0.1.1"
git tag -a v0.1.1 -m "Release version 0.1.1"
git push origin main
git push origin v0.1.1
```

### Tests Fail

**Problem**: The workflow fails during the test job.

**Solution**: Run tests locally and fix issues:
```bash
cargo test --all-features
cargo clippy --all-features -- -D warnings
cargo build --release
```

## Security Best Practices

✅ **DO**:
- Use "publish-update" scope for tokens (minimal permissions)
- Store tokens in GitHub Secrets (never commit them)
- Rotate tokens every 6-12 months
- Use annotated tags for releases
- Review workflow logs after each release

❌ **DON'T**:
- Don't share your API token
- Don't commit tokens to the repository
- Don't store tokens in environment variables
- Don't use overly permissive token scopes

## Next Steps

After successful setup:

1. Read [RELEASE.md](.github/RELEASE.md) for detailed release process
2. Use [RELEASE_CHECKLIST.md](.github/RELEASE_CHECKLIST.md) for quick reference
3. Consider setting up a CHANGELOG.md for tracking changes
4. Plan your version numbering strategy

## Getting Help

If you need assistance:

1. Check the [GitHub Actions logs](https://github.com/davidgaspardev/droidpi/actions)
2. Review the documentation in `.github/RELEASE.md`
3. Check [crates.io status page](https://status.crates.io/)
4. Visit [Rust Users Forum](https://users.rust-lang.org/)

---

**Congratulations! You're all set up for automated releases! 🎉**