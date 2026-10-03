// SPDX-License-Identifier: GPL-3.0-only

//! Pattern checking (grammar §4): binding allocation, constructor
//! resolution, match ergonomics for reference scrutinees, and the
//! exhaustiveness obligation every `match` owes.

use crate::host::ast::{self};
use crate::host::check::{Checker, table_index};
use crate::host::diag::{Diag, Span};
use crate::host::hir::{CtorOp, HirExpr, HirPat, HirPatKind, HostTy, ItemKind, Resolved};

impl Checker {
    /// Checks one pattern against the scrutinee type, binding fresh
    /// slots in the current scope.
    pub(crate) fn check_pattern(
        &mut self,
        pat: &ast::Pat,
        wanted: &HostTy,
    ) -> Result<HirPat, Diag> {
        let (shared, peeled) = match self.deep(wanted) {
            HostTy::Ref(shared, inner) => (Some(shared), *inner),
            other => (None, other),
        };
        let bind_ref = shared;
        let span = pat.span;
        let kind = match &pat.kind {
            ast::PatKind::Wild => HirPatKind::Wild,
            ast::PatKind::Bind(name) => {
                if let Some(kind) = self.unit_ident_pattern(&name.name, &peeled, span)? {
                    kind
                } else {
                    let ty = match bind_ref {
                        Some(shared) => HostTy::Ref(shared, Box::new(peeled)),
                        None => peeled,
                    };
                    let binding = self.fresh_bind(&name.name, ty, false);
                    self.ctx_mut()
                        .scopes
                        .last_mut()
                        .expect("the pattern scope")
                        .insert(name.name.clone(), binding);
                    HirPatKind::Bind(binding)
                }
            }
            ast::PatKind::IntLit { text, suffix } => {
                let clean: String = text.chars().filter(|c| *c != '_').collect();
                let raw = clean
                    .parse::<u64>()
                    .map_err(|_| Diag::syntax(span, "integer literal out of range"))?;
                let value = i64::try_from(raw)
                    .map_err(|_| Diag::syntax(span, "integer literal out of range for `i64`"))?;
                match (suffix, &peeled) {
                    (None, HostTy::I64) | (Some(crate::host::lexer::IntSuffix::I64), _) => {
                        HirPatKind::I64(value)
                    }
                    (None, HostTy::Usize) | (Some(crate::host::lexer::IntSuffix::Usize), _) => {
                        HirPatKind::Usize(raw)
                    }
                    _ => {
                        return Err(Diag::type_error(
                            span,
                            "this pattern needs an admitted integer type",
                        ));
                    }
                }
            }
            ast::PatKind::BoolLit(value) => {
                if !matches!(peeled, HostTy::Bool) {
                    return Err(Diag::type_error(span, "this pattern needs `bool`"));
                }
                HirPatKind::Bool(*value)
            }
            ast::PatKind::Tuple(left, right) => {
                let HostTy::Tuple(a, b) = peeled else {
                    return Err(Diag::type_error(span, "this pattern needs a tuple"));
                };
                let left = self.check_pattern(left, &in_mode(bind_ref, *a))?;
                let right = self.check_pattern(right, &in_mode(bind_ref, *b))?;
                HirPatKind::Tuple(Box::new(left), Box::new(right))
            }
            ast::PatKind::Path(path) => self.check_path_pattern(path, &peeled, span)?,
            ast::PatKind::TuplePath(path, subs) => {
                self.check_tuple_pattern(path, subs, &peeled, bind_ref, span)?
            }
            ast::PatKind::Struct { path, fields } => {
                self.check_struct_pattern(path, fields, &peeled, bind_ref, span)?
            }
        };
        Ok(HirPat { kind, span })
    }

    /// A bare identifier in pattern position names the prelude's `None`
    /// or a declared unit struct; any other identifier binds a fresh
    /// name (grammar §4). The parser hands single identifiers over as
    /// bindings, so the resolution lives here.
    fn unit_ident_pattern(
        &self,
        name: &str,
        wanted: &HostTy,
        span: Span,
    ) -> Result<Option<HirPatKind>, Diag> {
        if let Some(resolved) = builtin_unit(name) {
            expect_unit_pattern(&resolved, wanted, span)?;
            return Ok(Some(HirPatKind::UnitPath(resolved)));
        }
        let Some(item) = self.item_env.get(name).copied() else {
            return Ok(None);
        };
        let ItemKind::Struct(fields) = &self.sema.items[item.0 as usize].kind else {
            return Ok(None);
        };
        if !fields.is_empty() {
            return Ok(None);
        }
        if *wanted != HostTy::Struct(item) {
            return Err(Diag::type_error(
                span,
                "this pattern does not match the scrutinee type",
            ));
        }
        Ok(Some(HirPatKind::UnitPath(Resolved::UnitStruct(item))))
    }

