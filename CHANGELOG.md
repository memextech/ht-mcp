# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [0.1.1] - 2025-06-17

### Added
- Windows platform support
- Cross-platform terminal integration
- Platform-specific helpers for command execution
- Improved CI with Windows testing

### Changed
- Updated ht-core submodule to windows-support branch
- Refactored terminal size handling for cross-platform compatibility
- Enhanced port selection logic for different platforms
- Improved platform-specific shell detection

### Fixed
- Code formatting for CI compliance
- Platform-specific path handling
- Terminal window size structure differences between platforms

## [0.1.0] - 2025-06-13

Initial release of ht-mcp, a high-performance Rust MCP server for headless terminal automation.

### Added
- Complete MCP server implementation with 6 tools:
  - `ht_create_session` - Create terminal sessions  
  - `ht_send_keys` - Send keystrokes and special keys
  - `ht_take_snapshot` - Capture terminal state
  - `ht_execute_command` - Execute commands and get output
  - `ht_list_sessions` - List active sessions
  - `ht_close_session` - Close terminal sessions
- Direct [ht](https://github.com/andyk/ht) library integration via git submodule
- Optional web server for live terminal preview
- Multi-session support with thread-safe management
- Human-readable text responses with markdown formatting
- Cross-platform CI for Linux and macOS

### Performance
- 40x faster startup than TypeScript equivalent (~50ms vs ~2s)
- 70% less memory usage (~15MB vs ~50MB)
- Single binary deployment (4.7MB vs ~200MB Node.js)

### License
MIT License - Copyright (c) 2025 Atlas Futures Inc.