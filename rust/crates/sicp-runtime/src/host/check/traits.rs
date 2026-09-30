// SPDX-License-Identifier: GPL-3.0-only

//! The standard-library traits the admitted operations require of a
//! type (grammar §3, §4): `Clone` for `.clone()` and `vec![v; n]`,
//! `PartialEq` for `==` and `!=`, and `Debug` for `{:?}`. A type
//! implements a trait exactly when Rust's standard library implements
//! it for the structure or the program derives it, and a derive is
//! itself admitted only when every field implements the trait, as
//! `rustc` requires.

use crate::host::ast::{self, DeriveName};
use crate::host::check::Checker;
use crate::host::diag::{Diag, Span};
use crate::host::hir::{HostTy, ItemId, ItemKind};

/// A trait an admitted operation requires.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Trait {
    /// `Clone`.
    Clone,
    /// `PartialEq`.
    PartialEq,
    /// `Debug`.
    Debug,
}

impl Trait {
    fn derive_name(self) -> DeriveName {
        match self {
            Self::Clone => DeriveName::Clone,
            Self::PartialEq => DeriveName::PartialEq,
            Self::Debug => DeriveName::Debug,
        }
    }

    /// The trait's name, for diagnostics.
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Clone => "Clone",
            Self::PartialEq => "PartialEq",
            Self::Debug => "Debug",
        }
    }
}

impl Checker {
    /// Whether `ty` implements `tr`. A user type implements a trait
    /// when it derives it (the derive's fields were checked when the
    /// item was declared). An unresolved inference cell counts as
    /// implementing: [`Checker::require_trait`] records the obligation
    /// and re-checks it once inference settles the cell.
    pub(crate) fn implements(&self, ty: &HostTy, tr: Trait) -> bool {
        match self.deep(ty) {
            HostTy::Infer(_)
            | HostTy::Unit
            | HostTy::Bool
            | HostTy::I64
            | HostTy::Usize
            | HostTy::Str
            | HostTy::String => true,
            HostTy::Struct(id) | HostTy::Enum(id) => self.item_has_derive(id, tr.derive_name()),
            // `&T` is `Clone` for every `T`; `&mut T` never is. `Debug`
            // forwards to the referent. Rust compares references by
            // their referents, but the engines' reference values are
            // places, so a comparison reaching one is outside the
            // subset rather than answered by address.
            HostTy::Ref(mutable, inner) => match tr {
                Trait::Clone => !mutable,
                Trait::PartialEq => false,
                Trait::Debug => self.implements(&inner, tr),
            },
            HostTy::Box(inner)
            | HostTy::Vec(inner)
            | HostTy::Option(inner)
            | HostTy::HashMap(inner)
            | HostTy::Array(inner, _) => self.implements(&inner, tr),
            HostTy::Result(left, right) | HostTy::Tuple(left, right) => {
                self.implements(&left, tr) && self.implements(&right, tr)
            }
            // A function pointer clones, but comparing or printing one
            // observes an address the source does not determine. A
            // range or slice iterator clones too; a boxed closure
            // object and a mutable iterator implement none of the three.
            HostTy::FnPtr(..) | HostTy::Range(_) | HostTy::Iter(_) => tr == Trait::Clone,
            HostTy::DynFn(..) | HostTy::IterMut(_) => false,
            // The remaining iterators clone when their items do (a
            // shared borrow always does); none compares or prints in
            // an admitted way.
            HostTy::IntoIter(item) | HostTy::Enumerate(item) => {
                tr == Trait::Clone && self.implements(&item, Trait::Clone)
            }
            HostTy::Zip(left, right) => {
                tr == Trait::Clone
                    && self.implements(&left, Trait::Clone)
                    && self.implements(&right, Trait::Clone)
            }
        }
    }

    /// Whether `ty` still reaches an unresolved inference cell.
    fn has_open_cell(&self, ty: &HostTy) -> bool {
        match self.deep(ty) {
            HostTy::Infer(_) => true,
            HostTy::Ref(_, inner)
            | HostTy::Box(inner)
            | HostTy::Vec(inner)
            | HostTy::Option(inner)
            | HostTy::HashMap(inner)
            | HostTy::Array(inner, _)
            | HostTy::Range(inner)
            | HostTy::Iter(inner)
            | HostTy::IterMut(inner)
            | HostTy::IntoIter(inner)
            | HostTy::Enumerate(inner) => self.has_open_cell(&inner),
            HostTy::Result(left, right) | HostTy::Tuple(left, right) | HostTy::Zip(left, right) => {
                self.has_open_cell(&left) || self.has_open_cell(&right)
            }
            HostTy::FnPtr(params, ret) | HostTy::DynFn(_, params, ret) => {
                params.iter().any(|param| self.has_open_cell(param)) || self.has_open_cell(&ret)
            }
            HostTy::Unit
            | HostTy::Bool
            | HostTy::I64
            | HostTy::Usize
            | HostTy::Str
            | HostTy::String
            | HostTy::Struct(_)
            | HostTy::Enum(_) => false,
        }
    }