    /// A constructor pattern must build the scrutinee's own type.
    fn expect_pattern_type(
        &mut self,
        resolved: &Resolved,
        wanted: &HostTy,
        span: Span,
    ) -> Result<(), Diag> {
        let expected = match resolved {
            Resolved::Variant(id, _) => HostTy::Enum(*id),
            Resolved::TupleStruct(id) | Resolved::UnitStruct(id) => HostTy::Struct(*id),
            _ => return Ok(()),
        };
        match self.deep(wanted) {
            HostTy::Infer(_) => self.unify(&expected, wanted, span).map(|_| ()),
            found if found == expected => Ok(()),
            _ => Err(Diag::type_error(
                span,
                "this pattern does not match the scrutinee type",
            )),
        }
    }

    fn check_path_pattern(
        &mut self,
        path: &[ast::Ident],
        wanted: &HostTy,
        span: Span,
    ) -> Result<HirPatKind, Diag> {
        if path.len() == 2 {
            let resolved = self.variant_path(path)?;
            self.expect_pattern_type(&resolved, wanted, span)?;
            let (_, payload) = self.unit_variant(resolved.clone(), span)?;
            if payload > 0 {
                return Err(Diag::type_error(span, "this variant carries a payload"));
            }
            return Ok(HirPatKind::UnitPath(resolved));
        }
        Err(Diag::type_error(span, "unsupported pattern path"))
    }

    fn check_tuple_pattern(
        &mut self,
        path: &[ast::Ident],
        subs: &[ast::Pat],
        wanted: &HostTy,
        bind_ref: Option<bool>,
        span: Span,
    ) -> Result<HirPatKind, Diag> {
        if path.len() == 1 {
            let name = &path[0].name;
            if let Some(resolved) = builtin_ctor(name) {
                if subs.len() != 1 {
                    return Err(Diag::type_error(
                        span,
                        format!("`{name}` takes one sub-pattern"),
                    ));
                }
                let inner = ctor_payload(resolved, wanted, span)?;
                let sub = self.check_pattern(&subs[0], &in_mode(bind_ref, inner))?;
                return Ok(HirPatKind::TuplePath(Resolved::Ctor(resolved), vec![sub]));
            }
        }
        let resolved = if path.len() == 1 {
            self.tuple_struct(path, span)?
        } else {
            self.variant_path(path)?
        };
        self.expect_pattern_type(&resolved, wanted, span)?;
        let field_types = self.pattern_field_types(&resolved, span)?;
        if subs.len() != field_types.len() {
            return Err(Diag::type_error(
                span,
                "this pattern has the wrong number of sub-patterns",
            ));
        }
        let mut checked = Vec::with_capacity(subs.len());
        for (sub, want) in subs.iter().zip(field_types) {
            checked.push(self.check_pattern(sub, &in_mode(bind_ref, want))?);
        }
        Ok(HirPatKind::TuplePath(resolved, checked))
    }

    fn check_struct_pattern(
        &mut self,
        path: &[ast::Ident],
        fields: &[(ast::Ident, Option<ast::Pat>)],
        wanted: &HostTy,
        bind_ref: Option<bool>,
        span: Span,
    ) -> Result<HirPatKind, Diag> {
        let resolved = if path.len() == 1 {
            self.tuple_struct(path, span)?
        } else {
            self.variant_path(path)?
        };
        self.expect_pattern_type(&resolved, wanted, span)?;
        let names = self.pattern_field_names(&resolved, span)?;
        let types = self.pattern_field_types(&resolved, span)?;
        let mut checked: Vec<Option<HirPat>> = vec![None; names.len()];
        for (field, sub) in fields {
            let Some(position) = names.iter().position(|name| *name == field.name) else {
                return Err(Diag::type_error(
                    field.span,
                    format!("no field `{}` here", field.name),
                ));
            };
            if checked[position].is_some() {
                return Err(Diag::type_error(
                    field.span,
                    format!(
                        "field `{}` is bound more than once in this pattern",
                        field.name
                    ),
                ));
            }
            let want = in_mode(bind_ref, types[position].clone());
            let pat = match sub {
                Some(sub) => self.check_pattern(sub, &want)?,
                None => self.shorthand_pattern(&field.name, &want, field.span),
            };
            checked[position] = Some(pat);
        }
        // The subset has no `..`: a pattern names every field.
        let mut complete = Vec::with_capacity(names.len());
        for (name, pat) in names.iter().zip(checked) {
            let Some(pat) = pat else {
                return Err(Diag::type_error(
                    span,
                    format!("this pattern does not mention field `{name}`; the subset has no `..`"),
                ));
            };
            complete.push(pat);
        }
        Ok(HirPatKind::StructPath(resolved, complete))
    }

