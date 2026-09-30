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

    fn check_path_pattern(
        &mut self,
        path: &[ast::Ident],
        wanted: &HostTy,
        span: Span,
    ) -> Result<HirPatKind, Diag> {
        if path.len() == 1 {
            let name = &path[0].name;
            if name == "_" {
                return Ok(HirPatKind::Wild);
            }
            if matches!(name.as_str(), "true" | "false") {
                return Ok(HirPatKind::Bool(name == "true"));
            }
            if let Some(resolved) = builtin_unit(name) {
                expect_unit_pattern(&resolved, wanted, span)?;
                return Ok(HirPatKind::UnitPath(resolved));
            }
            if let Some(resolved) = self.variant_by_name(name, wanted, span)? {
                let (resolved, payload) = self.unit_variant(resolved, span)?;
                if payload > 0 {
                    return Err(Diag::type_error(
                        span,
                        format!("variant `{name}` carries a payload"),
                    ));
                }
                return Ok(HirPatKind::UnitPath(resolved));
            }
            if let Some(item) = self.item_env.get(name).copied()
                && let ItemKind::Struct(fields) = &self.sema.items[item.0 as usize].kind
                && fields.is_empty()
            {
                return Ok(HirPatKind::UnitPath(Resolved::UnitStruct(item)));
            }
            let binding = self.fresh_bind(name, wanted.clone(), false);
            self.ctx_mut()
                .scopes
                .last_mut()
                .expect("the pattern scope")
                .insert(name.clone(), binding);
            return Ok(HirPatKind::Bind(binding));
        }
        if path.len() == 2 {
            let resolved = self.variant_path(path)?;
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
            self.tuple_struct_or_variant(path, wanted, span)?
        } else {
            self.variant_path(path)?
        };
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
            self.tuple_struct_or_variant(path, wanted, span)?
        } else {
            self.variant_path(path)?
        };
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
            let want = in_mode(bind_ref, types[position].clone());
            let pat = match sub {
                Some(sub) => self.check_pattern(sub, &want)?,
                None => self.shorthand_pattern(&field.name, &want, field.span),
            };
            checked[position] = Some(pat);
        }
        Ok(HirPatKind::StructPath(resolved, checked))
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

    fn variant_by_name(
        &self,
        name: &str,
        wanted: &HostTy,
        span: Span,
    ) -> Result<Option<Resolved>, Diag> {
        let HostTy::Enum(id) = wanted else {
            return Ok(None);
        };
        let ItemKind::Enum(variants) = &self.sema.items[id.0 as usize].kind else {
            return Ok(None);
        };
        variants
            .iter()
            .position(|(variant, _)| variant == name)
            .map(|index| Ok(Resolved::Variant(*id, table_index(index, span)?)))
            .transpose()
    }

    fn tuple_struct_or_variant(
        &mut self,
        path: &[ast::Ident],
        wanted: &HostTy,
        span: Span,
    ) -> Result<Resolved, Diag> {
        let name = &path[0].name;
        if let Some(resolved) = self.variant_by_name(name, wanted, span)? {
            return Ok(resolved);
        }
        if let Some(item) = self.item_env.get(name).copied() {
            match &self.sema.items[item.0 as usize].kind {
                ItemKind::Struct(_) => return Ok(Resolved::TupleStruct(item)),
                ItemKind::Enum(variants) => {
                    if variants.len() == 1 {
                        return Ok(Resolved::Variant(item, 0));
                    }
                }
                ItemKind::Alias(_) => {}
            }
        }
        Err(Diag::type_error(
            span,
            format!("`{name}` does not name a constructor of this type"),
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

    /// Enforces the `match` exhaustiveness obligation.
    pub(crate) fn check_exhaustive(
        &self,
        arms: &[(HirPat, HirExpr)],
        wanted: &HostTy,
        span: Span,
    ) -> Result<(), Diag> {
        for (pat, _) in arms {
            if matches!(pat.kind, HirPatKind::Wild | HirPatKind::Bind(_)) {
                return Ok(());
            }
        }
        let covered_bool = |value: bool| {
            arms.iter()
                .any(|(pat, _)| matches!(pat.kind, HirPatKind::Bool(b) if b == value))
        };
        let covered_ctor = |op: CtorOp| {
            arms.iter().any(|(pat, _)| match &pat.kind {
                HirPatKind::UnitPath(Resolved::Ctor(found))
                | HirPatKind::TuplePath(Resolved::Ctor(found), _) => *found == op,
                _ => false,
            })
        };
        let covered_variant = |id: crate::host::hir::ItemId, index: u32| {
            arms.iter().any(|(pat, _)| match &pat.kind {
                HirPatKind::UnitPath(Resolved::Variant(found, at))
                | HirPatKind::TuplePath(Resolved::Variant(found, at), _)
                | HirPatKind::StructPath(Resolved::Variant(found, at), _) => {
                    *found == id && *at == index
                }
                HirPatKind::TuplePath(Resolved::TupleStruct(found), _)
                | HirPatKind::StructPath(
                    Resolved::TupleStruct(found) | Resolved::UnitStruct(found),
                    _,
                )
                | HirPatKind::UnitPath(
                    Resolved::TupleStruct(found) | Resolved::UnitStruct(found),
                ) => *found == id,
                _ => false,
            })
        };
        let exhaustive = match self.peel_refs(wanted) {
            HostTy::Bool => covered_bool(true) && covered_bool(false),
            HostTy::Option(_) => covered_ctor(CtorOp::OptSome) && covered_ctor(CtorOp::OptNone),
            HostTy::Result(..) => covered_ctor(CtorOp::ResOk) && covered_ctor(CtorOp::ResErr),
            HostTy::Enum(id) => match &self.sema.items[id.0 as usize].kind {
                ItemKind::Enum(variants) => (0..)
                    .zip(variants)
                    .all(|(index, _)| covered_variant(id, index)),
                _ => false,
            },
            HostTy::Struct(_) => arms.iter().any(|(pat, _)| {
                matches!(
                    pat.kind,
                    HirPatKind::StructPath(Resolved::UnitStruct(_) | Resolved::TupleStruct(_), _)
                        | HirPatKind::TuplePath(Resolved::TupleStruct(_), _)
                )
            }),
            _ => false,
        };
        if exhaustive {
            return Ok(());
        }
        Err(Diag::type_error(
            span,
            "this `match` is not exhaustive; add the missing patterns or `_`",
        ))
    }
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
