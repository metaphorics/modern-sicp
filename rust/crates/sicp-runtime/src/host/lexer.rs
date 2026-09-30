// SPDX-License-Identifier: GPL-3.0-only

//! The token layer of the Rust host subset (grammar §2 and §4): Rust's
//! lexical rules with the contract's explicit literal limits. Line and
//! nested block comments are atmosphere; raw, byte, and C-string
//! literals lex as one excluded-literal token so the parser can report
//! them as [`DiagKind::Unsupported`] rather than as syntax noise.

use crate::host::diag::{Diag, Span};

/// The integer literal suffixes the subset admits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntSuffix {
    /// The `i64` suffix.
    I64,
    /// The `usize` suffix.
    Usize,
}

/// One token's payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokKind {
    /// An identifier: `[A-Za-z_][A-Za-z0-9_]*`.
    Ident(String),
    /// An integer literal with its digits (separators kept) and the
    /// optional suffix.
    Int {
        /// The literal's digit text, `'_` separators included.
        text: String,
        /// The suffix, when written.
        suffix: Option<IntSuffix>,
    },
    /// A string literal's decoded value.
    Str(String),
    /// A floating-point literal, host-valid and excluded by §3.
    Float,
    /// A raw, byte, or C-string literal, host-valid and excluded by §3.
    ExcludedLit,
    /// `(`
    LParen,
    /// `)`
    RParen,
    /// `{`
    LBrace,
    /// `}`
    RBrace,
    /// `[`
    LBracket,
    /// `]`
    RBracket,
    /// `,`
    Comma,
    /// `:`
    Colon,
    /// `;`
    Semi,
    /// `.`
    Dot,
    /// `..`
    DotDot,
    /// `->`
    Arrow,
    /// `=>`
    FatArrow,
    /// `&`
    Amp,
    /// `|` (closure parameters or `||`)
    Pipe,
    /// `||`
    PipePipe,
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `%`
    Percent,
    /// `==`
    EqEq,
    /// `!=`
    NotEq,
    /// `<`
    Lt,
    /// `<=`
    Le,
    /// `>`
    Gt,
    /// `>=`
    Ge,
    /// `&&`
    AmpAmp,
    /// `!`
    Bang,
    /// `=`
    Eq,
    /// `+=`
    PlusEq,
    /// `-=`
    MinusEq,
    /// `?`
    Question,
    /// `#` (attributes only)
    Pound,
    /// `::`
    ColonColon,
    /// A named lifetime such as `'static`, admitted only in the
    /// listed boxed closure object types.
    Lifetime(String),
    /// A character literal, host-valid and excluded by §3.
    CharLit,
}

/// One token: its payload and location.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tok {
    /// The payload.
    pub kind: TokKind,
    /// The token's location.
    pub span: Span,
}

/// Lexes a guest source text into tokens.
///
/// # Errors
/// The first [`Diag`] the lexer raises: [`DiagKind::Syntax`] for an
/// unterminated string or an unknown character, never a typing class.
pub fn lex(source: &str) -> Result<Vec<Tok>, Diag> {
    Lexer::new(source).run()
}

struct Lexer {
    chars: Vec<char>,
    pos: usize,
    line: u32,
    col: u32,
}

