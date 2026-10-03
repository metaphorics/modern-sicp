// SPDX-License-Identifier: GPL-3.0-only

//! The recursive-descent parser for the Rust host subset (grammar §2
//! and §4): the exact EBNF of the contract, with the expression
//! ladder's ordering as the precedence. Excluded-but-host-valid forms
//! (floats, char literals, `unsafe`, user `impl` blocks, and friends)
//! report [`DiagKind::Unsupported`]; malformed text reports
//! [`DiagKind::Syntax`]. Nothing here invents the retired dynamic
//! language behind new punctuation.

use crate::host::ast::{
    Block, ClosureBody, DeriveName, EnumItem, Expr, ExprKind, FieldDecl, FnItem, Ident, Item,
    LetStmt, MatchArm, Param, Pat, PatKind, Program, Stmt, StructItem, Ty, TyKind, TypeAliasItem,
    UseItem, Variant,
};
use crate::host::diag::{Diag, Span};
use crate::host::hir::{BinOp, UnOp};
use crate::host::lexer::{IntSuffix, Tok, TokKind, lex};

/// A parse failure: the same diagnostic shape the checker answers.
pub type ParseError = Diag;

/// Parses one guest source into its located surface program.
///
/// # Errors
/// The first [`Diag`] the parser raises: [`DiagKind::Syntax`] for
/// malformed text or [`DiagKind::Unsupported`] for an excluded form.
pub fn parse_program(source: &str) -> Result<Program, Diag> {
    let tokens = lex(source)?;
    let mut parser = Parser::new(tokens);
    parser.parse_program()
}

struct Parser {
    tokens: Vec<Tok>,
    pos: usize,
    /// The local name the `use std::collections::HashMap` import binds
    /// (`HashMap`, or its `as` rename), when the program imports it:
    /// only that name spells the map type (grammar §2).
    map_local: Option<String>,
}

/// The name the program's `HashMap` import binds, scanned over the
/// leading `use` items only: the grammar is `use_item* item*`, so an
/// import after an ordinary item does not bind a map name.
fn scan_map_import(tokens: &[Tok]) -> Option<String> {
    let word = |at: usize, text: &str| matches!(tokens.get(at).map(|tok| &tok.kind), Some(TokKind::Ident(name)) if name == text);
    let colons = |at: usize| {
        tokens
            .get(at)
            .is_some_and(|tok| tok.kind == TokKind::ColonColon)
    };
    let mut at = 0;
    while word(at, "use") {
        if word(at + 1, "std")
            && colons(at + 2)
            && word(at + 3, "collections")
            && colons(at + 4)
            && word(at + 5, "HashMap")
        {
            if !word(at + 6, "as") {
                return Some("HashMap".to_owned());
            }
            return match tokens.get(at + 7).map(|tok| &tok.kind) {
                Some(TokKind::Ident(name)) => Some(name.clone()),
                _ => None,
            };
        }
        let mut end = at + 1;
        while tokens
            .get(end)
            .is_some_and(|tok| tok.kind != TokKind::Semi)
        {
            end += 1;
        }
        at = end + 1;
    }
    None
}

/// Rust spells these types, the subset does not admit them; naming one
/// in type position is a host-valid excluded form, not a type error.
fn excluded_primitive_type(word: &str) -> bool {
    matches!(
        word,
        "i8" | "i16"
            | "i32"
            | "i128"
            | "isize"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "u128"
            | "f32"
            | "f64"
            | "f128"
            | "char"
            | "str"
            | "Rc"
            | "Arc"
            | "RefCell"
            | "Cell"
    )
}

impl Parser {
    fn new(tokens: Vec<Tok>) -> Self {
        let map_local = scan_map_import(&tokens);
        Self {
            tokens,
            pos: 0,
            map_local,
        }
    }

    fn peek(&self) -> Option<&Tok> {
        self.tokens.get(self.pos)
    }

    fn peek_kind(&self) -> Option<&TokKind> {
        self.peek().map(|tok| &tok.kind)
    }

    fn span(&self) -> Span {
        self.peek().map_or(Span::new(1, 1, 1), |tok| tok.span)
    }

    fn bump(&mut self) -> Option<Tok> {
        let tok = self.tokens.get(self.pos).cloned();
        if tok.is_some() {
            self.pos += 1;
        }
        tok
    }

    fn at(&self, kind: &TokKind) -> bool {
        self.peek_kind().is_some_and(|k| k == kind)
    }

    fn at_ident(&self, word: &str) -> bool {
        matches!(self.peek_kind(), Some(TokKind::Ident(name)) if name == word)
    }

