# Scoop Package Testing

This branch is specifically for testing the Scoop package manager integration.

## Test Plan

1. **Local Scoop Manifest Test**
   - Test the manifest syntax with `scoop checkup`
   - Validate JSON structure

2. **Test Bucket Setup**
   - Create a test scoop bucket repository
   - Test installation from custom bucket

3. **Hash Validation Test**
   - Use actual release binary hash
   - Test checksum verification

4. **Auto-update Test**
   - Test the autoupdate functionality
   - Verify version detection works

## Commands to Test

```powershell
# Test manifest syntax
scoop checkup

# Install from local manifest
scoop install .\scoop\ht-mcp.json

# Test from custom bucket
scoop bucket add ht-mcp-test https://github.com/memextech/ht-mcp-scoop-test
scoop install ht-mcp-test/ht-mcp
```

## Expected Behavior

- Binary should be installed to scoop/apps/ht-mcp/current/
- `ht-mcp --version` should work
- `ht-mcp --help` should display help text
- Uninstall should clean up completely