    /// A field-shorthand sub-pattern `Path { name }` binds a fresh
    /// `name` to the field, exactly like `Path { name: name }`.
    fn shorthand_pattern(&mut self, name: &str, want: &HostTy, span: Span) -> HirPat {
        let binding = self.fresh_bind(name, want.clone(), false);
        self.ctx_mut()
            .scopes
            .last_mut()
            .expect("the pattern scope")
            .insert(name.to_owned(), binding);
        HirPat {
            kind: HirPatKind::Bind(binding),
            span,
        }
    }

    /// A bare constructor name in a pattern is a struct: enum variants
    /// are not imported, so they are named `Enum::Variant`.
    fn tuple_struct(&self, path: &[ast::Ident], span: Span) -> Result<Resolved, Diag> {
        let name = &path[0].name;
        if let Some(item) = self.item_env.get(name).copied()
            && matches!(self.sema.items[item.0 as usize].kind, ItemKind::Struct(_))
        {
            return Ok(Resolved::TupleStruct(item));
        }
        Err(Diag::type_error(
            span,
            format!("cannot find tuple struct `{name}` in this scope"),
        ))
    }

    fn variant_path(&self, path: &[ast::Ident]) -> Result<Resolved, Diag> {
        let (head, tail) = (&path[0].name, &path[1].name);
        if let Some(item) = self.item_env.get(head).copied()
            && let ItemKind::Enum(variants) = &self.sema.items[item.0 as usize].kind
            && let Some(index) = variants.iter().position(|(name, _)| name == tail)
        {
            return Ok(Resolved::Variant(item, table_index(index, path[1].span)?));
        }
        Err(Diag::type_error(
            path[1].span,
            format!("`{head}::{tail}` does not name a variant"),
        ))
    }

    fn unit_variant(&self, resolved: Resolved, span: Span) -> Result<(Resolved, usize), Diag> {
        match resolved {
            Resolved::Variant(id, index) => match &self.sema.items[id.0 as usize].kind {
                ItemKind::Enum(variants) => Ok((
                    Resolved::Variant(id, index),
                    variants[index as usize].1.len(),
                )),
                _ => Err(Diag::type_error(span, "not a variant")),
            },
            Resolved::TupleStruct(id) => match &self.sema.items[id.0 as usize].kind {
                ItemKind::Struct(fields) => Ok((Resolved::TupleStruct(id), fields.len())),
                _ => Err(Diag::type_error(span, "not a constructor")),
            },
            other => Ok((other, 0)),
        }
    }

    fn pattern_field_names(&self, resolved: &Resolved, span: Span) -> Result<Vec<String>, Diag> {
        match resolved {
            Resolved::Variant(id, index) => match &self.sema.items[id.0 as usize].kind {
                ItemKind::Enum(variants) => Ok(variants[*index as usize]
                    .1
                    .iter()
                    .map(|(name, _)| name.clone())
                    .collect()),
                _ => Err(Diag::type_error(span, "not a variant")),
            },
            Resolved::TupleStruct(id) | Resolved::UnitStruct(id) => {
                match &self.sema.items[id.0 as usize].kind {
                    ItemKind::Struct(fields) => {
                        Ok(fields.iter().map(|(name, _)| name.clone()).collect())
                    }
                    _ => Err(Diag::type_error(span, "not a struct")),
                }
            }
            _ => Ok(Vec::new()),
        }
    }

    fn pattern_field_types(&self, resolved: &Resolved, span: Span) -> Result<Vec<HostTy>, Diag> {
        match resolved {
            Resolved::Variant(id, index) => match &self.sema.items[id.0 as usize].kind {
                ItemKind::Enum(variants) => Ok(variants[*index as usize]
                    .1
                    .iter()
                    .map(|(_, ty)| ty.clone())
                    .collect()),
                _ => Err(Diag::type_error(span, "not a variant")),
            },
            Resolved::TupleStruct(id) | Resolved::UnitStruct(id) => {
                match &self.sema.items[id.0 as usize].kind {
                    ItemKind::Struct(fields) => {
                        Ok(fields.iter().map(|(_, ty)| ty.clone()).collect())
                    }
                    _ => Err(Diag::type_error(span, "not a struct")),
                }
            }
            _ => Ok(Vec::new()),
        }
    }

