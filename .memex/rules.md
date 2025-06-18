# ht-mcp Project Rules & Memory

## Overall Scope
- **Project**: ht-mcp - A Rust-based MCP (Model Context Protocol) server for headless terminal interactions
- **Repository**: https://github.com/memextech/ht-mcp (PUBLIC)
- **Current Branch**: feature/oss-setup  
- **Status**: Production-ready with fully functional CI infrastructure and public installation available

## User Preferences
- Always use `uv` for Python environments (already installed)
- Use `bun` instead of `npm` for Node.js projects
- Structure large file changes as sequence of smaller edits
- Be concise - do the task, give to-the-point answers
- Use `ripgrep` instead of `grep`
- Use `gh cli` for GitHub API interactions

## Architecture & Design

### Core Implementation
- **Language**: Pure Rust implementation
- **Dependencies**: Embedded ht library as git submodule
- **MCP Protocol**: Full implementation with 6 tools available
- **Submodule**: `ht-core/` pointing to https://github.com/memextech/ht.git

### MCP Tools Implemented
1. `ht_create_session` - Create new terminal sessions
2. `ht_take_snapshot` - Capture terminal state
3. `ht_send_keys` - Send input to terminal
4. `ht_execute_command` - Execute commands and return output
5. `ht_close_session` - Clean up sessions
6. `ht_list_sessions` - List active sessions

### Key Files
- `src/main.rs` - Main MCP server implementation
- `src/ht_integration/` - Direct library integration modules
  - `session_manager.rs` - Session management with ht-core library
  - `command_bridge.rs` - Command bridging
  - `event_handler.rs` - Event handling
- `.github/workflows/ci.yml` - Single consolidated CI pipeline
- `Cargo.toml` - Dependencies and lint configuration

## CI Infrastructure

### Supported Platforms
- **Ubuntu Latest (Stable Rust)** ✅
- **macOS Latest (Stable Rust)** ✅
- **Windows Latest (Stable Rust)** ✅ (Experimental)

### CI Validation Pipeline
- Code formatting (`cargo fmt --all -- --check`)
- Linting (`cargo clippy --all-targets --all-features -- -D warnings`)
- Build (`cargo build --verbose`)
- Tests (`cargo test --verbose` with `RUSTFLAGS="--cfg ci"`)
- Release build (`cargo build --release`)

### Critical CI Configuration
```yaml
- name: Run tests
  run: cargo test --verbose
  env:
    RUSTFLAGS: "--cfg ci"
```

### Test Strategy
- **Integration tests**: Disabled in CI via `#[cfg(not(ci))]` 
- **Reason**: CI environments can't run terminal-dependent processes
- **Local development**: Full test suite runs without `ci` flag
- **Unit tests**: Always run in CI

## Implementation Instructions

### Development Workflow
1. Work off `main` branch (CI configured for both `main` and `feature/oss-setup`)
2. **CRITICAL: ALWAYS run `cargo fmt --all` before every single commit** - CI will fail on formatting violations
3. **MANDATORY PRE-COMMIT CHECKLIST:**
   - `cargo fmt --all` (fix formatting)
   - `cargo fmt --all -- --check` (verify no formatting issues)
   - `cargo clippy --all-targets` (check linting)
   - `cargo build` (verify compilation)
4. All CI must pass before merging
5. Integration tests are disabled in CI but work locally
6. Use `shell: bash` for cross-platform CI commands

### Code Quality Requirements
- **Formatting**: Run `cargo fmt --all` before every commit
- **Linting**: `cargo clippy --all-targets` must pass
- **Tests**: All tests must pass with `RUSTFLAGS="--cfg ci"`
- **Build**: Both debug and release builds must succeed

### Diagnosing Issues
1. **Formatting Failures**: Run `cargo fmt --all` to fix code formatting - **MOST COMMON CI FAILURE**
2. **CI Failures**: Check for missing `RUSTFLAGS="--cfg ci"` 
3. **Test Failures**: Verify integration tests are properly marked with `#[cfg(not(ci))]`
4. **Platform Issues**: Focus on Unix platforms (Ubuntu/macOS)
5. **Submodule Issues**: Ensure pointing to correct fork (memextech/ht.git) and that commits are pushed to remote

### Key Patterns
```rust
// Conditional test compilation
#[tokio::test]
#[cfg(not(ci))] // Skip in CI environments
async fn test_native_session_manager() {
    // Terminal-dependent tests
}

// Conditional imports
#[cfg(test)]
mod tests {
    #[cfg(not(ci))]
    use super::*;
}
```

