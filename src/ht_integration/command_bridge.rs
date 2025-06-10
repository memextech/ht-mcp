// Command bridge between MCP and HT library
use ht_core::InputSeq;
use crate::error::Result;

pub struct CommandBridge;

impl CommandBridge {
    pub fn new() -> Self {
        Self
    }
    
    /// Translate key strings to HT InputSeq using HT's native key parsing
    pub fn translate_keys(&self, keys: &[String]) -> Result<Vec<InputSeq>> {
        let mut input_seqs = Vec::new();
        
        for key in keys {
            // Use HT's own key parsing logic
            input_seqs.push(parse_key(key.clone()));
        }
        
        Ok(input_seqs)
    }
}

/// Parse a key string into an InputSeq (copied from HT's stdio.rs)
fn parse_key(key: String) -> InputSeq {
    let seq = match key.as_str() {
        "C-@" | "C-Space" | "^@" => "\x00",
        "C-[" | "Escape" | "^[" => "\x1b",
        "C-\\" | "^\\" => "\x1c",
        "C-]" | "^]" => "\x1d",
        "C-^" | "C-/" => "\x1e",
        "C--" | "C-_" => "\x1f",
        "Tab" => "\x09",   // same as C-i
        "Enter" => "\x0d", // same as C-m
        "Space" => " ",
        "Left" => return cursor_key("\x1b[D", "\x1bOD"),
        "Right" => return cursor_key("\x1b[C", "\x1bOC"),
        "Up" => return cursor_key("\x1b[A", "\x1bOA"),
        "Down" => return cursor_key("\x1b[B", "\x1bOB"),
        "C-Left" => "\x1b[1;5D",
        "C-Right" => "\x1b[1;5C",
        "S-Left" => "\x1b[1;2D",
        "S-Right" => "\x1b[1;2C",
        "C-Up" => "\x1b[1;5A",
        "C-Down" => "\x1b[1;5B",
        "S-Up" => "\x1b[1;2A",
        "S-Down" => "\x1b[1;2B",
        "A-Left" => "\x1b[1;3D",
        "A-Right" => "\x1b[1;3C",
        "A-Up" => "\x1b[1;3A",
        "A-Down" => "\x1b[1;3B",
        "F1" => "\x1bOP",
        "F2" => "\x1bOQ",
        "F3" => "\x1bOR",
        "F4" => "\x1bOS",
        "F5" => "\x1b[15~",
        "F6" => "\x1b[17~",
        "F7" => "\x1b[18~",
        "F8" => "\x1b[19~",
        "F9" => "\x1b[20~",
        "F10" => "\x1b[21~",
        "F11" => "\x1b[23~",
        "F12" => "\x1b[24~",
        "Home" => return cursor_key("\x1b[H", "\x1bOH"),
        "End" => return cursor_key("\x1b[F", "\x1bOF"),
        "PageUp" => "\x1b[5~",
        "PageDown" => "\x1b[6~",
        "Insert" => "\x1b[2~",
        "Delete" => "\x1b[3~",
        "Backspace" => "\x7f",
        k => {
            let chars: Vec<char> = k.chars().collect();

            if chars.len() == 1 {
                return standard_key(k);
            }

            if chars.len() == 4 && chars[0] == 'C' && chars[1] == '-' {
                let c = chars[2];

                if c >= 'a' && c <= 'z' {
                    let ctrl_char = (c as u8 - b'a' + 1) as char;
                    return standard_key(ctrl_char);
                } else if c >= 'A' && c <= 'Z' {
                    let ctrl_char = (c as u8 - b'A' + 1) as char;
                    return standard_key(ctrl_char);
                }
            }

            return standard_key(k);
        }
    };

    standard_key(seq)
}

fn standard_key<S: ToString>(seq: S) -> InputSeq {
    InputSeq::Standard(seq.to_string())
}

fn cursor_key<S: ToString>(seq1: S, seq2: S) -> InputSeq {
    InputSeq::Cursor(seq1.to_string(), seq2.to_string())
}