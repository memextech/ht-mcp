# HT-MCP-Rust Test Suite

This directory contains the focused test suite for the HT-MCP-Rust project, covering unit and integration testing.

## Directory Structure

```
tests/
├── unit/                   # Unit tests for individual components
│   ├── command_bridge/     # Tests for key translation
│   ├── session_manager/    # Tests for session management
│   ├── mcp/               # Tests for MCP protocol handling
│   └── ht_integration/    # Tests for HT library integration
├── integration/           # Integration tests for complete workflows
│   ├── mcp_protocol/      # MCP protocol compliance tests
│   ├── terminal_sessions/ # Terminal session lifecycle tests
│   ├── error_handling/    # Error condition tests
│   └── performance/       # Basic performance validation
├── fixtures/              # Test data and mock files
└── helpers/               # Test utilities and helper functions
```

## Test Categories

### Unit Tests
- Individual component testing
- Mock dependencies where appropriate
- Fast execution (< 1s per test)
- High code coverage focus

### Integration Tests
- Component interaction testing
- Real terminal processes (limited)
- Moderate execution time (1-10s per test)
- Protocol compliance validation

### Manual Testing
- End-to-end scenarios are tested manually using provided test scripts
- Real-world usage validation
- Performance under load
- Long-running stability tests

## Running Tests

### All Tests
```bash
cargo test
```

### Unit Tests Only
```bash
cargo test --test unit
```

### Integration Tests Only
```bash
cargo test --test integration
```

### Benchmarks
```bash
cargo bench
```

### With Coverage
```bash
cargo tarpaulin --out html
```

## Test Data

Test fixtures and mock data are stored in the `fixtures/` directory. These include:
- Sample MCP messages
- Terminal output samples
- Configuration files for different test scenarios

## Continuous Integration

The test suite is designed to run in CI environments with:
- Fast feedback for unit tests
- Scheduled integration and performance tests
- Coverage reporting
- Performance regression detection

## Writing Tests

### Guidelines
1. Use descriptive test names that explain the scenario
2. Follow the Arrange-Act-Assert pattern
3. Use appropriate test fixtures for data
4. Mock external dependencies appropriately
5. Include both positive and negative test cases

### Naming Conventions
- Test files: `test_[component_name].rs`
- Test functions: `test_[behavior]_[expected_outcome]()`
- Test modules: Match the source code structure

## Performance Baselines

Performance tests include baseline measurements for:
- Session creation: < 100ms
- Key input latency: < 10ms
- Snapshot capture: < 50ms
- Memory usage: < 10MB per session
- Concurrent sessions: Support 100+ sessions

## Test Coverage Goals

- Unit tests: > 90% line coverage
- Integration tests: > 80% feature coverage
- E2E tests: 100% critical path coverage