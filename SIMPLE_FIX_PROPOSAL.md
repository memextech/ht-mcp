# Simple Fix for Complex Key Handling

## Current Problem
```rust
// This breaks for complex strings:
let input_seqs: Vec<ht_core::command::InputSeq> = args
    .keys
    .iter()
    .map(|key| ht_core::api::stdio::parse_key(key.clone()))  // ❌ Always parses as special key
    .collect();
```

## Simple Solution

Replace the problematic line with intelligent detection:

```rust
fn smart_parse_key(key: &str) -> ht_core::command::InputSeq {
    // Detect if this is a "real key" vs "text content"
    if is_special_key(key) {
        ht_core::api::stdio::parse_key(key.to_string())
    } else {
        ht_core::api::stdio::standard_key(key)  // Treat as literal text
    }
}

fn is_special_key(key: &str) -> bool {
    // Real keys are typically short and don't contain spaces
    if key.len() > 20 { return false; }
    if key.contains(' ') && !matches!(key, "C-Space" | "Space") { return false; }
    
    // Check if it matches known key patterns
    matches!(key,
        // Control keys
        "Enter" | "Tab" | "Space" | "Escape" |
        // Arrow keys  
        "Left" | "Right" | "Up" | "Down" |
        // Function keys
        "F1" | "F2" | "F3" | "F4" | "F5" | "F6" | "F7" | "F8" | "F9" | "F10" | "F11" | "F12" |
        // Home/End/Page
        "Home" | "End" | "PageUp" | "PageDown" |
        // Single characters
        _ if key.len() == 1 |
        // Control sequences
        _ if key.starts_with("C-") || key.starts_with("^") |
        // Alt sequences  
        _ if key.starts_with("A-") |
        // Shift sequences
        _ if key.starts_with("S-")
    )
}
```

## Even Simpler Heuristic

```rust
fn is_special_key(key: &str) -> bool {
    // If it contains spaces and isn't a known key, it's probably text
    key.len() <= 20 && (!key.contains(' ') || matches!(key, "C-Space" | "Space"))
}
```

## Implementation Change

In `session_manager.rs`:

```rust
let input_seqs: Vec<ht_core::command::InputSeq> = args
    .keys
    .iter()
    .map(|key| smart_parse_key(key))  // ✅ Intelligent detection
    .collect();
```

## Why This Works

1. **Real keys** like "Enter", "C-x", "F1" → parsed normally
2. **Text content** like "git commit -m ..." → treated as literal input  
3. **No complex parsing** needed - just simple heuristics
4. **Backward compatible** - existing usage unchanged

## Test Cases

```rust
assert_eq!(smart_parse_key("Enter"), parse_key("Enter"));           // Special key
assert_eq!(smart_parse_key("C-x"), parse_key("C-x"));               // Control key  
assert_eq!(smart_parse_key("hello world"), standard_key("hello world")); // Text
assert_eq!(smart_parse_key("git commit -m \"msg\""), standard_key("git commit -m \"msg\"")); // Command
```

## Complexity: **Low**
- ~20 lines of code
- Simple string matching
- No new dependencies
- Uses existing HT abstractions