impl Lexer {
    fn new(source: &str) -> Self {
        Self {
            chars: source.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    fn run(mut self) -> Result<Vec<Tok>, Diag> {
        let mut out = Vec::new();
        loop {
            self.skip_atmosphere()?;
            let Some(kind) = self.next_token()? else {
                return Ok(out);
            };
            let start = (self.line, self.col);
            // The token length is the consumed span; recompute from the
            // recorded start against the current position.
            let len = self.span_len_since(start);
            out.push(Tok {
                kind,
                span: Span::new(start.0, start.1, len),
            });
        }
    }

    fn span_len_since(&self, start: (u32, u32)) -> u32 {
        if start.0 == self.line {
            self.col.saturating_sub(start.1)
        } else {
            1
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn peek_at(&self, offset: usize) -> Option<char> {
        self.chars.get(self.pos + offset).copied()
    }

    fn bump(&mut self) -> Option<char> {
        let ch = self.chars.get(self.pos).copied()?;
        self.pos += 1;
        if ch == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(ch)
    }

    fn here(&self) -> Span {
        Span::new(self.line, self.col, 1)
    }

    fn skip_atmosphere(&mut self) -> Result<(), Diag> {
        loop {
            match self.peek() {
                Some(ch) if ch.is_whitespace() => {
                    self.bump();
                }
                Some('/') if self.peek_at(1) == Some('/') => {
                    while let Some(ch) = self.peek() {
                        if ch == '\n' {
                            break;
                        }
                        self.bump();
                    }
                }
                Some('/') if self.peek_at(1) == Some('*') => {
                    let start = self.here();
                    self.bump();
                    self.bump();
                    let mut depth = 1_u32;
                    while depth > 0 {
                        match (self.peek(), self.peek_at(1)) {
                            (Some('*'), Some('/')) => {
                                self.bump();
                                self.bump();
                                depth -= 1;
                            }
                            (Some('/'), Some('*')) => {
                                self.bump();
                                self.bump();
                                depth += 1;
                            }
                            (Some(_), _) => {
                                self.bump();
                            }
                            (None, _) => {
                                return Err(Diag::syntax(start, "unterminated block comment"));
                            }
                        }
                    }
                }
                _ => return Ok(()),
            }
        }
    }

    fn next_token(&mut self) -> Result<Option<TokKind>, Diag> {
        let Some(ch) = self.peek() else {
            return Ok(None);
        };
        if ch.is_ascii_digit() {
            return self.lex_number().map(Some);
        }
        if ch.is_ascii_alphabetic() || ch == '_' {
            return self.lex_word().map(Some);
        }
        if ch == '"' {
            return self.lex_string().map(Some);
        }
        if ch == '\'' {
            return self.lex_apostrophe().map(Some);
        }
        self.bump();
        self.lex_punctuation(ch).map(Some)
    }

    fn lex_punctuation(&mut self, ch: char) -> Result<TokKind, Diag> {
        let kind = match ch {
            '(' => TokKind::LParen,
            ')' => TokKind::RParen,
            '{' => TokKind::LBrace,
            '}' => TokKind::RBrace,
            '[' => TokKind::LBracket,
            ']' => TokKind::RBracket,
            ',' => TokKind::Comma,
            ';' => TokKind::Semi,
            '#' => TokKind::Pound,
            '?' => TokKind::Question,
            ':' => self.paired_or_single(':', TokKind::ColonColon, TokKind::Colon),
            '.' => self.paired_or_single('.', TokKind::DotDot, TokKind::Dot),
            '&' => self.paired_or_single('&', TokKind::AmpAmp, TokKind::Amp),
            '|' => self.paired_or_single('|', TokKind::PipePipe, TokKind::Pipe),
            '+' => self.paired_or_single('=', TokKind::PlusEq, TokKind::Plus),
            '-' => match self.peek() {
                Some('>') => {
                    self.bump();
                    TokKind::Arrow
                }
                Some('=') => {
                    self.bump();
                    TokKind::MinusEq
                }
                _ => TokKind::Minus,
            },
            '*' => TokKind::Star,
            '/' => TokKind::Slash,
            '%' => TokKind::Percent,
            '=' => {
                if self.peek() == Some('=') {
                    self.bump();
                    TokKind::EqEq
                } else if self.peek() == Some('>') {
                    self.bump();
                    TokKind::FatArrow
                } else {
                    TokKind::Eq
                }
            }
            '!' => self.paired_or_single('=', TokKind::NotEq, TokKind::Bang),
            '<' => self.paired_or_single('=', TokKind::Le, TokKind::Lt),
            '>' => self.paired_or_single('=', TokKind::Ge, TokKind::Gt),
            _ => {
                return Err(Diag::syntax(
                    self.here(),
                    format!("unexpected character {ch:?}"),
                ));
            }
        };
        Ok(kind)
    }

    fn paired_or_single(
        &mut self,
        continuation: char,
        paired: TokKind,
        single: TokKind,
    ) -> TokKind {
        if self.peek() == Some(continuation) {
            self.bump();
            paired
        } else {
            single
        }
    }

    fn lex_number(&mut self) -> Result<TokKind, Diag> {
        let start = self.here();
        let mut text = String::new();
        // grammar §2: `DIGIT ("_"? DIGIT)*`; separators sit between
        // digits, so a doubled or trailing separator is invalid syntax.
        let mut needs_digit = true;
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                text.push(ch);
                self.bump();
                needs_digit = false;
            } else if ch == '_' && !needs_digit {
                text.push(ch);
                self.bump();
                needs_digit = true;
            } else {
                break;
            }
        }
        if needs_digit && text.contains('_') {
            // Grammar §2 permits one `_` directly before the `i64`/`usize`
            // suffix (`3_i64`); the separator is not part of the digits.
            // Any other trailing separator stays invalid syntax.
            if !self.at_int_suffix() {
                return Err(Diag::syntax(
                    start,
                    "digit separators must sit between digits",
                ));
            }
            text.pop();
        }
        // A digit followed by `.` and a digit is a floating literal;
        // `..` after a number stays a range. Both separators and
        // exponents lex the whole float so the parser reports one
        // excluded literal.
        let dot_before_range = self.peek() == Some('.') && self.peek_at(1) != Some('.');
        let exponent_form = self.peek().is_some_and(|c| c == 'e' || c == 'E')
            && self.peek_at(1).is_some_and(|c| {
                c.is_ascii_digit()
                    || ((c == '-' || c == '+')
                        && self.peek_at(2).is_some_and(|d| d.is_ascii_digit()))
            });
        if dot_before_range || exponent_form {
            self.consume_float_tail();
            return Ok(TokKind::Float);
        }
        if self.peek() == Some('.') && self.peek_at(1).is_some_and(|c| c.is_ascii_digit()) {
            self.bump();
            while let Some(ch) = self.peek() {
                if ch.is_ascii_digit() || ch == '_' {
                    self.bump();
                } else {
                    break;
                }
            }
            if self.peek() == Some('e') || self.peek() == Some('E') {
                self.bump();
                if self.peek().is_some_and(|c| c == '-' || c == '+') {
                    self.bump();
                }
                while let Some(ch) = self.peek() {
                    if ch.is_ascii_digit() {
                        self.bump();
                    } else {
                        break;
                    }
                }
            }
            return Ok(TokKind::Float);
        }
        let suffix = if self.at_int_suffix() {
            Some(self.consume_int_suffix())
        } else {
            None
        };
        Ok(TokKind::Int { text, suffix })
    }

    /// True when the cursor spells a complete `i64` or `usize` suffix:
    /// the letters followed by a byte that cannot continue an identifier.
    fn at_int_suffix(&self) -> bool {
        let id_continue =
            |c: Option<char>| c.is_some_and(|c| c.is_ascii_alphanumeric() || c == '_');
        let i64_here = self.peek() == Some('i')
            && self.peek_at(1) == Some('6')
            && self.peek_at(2) == Some('4')
            && !id_continue(self.peek_at(3));
        let usize_here = self.peek() == Some('u')
            && self.peek_at(1) == Some('s')
            && self.peek_at(2) == Some('i')
            && self.peek_at(3) == Some('z')
            && self.peek_at(4) == Some('e')
            && !id_continue(self.peek_at(5));
        i64_here || usize_here
    }

    /// Consumes the complete suffix at the cursor; call only after
    /// [`Self::at_int_suffix`] holds.
    fn consume_int_suffix(&mut self) -> IntSuffix {
        if self.peek() == Some('u') {
            for _ in 0..5 {
                self.bump();
            }
            return IntSuffix::Usize;
        }
        for _ in 0..3 {
            self.bump();
        }
        IntSuffix::I64
    }

    fn consume_float_tail(&mut self) {
        if self.peek() == Some('.') {
            self.bump();
            while let Some(ch) = self.peek() {
                if ch.is_ascii_digit() || ch == '_' {
                    self.bump();
                } else {
                    break;
                }
            }
        }
        if self.peek().is_some_and(|c| c == 'e' || c == 'E') {
            self.bump();
            if self.peek().is_some_and(|c| c == '-' || c == '+') {
                self.bump();
            }
            while let Some(ch) = self.peek() {
                if ch.is_ascii_digit() {
                    self.bump();
                } else {
                    break;
                }
            }
        }
    }

    fn lex_word(&mut self) -> Result<TokKind, Diag> {
        let mut word = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                word.push(ch);
                self.bump();
            } else {
                break;
            }
        }
        // Raw, byte, and C-string forms are host-valid Rust excluded by
        // §3; lex them as one excluded literal.
        if matches!(word.as_str(), "r" | "b" | "c" | "br" | "rb" | "cr")
            && self.peek().is_some_and(|c| c == '"' || c == '#')
        {
            if self.peek() == Some('"') {
                self.bump();
                self.skip_raw_string_body()?;
                return Ok(TokKind::ExcludedLit);
            }
            // r#"..."# with any number of `#`.
            let mut hashes = 0_u32;
            while self.peek() == Some('#') {
                self.bump();
                hashes += 1;
            }
            if self.peek() == Some('"') {
                self.bump();
                self.skip_raw_string_hashed(hashes)?;
                return Ok(TokKind::ExcludedLit);
            }
            // `b` followed by `#` is not a literal; fall through.
        }
        Ok(TokKind::Ident(word))
    }