    /// Requires `ty: tr` now, or once inference settles the cells `ty`
    /// still holds.
    ///
    /// # Errors
    /// A type error when `ty` does not implement `tr`.
    pub(crate) fn require_trait(&mut self, ty: &HostTy, tr: Trait, span: Span) -> Result<(), Diag> {
        if !self.implements(ty, tr) {
            return Err(unimplemented_trait(&self.deep(ty), tr, span));
        }
        if self.has_open_cell(ty) {
            self.trait_obligations.push((ty.clone(), tr, span));
        }
        Ok(())
    }

    /// Requires that `recv.clone()` resolves. Method resolution finds
    /// `T::clone` first (auto-deref, then auto-ref), so a `&mut T`
    /// receiver needs `T: Clone`, while a `&T` receiver falls back to
    /// the `Clone` every shared reference has.
    ///
    /// # Errors
    /// A type error when no `clone` applies.
    pub(crate) fn require_clone_receiver(&mut self, recv: &HostTy, span: Span) -> Result<(), Diag> {
        match self.deep(recv) {
            HostTy::Ref(false, _) => Ok(()),
            HostTy::Ref(true, inner) => self.require_trait(&inner, Trait::Clone, span),
            other => self.require_trait(&other, Trait::Clone, span),
        }
    }

    /// Re-checks every deferred obligation now that every body has been
    /// checked.
    ///
    /// # Errors
    /// The first obligation whose type settled on a non-implementor.
    pub(crate) fn discharge_trait_obligations(&self) -> Result<(), Diag> {
        for (ty, tr, span) in &self.trait_obligations {
            if !self.implements(ty, *tr) {
                return Err(unimplemented_trait(&self.deep(ty), *tr, *span));
            }
        }
        Ok(())
    }

    /// Checks that every derive names a trait all of the item's fields
    /// implement, and that `Eq` accompanies `PartialEq`, as `rustc`
    /// requires of a derived impl.
    ///
    /// # Errors
    /// A type error at the item's name.
    pub(crate) fn check_derives(&self, program: &ast::Program) -> Result<(), Diag> {
        for item in &program.items {
            let (name, derives) = match item {
                ast::Item::Struct(struct_item) => (&struct_item.name, &struct_item.derives),
                ast::Item::Enum(enum_item) => (&enum_item.name, &enum_item.derives),
                _ => continue,
            };
            let id = self.item_env[&name.name];
            self.check_item_derives(id, name, derives)?;
        }
        Ok(())
    }

    fn check_item_derives(
        &self,
        id: ItemId,
        name: &ast::Ident,
        derives: &[DeriveName],
    ) -> Result<(), Diag> {
        let fields: Vec<&HostTy> = match &self.sema.items[id.0 as usize].kind {
            ItemKind::Struct(fields) => fields.iter().map(|(_, ty)| ty).collect(),
            ItemKind::Enum(variants) => variants
                .iter()
                .flat_map(|(_, payload)| payload.iter().map(|(_, ty)| ty))
                .collect(),
            ItemKind::Alias(_) => return Ok(()),
        };
        for derive in derives {
            let required = match derive {
                DeriveName::Clone => Some(Trait::Clone),
                DeriveName::PartialEq => Some(Trait::PartialEq),
                DeriveName::Debug => Some(Trait::Debug),
                DeriveName::Eq | DeriveName::Hash => None,
            };
            if *derive == DeriveName::Eq && !derives.contains(&DeriveName::PartialEq) {
                return Err(Diag::type_error(
                    name.span,
                    format!("`{}` derives `Eq` without `PartialEq`", name.name),
                ));
            }
            let Some(tr) = required else {
                continue;
            };
            if let Some(field) = fields.iter().find(|field| !self.implements(field, tr)) {
                return Err(Diag::type_error(
                    name.span,
                    format!(
                        "`{}` cannot derive `{}`: field type `{field:?}` does not implement it",
                        name.name,
                        tr.name()
                    ),
                ));
            }
        }
        Ok(())
    }
}

fn unimplemented_trait(ty: &HostTy, tr: Trait, span: Span) -> Diag {
    let message = match tr {
        Trait::Clone => format!("`{ty:?}` has no admitted `Clone` implementation"),
        Trait::PartialEq => format!("`{ty:?}` does not admit equality"),
        Trait::Debug => format!("`{ty:?}` has no admitted `Debug` rendering"),
    };
    Diag::type_error(span, message)
}
