// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0

use crate::{Error, Result};
use serde_json::{Map, Value};

pub const MAX_BYTES: usize = 1024 * 1024;
pub const MAX_DEPTH: usize = 64;
const MAX_INT: i64 = 9_007_199_254_740_991;

/// Strict wire decoder, before any information-losing Value conversion.
/// Root depth is zero; object keys and values both occupy the next depth.
pub fn decode(raw: &[u8]) -> Result<Value> {
    if raw.len() > MAX_BYTES {
        return Err(Error::Size);
    }
    let text = std::str::from_utf8(raw).map_err(|_| Error::Utf8)?;
    let mut parser = Parser { text, pos: 0 };
    let value = parser.value(0)?;
    parser.space();
    if parser.pos != raw.len() {
        return Err(Error::Json);
    }
    Ok(value)
}

struct Parser<'a> {
    text: &'a str,
    pos: usize,
}

impl Parser<'_> {
    fn byte(&self) -> Option<u8> {
        self.text.as_bytes().get(self.pos).copied()
    }

    fn space(&mut self) {
        while matches!(self.byte(), Some(b' ' | b'\r' | b'\n' | b'\t')) {
            self.pos += 1;
        }
    }

    fn consume(&mut self, byte: u8) -> Result<()> {
        self.space();
        if self.byte() != Some(byte) {
            return Err(Error::Json);
        }
        self.pos += 1;
        Ok(())
    }

    fn string(&mut self) -> Result<String> {
        self.space();
        let start = self.pos;
        self.consume(b'"')?;
        while let Some(byte) = self.byte() {
            self.pos += 1;
            match byte {
                b'"' => {
                    // serde_json checks escapes, control characters and paired
                    // surrogates. Both slice boundaries are ASCII delimiters.
                    return serde_json::from_str(&self.text[start..self.pos])
                        .map_err(|_| Error::Json);
                }
                b'\\' => {
                    if self.byte().is_none() {
                        return Err(Error::Json);
                    }
                    self.pos += 1;
                }
                _ => {}
            }
        }
        Err(Error::Json)
    }

    fn value(&mut self, depth: usize) -> Result<Value> {
        if depth > MAX_DEPTH {
            return Err(Error::Depth);
        }
        self.space();
        match self.byte() {
            Some(b'{') => {
                self.pos += 1;
                self.space();
                let mut map = Map::new();
                if self.byte() != Some(b'}') {
                    loop {
                        if depth == MAX_DEPTH {
                            return Err(Error::Depth);
                        }
                        let key = self.string()?;
                        // Compare decoded names: "a" and "\u0061" duplicate.
                        if map.contains_key(&key) {
                            return Err(Error::DuplicateKey);
                        }
                        self.consume(b':')?;
                        map.insert(key, self.value(depth + 1)?);
                        self.space();
                        if self.byte() != Some(b',') {
                            break;
                        }
                        self.pos += 1;
                    }
                }
                self.consume(b'}')?;
                Ok(Value::Object(map))
            }
            Some(b'[') => {
                self.pos += 1;
                self.space();
                let mut list = Vec::new();
                if self.byte() != Some(b']') {
                    loop {
                        list.push(self.value(depth + 1)?);
                        self.space();
                        if self.byte() != Some(b',') {
                            break;
                        }
                        self.pos += 1;
                    }
                }
                self.consume(b']')?;
                Ok(Value::Array(list))
            }
            Some(b'"') => Ok(Value::String(self.string()?)),
            Some(b't') => self.literal("true", Value::Bool(true)),
            Some(b'f') => self.literal("false", Value::Bool(false)),
            Some(b'n') => self.literal("null", Value::Null),
            Some(b'-' | b'0'..=b'9') => self.number(),
            _ => Err(Error::Json),
        }
    }

    fn literal(&mut self, token: &str, value: Value) -> Result<Value> {
        if !self.text[self.pos..].starts_with(token) {
            return Err(Error::Json);
        }
        self.pos += token.len();
        Ok(value)
    }

    fn number(&mut self) -> Result<Value> {
        let start = self.pos;
        if self.byte() == Some(b'-') {
            self.pos += 1;
        }
        match self.byte() {
            Some(b'0') => self.pos += 1,
            Some(b'1'..=b'9') => {
                while matches!(self.byte(), Some(b'0'..=b'9')) {
                    self.pos += 1;
                }
            }
            _ => return Err(Error::Json),
        }
        if matches!(self.byte(), Some(b'.' | b'e' | b'E')) {
            return Err(Error::NumberToken);
        }
        if matches!(self.byte(), Some(b'0'..=b'9')) {
            return Err(Error::Json);
        }
        let number: i64 = self.text[start..self.pos]
            .parse()
            .map_err(|_| Error::IntegerRange)?;
        if !(-MAX_INT..=MAX_INT).contains(&number) {
            return Err(Error::IntegerRange);
        }
        // Integer token -0 is permitted and canonicalizes to 0.
        Ok(Value::from(number))
    }
}

/// JCS for the IR's restricted integer-only JSON domain (not general float JCS).
/// Private: callers cannot bypass strict decoding with arbitrary serde Values.
pub(crate) fn canonical(value: &Value) -> String {
    match value {
        Value::Object(map) => {
            let mut keys: Vec<_> = map.keys().collect();
            keys.sort_by(|a, b| a.encode_utf16().cmp(b.encode_utf16()));
            let fields: Vec<_> = keys
                .into_iter()
                .map(|key| {
                    format!(
                        "{}:{}",
                        serde_json::to_string(key).expect("string serialization"),
                        canonical(&map[key])
                    )
                })
                .collect();
            format!("{{{}}}", fields.join(","))
        }
        Value::Array(list) => format!(
            "[{}]",
            list.iter().map(canonical).collect::<Vec<_>>().join(",")
        ),
        _ => serde_json::to_string(value).expect("decoded JSON serialization"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_string_roundtrips() {
        // Includes all control characters, BMP/non-BMP and combining sequences.
        // Fixed generation, not an ambient source of random process identities.
        let alphabet = [
            '\0', '\u{000f}', '\n', '\r', '\t', '"', '\\', '/', 'é', '\u{0301}', '\u{e000}', '😀',
        ];
        for a in alphabet {
            for b in alphabet {
                let text = format!("{a}{b}");
                let value = serde_json::json!({"text": text, "array": [false, null, 123]});
                let raw = serde_json::to_vec(&value).unwrap();
                assert_eq!(decode(&raw).unwrap(), value);
                assert_eq!(decode(canonical(&value).as_bytes()).unwrap(), value);
            }
        }
    }

    #[test]
    fn arbitrary_bytes_do_not_panic_or_accept_invalid_json() {
        for a in 0..=255u8 {
            for b in 0..=255u8 {
                let raw = [b'"', a, b, b'"'];
                if let Ok(value) = decode(&raw) {
                    assert_eq!(value, serde_json::from_slice::<Value>(&raw).unwrap());
                }
            }
        }
    }

    #[test]
    fn strict_integer_tokens_and_negative_zero() {
        assert_eq!(canonical(&decode(b"-0").unwrap()), "0");
        for raw in [b"0.0".as_slice(), b"-0e0", b"1E0"] {
            assert_eq!(decode(raw), Err(Error::NumberToken));
        }
        assert_eq!(decode(b"9007199254740992"), Err(Error::IntegerRange));
        assert_eq!(decode(b"-9007199254740992"), Err(Error::IntegerRange));
    }
}
