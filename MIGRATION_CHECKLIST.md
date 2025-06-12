# Repository Migration Checklist: atlasfutures/ht-mcp-rust → memextech/ht-mcp

## ✅ Pre-Migration Tasks (Completed)

### Code Updates
- [x] **Cargo.toml**: Package name changed from "ht-mcp-rust" to "ht-mcp"
- [x] **License**: Changed from Apache-2.0 to MIT with proper LICENSE file
- [x] **Repository URLs**: Updated to point to memextech/ht-mcp
- [x] **Binary Names**: Updated in all scripts and configurations
- [x] **Documentation**: README.md updated with installation instructions
- [x] **Build Scripts**: Updated build-release.sh and install scripts
- [x] **GitHub Actions**: Updated workflow files for new repository structure
- [x] **Source Code**: Updated command names in main.rs and main_rmcp.rs

### Verification
- [x] Code compiles successfully with `cargo check`
- [x] No remaining references to "ht-mcp-rust" or "atlasfutures" found
- [x] MIT license headers ready to be added to source files

## 🔄 Migration Tasks (Requires Admin Access)

### 1. Repository Transfer
**Person Required**: Admin of both atlasfutures and memextech organizations

**Steps**:
1. Log into GitHub as organization admin
2. Navigate to: `https://github.com/atlasfutures/ht-mcp-rust/settings`
3. Scroll to "Danger Zone" → "Transfer ownership"
4. Enter new repository name: `ht-mcp` (shortened from ht-mcp-rust)
5. Enter target organization: `memextech`
6. Confirm transfer

**Expected Outcome**: Repository available at `https://github.com/memextech/ht-mcp`

### 2. Immediate Post-Transfer Tasks

#### GitHub Repository Settings
- [ ] **Branch Protection**: Set up protection rules for `main` and `develop` branches
  - Require pull request reviews
  - Require status checks to pass
  - Restrict pushes to protected branches
- [ ] **Repository Settings**: 
  - Set repository description
  - Add website URL: `https://memex.tech`
  - Add topics: `mcp`, `terminal`, `rust`, `memex`, `headless`
- [ ] **Issues/Discussions**: Enable if desired for community engagement

#### Secrets and Environment Variables
- [ ] **GitHub Actions Secrets**: Add required secrets for automated releases
  - `CARGO_REGISTRY_TOKEN`: For crates.io publishing
  - Any other deployment tokens as needed

### 3. Verification Tasks
- [ ] **Repository Access**: Verify repository is accessible at new URL
- [ ] **History Preservation**: Confirm all commits, branches, and tags are intact
- [ ] **Redirects**: Test that old URLs redirect to new repository
- [ ] **Build Pipeline**: Trigger a test workflow to ensure Actions work correctly

## 📦 Post-Migration Tasks

### Crates.io Publication
- [ ] **Reserve Crate Name**: Ensure "ht-mcp" is available on crates.io
- [ ] **First Publication**: Publish v0.1.0 using GitHub Actions or manual process
- [ ] **Documentation**: Verify docs.rs builds correctly

### Distribution Setup
- [ ] **GitHub Releases**: Create first release with multi-platform binaries
- [ ] **Install Script**: Test universal installer from new repository
- [ ] **Homebrew Tap**: Create memextech/homebrew-tap repository
- [ ] **Package Managers**: Submit to other package managers as needed

### Community Setup
- [ ] **Contributing Guidelines**: Add CONTRIBUTING.md
- [ ] **Issue Templates**: Set up bug report and feature request templates
- [ ] **Security Policy**: Add SECURITY.md for vulnerability reporting
- [ ] **Code of Conduct**: Add community guidelines

### Documentation Updates
- [ ] **Website Integration**: Update memex.tech documentation if needed
- [ ] **Blog Post**: Consider announcement blog post for migration
- [ ] **Changelog**: Update CHANGELOG.md with migration details

## 🚨 Rollback Plan

If migration issues occur:

1. **Immediate Rollback**: Transfer repository back to atlasfutures
2. **Code Restoration**: Revert all name changes in code
3. **Build Verification**: Ensure original functionality works
4. **Issue Analysis**: Identify and document problems
5. **Retry Planning**: Plan corrective actions before re-attempting

## 📞 Contacts

- **Repository Admin**: [Contact person with admin access]
- **Technical Lead**: [Person responsible for code changes]
- **DevOps**: [Person handling CI/CD and deployment]

## 📝 Migration Notes

### Timing Considerations
- **Best Time**: During low-activity period to minimize disruption
- **Dependencies**: Ensure no critical builds are running
- **Communication**: Notify team members of maintenance window

### Risk Mitigation
- All code changes prepared and tested before migration
- Repository transfer is atomic (preserves full history)
- Old URLs will redirect automatically via GitHub
- Rollback plan ready if issues occur

---

**Status**: Ready for migration once admin access is available
**Last Updated**: 2025-06-12
**Prepared By**: Memex AI Assistant