# Git Commit Complex Message Issue Analysis

## Problem Summary

Git commits with complex messages (containing emojis, markdown, multiline text) fail when sent through `ht_send_keys` but work correctly with `git commit -F file.txt` approach.

## Root Cause Analysis

### 1. Key Sequence Duplication
When multiple strings are sent as individual keys through `ht_send_keys`, they get duplicated and concatenated incorrectly:
```
Input: ["🤖 Generated with [Memex](https://memex.tech)", "Enter", "Co-Authored-By: Memex <noreply@memex.tech>"]
Terminal Output: > Co-Authored-By: Memex <noreply@memex.tech>"Co-Authored-By: Memex <noreply@memex.tech>"Co-Authored-By: Memex <noreply@memex.tech>"...
```

### 2. UTF-8 Emoji Encoding Issues
Emojis sent as individual key sequences get corrupted:
```
Input: "🤖"
Terminal Output: "\360\237\244\226" (raw UTF-8 bytes)
```

### 3. Shell Parsing Problems
Markdown URLs with parentheses cause bash syntax errors:
```
Input: "[Memex](https://memex.tech)"
Error: "bash: syntax error near unexpected token '('"
```

### 4. Quote Escaping Issues
Complex strings aren't properly escaped when sent as individual sequences.

## Technical Analysis

### HT Library Key Processing
Located in `ht-core/src/api/stdio.rs`:
- `parse_key()` function handles individual key parsing
- Works correctly for simple keys (A-Z, Enter, C-x, etc.)
- **Not designed for long strings or complex content**

### HT-MCP Integration
Located in `src/ht_integration/session_manager.rs`:
```rust
let input_seqs: Vec<ht_core::command::InputSeq> = args
    .keys
    .iter()
    .map(|key| ht_core::api::stdio::parse_key(key.clone()))
    .collect();
```

The issue occurs because:
1. Each string in the `keys` array is processed individually
2. Long strings (like commit messages) aren't meant to be parsed this way
3. The `parse_key()` function expects single keys, not full text strings

## Solution Approaches

### 1. File-based Commits (Recommended)
Use `git commit -F filename` for complex messages:
```bash
echo "complex message with 🤖 emojis" > temp_commit_msg.txt
git commit -F temp_commit_msg.txt
rm temp_commit_msg.txt
```

### 2. Proper String Escaping
For simple cases, escape special characters:
```bash
git commit -m "Simple message without special chars"
```

### 3. Alternative: Fix HT-MCP Integration
Modify `send_keys` to detect long strings and handle them differently:
- Strings longer than X characters should be treated as raw text input
- Use `InputSeq::Standard()` directly instead of `parse_key()`

## Evidence

### Failed Complex Commit
```bash
git commit -m "Update with emoji 🤖 Generated with [Memex](https://memex.tech)"
# Results in: bash: syntax error near unexpected token '('
```

### Successful File-based Commit
```bash
git commit -F commit_message_file.txt
# [branch abc123] Update with emoji...
# Works perfectly with same content
```

## Recommendations

1. **Immediate Fix**: Use `git commit -F` for complex messages in automation
2. **Long-term**: Consider enhancing HT-MCP to detect and properly handle long text strings
3. **Detection**: Flag strings containing special characters `()[]{}` or emojis for file-based approach
4. **Temporary files**: Automatically create/cleanup temp files for complex commit messages

## Code Location References

- HT key parsing: `ht-core/src/api/stdio.rs:130-270`
- HT-MCP integration: `src/ht_integration/session_manager.rs:200-220`
- Issue reproduction: Successfully demonstrated with test cases