    /// Enforces the `match` exhaustiveness obligation: every value of
    /// the scrutinee type must reach an arm, judged through nested
    /// patterns (`Some(true)` covers only `Some(true)`).
    pub(crate) fn check_exhaustive(
        &self,
        arms: &[(HirPat, HirExpr)],
        wanted: &HostTy,
        span: Span,
    ) -> Result<(), Diag> {
        let rows: Vec<Vec<Cell<'_>>> = arms.iter().map(|(pat, _)| vec![cell(pat)]).collect();
        if self.covers(&rows, std::slice::from_ref(wanted)) {
            return Ok(());
        }
        Err(Diag::type_error(
            span,
            "this `match` is not exhaustive; add the missing patterns or `_`",
        ))
    }

    /// Whether the pattern rows, one cell per column type, match every
    /// value of those types. A finite type is covered when each of its
    /// constructors is, after specializing the rows to that constructor
    /// and expanding its fields into columns; a type with unboundedly
    /// many values (integers, strings, collections) is covered only by
    /// the rows that are wildcards in the first column.
    fn covers(&self, rows: &[Vec<Cell<'_>>], types: &[HostTy]) -> bool {
        let Some((first, rest)) = types.split_first() else {
            return !rows.is_empty();
        };
        let Some(ctors) = self.constructors_of(&self.peel_refs(first)) else {
            let defaults: Vec<Vec<Cell<'_>>> = rows
                .iter()
                .filter(|row| row[0].is_none())
                .map(|row| row[1..].to_vec())
                .collect();
            return self.covers(&defaults, rest);
        };
        ctors.iter().all(|ctor| {
            let specialized: Vec<Vec<Cell<'_>>> = rows
                .iter()
                .filter_map(|row| specialize(row, ctor))
                .collect();
            let columns: Vec<HostTy> = ctor.fields.iter().chain(rest).cloned().collect();
            self.covers(&specialized, &columns)
        })
    }

    /// The constructors of a finite scrutinee type with their field
    /// types, or `None` for a type whose values patterns cannot
    /// enumerate.
    fn constructors_of(&self, ty: &HostTy) -> Option<Vec<Ctor>> {
        match ty {
            HostTy::Bool => Some(vec![
                Ctor::new(CtorTag::Bool(true), Vec::new()),
                Ctor::new(CtorTag::Bool(false), Vec::new()),
            ]),
            HostTy::Option(inner) => Some(vec![
                Ctor::new(CtorTag::Builtin(CtorOp::OptSome), vec![(**inner).clone()]),
                Ctor::new(CtorTag::Builtin(CtorOp::OptNone), Vec::new()),
            ]),
            HostTy::Result(ok, err) => Some(vec![
                Ctor::new(CtorTag::Builtin(CtorOp::ResOk), vec![(**ok).clone()]),
                Ctor::new(CtorTag::Builtin(CtorOp::ResErr), vec![(**err).clone()]),
            ]),
            HostTy::Tuple(left, right) => Some(vec![Ctor::new(
                CtorTag::Tuple,
                vec![(**left).clone(), (**right).clone()],
            )]),
            HostTy::Struct(id) => match &self.sema.items[id.0 as usize].kind {
                ItemKind::Struct(fields) => Some(vec![Ctor::new(
                    CtorTag::Struct(*id),
                    fields.iter().map(|(_, ty)| ty.clone()).collect(),
                )]),
                _ => None,
            },
            HostTy::Enum(id) => match &self.sema.items[id.0 as usize].kind {
                ItemKind::Enum(variants) => Some(
                    (0..)
                        .zip(variants)
                        .map(|(index, (_, payload))| {
                            Ctor::new(
                                CtorTag::Variant(*id, index),
                                payload.iter().map(|(_, ty)| ty.clone()).collect(),
                            )
                        })
                        .collect(),
                ),
                _ => None,
            },
            _ => None,
        }
    }
}

/// One cell of the pattern matrix: a constructor or literal pattern,
/// or `None` for a wildcard (`_` or a binding).
type Cell<'a> = Option<&'a HirPat>;

fn cell(pat: &HirPat) -> Cell<'_> {
    match pat.kind {
        HirPatKind::Wild | HirPatKind::Bind(_) => None,
        _ => Some(pat),
    }
}

