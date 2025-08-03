# Release Guide for UniteSwap

## Creating a GitHub Release

Follow these steps to create a release that automatically builds and uploads CLI binaries:

### 1. Prepare for Release

```bash
# Ensure all changes are committed
git status

# Run tests to ensure everything works
cargo test --workspace

# Test the release build locally
./test-release-build.sh
```

### 2. Create and Push a Tag

```bash
# Create a new tag (use semantic versioning)
git tag v0.1.0

# Push the tag to GitHub
git push origin v0.1.0
```

### 3. Create the Release on GitHub

1. Go to https://github.com/susumutomita/UniteDefi/releases
2. Click "Draft a new release"
3. Select the tag you just created (v0.1.0)
4. Fill in the release details:

**Release Title**: `UniteSwap v0.1.0 - ETHGlobal Unite Hackathon`

**Release Description**:
```markdown
## UniteSwap CLI v0.1.0

First release of UniteSwap CLI for ETHGlobal Unite hackathon.

### Features
- ✅ Cross-chain atomic swaps between Ethereum and NEAR
- ✅ Integration with 1inch Limit Order Protocol
- ✅ HTLC implementation on both chains
- ✅ Real blockchain transaction submission
- ✅ Integrated swap command for easy usage

### Quick Start

Download the appropriate binary for your platform below, then:

```bash
tar -xzf fusion-cli-*.tar.gz
cd fusion-cli-*
./run.sh --help
```

See [RELEASES.md](https://github.com/susumutomita/UniteDefi/blob/main/RELEASES.md) for detailed instructions.

### Supported Platforms
- Linux (x86_64)
- macOS Intel (x86_64)
- macOS ARM (M1/M2)
- Windows (x86_64)

### Demo
For a live demonstration without setup, see our [Demo Guide](https://github.com/susumutomita/UniteDefi/blob/main/demo/DEMO_GUIDE.md).
```

5. Click "Publish release"

### 4. Monitor the Build

1. Go to Actions tab: https://github.com/susumutomita/UniteDefi/actions
2. You should see the "Release" workflow running
3. Wait for all builds to complete (usually 5-10 minutes)
4. Check the release page - binaries should be automatically attached

### 5. Test the Released Binaries

```bash
# Download one of the binaries
curl -LO https://github.com/susumutomita/UniteDefi/releases/download/v0.1.0/fusion-cli-v0.1.0-aarch64-apple-darwin.tar.gz

# Extract and test
tar -xzf fusion-cli-*.tar.gz
cd fusion-cli-*
./run.sh --help
```

## Troubleshooting

### If the workflow fails:

1. Check the Actions tab for error details
2. Common issues:
   - Missing Rust targets: The workflow installs these automatically
   - Compilation errors: Test locally first with `./test-release-build.sh`
   - Permission issues: Ensure GITHUB_TOKEN has proper permissions

### Manual upload (if needed):

If automatic upload fails, you can manually upload:

1. Build locally for your platform:
   ```bash
   cargo build -p fusion-cli --release
   ```

2. Create archive:
   ```bash
   VERSION=v0.1.0
   TARGET=x86_64-apple-darwin  # adjust for your platform
   STAGING="fusion-cli-$VERSION-$TARGET"
   
   mkdir -p "$STAGING"
   cp target/release/fusion-cli "$STAGING/"
   cp README.md RELEASES.md "$STAGING/"
   cp .env.example "$STAGING/"
   
   tar czf "$STAGING.tar.gz" "$STAGING"
   ```

3. Upload manually on the release page

## Version Numbering

Follow semantic versioning:
- `v0.1.0` - Initial hackathon release
- `v0.1.1` - Bug fixes
- `v0.2.0` - New features
- `v1.0.0` - Production ready

## Checklist Before Release

- [ ] All tests pass
- [ ] Documentation is up to date
- [ ] RELEASES.md has clear instructions
- [ ] Demo scripts work
- [ ] No hardcoded secrets or keys
- [ ] Version number updated if needed