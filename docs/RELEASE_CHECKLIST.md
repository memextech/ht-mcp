# Release Checklist

Follow this checklist when creating a new release.

## Pre-Release

### 1. Version Management
- [ ] Update version in `Cargo.toml`
- [ ] Update version in `docs/INSTALLATION.md` examples
- [ ] Update `CHANGELOG.md` with new version and changes
- [ ] Commit version changes: `git commit -m "chore: bump version to vX.Y.Z"`

### 2. Quality Assurance  
- [ ] Run full test suite: `cargo test`
- [ ] Run clippy: `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] Check formatting: `cargo fmt --all -- --check`
- [ ] Test local build: `cargo build --release`
- [ ] Test binary works: `./target/release/ht-mcp --version`

### 3. CI Validation
- [ ] Push to GitHub and ensure all CI checks pass
- [ ] Verify both `ci.yml` and `test-ci.yml` workflows succeed

## Release Process

### 4. Create GitHub Release
- [ ] Create and push version tag: `git tag vX.Y.Z && git push origin vX.Y.Z`
- [ ] Wait for release workflow to complete
- [ ] Verify all platform binaries are uploaded to GitHub release
- [ ] Verify checksums are present for all binaries

### 5. Homebrew Distribution
The Homebrew formula should update automatically via the `update-homebrew.yml` workflow:

- [ ] Wait for `Update Homebrew Formula` workflow to complete
- [ ] Verify the formula was updated in [homebrew-tap repository](https://github.com/memextech/homebrew-tap)
- [ ] Test Homebrew installation:
  ```bash
  brew uninstall ht-mcp 2>/dev/null || true
  brew untap memextech/tap 2>/dev/null || true
  brew tap memextech/tap
  brew install ht-mcp
  ht-mcp --version
  ```

### 6. Package Registries

#### Crates.io
- [ ] Wait for `publish-crate` job in release workflow (for stable releases)
- [ ] Verify package is available: `cargo search ht-mcp`
- [ ] Test cargo installation: `cargo install ht-mcp --version X.Y.Z`

## Post-Release

### 7. Documentation Updates
- [ ] Update README.md if needed
- [ ] Update installation docs if new platforms supported
- [ ] Update examples if CLI interface changed

### 8. Verification
- [ ] Test all installation methods:
  - [ ] Homebrew: `brew install memextech/tap/ht-mcp`
  - [ ] Cargo: `cargo install ht-mcp`
  - [ ] Pre-built binary download
- [ ] Verify version shows correctly: `ht-mcp --version`
- [ ] Test basic MCP functionality works

### 9. Communication
- [ ] Update project status in relevant documentation
- [ ] Consider announcing on relevant channels if major release

## Troubleshooting

### Failed Release Workflow
- Check workflow logs for specific errors
- Common issues:
  - Cargo.lock out of sync: `cargo update`
  - Test failures in CI: Check `RUSTFLAGS="--cfg ci"` configuration
  - Cross-compilation issues: Verify target support

### Failed Homebrew Update
- Check `update-homebrew.yml` workflow logs
- Verify `HOMEBREW_TAP_TOKEN` secret is configured
- Manual update if needed:
  ```bash
  ./scripts/update-homebrew-formula.sh vX.Y.Z
  ```

### Crates.io Publishing Issues
- Verify `CARGO_REGISTRY_TOKEN` secret is configured
- Check for name conflicts or existing versions
- Manual publish if needed: `cargo publish`

## Rollback Procedure

If a release has critical issues:

1. **Immediate Actions**
   - [ ] Mark GitHub release as "pre-release" 
   - [ ] Add warning to release notes

2. **Homebrew Rollback**
   - [ ] Revert formula to previous version in tap repository
   - [ ] Update version and checksums to last known good release

3. **Registry Actions**
   - [ ] Cannot unpublish from crates.io (policy)
   - [ ] Publish patch version with fix instead

4. **Communication**
   - [ ] Update README with known issues
   - [ ] Consider pinning issue in repository

## Version Numbering

Follow [Semantic Versioning](https://semver.org/):

- **MAJOR** (X.0.0): Breaking changes to MCP interface
- **MINOR** (X.Y.0): New features, new MCP tools
- **PATCH** (X.Y.Z): Bug fixes, performance improvements

### Pre-release Versions
- **Alpha**: X.Y.Z-alpha.N (early development)
- **Beta**: X.Y.Z-beta.N (feature complete, testing)
- **RC**: X.Y.Z-rc.N (release candidate)