// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The S-expression reader of `spec/scheme-subset/grammar.md`: the
//! edition's `read`, turning program text into `Value` forms. The
//! accepted surface is the subset the grammar fixes: `#t`/`#f`, decimal
//! integers and floats with an exponent, double-quoted strings whose
//! only escapes are `\"` and `\\`, symbols over letters, digits, and
//! `-?!*+/><=_.`, the quote sugar `'d` for `(quote d)`, and dotted
//! pairs `(a . b)`. Semicolon comments run to the end of the line.

use std::rc::Rc;

use crate::error::SchemeError;
use crate::pair::cons_cell;
use crate::value::Value;

/// Reads one datum and requires the rest of the input to be atmosphere,
/// the book's `read` over one complete expression.
///
/// # Errors
/// [`SchemeError::Parse`] on malformed surface syntax, an empty input,
/// or trailing input after the first datum.
pub fn read(src: &str) -> Result<Value, SchemeError> {
    let mut parser = Parser::new(src);
    parser.skip_atmosphere();
    let datum = parser.read_datum()?;
    parser.skip_atmosphere();
    if parser.peek().is_some() {
        return Err(parser.parse_error("trailing input after the first datum"));
    }
    Ok(datum)
}

/// Reads every datum until end of input, the book's `read` over a whole
/// program's forms; an input with no datums answers an empty program.
///
/// # Errors
/// [`SchemeError::Parse`] on malformed surface syntax.
pub fn read_program(src: &str) -> Result<Vec<Value>, SchemeError> {
    let mut parser = Parser::new(src);
    let mut forms = Vec::new();
    loop {
        parser.skip_atmosphere();
        if parser.peek().is_none() {
            return Ok(forms);
        }
        forms.push(parser.read_datum()?);
    }
}

struct Parser<'a> {
    src: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(src: &'a str) -> Self {
        Self {
            src: src.as_bytes(),
            pos: 0,
        }
    }

    fn parse_error(&self, detail: &str) -> SchemeError {
        SchemeError::Parse(format!("{} at byte {}", detail, self.pos))
    }

    fn peek(&self) -> Option<u8> {
        self.src.get(self.pos).copied()
    }

    fn bump(&mut self) -> Option<u8> {
        let byte = self.peek();
        if byte.is_some() {
            self.pos += 1;
        }
        byte
    }

    fn skip_atmosphere(&mut self) {
        loop {
            match self.peek() {
                Some(byte) if byte.is_ascii_whitespace() => self.pos += 1,
                Some(b';') => self.skip_line(),
                _ => return,
            }
        }
    }

    fn skip_line(&mut self) {
        while let Some(byte) = self.peek() {
            self.pos += 1;
            if byte == b'\n' {
                return;
            }
        }
    }

    fn read_datum(&mut self) -> Result<Value, SchemeError> {
        match self.peek() {
            None => Err(self.parse_error("unexpected end of input")),
            Some(b'(') => self.read_list(),
            Some(b'\'') => self.read_quoted(),
            Some(b'"') => self.read_string(),
            Some(b'#') => self.read_hash(),
            Some(b')') => Err(self.parse_error("unbalanced )")),
            Some(_) => self.read_atom(),
        }
    }

    fn read_quoted(&mut self) -> Result<Value, SchemeError> {
        self.pos += 1;
        let inner = self.read_datum()?;
        Ok(Value::list(vec![Value::sym("quote"), inner]))
    }

    fn read_list(&mut self) -> Result<Value, SchemeError> {
        self.pos += 1;
        let mut items = Vec::new();
        loop {
            self.skip_atmosphere();
            match self.peek() {
                None => return Err(self.parse_error("unbalanced ( at end of input")),
                Some(b')') => {
                    self.pos += 1;
                    return Ok(list_from(items, Value::Nil));
                }
                Some(b'.') if self.token_is_lone_dot() => {
                    return self.read_dotted_tail(items);
                }
                Some(_) => items.push(self.read_datum()?),
            }
        }
    }

    fn read_dotted_tail(&mut self, items: Vec<Value>) -> Result<Value, SchemeError> {
        self.pos += 1;
        self.skip_atmosphere();
        let tail = self.read_datum()?;
        self.skip_atmosphere();
        match self.bump() {
            Some(b')') => Ok(list_from(items, tail)),
            _ => Err(self.parse_error("the dotted tail must close the list")),
        }
    }

    /// Whether the `.` at the cursor stands alone, so it is the dotted
    /// marker and not the first byte of a symbol such as `...`.
    fn token_is_lone_dot(&self) -> bool {
        self.src
            .get(self.pos + 1)
            .is_some_and(|next| is_delimiter(*next))
    }

    fn read_string(&mut self) -> Result<Value, SchemeError> {
        self.pos += 1;
        let mut text = Vec::new();
        loop {
            match self.bump() {
                None => return Err(self.parse_error("unterminated string")),
                Some(b'"') => {
                    let text = String::from_utf8_lossy(&text).into_owned();
                    return Ok(Value::Str(Rc::from(text.as_str())));
                }
                Some(b'\\') => match self.bump() {
                    Some(b'"') => text.push(b'"'),
                    Some(b'\\') => text.push(b'\\'),
                    _ => return Err(self.parse_error("illegal backslash escape in string")),
                },
                Some(byte) => text.push(byte),
            }
        }
    }

    fn read_hash(&mut self) -> Result<Value, SchemeError> {
        let token = self.read_token();
        match token.as_str() {
            "#t" => Ok(Value::boolean(true)),
            "#f" => Ok(Value::boolean(false)),
            other => Err(self.parse_error(&format!("bad literal {other:?}"))),
        }
    }

    fn read_atom(&mut self) -> Result<Value, SchemeError> {
        let token = self.read_token();
        classify_atom(&token).ok_or_else(|| self.parse_error(&format!("bad atom {token:?}")))
    }

    fn read_token(&mut self) -> String {
        let start = self.pos;
        while self.peek().is_some_and(|byte| !is_delimiter(byte)) {
            self.pos += 1;
        }
        String::from_utf8_lossy(&self.src[start..self.pos]).into_owned()
    }
}