    fn skip_raw_string_body(&mut self) -> Result<(), Diag> {
        let start = self.here();
        while let Some(ch) = self.peek() {
            if ch == '"' {
                self.bump();
                return Ok(());
            }
            self.bump();
        }
        Err(Diag::syntax(start, "unterminated string literal"))
    }

    fn skip_raw_string_hashed(&mut self, hashes: u32) -> Result<(), Diag> {
        let start = self.here();
        loop {
            match self.peek() {
                Some('"') => {
                    self.bump();
                    let mut seen = 0_u32;
                    while self.peek() == Some('#') {
                        self.bump();
                        seen += 1;
                    }
                    if seen >= hashes {
                        return Ok(());
                    }
                }
                Some(_) => {
                    self.bump();
                }
                None => return Err(Diag::syntax(start, "unterminated string literal")),
            }
        }
    }

    fn lex_apostrophe(&mut self) -> Result<TokKind, Diag> {
        self.bump();
        match self.peek() {
            // A named lifetime: `'ident` with no closing quote.
            Some(ch) if ch.is_ascii_alphabetic() || ch == '_' => {
                if self.peek_at(1) == Some('\'') {
                    self.bump();
                    self.bump();
                    return Ok(TokKind::CharLit);
                }
                let mut name = String::new();
                while let Some(c) = self.peek() {
                    if c.is_ascii_alphanumeric() || c == '_' {
                        name.push(c);
                        self.bump();
                    } else {
                        break;
                    }
                }
                Ok(TokKind::Lifetime(name))
            }
            // An escaped scalar: `'\n'`, `'\u{1F600}'`, and friends.
            Some('\\') => {
                self.bump();
                self.bump();
                if self.peek().is_some_and(|c| c != '\'') {
                    self.bump();
                }
                if self.peek() == Some('\'') {
                    self.bump();
                    Ok(TokKind::CharLit)
                } else {
                    Err(Diag::syntax(self.here(), "malformed character literal"))
                }
            }
            // Any other scalar followed by a closing quote is a
            // host-valid character literal, excluded by §3.
            Some(_) => {
                self.bump();
                if self.peek() == Some('\'') {
                    self.bump();
                    Ok(TokKind::CharLit)
                } else {
                    Err(Diag::syntax(self.here(), "malformed character literal"))
                }
            }
            None => Err(Diag::syntax(self.here(), "malformed character literal")),
        }
    }

