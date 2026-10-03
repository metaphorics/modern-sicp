// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//! Reader for the flat JSON case descriptors consumed by the Rust teaching drivers.

use std::{fs, io};

/// Read a descriptor, require its embedded identity to match the selected case,
/// and return the artifact-provided constructor name.
pub(super) fn read_constructor(path: &str, expected_case: &str) -> io::Result<String> {
    let source = fs::read_to_string(path)
        .map_err(|error| io::Error::new(error.kind(), format!("cannot read {path}: {error}")))?;
    let mut parser = JsonParser::new(&source);
    let mut case_id = None;
    let mut constructor = None;
    let mut observation = None;

    parser.skip_whitespace();
    parser.expect(b'{')?;
    let mut first = true;
    loop {
        parser.skip_whitespace();
        if first && parser.take(b'}') {
            break;
        }
        if !first {
            parser.expect(b',')?;
            parser.skip_whitespace();
        }
        first = false;

        let key = parser.string()?;
        parser.skip_whitespace();
        parser.expect(b':')?;
        parser.skip_whitespace();
        let value = parser.string()?;
        let slot = match key.as_str() {
            "case_id" => &mut case_id,
            "constructor" => &mut constructor,
            "observation" => &mut observation,
            _ => {
                return Err(invalid_data(format!(
                    "{path}: unknown descriptor field {key:?}"
                )));
            }
        };
        if slot.replace(value).is_some() {
            return Err(invalid_data(format!(
                "{path}: duplicate descriptor field {key:?}"
            )));
        }

        parser.skip_whitespace();
        if parser.take(b'}') {
            break;
        }
    }
    parser.skip_whitespace();
    if !parser.is_finished() {
        return Err(invalid_data(format!(
            "{path}: trailing data after descriptor"
        )));
    }

    let case_id = case_id.ok_or_else(|| invalid_data(format!("{path}: missing case_id")))?;
    let constructor =
        constructor.ok_or_else(|| invalid_data(format!("{path}: missing constructor")))?;
    let _observation =
        observation.ok_or_else(|| invalid_data(format!("{path}: missing observation")))?;
    if case_id != expected_case {
        return Err(invalid_data(format!(
            "{path}: descriptor case_id {case_id:?} does not match requested case {expected_case:?}",
        )));
    }
    Ok(constructor)
}

fn invalid_data(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

struct JsonParser<'a> {
    source: &'a str,
    offset: usize,
}

impl<'a> JsonParser<'a> {
    fn new(source: &'a str) -> Self {
        Self { source, offset: 0 }
    }

    fn current(&self) -> Option<u8> {
        self.source.as_bytes().get(self.offset).copied()
    }

    fn is_finished(&self) -> bool {
        self.offset == self.source.len()
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.current(), Some(b' ' | b'\n' | b'\r' | b'\t')) {
            self.offset += 1;
        }
    }

    fn take(&mut self, expected: u8) -> bool {
        if self.current() == Some(expected) {
            self.offset += 1;
            true
        } else {
            false
        }
    }

    fn expect(&mut self, expected: u8) -> io::Result<()> {
        if self.take(expected) {
            Ok(())
        } else {
            Err(invalid_data(format!(
                "invalid descriptor at byte {}",
                self.offset
            )))
        }
    }

    fn string(&mut self) -> io::Result<String> {
        self.expect(b'"')?;
        let mut value = String::new();
        loop {
            match self.current() {
                Some(b'"') => {
                    self.offset += 1;
                    return Ok(value);
                }
                Some(b'\\') => {
                    self.offset += 1;
                    value.push(self.escape()?);
                }
                Some(byte) if byte < 0x20 => {
                    return Err(invalid_data(format!(
                        "control character in descriptor string at byte {}",
                        self.offset,
                    )));
                }
                Some(_) => {
                    let character = self.source[self.offset..]
                        .chars()
                        .next()
                        .expect("current() returned Some, so the suffix is nonempty");
                    value.push(character);
                    self.offset += character.len_utf8();
                }
                None => return Err(invalid_data("unterminated descriptor string")),
            }
        }
    }

    fn escape(&mut self) -> io::Result<char> {
        let byte = self
            .current()
            .ok_or_else(|| invalid_data("unterminated escape in descriptor string"))?;
        self.offset += 1;
        let character = match byte {
            b'"' => '"',
            b'\\' => '\\',
            b'/' => '/',
            b'b' => '\u{8}',
            b'f' => '\u{c}',
            b'n' => '\n',
            b'r' => '\r',
            b't' => '\t',
            b'u' => return self.unicode_escape(),
            _ => {
                return Err(invalid_data(format!(
                    "invalid escape in descriptor string at byte {}",
                    self.offset - 1,
                )));
            }
        };
        Ok(character)
    }

    fn unicode_escape(&mut self) -> io::Result<char> {
        let high = self.hex_quad()?;
        let scalar = if (0xD800..0xDC00).contains(&high) {
            if !self.take(b'\\') || !self.take(b'u') {
                return Err(invalid_data("unpaired surrogate in descriptor string"));
            }
            let low = self.hex_quad()?;
            if !(0xDC00..0xE000).contains(&low) {
                return Err(invalid_data("unpaired surrogate in descriptor string"));
            }
            0x10000 + ((high - 0xD800) << 10) + (low - 0xDC00)
        } else {
            high
        };
        char::from_u32(scalar)
            .ok_or_else(|| invalid_data("invalid unicode escape in descriptor string"))
    }

    fn hex_quad(&mut self) -> io::Result<u32> {
        let mut value = 0;
        for _ in 0..4 {
            let digit = match self.current() {
                Some(byte @ b'0'..=b'9') => byte - b'0',
                Some(byte @ b'a'..=b'f') => byte - b'a' + 10,
                Some(byte @ b'A'..=b'F') => byte - b'A' + 10,
                _ => {
                    return Err(invalid_data(format!(
                        "invalid unicode escape in descriptor string at byte {}",
                        self.offset,
                    )));
                }
            };
            value = value * 16 + u32::from(digit);
            self.offset += 1;
        }
        Ok(value)
    }
}