/// Whitespace, parentheses, quote, string quote, comment, and the EOF
/// boundary: the bytes that end a token.
fn is_delimiter(byte: u8) -> bool {
    matches!(
        byte,
        b' ' | b'\t' | b'\r' | b'\n' | b'(' | b')' | b'"' | b';' | b'\''
    )
}

/// Builds `(items..., tail)` right to left.
fn list_from(mut items: Vec<Value>, tail: Value) -> Value {
    let mut out = tail;
    while let Some(item) = items.pop() {
        out = Value::Pair(cons_cell(item, out));
    }
    out
}

/// Classifies one token as a number or a symbol; `None` is a bad atom.
fn classify_atom(token: &str) -> Option<Value> {
    if let Some(n) = as_integer(token) {
        return Some(Value::Int(n));
    }
    if let Some(x) = as_float(token) {
        return Some(Value::Real(x));
    }
    if token.is_empty() || token.starts_with('#') {
        return None;
    }
    Some(Value::Sym(Rc::from(token)))
}

/// `[+-]?digits` over the exact `i128` width.
fn as_integer(token: &str) -> Option<i128> {
    let digits = token.strip_prefix(['+', '-']).unwrap_or(token);
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    token.parse().ok()
}

/// The grammar's float shape: `[+-]?digits.digits[e[+-]?digits]`, so
/// `1.` and `.5` are symbols and `1.5e3` is a float.
fn as_float(token: &str) -> Option<f64> {
    let body = token.strip_prefix(['+', '-']).unwrap_or(token);
    let (mantissa, exponent) = match body.split_once(['e', 'E']) {
        Some((mantissa, exponent)) => (mantissa, Some(exponent)),
        None => (body, None),
    };
    let (whole, fraction) = mantissa.split_once('.')?;
    if whole.is_empty() || fraction.is_empty() {
        return None;
    }
    let all_digits = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit());
    if !all_digits(whole) || !all_digits(fraction) {
        return None;
    }
    if let Some(exponent) = exponent {
        let digits = exponent.strip_prefix(['+', '-']).unwrap_or(exponent);
        if !all_digits(digits) {
            return None;
        }
    }
    token.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::{read, read_program};
    use crate::error::SchemeError;
    use crate::value::Value;

    #[test]
    fn reads_numbers_booleans_symbols_and_strings() {
        assert_eq!(read("42"), Ok(Value::int(42)));
        assert_eq!(read("-7"), Ok(Value::int(-7)));
        assert_eq!(read("+7"), Ok(Value::int(7)));
        assert_eq!(read("2.5"), Ok(Value::real(2.5)));
        assert_eq!(read("1.5e3"), Ok(Value::real(1500.0)));
        assert_eq!(read("#t"), Ok(Value::boolean(true)));
        assert_eq!(read("#f"), Ok(Value::boolean(false)));
        assert_eq!(read("set!"), Ok(Value::sym("set!")));
        assert_eq!(read("..."), Ok(Value::sym("...")));
        assert_eq!(read("1.5e"), Ok(Value::sym("1.5e")));
        assert_eq!(read("\"hi\""), Ok(Value::string("hi")));
    }

    #[test]
    fn reads_lists_quotes_and_dotted_pairs() {
        assert_eq!(
            read("(a b 42)"),
            Ok(Value::list(vec![
                Value::sym("a"),
                Value::sym("b"),
                Value::int(42)
            ]))
        );
        assert_eq!(
            read("'x"),
            Ok(Value::list(vec![Value::sym("quote"), Value::sym("x")]))
        );
        assert_eq!(read("()"), Ok(Value::Nil));
        let pair = |a: Value, b: Value| Value::Pair(crate::pair::cons_cell(a, b));
        assert_eq!(read("(1 . 2)"), Ok(pair(Value::int(1), Value::int(2))));
        assert_eq!(
            read("(a b . c)"),
            Ok(pair(
                Value::sym("a"),
                pair(Value::sym("b"), Value::sym("c"))
            ))
        );
    }

    #[test]
    fn skips_comments_and_whitespace_between_forms() {
        let program = ";; header\n(define x 1) ; trailing\n  x\n";
        let forms = read_program(program).expect("parses");
        assert_eq!(forms.len(), 2);
        assert_eq!(forms[1], Value::sym("x"));
    }

    #[test]
    fn reads_string_escapes_and_rejects_others() {
        assert_eq!(read("\"a\\\"b\\\\c\""), Ok(Value::string("a\"b\\c")));
        let bad = read("\"a\\nb\"").expect_err("illegal escape");
        assert!(matches!(bad, SchemeError::Parse(_)));
    }

    #[test]
    fn rejects_malformed_surface() {
        assert!(matches!(read(""), Err(SchemeError::Parse(_))));
        assert!(matches!(read("(a"), Err(SchemeError::Parse(_))));
        assert!(matches!(read("a)"), Err(SchemeError::Parse(_))));
        assert!(matches!(read("#u"), Err(SchemeError::Parse(_))));
        assert!(matches!(read("(a . b c)"), Err(SchemeError::Parse(_))));
        assert!(matches!(read("(1) (2)"), Err(SchemeError::Parse(_))));
    }
}