/// Which constructor of its type a pattern or a type case names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CtorTag {
    Bool(bool),
    Builtin(CtorOp),
    Variant(crate::host::hir::ItemId, u32),
    Struct(crate::host::hir::ItemId),
    Tuple,
}

/// A constructor of a finite type and the types of its fields.
struct Ctor {
    tag: CtorTag,
    fields: Vec<HostTy>,
}

impl Ctor {
    fn new(tag: CtorTag, fields: Vec<HostTy>) -> Self {
        Self { tag, fields }
    }
}

/// The constructor a pattern names and its sub-pattern cells, or
/// `None` for a literal pattern, which no finite type has.
fn head_of(pat: &HirPat) -> Option<(CtorTag, Vec<Cell<'_>>)> {
    match &pat.kind {
        HirPatKind::Bool(value) => Some((CtorTag::Bool(*value), Vec::new())),
        HirPatKind::Tuple(left, right) => Some((CtorTag::Tuple, vec![cell(left), cell(right)])),
        HirPatKind::UnitPath(resolved) => Some((tag_of(resolved)?, Vec::new())),
        HirPatKind::TuplePath(resolved, subs) | HirPatKind::StructPath(resolved, subs) => {
            Some((tag_of(resolved)?, subs.iter().map(cell).collect()))
        }
        HirPatKind::Wild | HirPatKind::Bind(_) | HirPatKind::I64(_) | HirPatKind::Usize(_) => None,
    }
}

fn tag_of(resolved: &Resolved) -> Option<CtorTag> {
    match resolved {
        Resolved::Ctor(op) => Some(CtorTag::Builtin(*op)),
        Resolved::Variant(id, index) => Some(CtorTag::Variant(*id, *index)),
        Resolved::UnitStruct(id) | Resolved::TupleStruct(id) => Some(CtorTag::Struct(*id)),
        _ => None,
    }
}

/// The row that remains when the first column is known to hold `ctor`:
/// its sub-patterns (or wildcards) replace the first cell, and a row
/// naming another constructor drops out.
fn specialize<'a>(row: &[Cell<'a>], ctor: &Ctor) -> Option<Vec<Cell<'a>>> {
    let (head, rest) = row.split_first()?;
    let mut cells = match head {
        None => vec![None; ctor.fields.len()],
        Some(pat) => {
            let (tag, subs) = head_of(pat)?;
            if tag != ctor.tag {
                return None;
            }
            subs
        }
    };
    cells.extend_from_slice(rest);
    Some(cells)
}

fn builtin_ctor(name: &str) -> Option<CtorOp> {
    match name {
        "Some" => Some(CtorOp::OptSome),
        "Ok" => Some(CtorOp::ResOk),
        "Err" => Some(CtorOp::ResErr),
        _ => None,
    }
}

fn builtin_unit(name: &str) -> Option<Resolved> {
    (name == "None").then_some(Resolved::Ctor(CtorOp::OptNone))
}

fn expect_unit_pattern(resolved: &Resolved, wanted: &HostTy, span: Span) -> Result<(), Diag> {
    match (resolved, wanted) {
        (Resolved::Ctor(CtorOp::OptNone), HostTy::Option(_)) => Ok(()),
        _ => Err(Diag::type_error(
            span,
            "this pattern does not match the scrutinee type",
        )),
    }
}

fn ctor_payload(resolved: CtorOp, wanted: &HostTy, span: Span) -> Result<HostTy, Diag> {
    match (resolved, wanted) {
        (CtorOp::OptSome, HostTy::Option(inner)) => Ok((**inner).clone()),
        (CtorOp::ResOk, HostTy::Result(ok, _)) => Ok((**ok).clone()),
        (CtorOp::ResErr, HostTy::Result(_, err)) => Ok((**err).clone()),
        _ => Err(Diag::type_error(
            span,
            "this pattern does not match the scrutinee type",
        )),
    }
}

/// The type a sub-pattern sees under the current default binding mode:
/// beneath a reference scrutinee (`Some(mutable)`), each part is
/// matched as a reference to that part, as Rust's match ergonomics do.
fn in_mode(bind_ref: Option<bool>, part: HostTy) -> HostTy {
    match bind_ref {
        Some(mutable) => HostTy::Ref(mutable, Box::new(part)),
        None => part,
    }
}