    fn eat(&mut self, kind: &TokKind) -> bool {
        if self.at(kind) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn expect(&mut self, kind: &TokKind, what: &str) -> Result<Span, Diag> {
        if self.at(kind) {
            let span = self.span();
            self.pos += 1;
            Ok(span)
        } else {
            Err(Diag::syntax(self.span(), format!("expected {what}")))
        }
    }

    fn expect_ident(&mut self, what: &str) -> Result<Ident, Diag> {
        match self.bump() {
            Some(Tok {
                kind: TokKind::Ident(name),
                span,
            }) => Ok(Ident { name, span }),
            Some(tok) => Err(Diag::syntax(tok.span, format!("expected {what}"))),
            None => Err(Diag::syntax(self.span(), format!("expected {what}"))),
        }
    }

    fn reject_excluded_keywords(tok: &Tok) -> Option<Diag> {
        let TokKind::Ident(word) = &tok.kind else {
            return None;
        };
        if matches!(
            word.as_str(),
            "impl"
                | "unsafe"
                | "trait"
                | "const"
                | "static"
                | "mod"
                | "extern"
                | "async"
                | "await"
                | "union"
                | "macro_rules"
                | "where"
                | "pub"
                | "ref"
                | "super"
                | "crate"
                | "Self"
                | "abstract"
                | "box"
                | "final"
                | "override"
                | "priv"
                | "try"
                | "typeof"
                | "unsized"
                | "virtual"
                | "yield"
                | "do"
                | "gen"
        ) {
            return Some(Diag::unsupported(
                tok.span,
                format!("`{word}` is outside the admitted subset"),
            ));
        }
        None
    }

    fn parse_program(&mut self) -> Result<Program, Diag> {
        let mut items = Vec::new();
        let mut past_imports = false;
        while let Some(tok) = self.peek() {
            if let Some(diag) = Self::reject_excluded_keywords(tok) {
                return Err(diag);
            }
            if past_imports
                && matches!(&tok.kind, TokKind::Ident(word) if word == "use")
            {
                return Err(Diag::syntax(
                    tok.span,
                    "imports must lead the unit (the grammar is `use_item* item*`)",
                ));
            }
            let item = self.parse_item()?;
            if !matches!(item, Item::Use(_)) {
                past_imports = true;
            }
            items.push(item);
        }
        Ok(Program { items })
    }

    fn parse_item(&mut self) -> Result<Item, Diag> {
        let derives = self.parse_derives()?;
        if !derives.is_empty()
            && !matches!(self.peek_kind(), Some(TokKind::Ident(word)) if word == "struct" || word == "enum")
        {
            return Err(Diag::syntax(
                self.span(),
                "`#[derive(...)]` belongs on a `struct` or `enum`",
            ));
        }
        match self.peek_kind() {
            Some(TokKind::Ident(word)) if word == "use" => {
                self.bump();
                self.parse_use()
            }
            Some(TokKind::Ident(word)) if word == "struct" => {
                self.bump();
                self.parse_struct(derives)
            }
            Some(TokKind::Ident(word)) if word == "enum" => {
                self.bump();
                self.parse_enum(derives)
            }
            Some(TokKind::Ident(word)) if word == "type" => {
                self.bump();
                self.parse_type_alias()
            }
            Some(TokKind::Ident(word)) if word == "fn" => {
                self.bump();
                self.parse_fn()
            }
            _ => Err(Diag::syntax(self.span(), "expected an item")),
        }
    }

    fn parse_derives(&mut self) -> Result<Vec<DeriveName>, Diag> {
        if !self.at(&TokKind::Pound) {
            return Ok(Vec::new());
        }
        self.bump();
        self.expect(&TokKind::LBracket, "`[` after `#`")?;
        let name = self.expect_ident("a derive name")?;
        if name.name != "derive" {
            return Err(Diag::unsupported(
                name.span,
                "only `#[derive(...)]` attributes are admitted",
            ));
        }
        self.expect(&TokKind::LParen, "`(` after `derive`")?;
        let mut derives = Vec::new();
        loop {
            let name = self.expect_ident("a derive name")?;
            let derive = match name.name.as_str() {
                "Clone" => DeriveName::Clone,
                "Debug" => DeriveName::Debug,
                "PartialEq" => DeriveName::PartialEq,
                "Eq" => DeriveName::Eq,
                "Hash" => DeriveName::Hash,
                _ => {
                    return Err(Diag::unsupported(
                        name.span,
                        format!("derive `{}` is not admitted", name.name),
                    ));
                }
            };
            derives.push(derive);
            if !self.eat(&TokKind::Comma) {
                break;
            }
            if self.at(&TokKind::RParen) {
                break;
            }
        }
        self.expect(&TokKind::RParen, "`)` after derive names")?;
        self.expect(&TokKind::RBracket, "`]` after the derive list")?;
        Ok(derives)
    }

    fn parse_use(&mut self) -> Result<Item, Diag> {
        let first = self.expect_ident("an import path")?;
        if first.name != "std" {
            return Err(Diag::unsupported(
                first.span,
                "only `std::collections::HashMap` may be imported",
            ));
        }
        let mut current = first;
        let mut parts = vec![current.name.clone()];
        while self.eat(&TokKind::ColonColon) {
            current = self.expect_ident("an import path")?;
            parts.push(current.name.clone());
        }
        if parts.as_slice() != ["std", "collections", "HashMap"] {
            return Err(Diag::unsupported(
                current.span,
                "only `std::collections::HashMap` may be imported",
            ));
        }
        let local = if self.at_ident("as") {
            self.bump();
            self.expect_ident("an import rename")?
        } else {
            Ident {
                name: "HashMap".to_owned(),
                span: current.span,
            }
        };
        self.expect(&TokKind::Semi, "`;` after the import")?;
        Ok(Item::Use(UseItem { local }))
    }

    fn parse_struct(&mut self, derives: Vec<DeriveName>) -> Result<Item, Diag> {
        let name = self.expect_ident("a struct name")?;
        if self.at(&TokKind::LBrace) {
            self.bump();
            let mut fields = Vec::new();
            while !self.at(&TokKind::RBrace) {
                let field_name = self.expect_ident("a field name")?;
                self.expect(&TokKind::Colon, "`:` after the field name")?;
                let ty = self.parse_ty()?;
                fields.push(FieldDecl {
                    name: field_name,
                    ty,
                });
                if !self.eat(&TokKind::Comma) {
                    break;
                }
            }
            self.expect(&TokKind::RBrace, "`}` to close the struct")?;
            return Ok(Item::Struct(StructItem {
                derives,
                name,
                fields,
                tuple: Vec::new(),
            }));
        }
        self.expect(&TokKind::LParen, "`(` or `{` after the struct name")?;
        let mut tuple = Vec::new();
        while !self.at(&TokKind::RParen) {
            tuple.push(self.parse_ty()?);
            if !self.eat(&TokKind::Comma) {
                break;
            }
        }
        self.expect(&TokKind::RParen, "`)` to close the tuple struct")?;
        self.expect(&TokKind::Semi, "`;` after the tuple struct")?;
        Ok(Item::Struct(StructItem {
            derives,
            name,
            fields: Vec::new(),
            tuple,
        }))
    }

    fn parse_enum(&mut self, derives: Vec<DeriveName>) -> Result<Item, Diag> {
        let name = self.expect_ident("an enum name")?;
        self.expect(&TokKind::LBrace, "`{` after the enum name")?;
        let mut variants = Vec::new();
        while !self.at(&TokKind::RBrace) {
            variants.push(self.parse_variant()?);
            if !self.eat(&TokKind::Comma) {
                break;
            }
        }
        self.expect(&TokKind::RBrace, "`}` to close the enum")?;
        Ok(Item::Enum(EnumItem {
            derives,
            name,
            variants,
        }))
    }

    fn parse_variant(&mut self) -> Result<Variant, Diag> {
        let name = self.expect_ident("a variant name")?;
        if self.at(&TokKind::LParen) {
            self.bump();
            let mut tuple = Vec::new();
            while !self.at(&TokKind::RParen) {
                tuple.push(self.parse_ty()?);
                if !self.eat(&TokKind::Comma) {
                    break;
                }
            }
            self.expect(&TokKind::RParen, "`)` to close the variant")?;
            return Ok(Variant {
                name,
                tuple,
                fields: Vec::new(),
            });
        }
        if self.at(&TokKind::LBrace) {
            self.bump();
            let mut fields = Vec::new();
            while !self.at(&TokKind::RBrace) {
                let field_name = self.expect_ident("a field name")?;
                self.expect(&TokKind::Colon, "`:` after the field name")?;
                let ty = self.parse_ty()?;
                fields.push(FieldDecl {
                    name: field_name,
                    ty,
                });
                if !self.eat(&TokKind::Comma) {
                    break;
                }
            }
            self.expect(&TokKind::RBrace, "`}` to close the variant")?;
            return Ok(Variant {
                name,
                tuple: Vec::new(),
                fields,
            });
        }
        Ok(Variant {
            name,
            tuple: Vec::new(),
            fields: Vec::new(),
        })
    }

    fn parse_type_alias(&mut self) -> Result<Item, Diag> {
        let name = self.expect_ident("an alias name")?;
        self.expect(&TokKind::Eq, "`=` in a type alias")?;
        let ty = self.parse_ty()?;
        self.expect(&TokKind::Semi, "`;` after the type alias")?;
        Ok(Item::TypeAlias(TypeAliasItem { name, ty }))
    }

    fn parse_fn(&mut self) -> Result<Item, Diag> {
        let name = self.expect_ident("a function name")?;
        self.expect(&TokKind::LParen, "`(` after the function name")?;
        let mut params = Vec::new();
        while !self.at(&TokKind::RParen) {
            let mutable = self.at_ident("mut");
            if mutable {
                self.bump();
            }
            let param_name = self.expect_ident("a parameter name")?;
            self.expect(&TokKind::Colon, "`:` after the parameter name")?;
            let ty = self.parse_ty()?;
            params.push(Param {
                mutable,
                name: param_name,
                ty,
            });
            if !self.eat(&TokKind::Comma) {
                break;
            }
        }
        self.expect(&TokKind::RParen, "`)` after the parameters")?;
        let ret = if self.eat(&TokKind::Arrow) {
            Some(self.parse_ty()?)
        } else {
            None
        };
        let body = self.parse_block()?;
        Ok(Item::Fn(FnItem {
            name,
            params,
            ret,
            body,
        }))
    }

    fn parse_ty(&mut self) -> Result<Ty, Diag> {
        let span = self.span();
        let kind = self.parse_ty_kind()?;
        Ok(Ty { kind, span })
    }

    fn parse_ty_kind(&mut self) -> Result<TyKind, Diag> {
        let tok = self.peek().cloned();
        match tok {
            Some(Tok {
                kind: TokKind::Ident(word),
                span,
            }) => self.parse_ty_named(&word, span),
            Some(Tok {
                kind: TokKind::LParen,
                ..
            }) => {
                self.bump();
                if self.eat(&TokKind::RParen) {
                    return Ok(TyKind::Unit);
                }
                let first = self.parse_ty()?;
                self.expect(&TokKind::Comma, "`,` in a tuple type")?;
                let second = self.parse_ty()?;
                self.expect(&TokKind::RParen, "`)` to close the tuple type")?;
                Ok(TyKind::Tuple(Box::new(first), Box::new(second)))
            }
            Some(Tok {
                kind: TokKind::LBracket,
                ..
            }) => {
                self.bump();
                let inner = self.parse_ty()?;
                self.expect(&TokKind::Semi, "`;` in an array type")?;
                let len = match self.bump() {
                    Some(Tok {
                        kind: TokKind::Int { text, .. },
                        ..
                    }) => text,
                    Some(tok) => {
                        return Err(Diag::syntax(tok.span, "array lengths are integer literals"));
                    }
                    None => return Err(Diag::syntax(self.span(), "expected an array length")),
                };
                self.expect(&TokKind::RBracket, "`]` to close the array type")?;
                Ok(TyKind::Array(Box::new(inner), len))
            }
            Some(Tok {
                kind: TokKind::Amp,
                span,
            }) => {
                self.bump();
                if let Some(Tok {
                    kind: TokKind::Lifetime(name),
                    span: life_span,
                }) = self.peek().cloned()
                {
                    let _ = name;
                    return Err(Diag::unsupported(
                        life_span,
                        "named lifetimes are excluded; reference lifetimes are elided",
                    ));
                }
                let _ = span;
                let mutable = self.at_ident("mut");
                if mutable {
                    self.bump();
                }
                // `&str` is one admitted type (grammar §3); a bare `str`
                // or `&mut str` is not.
                if !mutable && self.at_ident("str") {
                    self.bump();
                    return Ok(TyKind::Str);
                }
                let inner = self.parse_ty()?;
                Ok(TyKind::Ref(mutable, Box::new(inner)))
            }
            Some(tok) if tok.kind == TokKind::Star => Err(Diag::unsupported(
                tok.span,
                "raw pointers are outside the subset",
            )),
            Some(tok) => Err(Diag::syntax(tok.span, "expected a type")),
            None => Err(Diag::syntax(self.span(), "expected a type")),
        }
    }

    fn parse_ty_named(&mut self, word: &str, span: Span) -> Result<TyKind, Diag> {
        self.bump();
        Ok(match word {
            // The import's local name shadows every built-in spelling:
            // `use std::collections::HashMap as Vec` makes `Vec<K, V>`
            // a map type, not a `Vec`.
            name if self.map_local.as_deref() == Some(name) => {
                self.parse_angle_open()?;
                let key = self.expect_ident("the `HashMap` key type")?;
                if key.name != "String" {
                    return Err(Diag::unsupported(
                        key.span,
                        "`HashMap` keys are `String` in the admitted subset",
                    ));
                }
                self.expect(&TokKind::Comma, "`,` in `HashMap<...>`")?;
                let value = self.parse_ty()?;
                self.expect(&TokKind::Gt, "`>` to close `HashMap<...>`")?;
                TyKind::HashMap(Box::new(value))
            }
            "bool" => TyKind::Bool,
            "i64" => TyKind::I64,
            "usize" => TyKind::Usize,
            "String" => TyKind::String,
            "Box" => {
                self.parse_angle_open()?;
                if self.at_ident("dyn") {
                    let closure = self.parse_dyn_closure()?;
                    self.expect(&TokKind::Gt, "`>` to close `Box<dyn ...>`")?;
                    TyKind::DynClosure(closure.0, closure.1, closure.2)
                } else {
                    let inner = self.parse_ty()?;
                    self.expect(&TokKind::Gt, "`>` to close `Box<...>`")?;
                    TyKind::Box(Box::new(inner))
                }
            }
            "Vec" => {
                self.parse_angle_open()?;
                let inner = self.parse_ty()?;
                self.expect(&TokKind::Gt, "`>` to close `Vec<...>`")?;
                TyKind::Vec(Box::new(inner))
            }
            "Option" => {
                self.parse_angle_open()?;
                let inner = self.parse_ty()?;
                self.expect(&TokKind::Gt, "`>` to close `Option<...>`")?;
                TyKind::Option(Box::new(inner))
            }
            "Result" => {
                self.parse_angle_open()?;
                let ok = self.parse_ty()?;
                self.expect(&TokKind::Comma, "`,` in `Result<...>`")?;
                let err = self.parse_ty()?;
                self.expect(&TokKind::Gt, "`>` to close `Result<...>`")?;
                TyKind::Result(Box::new(ok), Box::new(err))
            }
            "fn" => {
                self.expect(&TokKind::LParen, "`(` in a function type")?;
                let mut params = Vec::new();
                while !self.at(&TokKind::RParen) {
                    params.push(self.parse_ty()?);
                    if !self.eat(&TokKind::Comma) {
                        break;
                    }
                }
                self.expect(&TokKind::RParen, "`)` in a function type")?;
                self.expect(&TokKind::Arrow, "`->` in a function type")?;
                let ret = self.parse_ty()?;
                TyKind::FnPtr(params, Box::new(ret))
            }
            "HashMap" => {
                return Err(Diag::type_error(
                    span,
                    "cannot find type `HashMap` in this scope; \
                     import it with `use std::collections::HashMap;`",
                ));
            }
            other => {
                if excluded_primitive_type(other) {
                    return Err(Diag::unsupported(
                        span,
                        format!("`{other}` is outside the subset's types"),
                    ));
                }
                TyKind::Named(other.to_owned())
            }
        })
    }

    fn parse_angle_open(&mut self) -> Result<(), Diag> {
        self.expect(&TokKind::Lt, "`<` to open a generic type")?;
        Ok(())
    }

    fn parse_dyn_closure(
        &mut self,
    ) -> Result<(crate::host::ast::ClosureTrait, Vec<Ty>, Box<Ty>), Diag> {
        self.bump();
        let name = self.expect_ident("a closure trait")?;
        let kind = match name.name.as_str() {
            "Fn" => crate::host::ast::ClosureTrait::Fn,
            "FnMut" => crate::host::ast::ClosureTrait::FnMut,
            "FnOnce" => crate::host::ast::ClosureTrait::FnOnce,
            _ => {
                return Err(Diag::unsupported(
                    name.span,
                    "only `dyn Fn`, `dyn FnMut`, and `dyn FnOnce` objects are admitted",
                ));
            }
        };
        self.expect(&TokKind::LParen, "`(` in a closure object type")?;
        let mut params = Vec::new();
        while !self.at(&TokKind::RParen) {
            params.push(self.parse_ty()?);
            if !self.eat(&TokKind::Comma) {
                break;
            }
        }
        self.expect(&TokKind::RParen, "`)` in a closure object type")?;
        self.expect(&TokKind::Arrow, "`->` in a closure object type")?;
        let ret = self.parse_ty()?;
        self.expect(&TokKind::Plus, "`+` before the object lifetime")?;
        match self.bump() {
            Some(Tok {
                kind: TokKind::Lifetime(name),
                span,
            }) => {
                if name != "static" {
                    return Err(Diag::unsupported(
                        span,
                        "boxed closure objects admit only the `'static` lifetime",
                    ));
                }
            }
            Some(tok) => {
                return Err(Diag::unsupported(
                    tok.span,
                    "boxed closure objects admit only the `'static` lifetime",
                ));
            }
            None => return Err(Diag::syntax(self.span(), "expected `'static`")),
        }
        Ok((kind, params, Box::new(ret)))
    }

    fn parse_block(&mut self) -> Result<Block, Diag> {
        let span = self.expect(&TokKind::LBrace, "`{` to open a block")?;
        let mut stmts = Vec::new();
        let mut tail = None;
        while !self.at(&TokKind::RBrace) {
            if self.at_ident("let") {
                self.bump();
                stmts.push(Stmt::Let(self.parse_let()?));
                continue;
            }
            let expr = self.parse_expr()?;
            let block_like = matches!(
                expr.kind,
                ExprKind::If { .. }
                    | ExprKind::IfLet { .. }
                    | ExprKind::Match { .. }
                    | ExprKind::While { .. }
                    | ExprKind::WhileLet { .. }
                    | ExprKind::For { .. }
                    | ExprKind::Loop(_)
                    | ExprKind::Block(_)
            );
            if self.eat(&TokKind::Semi) {
                stmts.push(Stmt::Expr { expr, semi: true });
                continue;
            }
            if self.at(&TokKind::RBrace) {
                tail = Some(Box::new(expr));
                break;
            }
            if block_like {
                stmts.push(Stmt::Expr { expr, semi: false });
                continue;
            }
            return Err(Diag::syntax(
                self.span(),
                "expected `;` after an expression statement",
            ));
        }
        self.expect(&TokKind::RBrace, "`}` to close the block")?;
        Ok(Block { stmts, tail, span })
    }

    fn parse_let(&mut self) -> Result<LetStmt, Diag> {
        let span = self.span();
        let mutable = self.at_ident("mut");
        if mutable {
            self.bump();
        }
        let pat = self.parse_let_pattern()?;
        let annotation = if self.eat(&TokKind::Colon) {
            Some(self.parse_ty()?)
        } else {
            None
        };
        self.expect(&TokKind::Eq, "`=` in a let statement")?;
        let value = self.parse_expr()?;
        self.expect(&TokKind::Semi, "`;` after a let statement")?;
        Ok(LetStmt {
            mutable,
            pat,
            annotation,
            value,
            span,
        })
    }

    fn parse_let_pattern(&mut self) -> Result<Pat, Diag> {
        let span = self.span();
        if self.at(&TokKind::LParen) {
            self.bump();
            let first = self.parse_pattern()?;
            self.expect(&TokKind::Comma, "`,` in a tuple pattern")?;
            let second = self.parse_pattern()?;
            self.expect(&TokKind::RParen, "`)` to close the tuple pattern")?;
            return Ok(Pat {
                kind: PatKind::Tuple(Box::new(first), Box::new(second)),
                span,
            });
        }
        self.parse_pattern()
    }

    fn parse_pattern(&mut self) -> Result<Pat, Diag> {
        let span = self.span();
        let tok = self.peek().cloned();
        let kind = match tok {
            Some(Tok {
                kind: TokKind::Ident(word),
                span: _,
            }) if word == "_" => {
                self.bump();
                PatKind::Wild
            }
            Some(Tok {
                kind: TokKind::Ident(word),
                ..
            }) if word == "true" || word == "false" => {
                self.bump();
                PatKind::BoolLit(word == "true")
            }
            Some(Tok {
                kind: TokKind::Int { text, suffix },
                ..
            }) => {
                self.bump();
                PatKind::IntLit { text, suffix }
            }
            Some(Tok {
                kind: TokKind::LParen,
                ..
            }) => {
                self.bump();
                let first = self.parse_pattern()?;
                self.expect(&TokKind::Comma, "`,` in a tuple pattern")?;
                let second = self.parse_pattern()?;
                self.expect(&TokKind::RParen, "`)` to close the tuple pattern")?;
                PatKind::Tuple(Box::new(first), Box::new(second))
            }
            Some(Tok {
                kind: TokKind::Ident(_),
                span: _,
            }) => {
                let path = self.parse_path()?;
                if self.at(&TokKind::LParen) {
                    self.bump();
                    let mut args = Vec::new();
                    while !self.at(&TokKind::RParen) {
                        args.push(self.parse_pattern()?);
                        if !self.eat(&TokKind::Comma) {
                            break;
                        }
                    }
                    self.expect(&TokKind::RParen, "`)` to close the pattern")?;
                    PatKind::TuplePath(path, args)
                } else if self.at(&TokKind::LBrace) {
                    self.bump();
                    let mut fields = Vec::new();
                    while !self.at(&TokKind::RBrace) {
                        let name = self.expect_ident("a field name")?;
                        let sub = if self.eat(&TokKind::Colon) {
                            Some(self.parse_pattern()?)
                        } else {
                            None
                        };
                        fields.push((name, sub));
                        if !self.eat(&TokKind::Comma) {
                            break;
                        }
                    }
                    self.expect(&TokKind::RBrace, "`}` to close the pattern")?;
                    PatKind::Struct { path, fields }
                } else if path.len() == 1 {
                    PatKind::Bind(path.into_iter().next().unwrap_or(Ident {
                        name: String::new(),
                        span,
                    }))
                } else {
                    PatKind::Path(path)
                }
            }
            Some(tok) => return Err(Diag::syntax(tok.span, "expected a pattern")),
            None => return Err(Diag::syntax(self.span(), "expected a pattern")),
        };
        Ok(Pat { kind, span })
    }

    fn parse_path(&mut self) -> Result<Vec<Ident>, Diag> {
        let mut path = vec![self.expect_ident("a path")?];
        while self.at(&TokKind::ColonColon) {
            self.bump();
            path.push(self.expect_ident("a path segment")?);
        }
        Ok(path)
    }

    fn parse_expr(&mut self) -> Result<Expr, Diag> {
        self.parse_expr_inner(true)
    }

    fn parse_expr_no_struct(&mut self) -> Result<Expr, Diag> {
        self.parse_expr_inner(false)
    }

    fn parse_expr_inner(&mut self, allow_struct: bool) -> Result<Expr, Diag> {
        self.parse_assignment(allow_struct)
    }

    fn parse_assignment(&mut self, allow_struct: bool) -> Result<Expr, Diag> {
        let span = self.span();
        let target = self.parse_range(allow_struct)?;
        let op = if self.at(&TokKind::Eq) {
            self.bump();
            Some(None)
        } else if self.at(&TokKind::PlusEq) {
            self.bump();
            Some(Some(BinOp::Add))
        } else if self.at(&TokKind::MinusEq) {
            self.bump();
            Some(Some(BinOp::Sub))
        } else {
            None
        };
        let Some(op) = op else {
            return Ok(target);
        };
        let value = self.parse_assignment(allow_struct)?;
        Ok(Expr {
            kind: ExprKind::Assign {
                op,
                target: Box::new(target),
                value: Box::new(value),
            },
            span,
        })
    }

    fn parse_range(&mut self, allow_struct: bool) -> Result<Expr, Diag> {
        let span = self.span();
        let start = self.parse_binary(allow_struct)?;
        if self.at(&TokKind::DotDot) {
            self.bump();
            let end = self.parse_binary(allow_struct)?;
            return Ok(Expr {
                kind: ExprKind::Range(Box::new(start), Box::new(end)),
                span,
            });
        }
        Ok(start)
    }

    fn parse_binary(&mut self, allow_struct: bool) -> Result<Expr, Diag> {
        self.parse_or(allow_struct)
    }

    fn parse_or(&mut self, allow_struct: bool) -> Result<Expr, Diag> {
        let span = self.span();
        let mut left = self.parse_and(allow_struct)?;
        while self.at(&TokKind::PipePipe) {
            self.bump();
            let right = self.parse_and(allow_struct)?;
            left = Expr {
                kind: ExprKind::Binary {
                    op: BinOp::Or,
                    left: Box::new(left),
                    right: Box::new(right),
                },
                span,
            };
        }
        Ok(left)
    }

    fn parse_and(&mut self, allow_struct: bool) -> Result<Expr, Diag> {
        let span = self.span();
        let mut left = self.parse_equality(allow_struct)?;
        while self.at(&TokKind::AmpAmp) {
            self.bump();
            let right = self.parse_equality(allow_struct)?;
            left = Expr {
                kind: ExprKind::Binary {
                    op: BinOp::And,
                    left: Box::new(left),
                    right: Box::new(right),
                },
                span,
            };
        }
        Ok(left)
    }

    fn parse_equality(&mut self, allow_struct: bool) -> Result<Expr, Diag> {
        let span = self.span();
        let left = self.parse_comparison(allow_struct)?;
        let op = if self.at(&TokKind::EqEq) {
            self.bump();
            Some(BinOp::Eq)
        } else if self.at(&TokKind::NotEq) {
            self.bump();
            Some(BinOp::Ne)
        } else {
            None
        };
        let Some(op) = op else {
            return Ok(left);
        };
        let right = self.parse_comparison(allow_struct)?;
        Ok(Expr {
            kind: ExprKind::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            },
            span,
        })
    }

