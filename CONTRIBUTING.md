# Contributing to HT-MCP

Thank you for your interest in contributing to HT-MCP! This document provides guidelines and information for contributors.

## 🚀 Quick Start

### Development Setup

1. **Prerequisites**
   - Rust 1.70 or later
   - Git
   - A GitHub account

2. **Clone the Repository**
   ```bash
   git clone --recursive https://github.com/memextech/ht-mcp.git
   cd ht-mcp
   ```

3. **Build and Test**
   ```bash
   # Build the project
   cargo build
   
   # Run tests
   cargo test
   
   # Check code formatting
   cargo fmt --check
   
   # Run linting
   cargo clippy -- -D warnings
   ```

### Making Changes

1. **Create a Fork**: Fork the repository on GitHub
2. **Create a Branch**: `git checkout -b feature/your-feature-name`
3. **Make Changes**: Implement your changes
4. **Test**: Ensure all tests pass and add new tests if needed
5. **Format**: Run `cargo fmt` to format your code
6. **Lint**: Run `cargo clippy` and fix any issues
7. **Commit**: Use conventional commit messages (see below)
8. **Push**: Push your branch to your fork
9. **Pull Request**: Create a pull request to the main repository

## 📝 Commit Convention

We use [Conventional Commits](https://conventionalcommits.org/) for commit messages:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Types
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks
- `ci`: CI/CD changes

### Examples
```bash
feat: add new MCP tool for session management
fix: resolve memory leak in session cleanup
docs: update installation instructions
test: add integration tests for error handling
```

## 🧪 Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests for specific package
cargo test --package ht-core
```

### Test Categories

1. **Unit Tests**: Test individual functions and modules
2. **Integration Tests**: Test component interactions
3. **End-to-End Tests**: Test complete workflows

### Writing Tests

- Place unit tests in the same file as the code being tested using `#[cfg(test)]`
- Place integration tests in the `tests/` directory
- Use descriptive test names that explain what is being tested
- Include both positive and negative test cases
- Mock external dependencies when appropriate

## 🏗️ Code Style

### Rust Guidelines

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for consistent formatting
- Fix all `cargo clippy` warnings
- Write documentation for public APIs
- Use meaningful variable and function names

### Documentation

- Document all public functions, structs, and modules
- Include examples in documentation when helpful
- Update README.md if your changes affect usage
- Update CHANGELOG.md following [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) format

## 🐛 Bug Reports

When reporting bugs, please include:

1. **Environment**: OS, Rust version, HT-MCP version
2. **Steps to Reproduce**: Clear, step-by-step instructions
3. **Expected Behavior**: What should happen
4. **Actual Behavior**: What actually happens
5. **Error Messages**: Include full error messages and stack traces
6. **Additional Context**: Any other relevant information

Use our [Bug Report Template](.github/ISSUE_TEMPLATE/bug_report.md) when creating issues.

## 💡 Feature Requests

For feature requests, please:

1. Check existing issues to avoid duplicates
2. Describe the problem your feature would solve
3. Provide a detailed description of the proposed solution
4. Consider alternative solutions
5. Discuss the impact on existing functionality

Use our [Feature Request Template](.github/ISSUE_TEMPLATE/feature_request.md) when creating issues.

## 🔒 Security

If you discover a security vulnerability, please:

1. **Do NOT** open a public issue
2. Email security@memex.tech with details
3. Include steps to reproduce the vulnerability
4. Allow time for us to address the issue before public disclosure

See our [Security Policy](SECURITY.md) for more information.

## 📦 Dependencies

### Adding Dependencies

When adding new dependencies:

1. Ensure the dependency is actively maintained
2. Check the license is compatible with MIT
3. Minimize the number of new dependencies
4. Update `Cargo.toml` with appropriate version constraints
5. Document why the dependency is needed

### Dependency Guidelines

- Prefer standard library functionality when possible
- Use well-established crates from the Rust ecosystem
- Avoid dependencies with known security issues
- Keep dependency versions up to date

## 🚀 Release Process

Releases are automated through GitHub Actions:

1. **Prepare Release**
   - Update version in `Cargo.toml`
   - Update `CHANGELOG.md`
   - Ensure all tests pass

2. **Create Release**
   - Create and push a git tag: `git tag v0.x.x && git push origin v0.x.x`
   - GitHub Actions will automatically build and publish the release

3. **Post-Release**
   - Verify the release on GitHub and crates.io
   - Update any dependent projects
   - Announce the release if significant

## 🤝 Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you agree to uphold this code.

## 📞 Getting Help

- **Documentation**: Check the README and inline documentation first
- **Discussions**: Use GitHub Discussions for questions and ideas
- **Issues**: Create an issue for bugs or feature requests
- **Community**: Join our community channels (links in README)

## 🙏 Recognition

Contributors are recognized in the following ways:

- Listed in the repository contributors
- Mentioned in release notes for significant contributions
- Invited to join the maintainers team for outstanding contributions

Thank you for contributing to HT-MCP! 🎉