    fn push_escape(&mut self, start: Span, value: &mut String) -> Result<(), Diag> {
        let Some(esc) = self.bump() else {
            return Err(Diag::syntax(start, "unterminated string literal"));
        };
        match esc {
            'n' => value.push('\n'),
            'r' => value.push('\r'),
            't' => value.push('\t'),
            '0' => value.push('\0'),
            '\\' => value.push('\\'),
            '"' => value.push('"'),
            '\'' => value.push('\''),
            'u' => self.push_unicode_escape(value)?,
            _ => {
                return Err(Diag::syntax(
                    self.here(),
                    format!("unknown string escape \\{esc}"),
                ));
            }
        }
        Ok(())
    }

    fn push_unicode_escape(&mut self, value: &mut String) -> Result<(), Diag> {
        if self.bump() != Some('{') {
            return Err(Diag::syntax(self.here(), "malformed unicode escape"));
        }
        let mut digits = String::new();
        loop {
            match self.peek() {
                Some('}') => {
                    self.bump();
                    break;
                }
                Some(c) if c.is_ascii_hexdigit() || c == '_' => {
                    if c != '_' {
                        digits.push(c);
                    }
                    self.bump();
                }
                _ => return Err(Diag::syntax(self.here(), "malformed unicode escape")),
            }
        }
        let code = u32::from_str_radix(&digits, 16)
            .map_err(|_| Diag::syntax(self.here(), "malformed unicode escape"))?;
        let Some(decoded) = char::from_u32(code) else {
            return Err(Diag::syntax(
                self.here(),
                "unicode escape is not a scalar value",
            ));
        };
        value.push(decoded);
        Ok(())
    }

    fn lex_string(&mut self) -> Result<TokKind, Diag> {
        let start = self.here();
        self.bump();
        let mut value = String::new();
        loop {
            let Some(ch) = self.peek() else {
                return Err(Diag::syntax(start, "unterminated string literal"));
            };
            self.bump();
            match ch {
                '"' => return Ok(TokKind::Str(value)),
                '\\' => self.push_escape(start, &mut value)?,
                _ => value.push(ch),
            }
        }
    }
}

/// The reserved words the grammar names. The lexer keeps them as
/// identifiers; the parser consults this table so the token layer stays
/// Rust's.
///
/// # Panics
/// Never: the table is static text.
#[must_use]
pub fn is_keyword(word: &str) -> bool {
    matches!(
        word,
        "as" | "break"
            | "continue"
            | "dyn"
            | "else"
            | "enum"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "struct"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
    )
}