    fn parse_comparison(&mut self, allow_struct: bool) -> Result<Expr, Diag> {
        let span = self.span();
        let mut left = self.parse_additive(allow_struct)?;
        loop {
            let op = if self.at(&TokKind::Lt) {
                self.bump();
                BinOp::Lt
            } else if self.at(&TokKind::Le) {
                self.bump();
                BinOp::Le
            } else if self.at(&TokKind::Gt) {
                self.bump();
                BinOp::Gt
            } else if self.at(&TokKind::Ge) {
                self.bump();
                BinOp::Ge
            } else {
                break;
            };
            let right = self.parse_additive(allow_struct)?;
            left = Expr {
                kind: ExprKind::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                },
                span,
            };
        }
        Ok(left)
    }

    fn parse_additive(&mut self, allow_struct: bool) -> Result<Expr, Diag> {
        let span = self.span();
        let mut left = self.parse_multiplicative(allow_struct)?;
        loop {
            let op = if self.at(&TokKind::Plus) {
                self.bump();
                BinOp::Add
            } else if self.at(&TokKind::Minus) {
                self.bump();
                BinOp::Sub
            } else {
                break;
            };
            let right = self.parse_multiplicative(allow_struct)?;
            left = Expr {
                kind: ExprKind::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                },
                span,
            };
        }
        Ok(left)
    }

    fn parse_multiplicative(&mut self, allow_struct: bool) -> Result<Expr, Diag> {
        let span = self.span();
        let mut left = self.parse_unary(allow_struct)?;
        loop {
            let op = if self.at(&TokKind::Star) {
                self.bump();
                BinOp::Mul
            } else if self.at(&TokKind::Slash) {
                self.bump();
                BinOp::Div
            } else if self.at(&TokKind::Percent) {
                self.bump();
                BinOp::Rem
            } else {
                break;
            };
            let right = self.parse_unary(allow_struct)?;
            left = Expr {
                kind: ExprKind::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                },
                span,
            };
        }
        Ok(left)
    }

    fn parse_unary(&mut self, allow_struct: bool) -> Result<Expr, Diag> {
        let span = self.span();
        let op = if self.at(&TokKind::Minus) {
            self.bump();
            Some(UnOp::Neg)
        } else if self.at(&TokKind::Bang) {
            self.bump();
            Some(UnOp::Not)
        } else if self.at(&TokKind::Star) {
            self.bump();
            Some(UnOp::Deref)
        } else if self.at(&TokKind::Amp) {
            self.bump();
            let mutable = self.at_ident("mut");
            if mutable {
                self.bump();
            }
            Some(if mutable { UnOp::RefMut } else { UnOp::Ref })
        } else {
            None
        };
        let Some(op) = op else {
            return self.parse_postfix(allow_struct);
        };
        let operand = self.parse_unary(allow_struct)?;
        Ok(Expr {
            kind: ExprKind::Unary {
                op,
                operand: Box::new(operand),
            },
            span,
        })
    }

    fn parse_postfix(&mut self, allow_struct: bool) -> Result<Expr, Diag> {
        let span = self.span();
        let mut expr = self.parse_primary(allow_struct)?;
        loop {
            if self.at(&TokKind::Dot) {
                self.bump();
                // A tuple position `.0`/`.1` reads that element; any
                // other dotted access names a field or method.
                let name = match self.peek().cloned() {
                    Some(Tok {
                        kind: TokKind::Int { text, .. },
                        span,
                    }) => {
                        self.bump();
                        crate::host::ast::Ident { name: text, span }
                    }
                    _ => self.expect_ident("a field or method name")?,
                };
                if self.at(&TokKind::LParen) {
                    self.bump();
                    let args = self.parse_arg_list()?;
                    self.expect(&TokKind::RParen, "`)` after the arguments")?;
                    expr = Expr {
                        kind: ExprKind::MethodCall {
                            receiver: Box::new(expr),
                            name,
                            args,
                        },
                        span,
                    };
                } else {
                    expr = Expr {
                        kind: ExprKind::Field {
                            base: Box::new(expr),
                            name,
                        },
                        span,
                    };
                }
                continue;
            }
            if self.at(&TokKind::LParen) {
                self.bump();
                let args = self.parse_arg_list()?;
                self.expect(&TokKind::RParen, "`)` after the arguments")?;
                expr = Expr {
                    kind: ExprKind::Call {
                        callee: Box::new(expr),
                        args,
                    },
                    span,
                };
                continue;
            }
            if self.at(&TokKind::LBracket) {
                self.bump();
                let index = self.parse_expr()?;
                self.expect(&TokKind::RBracket, "`]` after the index")?;
                expr = Expr {
                    kind: ExprKind::Index {
                        base: Box::new(expr),
                        index: Box::new(index),
                    },
                    span,
                };
                continue;
            }
            if self.at(&TokKind::Question) {
                self.bump();
                expr = Expr {
                    kind: ExprKind::Try(Box::new(expr)),
                    span,
                };
                continue;
            }
            // The postfix grammar admits `.`, calls, indexes, and `?`;
            // an `as` cast is host-valid Rust outside the subset.
            if self.at_ident("as") {
                return Err(Diag::unsupported(
                    self.span(),
                    "`as` casts are outside the subset",
                ));
            }
            break;
        }
        Ok(expr)
    }

    fn parse_arg_list(&mut self) -> Result<Vec<Expr>, Diag> {
        let mut args = Vec::new();
        while !self.at(&TokKind::RParen) {
            args.push(self.parse_expr()?);
            if !self.eat(&TokKind::Comma) {
                break;
            }
        }
        Ok(args)
    }

    fn parse_primary(&mut self, allow_struct: bool) -> Result<Expr, Diag> {
        let span = self.span();
        match self.peek().cloned() {
            Some(
                tok @ Tok {
                    kind:
                        TokKind::Int { .. }
                        | TokKind::Str(_)
                        | TokKind::Float
                        | TokKind::CharLit
                        | TokKind::ExcludedLit,
                    ..
                },
            ) => self.parse_primary_literal(tok, span),
            Some(Tok {
                kind: TokKind::Ident(word),
                span: word_span,
            }) => self.parse_primary_word(&word, word_span, allow_struct),
            Some(Tok {
                kind: TokKind::LParen,
                ..
            }) => self.parse_parenthesized(span),
            Some(Tok {
                kind: TokKind::LBracket,
                ..
            }) => self.parse_array(span),
            Some(Tok {
                kind: TokKind::Pipe | TokKind::PipePipe,
                ..
            }) => self.parse_closure(false, span),
            Some(Tok {
                kind: TokKind::Minus | TokKind::Bang | TokKind::Star | TokKind::Amp,
                ..
            }) => self.parse_unary(true),
            // A bare block is a block expression (grammar §4); a struct
            // literal always leads with its path.
            Some(Tok {
                kind: TokKind::LBrace,
                ..
            }) => Ok(Expr {
                kind: ExprKind::Block(self.parse_block()?),
                span,
            }),
            Some(tok) => {
                if let Some(diag) = Self::reject_excluded_keywords(&tok) {
                    return Err(diag);
                }
                Err(Diag::syntax(tok.span, "expected an expression"))
            }
            None => Err(Diag::syntax(self.span(), "expected an expression")),
        }
    }

    fn parse_primary_literal(&mut self, tok: Tok, span: Span) -> Result<Expr, Diag> {
        let tok_span = tok.span;
        self.bump();
        match tok.kind {
            TokKind::Int { text, suffix } => Ok(Expr {
                kind: ExprKind::IntLit { text, suffix },
                span,
            }),
            TokKind::Str(value) => Ok(Expr {
                kind: ExprKind::StrLit(value),
                span,
            }),
            TokKind::Float => Err(Diag::unsupported(
                tok_span,
                "floating-point literals are outside the subset",
            )),
            TokKind::CharLit => Err(Diag::unsupported(
                tok_span,
                "character literals are outside the subset",
            )),
            TokKind::ExcludedLit => Err(Diag::unsupported(
                tok_span,
                "raw, byte, and C-string literals are outside the subset",
            )),
            _ => Err(Diag::syntax(tok_span, "expected an expression")),
        }
    }

    fn parse_parenthesized(&mut self, span: Span) -> Result<Expr, Diag> {
        self.bump();
        if self.eat(&TokKind::RParen) {
            return Ok(Expr {
                kind: ExprKind::UnitLit,
                span,
            });
        }
        let first = self.parse_expr()?;
        if self.eat(&TokKind::Comma) {
            let second = self.parse_expr()?;
            self.expect(&TokKind::RParen, "`)` to close the tuple")?;
            return Ok(Expr {
                kind: ExprKind::Tuple(Box::new(first), Box::new(second)),
                span,
            });
        }
        self.expect(&TokKind::RParen, "`)` to close the expression")?;
        Ok(first)
    }

    fn parse_array(&mut self, span: Span) -> Result<Expr, Diag> {
        self.bump();
        let mut items = Vec::new();
        while !self.at(&TokKind::RBracket) {
            items.push(self.parse_expr()?);
            if !self.eat(&TokKind::Comma) {
                break;
            }
        }
        self.expect(&TokKind::RBracket, "`]` to close the array")?;
        Ok(Expr {
            kind: ExprKind::Array(items),
            span,
        })
    }

    fn parse_primary_word(
        &mut self,
        word: &str,
        span: Span,
        allow_struct: bool,
    ) -> Result<Expr, Diag> {
        match word {
            "true" | "false" => {
                self.bump();
                Ok(Expr {
                    kind: ExprKind::BoolLit(word == "true"),
                    span,
                })
            }
            "if" => {
                self.bump();
                self.parse_if(span)
            }
            "match" => {
                self.bump();
                self.parse_match(span)
            }
            "while" => {
                self.bump();
                self.parse_while(span)
            }
            "for" => {
                self.bump();
                self.parse_for(span)
            }
            "loop" => {
                self.bump();
                let body = self.parse_block()?;
                Ok(Expr {
                    kind: ExprKind::Loop(body),
                    span,
                })
            }
            "return" => {
                self.bump();
                let value = if self.at_expr_end() {
                    None
                } else {
                    Some(Box::new(self.parse_expr()?))
                };
                Ok(Expr {
                    kind: ExprKind::Return(value),
                    span,
                })
            }
            "break" => {
                self.bump();
                let value = if self.at_expr_end() {
                    None
                } else {
                    Some(Box::new(self.parse_expr()?))
                };
                Ok(Expr {
                    kind: ExprKind::Break(value),
                    span,
                })
            }
            "continue" => {
                self.bump();
                Ok(Expr {
                    kind: ExprKind::Continue,
                    span,
                })
            }
            "move" => {
                self.bump();
                self.parse_closure(true, span)
            }
            "vec" if self.at_macro_bang("vec") => {
                self.bump();
                self.bump();
                self.parse_vec_macro(span)
            }
            "format" | "print" | "println" if self.at_macro_bang(word) => {
                self.bump();
                self.bump();
                self.parse_format_macro(word, span)
            }
            _ => {
                if let Some(diag) =
                    Self::reject_excluded_keywords(&self.peek().cloned().unwrap_or(Tok {
                        kind: TokKind::Ident(word.to_owned()),
                        span,
                    }))
                {
                    return Err(diag);
                }
                let path = self.parse_path()?;
                self.parse_path_expr(path, span, allow_struct)
            }
        }
    }

    fn at_macro_bang(&self, name: &str) -> bool {
        self.at_ident(name)
            && self
                .tokens
                .get(self.pos + 1)
                .is_some_and(|tok| tok.kind == TokKind::Bang)
    }

    fn at_expr_end(&self) -> bool {
        matches!(
            self.peek_kind(),
            Some(
                TokKind::Semi
                    | TokKind::RParen
                    | TokKind::RBrace
                    | TokKind::RBracket
                    | TokKind::Comma
            )
        ) || self.peek().is_none()
    }

    fn parse_vec_macro(&mut self, span: Span) -> Result<Expr, Diag> {
        self.expect(&TokKind::LBracket, "`[` after `vec!`")?;
        let first = self.parse_expr()?;
        if self.eat(&TokKind::Semi) {
            let count = self.parse_expr()?;
            self.expect(&TokKind::RBracket, "`]` to close `vec!`")?;
            return Ok(Expr {
                kind: ExprKind::VecRepeat(Box::new(first), Box::new(count)),
                span,
            });
        }
        let mut items = vec![first];
        while self.eat(&TokKind::Comma) {
            if self.at(&TokKind::RBracket) {
                break;
            }
            items.push(self.parse_expr()?);
        }
        self.expect(&TokKind::RBracket, "`]` to close `vec!`")?;
        Ok(Expr {
            kind: ExprKind::VecList(items),
            span,
        })
    }

    fn parse_format_macro(&mut self, name: &str, span: Span) -> Result<Expr, Diag> {
        let kind = match name {
            "format" => crate::host::ast::FormatKind::Format,
            "print" => crate::host::ast::FormatKind::Print,
            _ => crate::host::ast::FormatKind::Println,
        };
        self.expect(&TokKind::LParen, "`(` after the format macro")?;
        let fmt = match self.bump() {
            Some(Tok {
                kind: TokKind::Str(value),
                ..
            }) => value,
            Some(tok) => {
                return Err(Diag::unsupported(
                    tok.span,
                    "format macros take a string literal",
                ));
            }
            None => return Err(Diag::syntax(self.span(), "expected a format string")),
        };
        let mut args = Vec::new();
        while self.eat(&TokKind::Comma) {
            if self.at(&TokKind::RParen) {
                break;
            }
            args.push(self.parse_expr()?);
        }
        self.expect(&TokKind::RParen, "`)` to close the format macro")?;
        Ok(Expr {
            kind: ExprKind::Format { kind, fmt, args },
            span,
        })
    }

    fn parse_closure(&mut self, mov: bool, span: Span) -> Result<Expr, Diag> {
        let mut params = Vec::new();
        if self.at(&TokKind::PipePipe) {
            self.bump();
        } else {
            self.expect(&TokKind::Pipe, "`|` to open closure parameters")?;
            while !self.at(&TokKind::Pipe) {
                let name = self.expect_ident("a closure parameter")?;
                let ty = if self.eat(&TokKind::Colon) {
                    Some(self.parse_ty()?)
                } else {
                    None
                };
                params.push((name, ty));
                if !self.eat(&TokKind::Comma) {
                    break;
                }
            }
            self.expect(&TokKind::Pipe, "`|` to close closure parameters")?;
        }
        let body = if self.at(&TokKind::LBrace) {
            ClosureBody::Block(self.parse_block()?)
        } else {
            ClosureBody::Expr(Box::new(self.parse_expr()?))
        };
        Ok(Expr {
            kind: ExprKind::Closure { mov, params, body },
            span,
        })
    }

    fn parse_if(&mut self, span: Span) -> Result<Expr, Diag> {
        if self.at_ident("let") {
            self.bump();
            let pat = self.parse_pattern()?;
            self.expect(&TokKind::Eq, "`=` in `if let`")?;
            let value = self.parse_expr_no_struct()?;
            let then = self.parse_block()?;
            let else_branch = self.parse_else()?;
            return Ok(Expr {
                kind: ExprKind::IfLet {
                    pat,
                    value: Box::new(value),
                    then,
                    else_branch,
                },
                span,
            });
        }
        let test = self.parse_expr_no_struct()?;
        let then = self.parse_block()?;
        let else_branch = self.parse_else()?;
        Ok(Expr {
            kind: ExprKind::If {
                test: Box::new(test),
                then,
                else_branch,
            },
            span,
        })
    }

    fn parse_else(&mut self) -> Result<Option<Box<Expr>>, Diag> {
        if !self.at_ident("else") {
            return Ok(None);
        }
        self.bump();
        let span = self.span();
        if self.at_ident("if") {
            self.bump();
            let inner = self.parse_if(span)?;
            return Ok(Some(Box::new(inner)));
        }
        let block = self.parse_block()?;
        Ok(Some(Box::new(Expr {
            kind: ExprKind::Block(block),
            span,
        })))
    }

    fn parse_match(&mut self, span: Span) -> Result<Expr, Diag> {
        let scrutinee = self.parse_expr_no_struct()?;
        self.expect(&TokKind::LBrace, "`{` to open the match")?;
        let mut arms = Vec::new();
        while !self.at(&TokKind::RBrace) {
            let pat = self.parse_pattern()?;
            self.expect(&TokKind::FatArrow, "`=>` in a match arm")?;
            let body = if self.at(&TokKind::LBrace) {
                let block = self.parse_block()?;
                Expr {
                    span: block.span,
                    kind: ExprKind::Block(block),
                }
            } else {
                self.parse_expr()?
            };
            self.eat(&TokKind::Comma);
            arms.push(MatchArm { pat, body });
        }
        self.expect(&TokKind::RBrace, "`}` to close the match")?;
        Ok(Expr {
            kind: ExprKind::Match {
                scrutinee: Box::new(scrutinee),
                arms,
            },
            span,
        })
    }

    fn parse_while(&mut self, span: Span) -> Result<Expr, Diag> {
        if self.at_ident("let") {
            self.bump();
            let pat = self.parse_pattern()?;
            self.expect(&TokKind::Eq, "`=` in `while let`")?;
            let value = self.parse_expr_no_struct()?;
            let body = self.parse_block()?;
            return Ok(Expr {
                kind: ExprKind::WhileLet {
                    pat,
                    value: Box::new(value),
                    body,
                },
                span,
            });
        }
        let test = self.parse_expr_no_struct()?;
        let body = self.parse_block()?;
        Ok(Expr {
            kind: ExprKind::While {
                test: Box::new(test),
                body,
            },
            span,
        })
    }

    fn parse_for(&mut self, span: Span) -> Result<Expr, Diag> {
        let pat = self.parse_pattern()?;
        if !self.at_ident("in") {
            return Err(Diag::syntax(self.span(), "expected `in` after the pattern"));
        }
        self.bump();
        let iterable = self.parse_expr_no_struct()?;
        let body = self.parse_block()?;
        Ok(Expr {
            kind: ExprKind::For {
                pat,
                iterable: Box::new(iterable),
                body,
            },
            span,
        })
    }

    fn parse_path_expr(
        &mut self,
        path: Vec<Ident>,
        span: Span,
        allow_struct: bool,
    ) -> Result<Expr, Diag> {
        if allow_struct && self.at(&TokKind::LBrace) {
            self.bump();
            let mut fields = Vec::new();
            while !self.at(&TokKind::RBrace) {
                let name = self.expect_ident("a field name")?;
                let value = if self.eat(&TokKind::Colon) {
                    Some(self.parse_expr()?)
                } else {
                    None
                };
                fields.push((name, value));
                if !self.eat(&TokKind::Comma) {
                    break;
                }
            }
            self.expect(&TokKind::RBrace, "`}` to close the struct literal")?;
            return Ok(Expr {
                kind: ExprKind::StructLit { path, fields },
                span,
            });
        }
        Ok(Expr {
            kind: ExprKind::Path(path),
            span,
        })
    }
}

/// Reports whether a suffix names an admitted integer type; used by
/// the checker when literal types resolve.
#[must_use]
pub fn suffix_of(suffix: Option<IntSuffix>) -> Option<&'static str> {
    match suffix {
        Some(IntSuffix::I64) => Some("i64"),
        Some(IntSuffix::Usize) => Some("usize"),
        None => None,
    }
}