### Deployment
- Repository is PUBLIC and ready for production use
- Install via `cargo install --git https://github.com/memextech/ht-mcp --branch feature/oss-setup ht-mcp`
- Future: `cargo install ht-mcp` (once MCP SDK publishes to crates.io)
- Requires Unix-like environment (Linux/macOS)

## Recent Achievements
- ✅ Fixed all CI pipeline issues (including formatting requirements)
- ✅ Resolved cross-platform command compatibility  
- ✅ Implemented conditional test compilation
- ✅ Configured proper Rust linting
- ✅ Established reliable CI matrix with `main` branch support
- ✅ **CONSOLIDATED: CI workflows to prevent duplicate builds**
- ✅ **FIXED: Parameter name mismatch in MCP schemas** 
- ✅ **FIXED: Response formatting to match TypeScript implementation**
- ✅ **FIXED: Code formatting compliance for CI**
- ✅ **FIXED: Centralized tool definitions to eliminate duplicate schemas**
- ✅ **FIXED: CTRL+KEY sequence parsing bug in library integration**
- ✅ **FIXED: Enter key for interactive CLI tools (v0.1.2)**
- ✅ **REMOVED: Dead code from native webserver approach**
- ✅ **ADDED: Comprehensive test coverage for key parsing**

### CTRL+KEY Sequence Bug Fix
- **Issue**: `C-c`, `C-x` format keys were sent as literal text instead of control characters
- **Root Cause**: Missing pattern matching for `C-key` format in `parse_key_to_input_seq()`
- **Solution**: Added support for both `C-key` and `^key` formats with identical behavior
- **Testing**: Added 9 comprehensive test cases covering all key parsing scenarios
- **Result**: Both `C-x C-c` and `^X^C` now work to quit emacs properly

### Tool Definition Centralization Fix
- **Issue**: Tool definitions were duplicated between main.rs (hardcoded) and tools.rs (centralized)
- **Root Cause**: main.rs was using hardcoded JSON arrays instead of calling get_tool_definitions()
- **Solution**: Updated main.rs to use `crate::mcp::tools::get_tool_definitions()` for single source of truth
- **Result**: All tool definitions now come from centralized location with consistent schemas

## Critical Bug Fixes (Latest)
### Parameter Name Mismatch Fix
- **Issue**: MCP schema exposed `session_id` but Rust structs expected `sessionId`
- **Root Cause**: Hardcoded schemas in main.rs vs proper schema functions in types.rs
- **Solution**: Updated main.rs to use schema functions from types.rs
- **Result**: All tools now work correctly with proper parameter names

### Response Formatting Fix  
- **Issue**: Server returned raw JSON instead of formatted text like TypeScript version
- **Root Cause**: main.rs serialized JSON response instead of extracting formatted text
- **Solution**: Added `format_tool_response()` function matching TypeScript patterns
- **Result**: Perfect response format compatibility with markdown code blocks

## Current Status
- **All MCP Tools**: Fully functional with proper parameter handling ✅
- **Response Format**: Matches TypeScript implementation exactly ✅ 
- **Text Formatting**: Human-readable with markdown code blocks ✅
- **Web Server Integration**: Working with emoji indicators ✅
- **CI Status**: All workflows passing ✅
- **Documentation**: Complete overhaul for v0.1.0 release ✅
- **License**: Updated to MIT with correct copyright ✅
- **Repository**: Clean and production-ready ✅
- **Status**: **PRODUCTION READY** - Version 0.1.0 released

## Key Learnings
1. **Platform Focus**: ht-mcp is inherently Unix/Linux focused
2. **CI Environment**: Can't run terminal-dependent integration tests
3. **Rust Ecosystem**: Custom cfg flags require proper declaration
4. **Formatting Critical**: `cargo fmt --all` must be run before every commit - CI fails on formatting violations
5. **Strategy**: Focus on supported platforms rather than universal compatibility

## Memory Notes
- The project uses embedded ht library via git submodule
- Integration tests require actual terminal processes, hence CI exclusion
- CI configuration is critical - missing `RUSTFLAGS` breaks test runs
- **CRITICAL**: Always run `cargo fmt --all` before committing - CI will fail otherwise
- Windows support is experimental - requires windows-support branch of ht-core
- Platform-specific code should be encapsulated in helper functions
- Use conditional compilation with cfg(windows) and cfg(unix) for platform differences - CI will fail otherwise
- Project is production-ready with robust CI infrastructure