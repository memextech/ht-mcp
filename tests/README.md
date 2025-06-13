# Testing Documentation

This directory contains the test suite for ht-mcp, organized according to Rust best practices.

## Test Structure

### Integration Tests (`tests/`)
- `integration_mcp_protocol.rs` - MCP protocol compliance tests
- `integration_terminal_functionality.rs` - End-to-end terminal workflow tests  
- `unit_response_formatting.rs` - Response formatting unit tests

### Unit Tests
Unit tests are embedded in source files using `#[cfg(test)]` modules:
- `src/ht_integration/native_session_manager.rs` - Session management tests
- `src/ht_integration/native_webserver.rs` - Web server tests

### Examples (`examples/`)
- `ht_library_usage.rs` - Direct HT library usage example

## Running Tests

```bash
# Run all tests (unit + integration)
cargo test

# Run only integration tests
cargo test --test integration_mcp_protocol
cargo test --test integration_terminal_functionality

# Run only unit tests
cargo test --lib

# Run with debug output
RUST_LOG=debug cargo test -- --nocapture

# Run examples
cargo run --example ht_library_usage
```

## Test Categories

### Protocol Tests
- MCP JSON-RPC 2.0 compliance
- Tool registration and discovery
- Parameter validation
- Error response formats

### Functionality Tests  
- Complete terminal workflow (create → use → close)
- Session lifecycle management
- Command execution and output capture
- Web server integration
- Response format validation

### Unit Tests
- Response formatting functions
- Parameter parsing logic
- Error handling edge cases

## CI Considerations

Tests are configured for CI environments:
- Integration tests that require terminals are marked with `#[cfg(not(ci))]`
- No external dependencies required
- Proper resource cleanup
- Clear error messages

## Adding New Tests

### Integration Tests
Create new files in `tests/` for integration testing:
```rust
#[tokio::test]
async fn test_new_feature() {
    // Test complete workflows
}
```

### Unit Tests
Add to existing source files:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_function() {
        // Test individual functions
    }
}
```

### Examples
Add to `examples/` for demonstration code:
```rust
//! Example demonstrating feature X
//! Run with: `cargo run --example example_name`

fn main() {
    // Demonstration code
}
```

## Test Coverage

Current coverage:
- ✅ MCP protocol compliance
- ✅ All 6 tools functionality  
- ✅ Response format validation
- ✅ Error handling
- ✅ Web server integration
- ✅ Session lifecycle management

Future additions:
- Performance benchmarks (`benches/`)
- Property-based testing
- Fuzzing tests
- Load testing