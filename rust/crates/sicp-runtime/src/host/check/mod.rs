// SPDX-License-Identifier: GPL-3.0-only

//! The subset checker (grammar §1, §3, §5, §6): resolution, typing,
//! move and borrow checking, and closure-capture classification over
//! the located surface tree. It answers the typed [`Sema`] program the
//! teaching engines share, or a [`Diag`] in exactly one of the
//! rejection classes. Checking completes before any effect: engines
//! receive a program only after [`check_program`] succeeds.

use std::collections::HashMap;

use crate::host::ast::{self, DeriveName};
use crate::host::diag::{Diag, Span};
use crate::host::hir::{
    BindId, BindingInfo, CaptureMode, ClosureKind, FunDef, FunId, HirBlock, HirClosure, HirExpr,
    HirExprKind, HirPat, HirPatKind, HirStmt, HostTy, ItemDef, ItemId, ItemKind, Place, Proj, Sema,
};

mod expr;
mod format;
mod pat;
#[cfg(test)]
mod tests;
mod traits;

use traits::Trait;

/// A checked program: the typed item, function, and binding tables
/// every engine executes.
#[derive(Debug, Clone)]
pub struct CheckedProgram {
    /// The typed program.
    pub sema: Sema,
}

impl CheckedProgram {
    /// The typed program.
    #[must_use]
    pub fn sema(&self) -> &Sema {
        &self.sema
    }
}

/// Resolves, types, and ownership-checks one surface program.
///
/// # Errors
/// The first [`Diag`]: [`DiagKind::Type`], [`DiagKind::Ownership`],
/// or [`DiagKind::Unsupported`]. The parser already excluded syntax
/// errors, so [`DiagKind::Syntax`] cannot occur here.
pub fn check_program(program: &ast::Program) -> Result<CheckedProgram, Diag> {
    let mut checker = Checker::new();
    checker.collect_items(program)?;
    checker.resolve_definitions(program)?;
    checker.check_derives(program)?;
    checker.check_recursion()?;
    checker.check_signatures(program)?;
    checker.check_all_bodies(program)?;
    checker.discharge_obligations()?;
    checker.default_never_cells();
    let cells = checker.cells;
    let mut sema = checker.sema;
    resolve_infer_cells(&mut sema, &cells)?;
    Ok(CheckedProgram { sema })
}

/// Narrows a table position to the `u32` width of the typed program's
/// item, variant, and field indices; a program whose tables outgrow it
/// is outside the subset.
pub(crate) fn table_index(index: usize, span: Span) -> Result<u32, Diag> {
    u32::try_from(index)
        .map_err(|_| Diag::unsupported(span, "the program's tables exceed `u32::MAX` entries"))
}

impl Default for FnCtx {
    fn default() -> Self {
        Self {
            scopes: Vec::new(),
            uninit: Vec::new(),
            loans: Vec::new(),
            ret: HostTy::Unit,
            loops: Vec::new(),
            captures: HashMap::new(),
            uses: HashMap::new(),
            here: Span::default(),
            slots: 0,
        }
    }
}

/// One outstanding borrow on a binding root (grammar §5).
#[derive(Debug, Clone)]
struct Loan {
    /// The borrowed binding.
    root: BindId,
    /// Whether the borrow is exclusive.
    mutable: bool,
    /// The borrowing reference's name, when the borrow lands in a
    /// binding: the loan ends when that name has no later textual use.
    holder: Option<String>,
    /// The scope depth at creation.
    depth: usize,
}

/// How an expression touches a place (grammar §5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Access {
    /// A `Copy` read.
    Read,
    /// Reading a non-`Copy` value out of the place.
    Move,
    /// Assigning to the place.
    Write,
    /// `&place`.
    BorrowShared,
    /// `&mut place`.
    BorrowMut,
}

/// One enclosing loop of the function being checked.
#[derive(Debug)]
struct LoopCtx {
    /// The type a `break` value carries.
    break_ty: HostTy,
    /// Whether a `break` occurred.
    saw_break: bool,
    /// The bindings moved at each `break`: the code after the loop
    /// starts from one of these states.
    break_moves: Vec<Vec<BindId>>,
    /// The bindings moved at each `continue`: the next iteration
    /// starts from one of these states.
    continue_moves: Vec<Vec<BindId>>,
}

impl LoopCtx {
    fn new(break_ty: HostTy) -> Self {
        Self {
            break_ty,
            saw_break: false,
            break_moves: Vec::new(),
            continue_moves: Vec::new(),
        }
    }
}

/// The per-function checking state.
#[derive(Debug)]
struct FnCtx {
    /// The lexical scopes, innermost last.
    scopes: Vec<HashMap<String, BindId>>,
    /// Bindings moved out of, until reassigned.
    uninit: Vec<BindId>,
    /// The outstanding loans.
    loans: Vec<Loan>,
    /// The enclosing function's return type.
    ret: HostTy,
    /// The enclosing loops, innermost last.
    loops: Vec<LoopCtx>,
    /// Captures recorded for the closure being checked.
    captures: HashMap<BindId, (CaptureMode, bool)>,
    /// Later textual uses per name, for loan liveness.
    uses: HashMap<String, Vec<Span>>,
    /// The span currently being checked.
    here: Span,
    /// The number of slots allocated so far.
    slots: u32,
}

/// The checker's whole state.
struct Checker {
    /// The typed program under construction.
    sema: Sema,
    /// Name to item.
    item_env: HashMap<String, ItemId>,
    /// Name to function.
    fun_env: HashMap<String, FunId>,
    /// Alias bodies, resolved on demand.
    aliases: HashMap<ItemId, ast::Ty>,
    /// Derives recorded per item.
    derives: HashMap<ItemId, Vec<DeriveName>>,
    /// Inference cell contents.
    cells: Vec<Option<HostTy>>,
    /// Cells created for unsuffixed integer literals.
    int_cells: Vec<usize>,
    /// Cells typing diverging expressions (`return`, `break`,
    /// `continue`): Rust's never type, which falls back to `()` when no
    /// context constrains it.
    never_cells: Vec<usize>,
    /// Operands of `*`, `/`, `%` whose type was an unresolved cell when
    /// checked: each must resolve to `i64`.
    i64_obligations: Vec<(HostTy, Span)>,
    /// Operands of `==`/`!=`, receivers of `.clone()`, and elements of
    /// `vec![v; n]` whose type still held an unresolved cell when
    /// checked: each must settle on a type implementing its trait.
    trait_obligations: Vec<(HostTy, Trait, Span)>,
    /// Each top-level function's declared parameter types, by
    /// [`FunId`]: callers check against the signature, which is known
    /// before any body (including a recursive caller's own) is checked.
    fun_params: Vec<Vec<HostTy>>,
    /// The import rename for `HashMap`, when present.
    map_alias: Option<String>,
    /// The function state stack: the outer function plus closures.
    ctxs: Vec<FnCtx>,
    /// The next binding index.
    next_bind: u32,
    /// The next function index.
    next_fun: u32,
    /// The top-level function the current closure chain lives in.
    owner_fun: FunId,
}

impl Checker {
    fn new() -> Self {
        Self {
            sema: Sema {
                items: Vec::new(),
                funs: Vec::new(),
                main: FunId(0),
                map_alias: None,
                bindings: Vec::new(),
            },
            item_env: HashMap::new(),
            fun_env: HashMap::new(),
            aliases: HashMap::new(),
            derives: HashMap::new(),
            cells: Vec::new(),
            int_cells: Vec::new(),
            never_cells: Vec::new(),
            i64_obligations: Vec::new(),
            trait_obligations: Vec::new(),
            fun_params: Vec::new(),
            map_alias: None,
            ctxs: Vec::new(),
            next_bind: 0,
            next_fun: 0,
            owner_fun: FunId(0),
        }
    }

    fn ctx(&self) -> &FnCtx {
        self.ctxs.last().expect("a function context is active")
    }

    fn ctx_mut(&mut self) -> &mut FnCtx {
        self.ctxs.last_mut().expect("a function context is active")
    }

    fn fresh_bind(&mut self, name: &str, ty: HostTy, mutable: bool) -> BindId {
        let id = BindId(self.next_bind);
        self.next_bind += 1;
        self.sema.bindings.push(BindingInfo {
            name: name.to_owned(),
            ty,
            mutable,
        });
        let ctx = self.ctx_mut();
        ctx.slots += 1;
        id
    }

    fn fresh_cell(&mut self, is_int: bool) -> HostTy {
        let cell = HostTy::Infer(self.cells.len());
        self.cells.push(None);
        if is_int {
            self.int_cells.push(self.cells.len() - 1);
        }
        cell
    }

    /// A cell for a diverging expression's type (see `never_cells`).
    fn fresh_never_cell(&mut self) -> HostTy {
        let cell = self.fresh_cell(false);
        if let HostTy::Infer(index) = cell {
            self.never_cells.push(index);
        }
        cell
    }

    /// Applies the never-type fallback: a diverging expression's type
    /// that no context constrained becomes `()`. Integer-literal cells
    /// keep their own inference obligation.
    fn default_never_cells(&mut self) {
        for index in std::mem::take(&mut self.never_cells) {
            if let HostTy::Infer(root) = self.deep(&HostTy::Infer(index))
                && !self.int_cells.contains(&root)
            {
                self.cells[root] = Some(HostTy::Unit);
            }
        }
    }

    fn collect_items(&mut self, program: &ast::Program) -> Result<(), Diag> {
        for item in &program.items {
            self.collect_item(item)?;
        }
        let main = self
            .fun_env
            .get("main")
            .copied()
            .ok_or_else(|| Diag::type_error(Span::new(1, 1, 1), "the program has no `main`"))?;
        self.sema.main = main;
        self.sema.map_alias = self.map_alias.clone();
        Ok(())
    }

    fn collect_item(&mut self, item: &ast::Item) -> Result<(), Diag> {
        match item {
            ast::Item::Use(use_item) => {
                if self.map_alias.is_some() {
                    return Err(Diag::type_error(
                        use_item.local.span,
                        "only one `HashMap` import is admitted",
                    ));
                }
                self.map_alias = Some(use_item.local.name.clone());
                Ok(())
            }
            ast::Item::Struct(struct_item) => {
                let id = self.declare_item(&struct_item.name, ItemKind::Struct(Vec::new()))?;
                self.derives.insert(id, struct_item.derives.clone());
                Ok(())
            }
            ast::Item::Enum(enum_item) => {
                let id = self.declare_item(&enum_item.name, ItemKind::Enum(Vec::new()))?;
                self.derives.insert(id, enum_item.derives.clone());
                Ok(())
            }
            ast::Item::TypeAlias(alias) => {
                let id = self.declare_item(&alias.name, ItemKind::Alias(HostTy::Unit))?;
                self.aliases.insert(id, alias.ty.clone());
                Ok(())
            }
            ast::Item::Fn(fun) => {
                if self.fun_env.contains_key(&fun.name.name) {
                    return Err(Diag::type_error(
                        fun.name.span,
                        format!("duplicate function `{}`", fun.name.name),
                    ));
                }
                if self.item_env.contains_key(&fun.name.name) {
                    return Err(Diag::type_error(
                        fun.name.span,
                        format!("`{}` is already an item", fun.name.name),
                    ));
                }
                let id = FunId(self.next_fun);
                self.next_fun += 1;
                self.fun_env.insert(fun.name.name.clone(), id);
                self.fun_params.push(Vec::new());
                self.sema.funs.push(FunDef {
                    id,
                    name: fun.name.name.clone(),
                    params: Vec::new(),
                    ret: HostTy::Unit,
                    bind_base: 0,
                    frame_slots: 0,
                    body: HirBlock {
                        stmts: Vec::new(),
                        tail: None,
                    },
                });
                Ok(())
            }
        }
    }

    fn declare_item(&mut self, name: &ast::Ident, kind: ItemKind) -> Result<ItemId, Diag> {
        if self.item_env.contains_key(&name.name) || self.fun_env.contains_key(&name.name) {
            return Err(Diag::type_error(
                name.span,
                format!("duplicate item `{}`", name.name),
            ));
        }
        let id = ItemId(table_index(self.sema.items.len(), name.span)?);
        self.item_env.insert(name.name.clone(), id);
        self.sema.items.push(ItemDef {
            id,
            name: name.name.clone(),
            kind,
        });
        Ok(id)
    }

    fn resolve_definitions(&mut self, program: &ast::Program) -> Result<(), Diag> {
        for item in &program.items {
            match item {
                ast::Item::Struct(struct_item) => {
                    let id = self.item_env[&struct_item.name.name];
                    let mut fields = Vec::with_capacity(struct_item.fields.len());
                    for field in &struct_item.fields {
                        let ty =
                            self.resolve_ty_in(&field.ty, &mut Vec::new(), Elision::Missing)?;
                        fields.push((field.name.name.clone(), ty));
                    }
                    if fields.is_empty() {
                        // A tuple struct names its fields `0`, `1`, ...
                        for (index, ty) in struct_item.tuple.iter().enumerate() {
                            let ty = self.resolve_ty_in(ty, &mut Vec::new(), Elision::Missing)?;
                            fields.push((index.to_string(), ty));
                        }
                    }
                    self.sema.items[id.0 as usize].kind = ItemKind::Struct(fields);
                }
                ast::Item::Enum(enum_item) => {
                    let id = self.item_env[&enum_item.name.name];
                    let mut variants = Vec::with_capacity(enum_item.variants.len());
                    for variant in &enum_item.variants {
                        let mut payload: Vec<(String, HostTy)> = Vec::new();
                        for (index, ty) in variant.tuple.iter().enumerate() {
                            let ty = self.resolve_ty_in(ty, &mut Vec::new(), Elision::Missing)?;
                            payload.push((index.to_string(), ty));
                        }
                        for field in &variant.fields {
                            let ty =
                                self.resolve_ty_in(&field.ty, &mut Vec::new(), Elision::Missing)?;
                            payload.push((field.name.name.clone(), ty));
                        }
                        variants.push((variant.name.name.clone(), payload));
                    }
                    self.sema.items[id.0 as usize].kind = ItemKind::Enum(variants);
                }
                ast::Item::TypeAlias(alias) => {
                    let id = self.item_env[&alias.name.name];
                    let ty = self.resolve_ty_in(&alias.ty, &mut Vec::new(), Elision::Missing)?;
                    self.sema.items[id.0 as usize].kind = ItemKind::Alias(ty);
                }
                ast::Item::Use(_) | ast::Item::Fn(_) => {}
            }
        }
        Ok(())
    }

    fn check_recursion(&self) -> Result<(), Diag> {
        for item in &self.sema.items {
            let span = Span::new(1, 1, 1);
            let mut stack = vec![item.id];
            self.walk_direct_deps(item.id, &mut stack, span)?;
        }
        Ok(())
    }

    fn walk_direct_deps(
        &self,
        id: ItemId,
        stack: &mut Vec<ItemId>,
        span: Span,
    ) -> Result<(), Diag> {
        let deps: Vec<HostTy> = match &self.sema.items[id.0 as usize].kind {
            ItemKind::Struct(fields) => fields.iter().map(|(_, ty)| ty.clone()).collect(),
            ItemKind::Enum(variants) => variants
                .iter()
                .flat_map(|(_, payload)| payload.iter().map(|(_, ty)| ty.clone()))
                .collect(),
            ItemKind::Alias(ty) => vec![ty.clone()],
        };
        for dep in deps {
            self.walk_ty_deps(&dep, stack, span)?;
        }
        Ok(())
    }

    fn walk_ty_deps(&self, ty: &HostTy, stack: &mut Vec<ItemId>, span: Span) -> Result<(), Diag> {
        match ty {
            HostTy::Struct(id) | HostTy::Enum(id) => {
                if stack.contains(id) {
                    return Err(Diag::type_error(
                        span,
                        "a recursive type needs an admitted indirection \
                         (`Box` or `Vec`), not a direct field",
                    ));
                }
                stack.push(*id);
                self.walk_direct_deps(*id, stack, span)?;
                stack.pop();
            }
            HostTy::Tuple(left, right) => {
                self.walk_ty_deps(left, stack, span)?;
                self.walk_ty_deps(right, stack, span)?;
            }
            // Admitted indirections break direct recursion.
            _ => {}
        }
        Ok(())
    }

    fn check_signatures(&mut self, program: &ast::Program) -> Result<(), Diag> {
        for item in &program.items {
            let ast::Item::Fn(fun) = item else {
                continue;
            };
            let id = self.fun_env[&fun.name.name];
            let mut params = Vec::with_capacity(fun.params.len());
            for param in &fun.params {
                let ty = self.resolve_ty(&param.ty, &mut Vec::new())?;
                params.push((param.name.name.clone(), ty));
            }
            // Rust's elision rule: a returned reference borrows from
            // the one reference among the parameters, and from nothing
            // when there are none or several to choose from.
            let inputs: usize = fun.params.iter().map(|param| elided_refs(&param.ty)).sum();
            let ret_elision = if inputs == 1 {
                Elision::Allowed
            } else {
                Elision::Missing
            };
            let ret = match &fun.ret {
                Some(ty) => self.resolve_ty_in(ty, &mut Vec::new(), ret_elision)?,
                None => HostTy::Unit,
            };
            if fun.name.name == "main" {
                if !params.is_empty() {
                    return Err(Diag::type_error(
                        fun.name.span,
                        "`main` takes no parameters",
                    ));
                }
                if ret != HostTy::Unit {
                    return Err(Diag::type_error(fun.name.span, "`main` must return `()`"));
                }
            }
            self.fun_params[id.0 as usize] = params.iter().map(|(_, ty)| ty.clone()).collect();
            let def = &mut self.sema.funs[id.0 as usize];
            def.params = params
                .into_iter()
                .map(|(name, _)| (BindId(0), name))
                .collect();
            def.ret = ret;
        }
        Ok(())
    }

    fn check_all_bodies(&mut self, program: &ast::Program) -> Result<(), Diag> {
        for item in &program.items {
            let ast::Item::Fn(fun) = item else {
                continue;
            };
            let id = self.fun_env[&fun.name.name];
            self.check_one_fn(fun, id)?;
        }
        Ok(())
    }

    fn check_one_fn(&mut self, fun: &ast::FnItem, id: FunId) -> Result<(), Diag> {
        let ret = self.sema.funs[id.0 as usize].ret.clone();
        let uses = collect_uses(&fun.body);
        let bind_base = self.next_bind;
        self.ctxs.push(FnCtx {
            scopes: vec![HashMap::new()],
            uninit: Vec::new(),
            loans: Vec::new(),
            ret,
            loops: Vec::new(),
            captures: HashMap::new(),
            uses,
            here: fun.body.span,
            slots: 0,
        });
        self.owner_fun = id;
        let mut params = Vec::with_capacity(fun.params.len());
        for param in &fun.params {
            let ty = self.resolve_ty(&param.ty, &mut Vec::new())?;
            let binding = self.fresh_bind(&param.name.name, ty.clone(), param.mutable);
            self.ctx_mut()
                .scopes
                .last_mut()
                .expect("the function scope")
                .insert(param.name.name.clone(), binding);
            params.push((binding, param.name.name.clone()));
        }
        let ret = self.ctx().ret.clone();
        let body = self.check_block_body(&fun.body, Some(&ret))?;
        // The frame covers every id allocated while checking the body,
        // including nested closures' ranges: slot addressing stays a
        // contiguous span even when nested scopes interleave ids.
        let slots = self.next_bind - bind_base;
        self.ctxs.pop();
        let def = &mut self.sema.funs[id.0 as usize];
        def.params = params;
        def.bind_base = bind_base;
        def.frame_slots = slots;
        def.body = body;
        Ok(())
    }

    fn check_closure_body(
        &mut self,
        body: &ast::Block,
        expect: Option<&HostTy>,
        param_specs: &[(String, HostTy)],
    ) -> Result<(HirClosure, HostTy), Diag> {
        let uses = collect_uses(body);
        let bind_base = self.next_bind;
        let ret = match expect {
            Some(ty) => ty.clone(),
            None => self.fresh_cell(false),
        };
        self.ctxs.push(FnCtx {
            scopes: vec![HashMap::new()],
            uninit: Vec::new(),
            loans: Vec::new(),
            ret: ret.clone(),
            loops: Vec::new(),
            captures: HashMap::new(),
            uses,
            here: body.span,
            slots: 0,
        });
        let mut params = Vec::with_capacity(param_specs.len());
        for (name, ty) in param_specs {
            let binding = self.fresh_bind(name, ty.clone(), false);
            params.push((binding, ty.clone()));
            self.ctx_mut()
                .scopes
                .last_mut()
                .expect("the closure scope")
                .insert(name.clone(), binding);
        }
        let ret_snapshot = self.ctx().ret.clone();
        let checked = self.check_block_body(body, Some(&ret_snapshot))?;
        // Same span discipline as functions: the closure frame covers
        // its whole allocation range, disjoint by construction from
        // every id allocated before or after it.
        let slots = self.next_bind - bind_base;
        let captures: Vec<crate::host::hir::Capture> = self
            .ctx()
            .captures
            .iter()
            .map(|(binding, (mode, consumed))| crate::host::hir::Capture {
                binding: *binding,
                mode: *mode,
                consumed: *consumed,
            })
            .collect();
        self.ctxs.pop();
        let closure = HirClosure {
            kind: ClosureKind::Fn,
            bind_base,
            frame_slots: slots,
            params,
            body: checked,
            captures,
            ret: ret_snapshot.clone(),
        };
        Ok((closure, ret_snapshot))
    }

    /// Resolves a surface type where a reference's lifetime is
    /// inferred from context (a `let` annotation, a parameter, a
    /// closure parameter).
    fn resolve_ty(&self, ty: &ast::Ty, seen: &mut Vec<String>) -> Result<HostTy, Diag> {
        self.resolve_ty_in(ty, seen, Elision::Allowed)
    }

    /// Resolves a surface type, expanding aliases with cycle
    /// detection. `elision` says whether an elided reference lifetime
    /// has anything to elide to here: named lifetimes are excluded, so
    /// a reference in a field, an alias, or an unmatched return type is
    /// Rust's E0106 rather than a type.
    fn resolve_ty_in(
        &self,
        ty: &ast::Ty,
        seen: &mut Vec<String>,
        elision: Elision,
    ) -> Result<HostTy, Diag> {
        Ok(match &ty.kind {
            ast::TyKind::Unit => HostTy::Unit,
            ast::TyKind::Bool => HostTy::Bool,
            ast::TyKind::I64 => HostTy::I64,
            ast::TyKind::Usize => HostTy::Usize,
            ast::TyKind::Str => {
                require_elision(elision, ty.span)?;
                HostTy::Str
            }
            ast::TyKind::String => HostTy::String,
            ast::TyKind::Ref(mutable, inner) => {
                require_elision(elision, ty.span)?;
                HostTy::Ref(
                    *mutable,
                    Box::new(self.resolve_ty_in(inner, seen, elision)?),
                )
            }
            ast::TyKind::Box(inner) => {
                HostTy::Box(Box::new(self.resolve_ty_in(inner, seen, elision)?))
            }
            ast::TyKind::Vec(inner) => {
                HostTy::Vec(Box::new(self.resolve_ty_in(inner, seen, elision)?))
            }
            ast::TyKind::Option(inner) => {
                HostTy::Option(Box::new(self.resolve_ty_in(inner, seen, elision)?))
            }
            ast::TyKind::Result(ok, err) => HostTy::Result(
                Box::new(self.resolve_ty_in(ok, seen, elision)?),
                Box::new(self.resolve_ty_in(err, seen, elision)?),
            ),
            ast::TyKind::HashMap(value) => {
                HostTy::HashMap(Box::new(self.resolve_ty_in(value, seen, elision)?))
            }
            ast::TyKind::Array(inner, len) => {
                let clean: String = len.chars().filter(|c| *c != '_').collect();
                let count = clean
                    .parse::<u64>()
                    .map_err(|_| Diag::syntax(ty.span, "array lengths are integer literals"))?;
                HostTy::Array(Box::new(self.resolve_ty_in(inner, seen, elision)?), count)
            }
            ast::TyKind::Tuple(left, right) => HostTy::Tuple(
                Box::new(self.resolve_ty_in(left, seen, elision)?),
                Box::new(self.resolve_ty_in(right, seen, elision)?),
            ),
            ast::TyKind::Named(name) => self.resolve_named(name, ty.span, seen)?,
            ast::TyKind::FnPtr(params, ret) => {
                let (resolved, ret) = self.resolve_signature_types(params, ret, seen)?;
                HostTy::FnPtr(resolved, Box::new(ret))
            }
            ast::TyKind::DynClosure(kind, params, ret) => {
                let (resolved, ret) = self.resolve_signature_types(params, ret, seen)?;
                HostTy::DynFn(closure_kind_of(*kind), resolved, Box::new(ret))
            }
        })
    }

    /// The parameter and result types of a function-pointer or boxed
    /// closure type. Such a type is its own elision scope: its
    /// parameters' references are fresh, and its result may borrow only
    /// from exactly one of them.
    fn resolve_signature_types(
        &self,
        params: &[ast::Ty],
        ret: &ast::Ty,
        seen: &mut Vec<String>,
    ) -> Result<(Vec<HostTy>, HostTy), Diag> {
        let mut resolved = Vec::with_capacity(params.len());
        for param in params {
            resolved.push(self.resolve_ty_in(param, seen, Elision::Allowed)?);
        }
        let inputs: usize = params.iter().map(elided_refs).sum();
        let ret_elision = if inputs == 1 {
            Elision::Allowed
        } else {
            Elision::Missing
        };
        let ret = self.resolve_ty_in(ret, seen, ret_elision)?;
        Ok((resolved, ret))
    }

    fn resolve_named(
        &self,
        name: &str,
        span: Span,
        seen: &mut Vec<String>,
    ) -> Result<HostTy, Diag> {
        if let Some(item_id) = self.item_env.get(name).copied() {
            return match &self.sema.items[item_id.0 as usize].kind {
                ItemKind::Struct(_) => Ok(HostTy::Struct(item_id)),
                ItemKind::Enum(_) => Ok(HostTy::Enum(item_id)),
                ItemKind::Alias(_) => {
                    if seen.iter().any(|entry| entry == name) {
                        return Err(Diag::type_error(
                            span,
                            format!("cyclic type alias `{name}`"),
                        ));
                    }
                    let surface =
                        self.aliases.get(&item_id).cloned().ok_or_else(|| {
                            Diag::type_error(span, format!("unknown type `{name}`"))
                        })?;
                    seen.push(name.to_owned());
                    let expanded = self.resolve_ty(&surface, seen);
                    seen.pop();
                    expanded
                }
            };
        }
        if self.map_alias.as_deref() == Some(name) {
            return Err(Diag::type_error(
                span,
                "`HashMap` needs its value type: `HashMap<String, T>`",
            ));
        }
        if matches!(
            name,
            "HashMap" | "Vec" | "Box" | "Option" | "Result" | "dyn"
        ) {
            return Err(Diag::type_error(
                span,
                format!("`{name}` needs its type arguments"),
            ));
        }
        Err(Diag::type_error(span, format!("unknown type `{name}`")))
    }

    /// Follows inference cells, through chains of bound cells, to their
    /// contents.
    fn deep(&self, ty: &HostTy) -> HostTy {
        let mut current = ty.clone();
        while let HostTy::Infer(index) = current {
            match &self.cells[index] {
                Some(bound) => current = bound.clone(),
                None => break,
            }
        }
        current
    }

    /// Follows `&T` and `&mut T` to the referent type: field, index,
    /// and method resolution see through borrows the way the native
    /// compiler's autoref adjustment does. Types only; values keep
    /// their reference wrappers for the engines to resolve.
    fn peel_refs(&self, ty: &HostTy) -> HostTy {
        let mut current = self.deep(ty);
        while let HostTy::Ref(_, inner) = current {
            current = self.deep(&inner);
        }
        current
    }

    /// Unifies two types, resolving inference cells.
    fn unify(&mut self, left: &HostTy, right: &HostTy, span: Span) -> Result<HostTy, Diag> {
        let a = self.deep(left);
        let b = self.deep(right);
        match (&a, &b) {
            (HostTy::Infer(index), _) => self.bind_cell(*index, &b, span),
            (_, HostTy::Infer(index)) => self.bind_cell(*index, &a, span),
            (HostTy::Unit, HostTy::Unit)
            | (HostTy::Bool, HostTy::Bool)
            | (HostTy::I64, HostTy::I64)
            | (HostTy::Usize, HostTy::Usize)
            | (HostTy::Str, HostTy::Str)
            | (HostTy::String, HostTy::String) => Ok(a),
            (HostTy::Ref(m1, t1), HostTy::Ref(m2, t2)) if m1 == m2 => {
                Ok(HostTy::Ref(*m1, Box::new(self.unify(t1, t2, span)?)))
            }
            (HostTy::Box(t1), HostTy::Box(t2)) => {
                Ok(HostTy::Box(Box::new(self.unify(t1, t2, span)?)))
            }
            (HostTy::Vec(t1), HostTy::Vec(t2)) => {
                Ok(HostTy::Vec(Box::new(self.unify(t1, t2, span)?)))
            }
            (HostTy::Option(t1), HostTy::Option(t2)) => {
                Ok(HostTy::Option(Box::new(self.unify(t1, t2, span)?)))
            }
            (HostTy::Result(o1, e1), HostTy::Result(o2, e2)) => Ok(HostTy::Result(
                Box::new(self.unify(o1, o2, span)?),
                Box::new(self.unify(e1, e2, span)?),
            )),
            (HostTy::HashMap(t1), HostTy::HashMap(t2)) => {
                Ok(HostTy::HashMap(Box::new(self.unify(t1, t2, span)?)))
            }
            (HostTy::Array(t1, n1), HostTy::Array(t2, n2)) if n1 == n2 => {
                Ok(HostTy::Array(Box::new(self.unify(t1, t2, span)?), *n1))
            }
            (HostTy::Tuple(l1, r1), HostTy::Tuple(l2, r2)) => Ok(HostTy::Tuple(
                Box::new(self.unify(l1, l2, span)?),
                Box::new(self.unify(r1, r2, span)?),
            )),
            (HostTy::Struct(i1), HostTy::Struct(i2)) if i1 == i2 => Ok(a),
            (HostTy::Enum(i1), HostTy::Enum(i2)) if i1 == i2 => Ok(a),
            (HostTy::FnPtr(p1, r1), HostTy::FnPtr(p2, r2)) if p1.len() == p2.len() => {
                let mut params = Vec::with_capacity(p1.len());
                for (x, y) in p1.iter().zip(p2.iter()) {
                    params.push(self.unify(x, y, span)?);
                }
                Ok(HostTy::FnPtr(params, Box::new(self.unify(r1, r2, span)?)))
            }
            (HostTy::DynFn(k1, p1, r1), HostTy::DynFn(k2, p2, r2)) if p1.len() == p2.len() => {
                if !closure_kind_fits(*k1, *k2) {
                    return Err(Diag::type_error(
                        span,
                        format!("closure kind {k1:?} cannot stand where {k2:?} is required"),
                    ));
                }
                let mut params = Vec::with_capacity(p1.len());
                for (x, y) in p1.iter().zip(p2.iter()) {
                    params.push(self.unify(x, y, span)?);
                }
                Ok(HostTy::DynFn(
                    *k1,
                    params,
                    Box::new(self.unify(r1, r2, span)?),
                ))
            }
            (HostTy::Range(t1), HostTy::Range(t2)) => {
                Ok(HostTy::Range(Box::new(self.unify(t1, t2, span)?)))
            }
            (HostTy::Iter(t1), HostTy::Iter(t2)) => {
                Ok(HostTy::Iter(Box::new(self.unify(t1, t2, span)?)))
            }
            (HostTy::IterMut(t1), HostTy::IterMut(t2)) => {
                Ok(HostTy::IterMut(Box::new(self.unify(t1, t2, span)?)))
            }
            (HostTy::IntoIter(t1), HostTy::IntoIter(t2)) => {
                Ok(HostTy::IntoIter(Box::new(self.unify(t1, t2, span)?)))
            }
            (HostTy::Enumerate(t1), HostTy::Enumerate(t2)) => {
                Ok(HostTy::Enumerate(Box::new(self.unify(t1, t2, span)?)))
            }
            (HostTy::Zip(a1, b1), HostTy::Zip(a2, b2)) => Ok(HostTy::Zip(
                Box::new(self.unify(a1, a2, span)?),
                Box::new(self.unify(b1, b2, span)?),
            )),
            _ => Err(Diag::type_error(
                span,
                format!("mismatched types: expected `{a:?}`, found `{b:?}`"),
            )),
        }
    }

    fn bind_cell(&mut self, index: usize, ty: &HostTy, span: Span) -> Result<HostTy, Diag> {
        if let HostTy::Infer(other) = ty {
            if *other == index {
                return Ok(ty.clone());
            }
            // Two unbound cells meet: an integer-literal cell keeps its
            // integer obligation, so the unconstrained cell points at it.
            if self.int_cells.contains(&index) && !self.int_cells.contains(other) {
                let target = HostTy::Infer(index);
                self.cells[*other] = Some(target.clone());
                return Ok(target);
            }
            self.cells[index] = Some(ty.clone());
            return Ok(ty.clone());
        }
        if type_contains_cell(ty, index, &self.cells) {
            return Err(Diag::type_error(
                span,
                "cannot construct the infinite type this binding would require",
            ));
        }
        if self.int_cells.contains(&index) && !matches!(ty, HostTy::I64 | HostTy::Usize) {
            return Err(Diag::unsupported(
                span,
                "an integer literal must be inferred as `i64` or `usize` \
                 from an admitted context",
            ));
        }
        self.cells[index] = Some(ty.clone());
        Ok(ty.clone())
    }

    /// Looks a name up, recording captures across closure boundaries.
    /// A capture forwards the original binding identity to every
    /// intervening closure, so nested closures chain at run time.
    fn resolve_local(&mut self, name: &str) -> Option<BindId> {
        let owner_depth = self.lookup_depth(name)?;
        let binding = self
            .binding_at(owner_depth, name)
            .expect("the lookup matched a scope");
        let current = self.ctxs.len() - 1;
        for level in owner_depth + 1..=current {
            self.ctxs[level]
                .captures
                .entry(binding)
                .or_insert((CaptureMode::Shared, false));
        }
        Some(binding)
    }

    fn lookup_depth(&self, name: &str) -> Option<usize> {
        for (level, ctx) in self.ctxs.iter().enumerate().rev() {
            for scope in ctx.scopes.iter().rev() {
                if scope.contains_key(name) {
                    return Some(level);
                }
            }
        }
        None
    }

    fn binding_at(&self, level: usize, name: &str) -> Option<BindId> {
        for scope in self.ctxs[level].scopes.iter().rev() {
            if let Some(binding) = scope.get(name) {
                return Some(*binding);
            }
        }
        None
    }

    fn note_capture_use(&mut self, binding: BindId, use_kind: Access) {
        // A capture forwards through every intervening closure, so the
        // refinement applies to every context that recorded it.
        for ctx in self.ctxs.iter_mut().rev() {
            if let Some((mode, consumed)) = ctx.captures.get_mut(&binding) {
                match use_kind {
                    Access::Move => {
                        *mode = CaptureMode::Owned;
                        *consumed = true;
                    }
                    Access::Write | Access::BorrowMut => {
                        if *mode == CaptureMode::Shared {
                            *mode = CaptureMode::Mut;
                        }
                    }
                    Access::Read | Access::BorrowShared => {}
                }
            }
        }
    }

    fn check_access(&mut self, root: BindId, access: Access, span: Span) -> Result<(), Diag> {
        let ty = self.sema.bindings[root.0 as usize].ty.clone();
        let moved = self.ctxs.iter().any(|ctx| ctx.uninit.contains(&root));
        match access {
            Access::Read | Access::Move => {
                if moved {
                    return Err(Diag::ownership(
                        span,
                        format!(
                            "use of moved value `{}`",
                            self.sema.bindings[root.0 as usize].name
                        ),
                    ));
                }
            }
            Access::Write => {
                if !self.sema.bindings[root.0 as usize].mutable {
                    return Err(Diag::ownership(
                        span,
                        format!(
                            "cannot assign to immutable binding `{}`",
                            self.sema.bindings[root.0 as usize].name
                        ),
                    ));
                }
            }
            Access::BorrowShared | Access::BorrowMut => {
                if moved {
                    return Err(Diag::ownership(
                        span,
                        format!(
                            "borrow of moved value `{}`",
                            self.sema.bindings[root.0 as usize].name
                        ),
                    ));
                }
                if access == Access::BorrowMut && !self.sema.bindings[root.0 as usize].mutable {
                    return Err(Diag::ownership(
                        span,
                        format!(
                            "cannot mutably borrow immutable binding `{}`",
                            self.sema.bindings[root.0 as usize].name
                        ),
                    ));
                }
            }
        }
        self.check_loans(root, access, span)?;
        if access == Access::Move && !self.is_copy_ty(&ty) {
            self.ctx_mut().uninit.push(root);
        }
        Ok(())
    }

    /// Whether values of `ty` are `Copy`, judged through the inference
    /// cells settled so far. An integer-literal cell will be an
    /// integer, hence `Copy`; any other unresolved cell is treated as a
    /// value that moves.
    pub(crate) fn is_copy_ty(&self, ty: &HostTy) -> bool {
        match self.deep(ty) {
            HostTy::Infer(index) => self.int_cells.contains(&index),
            HostTy::Option(inner) | HostTy::Array(inner, _) => self.is_copy_ty(&inner),
            HostTy::Tuple(left, right) | HostTy::Result(left, right) => {
                self.is_copy_ty(&left) && self.is_copy_ty(&right)
            }
            other => other.is_copy(),
        }
    }

    fn check_loans(&mut self, root: BindId, access: Access, span: Span) -> Result<(), Diag> {
        let here = self.ctx().here;
        let has_mut = self
            .ctx()
            .loans
            .iter()
            .any(|loan| loan.root == root && loan.mutable && self.loan_live(loan, here));
        let has_shared = self
            .ctx()
            .loans
            .iter()
            .any(|loan| loan.root == root && !loan.mutable && self.loan_live(loan, here));
        if has_mut {
            return Err(Diag::ownership(
                span,
                "cannot use a binding while its exclusive borrow is live",
            ));
        }
        if has_shared && matches!(access, Access::Write | Access::Move | Access::BorrowMut) {
            return Err(Diag::ownership(
                span,
                "cannot mutate a binding while a shared borrow is live",
            ));
        }
        Ok(())
    }

    fn loan_live(&self, loan: &Loan, here: Span) -> bool {
        match &loan.holder {
            Some(name) => self.ctx().uses.get(name).is_none_or(|spans| {
                spans
                    .iter()
                    .any(|s| s.line > here.line || (s.line == here.line && s.col > here.col))
            }),
            None => true,
        }
    }

    fn begin_loan(&mut self, root: BindId, mutable: bool, holder: Option<String>) {
        let depth = self.ctx().scopes.len();
        self.ctx_mut().loans.push(Loan {
            root,
            mutable,
            holder,
            depth,
        });
    }

    fn pop_scope(&mut self) {
        let ctx = self.ctx_mut();
        let depth = ctx.scopes.len();
        ctx.loans.retain(|loan| loan.depth < depth);
        ctx.scopes.pop();
    }

    fn reinit(&mut self, root: BindId) {
        let ctx = self.ctx_mut();
        ctx.uninit.retain(|b| *b != root);
    }
}

/// Whether an elided reference lifetime has anything to elide to at a
/// position of a written type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Elision {
    /// Inferred from context: a `let` annotation, a parameter, or the
    /// result of a signature with exactly one reference parameter.
    Allowed,
    /// Nothing to borrow from: a field, an alias body, or a result with
    /// no single reference parameter.
    Missing,
}

fn require_elision(elision: Elision, span: Span) -> Result<(), Diag> {
    match elision {
        Elision::Allowed => Ok(()),
        Elision::Missing => Err(Diag::type_error(
            span,
            "missing lifetime specifier: this reference has no lifetime to elide to \
             and the subset has no named lifetimes",
        )),
    }
}

/// The elided reference lifetimes a written type contributes to its
/// enclosing signature: one per `&`/`&mut`/`&str`, not counting those
/// inside a function-pointer or boxed-closure type, which form their
/// own scope.
fn elided_refs(ty: &ast::Ty) -> usize {
    match &ty.kind {
        ast::TyKind::Ref(_, inner) => 1 + elided_refs(inner),
        ast::TyKind::Str => 1,
        ast::TyKind::Box(inner)
        | ast::TyKind::Vec(inner)
        | ast::TyKind::Option(inner)
        | ast::TyKind::HashMap(inner)
        | ast::TyKind::Array(inner, _) => elided_refs(inner),
        ast::TyKind::Result(left, right) | ast::TyKind::Tuple(left, right) => {
            elided_refs(left) + elided_refs(right)
        }
        ast::TyKind::Unit
        | ast::TyKind::Bool
        | ast::TyKind::I64
        | ast::TyKind::Usize
        | ast::TyKind::String
        | ast::TyKind::Named(_)
        | ast::TyKind::FnPtr(..)
        | ast::TyKind::DynClosure(..) => 0,
    }
}

/// Whether `ty` reaches one inference cell, directly or through its
/// structure: binding a cell to such a type would build an infinite
/// type, which Rust rejects and the subset must reject before effects.
fn type_contains_cell(ty: &HostTy, index: usize, cells: &[Option<HostTy>]) -> bool {
    match ty {
        HostTy::Infer(found) => {
            *found == index
                || cells
                    .get(*found)
                    .and_then(|cell| cell.as_ref())
                    .is_some_and(|bound| type_contains_cell(bound, index, cells))
        }
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
        | HostTy::Enumerate(inner) => type_contains_cell(inner, index, cells),
        HostTy::Result(ok, err) | HostTy::Tuple(ok, err) | HostTy::Zip(ok, err) => {
            type_contains_cell(ok, index, cells) || type_contains_cell(err, index, cells)
        }
        HostTy::FnPtr(params, ret) | HostTy::DynFn(_, params, ret) => {
            params
                .iter()
                .any(|param| type_contains_cell(param, index, cells))
                || type_contains_cell(ret, index, cells)
        }
        _ => false,
    }
}

/// Whether a closure of kind `actual` may stand where `expected` is
/// required: `Fn` implements `FnMut` and `FnOnce`, `FnMut` implements
/// `FnOnce`.
#[must_use]
pub const fn closure_kind_fits(actual: ClosureKind, expected: ClosureKind) -> bool {
    matches!(
        (actual, expected),
        (ClosureKind::Fn, _)
            | (ClosureKind::FnMut, ClosureKind::FnMut | ClosureKind::FnOnce)
            | (ClosureKind::FnOnce, ClosureKind::FnOnce)
    )
}

fn closure_kind_of(kind: ast::ClosureTrait) -> ClosureKind {
    match kind {
        ast::ClosureTrait::Fn => ClosureKind::Fn,
        ast::ClosureTrait::FnMut => ClosureKind::FnMut,
        ast::ClosureTrait::FnOnce => ClosureKind::FnOnce,
    }
}

/// Collects every value-position identifier use with its location, for
/// loan liveness.
fn collect_uses(body: &ast::Block) -> HashMap<String, Vec<Span>> {
    let mut uses: HashMap<String, Vec<Span>> = HashMap::new();
    collect_block_uses(body, &mut uses);
    uses
}

fn collect_block_uses(block: &ast::Block, uses: &mut HashMap<String, Vec<Span>>) {
    for stmt in &block.stmts {
        match stmt {
            ast::Stmt::Let(let_stmt) => collect_expr_uses(&let_stmt.value, uses),
            ast::Stmt::Expr { expr, .. } => collect_expr_uses(expr, uses),
        }
    }
    if let Some(tail) = &block.tail {
        collect_expr_uses(tail, uses);
    }
}

fn collect_expr_uses(expr: &ast::Expr, uses: &mut HashMap<String, Vec<Span>>) {
    match &expr.kind {
        ast::ExprKind::Path(path) => {
            if let Some(last) = path.last() {
                uses.entry(last.name.clone()).or_default().push(last.span);
            }
        }
        ast::ExprKind::StructLit { fields, .. } => collect_struct_field_uses(fields, uses),
        ast::ExprKind::Field { base: inner, .. }
        | ast::ExprKind::Unary { operand: inner, .. }
        | ast::ExprKind::Try(inner) => collect_expr_uses(inner, uses),
        ast::ExprKind::Index {
            base: left,
            index: right,
        }
        | ast::ExprKind::Binary { left, right, .. }
        | ast::ExprKind::Assign {
            target: left,
            value: right,
            ..
        }
        | ast::ExprKind::Range(left, right)
        | ast::ExprKind::Tuple(left, right)
        | ast::ExprKind::VecRepeat(left, right) => {
            collect_expr_uses(left, uses);
            collect_expr_uses(right, uses);
        }
        ast::ExprKind::Call { callee: head, args }
        | ast::ExprKind::MethodCall {
            receiver: head,
            args,
            ..
        } => {
            collect_expr_uses(head, uses);
            for arg in args {
                collect_expr_uses(arg, uses);
            }
        }
        ast::ExprKind::Array(items)
        | ast::ExprKind::VecList(items)
        | ast::ExprKind::Format { args: items, .. } => {
            for item in items {
                collect_expr_uses(item, uses);
            }
        }
        ast::ExprKind::If {
            test: head,
            then,
            else_branch,
        }
        | ast::ExprKind::IfLet {
            value: head,
            then,
            else_branch,
            ..
        } => {
            collect_expr_uses(head, uses);
            collect_block_uses(then, uses);
            if let Some(branch) = else_branch {
                collect_expr_uses(branch, uses);
            }
        }
        ast::ExprKind::Match { scrutinee, arms } => {
            collect_expr_uses(scrutinee, uses);
            for arm in arms {
                collect_expr_uses(&arm.body, uses);
            }
        }
        ast::ExprKind::Block(block) | ast::ExprKind::Loop(block) => {
            collect_block_uses(block, uses);
        }
        ast::ExprKind::While { test: head, body }
        | ast::ExprKind::WhileLet {
            value: head, body, ..
        }
        | ast::ExprKind::For {
            iterable: head,
            body,
            ..
        } => {
            collect_expr_uses(head, uses);
            collect_block_uses(body, uses);
        }
        ast::ExprKind::Closure { body, .. } => match body {
            ast::ClosureBody::Block(block) => collect_block_uses(block, uses),
            ast::ClosureBody::Expr(inner) => collect_expr_uses(inner, uses),
        },
        ast::ExprKind::Return(value) | ast::ExprKind::Break(value) => {
            if let Some(value) = value {
                collect_expr_uses(value, uses);
            }
        }
        ast::ExprKind::IntLit { .. }
        | ast::ExprKind::StrLit(_)
        | ast::ExprKind::BoolLit(_)
        | ast::ExprKind::UnitLit
        | ast::ExprKind::Continue => {}
    }
}
fn collect_struct_field_uses(
    fields: &[(ast::Ident, Option<ast::Expr>)],
    uses: &mut HashMap<String, Vec<Span>>,
) {
    for (name, value) in fields {
        match value {
            Some(value) => collect_expr_uses(value, uses),
            None => uses.entry(name.name.clone()).or_default().push(name.span),
        }
    }
}

/// Replaces every inference cell in the typed program with its
/// resolved type.
fn resolve_infer_cells(sema: &mut Sema, cells: &[Option<HostTy>]) -> Result<(), Diag> {
    for binding in &mut sema.bindings {
        binding.ty = concrete_ty(&binding.ty, cells).map_err(|diag| Diag {
            message: format!("`{}`: {}", binding.name, diag.message),
            ..diag
        })?;
    }
    for item in &mut sema.items {
        match &mut item.kind {
            ItemKind::Struct(fields) => {
                for (_, ty) in fields.iter_mut() {
                    *ty = concrete_ty(ty, cells)?;
                }
            }
            ItemKind::Enum(variants) => {
                for (_, payload) in variants.iter_mut() {
                    for (_, ty) in payload.iter_mut() {
                        *ty = concrete_ty(ty, cells)?;
                    }
                }
            }
            ItemKind::Alias(ty) => *ty = concrete_ty(ty, cells)?,
        }
    }
    for index in 0..sema.funs.len() {
        let ret = concrete_ty(&sema.funs[index].ret, cells)?;
        sema.funs[index].ret = ret;
        let params = sema.funs[index].params.clone();
        let mut resolved_params = Vec::with_capacity(params.len());
        for (binding, name) in params {
            let ty = concrete_ty(&sema.bindings[binding.0 as usize].ty.clone(), cells)?;
            resolved_params.push((binding, name, ty));
        }
        for (binding, _, ty) in resolved_params {
            sema.bindings[binding.0 as usize].ty = ty;
        }
        let body = sema.funs[index].body.clone();
        let resolved = resolve_block(&body, cells)?;
        sema.funs[index].body = resolved;
    }
    Ok(())
}

fn concrete_ty(ty: &HostTy, cells: &[Option<HostTy>]) -> Result<HostTy, Diag> {
    match ty {
        HostTy::Infer(index) => {
            let bound = cells
                .get(*index)
                .and_then(|cell| cell.as_ref())
                .ok_or_else(|| {
                    Diag::unsupported(
                        Span::new(1, 1, 1),
                        "this type cannot be inferred; add an annotation (integer \
                     literals need an admitted `i64` or `usize` context)",
                    )
                })?;
            concrete_ty(bound, cells)
        }
        HostTy::Ref(mutable, inner) => {
            Ok(HostTy::Ref(*mutable, Box::new(concrete_ty(inner, cells)?)))
        }
        HostTy::Box(inner) => Ok(HostTy::Box(Box::new(concrete_ty(inner, cells)?))),
        HostTy::Vec(inner) => Ok(HostTy::Vec(Box::new(concrete_ty(inner, cells)?))),
        HostTy::Option(inner) => Ok(HostTy::Option(Box::new(concrete_ty(inner, cells)?))),
        HostTy::Result(ok, err) => Ok(HostTy::Result(
            Box::new(concrete_ty(ok, cells)?),
            Box::new(concrete_ty(err, cells)?),
        )),
        HostTy::HashMap(inner) => Ok(HostTy::HashMap(Box::new(concrete_ty(inner, cells)?))),
        HostTy::Array(inner, count) => {
            Ok(HostTy::Array(Box::new(concrete_ty(inner, cells)?), *count))
        }
        HostTy::Tuple(left, right) => Ok(HostTy::Tuple(
            Box::new(concrete_ty(left, cells)?),
            Box::new(concrete_ty(right, cells)?),
        )),
        HostTy::FnPtr(params, ret) => {
            let mut out = Vec::with_capacity(params.len());
            for param in params {
                out.push(concrete_ty(param, cells)?);
            }
            Ok(HostTy::FnPtr(out, Box::new(concrete_ty(ret, cells)?)))
        }
        HostTy::DynFn(kind, params, ret) => {
            let mut out = Vec::with_capacity(params.len());
            for param in params {
                out.push(concrete_ty(param, cells)?);
            }
            Ok(HostTy::DynFn(
                *kind,
                out,
                Box::new(concrete_ty(ret, cells)?),
            ))
        }
        HostTy::Range(inner) => Ok(HostTy::Range(Box::new(concrete_ty(inner, cells)?))),
        HostTy::Iter(inner) => Ok(HostTy::Iter(Box::new(concrete_ty(inner, cells)?))),
        HostTy::IterMut(inner) => Ok(HostTy::IterMut(Box::new(concrete_ty(inner, cells)?))),
        HostTy::IntoIter(inner) => Ok(HostTy::IntoIter(Box::new(concrete_ty(inner, cells)?))),
        HostTy::Enumerate(inner) => Ok(HostTy::Enumerate(Box::new(concrete_ty(inner, cells)?))),
        HostTy::Zip(left, right) => Ok(HostTy::Zip(
            Box::new(concrete_ty(left, cells)?),
            Box::new(concrete_ty(right, cells)?),
        )),
        other => Ok(other.clone()),
    }
}

fn resolve_block(block: &HirBlock, cells: &[Option<HostTy>]) -> Result<HirBlock, Diag> {
    let mut stmts = Vec::with_capacity(block.stmts.len());
    for stmt in &block.stmts {
        stmts.push(match stmt {
            HirStmt::Let {
                binding,
                destruct,
                value,
            } => {
                let value = resolve_expr(value, cells)?;
                HirStmt::Let {
                    binding: *binding,
                    destruct: *destruct,
                    value,
                }
            }
            HirStmt::Expr(expr) => HirStmt::Expr(resolve_expr(expr, cells)?),
        });
    }
    let tail = match &block.tail {
        Some(tail) => Some(Box::new(resolve_expr(tail, cells)?)),
        None => None,
    };
    Ok(HirBlock { stmts, tail })
}

fn resolve_expr(expr: &HirExpr, cells: &[Option<HostTy>]) -> Result<HirExpr, Diag> {
    let ty = concrete_ty(&expr.ty, cells).map_err(|diag| Diag {
        span: expr.span,
        ..diag
    })?;
    let kind = resolve_expr_kind(&expr.kind, &ty, expr.span, cells)?;
    Ok(HirExpr {
        kind,
        ty,
        diverges: expr.diverges,
        span: expr.span,
    })
}

fn resolve_expr_kind(
    kind: &HirExprKind,
    ty: &HostTy,
    span: Span,
    cells: &[Option<HostTy>],
) -> Result<HirExprKind, Diag> {
    if let Some(kind) = resolve_aggregate_expr(kind, cells)? {
        return Ok(kind);
    }
    if let Some(kind) = resolve_control_expr(kind, cells)? {
        return Ok(kind);
    }
    Ok(match kind {
        HirExprKind::I64(_) | HirExprKind::Usize(_) => retype_int_literal(kind, ty, span)?,
        HirExprKind::Bool(_)
        | HirExprKind::Unit
        | HirExprKind::Str(_)
        | HirExprKind::FunRef(_)
        | HirExprKind::Continue => kind.clone(),
        HirExprKind::Place { place, mode } => HirExprKind::Place {
            place: resolve_place(place, cells)?,
            mode: *mode,
        },
        HirExprKind::Field { base, index } => HirExprKind::Field {
            base: boxed(base, cells)?,
            index: *index,
        },
        HirExprKind::Index { base, index } => HirExprKind::Index {
            base: boxed(base, cells)?,
            index: boxed(index, cells)?,
        },
        HirExprKind::Call { callee, args } => HirExprKind::Call {
            callee: *callee,
            args: resolve_list(args, cells)?,
        },
        HirExprKind::Ctor(op, args) => HirExprKind::Ctor(*op, resolve_list(args, cells)?),
        HirExprKind::IndirectCall { callee, args } => HirExprKind::IndirectCall {
            callee: boxed(callee, cells)?,
            args: resolve_list(args, cells)?,
        },
        HirExprKind::Method {
            op,
            receiver,
            receiver_place,
            args,
        } => HirExprKind::Method {
            op: *op,
            receiver: boxed(receiver, cells)?,
            receiver_place: receiver_place
                .as_ref()
                .map(|place| resolve_place(place, cells))
                .transpose()?,
            args: resolve_list(args, cells)?,
        },
        HirExprKind::Unary { op, operand } => HirExprKind::Unary {
            op: *op,
            operand: boxed(operand, cells)?,
        },
        HirExprKind::Binary { op, left, right } => HirExprKind::Binary {
            op: *op,
            left: boxed(left, cells)?,
            right: boxed(right, cells)?,
        },
        HirExprKind::Assign { op, target, value } => HirExprKind::Assign {
            op: *op,
            target: resolve_place(target, cells)?,
            value: boxed(value, cells)?,
        },
        HirExprKind::Try(inner) => HirExprKind::Try(boxed(inner, cells)?),
        HirExprKind::Range(left, right) => {
            HirExprKind::Range(boxed(left, cells)?, boxed(right, cells)?)
        }
        _ => unreachable!("aggregate and control-flow kinds were resolved above"),
    })
}

fn resolve_aggregate_expr(
    kind: &HirExprKind,
    cells: &[Option<HostTy>],
) -> Result<Option<HirExprKind>, Diag> {
    let resolved = match kind {
        HirExprKind::StructLit(id, args) => HirExprKind::StructLit(*id, resolve_list(args, cells)?),
        HirExprKind::TupleStructLit(id, args) => {
            HirExprKind::TupleStructLit(*id, resolve_list(args, cells)?)
        }
        HirExprKind::VariantLit(id, index, args) => {
            HirExprKind::VariantLit(*id, *index, resolve_list(args, cells)?)
        }
        HirExprKind::Tuple(left, right) => {
            HirExprKind::Tuple(boxed(left, cells)?, boxed(right, cells)?)
        }
        HirExprKind::Array(items) => HirExprKind::Array(resolve_list(items, cells)?),
        HirExprKind::VecList(items) => HirExprKind::VecList(resolve_list(items, cells)?),
        HirExprKind::VecRepeat(value, count) => {
            HirExprKind::VecRepeat(boxed(value, cells)?, boxed(count, cells)?)
        }
        HirExprKind::Format { kind, spec, args } => HirExprKind::Format {
            kind: *kind,
            spec: spec.clone(),
            args: resolve_list(args, cells)?,
        },
        _ => return Ok(None),
    };
    Ok(Some(resolved))
}

fn resolve_control_expr(
    kind: &HirExprKind,
    cells: &[Option<HostTy>],
) -> Result<Option<HirExprKind>, Diag> {
    let resolved = match kind {
        HirExprKind::If {
            test,
            then,
            else_branch,
        } => HirExprKind::If {
            test: boxed(test, cells)?,
            then: boxed(then, cells)?,
            else_branch: boxed(else_branch, cells)?,
        },
        HirExprKind::IfLet {
            pat,
            value,
            then,
            else_branch,
        } => HirExprKind::IfLet {
            pat: resolve_pat(pat),
            value: boxed(value, cells)?,
            then: boxed(then, cells)?,
            else_branch: boxed(else_branch, cells)?,
        },
        HirExprKind::Match { scrutinee, arms } => HirExprKind::Match {
            scrutinee: boxed(scrutinee, cells)?,
            arms: arms
                .iter()
                .map(|(pat, body)| Ok((resolve_pat(pat), resolve_expr(body, cells)?)))
                .collect::<Result<_, Diag>>()?,
        },
        HirExprKind::Block(block) => HirExprKind::Block(resolve_block(block, cells)?),
        HirExprKind::Loop { body, break_ty } => HirExprKind::Loop {
            body: resolve_block(body, cells)?,
            break_ty: concrete_ty(break_ty, cells)?,
        },
        HirExprKind::While { test, body } => HirExprKind::While {
            test: boxed(test, cells)?,
            body: resolve_block(body, cells)?,
        },
        HirExprKind::WhileLet { pat, value, body } => HirExprKind::WhileLet {
            pat: resolve_pat(pat),
            value: boxed(value, cells)?,
            body: resolve_block(body, cells)?,
        },
        HirExprKind::For {
            pat,
            iterable,
            body,
        } => HirExprKind::For {
            pat: resolve_pat(pat),
            iterable: boxed(iterable, cells)?,
            body: resolve_block(body, cells)?,
        },
        HirExprKind::Closure(closure) => HirExprKind::Closure(resolve_closure(closure, cells)?),
        HirExprKind::Return(value) => HirExprKind::Return(resolve_opt(value.as_deref(), cells)?),
        HirExprKind::Break(value) => HirExprKind::Break(resolve_opt(value.as_deref(), cells)?),
        _ => return Ok(None),
    };
    Ok(Some(resolved))
}

fn boxed(expr: &HirExpr, cells: &[Option<HostTy>]) -> Result<Box<HirExpr>, Diag> {
    resolve_expr(expr, cells).map(Box::new)
}

fn resolve_opt(
    value: Option<&HirExpr>,
    cells: &[Option<HostTy>],
) -> Result<Option<Box<HirExpr>>, Diag> {
    value.map(|inner| boxed(inner, cells)).transpose()
}

/// Retypes an integer literal to the width inference settled on,
/// rejecting a value the settled type cannot hold.
fn retype_int_literal(kind: &HirExprKind, ty: &HostTy, span: Span) -> Result<HirExprKind, Diag> {
    Ok(match (kind, ty) {
        (HirExprKind::I64(value), HostTy::Usize) => HirExprKind::Usize(
            u64::try_from(*value)
                .map_err(|_| Diag::type_error(span, "integer literal out of range for `usize`"))?,
        ),
        (HirExprKind::Usize(value), HostTy::I64) => HirExprKind::I64(
            i64::try_from(*value)
                .map_err(|_| Diag::type_error(span, "integer literal out of range for `i64`"))?,
        ),
        (other, _) => other.clone(),
    })
}

fn resolve_closure(closure: &HirClosure, cells: &[Option<HostTy>]) -> Result<HirClosure, Diag> {
    let mut params = Vec::with_capacity(closure.params.len());
    for (binding, param_ty) in &closure.params {
        params.push((*binding, concrete_ty(param_ty, cells)?));
    }
    Ok(HirClosure {
        kind: closure.kind,
        bind_base: closure.bind_base,
        frame_slots: closure.frame_slots,
        params,
        body: resolve_block(&closure.body, cells)?,
        captures: closure.captures.clone(),
        ret: concrete_ty(&closure.ret, cells)?,
    })
}

fn resolve_list(items: &[HirExpr], cells: &[Option<HostTy>]) -> Result<Vec<HirExpr>, Diag> {
    let mut out = Vec::with_capacity(items.len());
    for item in items {
        out.push(resolve_expr(item, cells)?);
    }
    Ok(out)
}

fn resolve_place(place: &Place, cells: &[Option<HostTy>]) -> Result<Place, Diag> {
    let root = match &place.root {
        crate::host::hir::PlaceRoot::Local(binding) => crate::host::hir::PlaceRoot::Local(*binding),
        crate::host::hir::PlaceRoot::Deref(expr) => {
            crate::host::hir::PlaceRoot::Deref(Box::new(resolve_expr(expr, cells)?))
        }
    };
    let mut proj = Vec::with_capacity(place.proj.len());
    for step in &place.proj {
        proj.push(match step {
            Proj::Field(index) => Proj::Field(*index),
            Proj::Index(expr) => Proj::Index(Box::new(resolve_expr(expr, cells)?)),
        });
    }
    Ok(Place {
        root,
        proj,
        span: place.span,
    })
}

fn resolve_pat(pat: &HirPat) -> HirPat {
    let kind = match &pat.kind {
        HirPatKind::Wild
        | HirPatKind::Bind(_)
        | HirPatKind::I64(_)
        | HirPatKind::Usize(_)
        | HirPatKind::Bool(_)
        | HirPatKind::UnitPath(_) => pat.kind.clone(),
        HirPatKind::Tuple(left, right) => {
            HirPatKind::Tuple(Box::new(resolve_pat(left)), Box::new(resolve_pat(right)))
        }
        HirPatKind::TuplePath(resolved, sub) => {
            HirPatKind::TuplePath(resolved.clone(), sub.iter().map(resolve_pat).collect())
        }
        HirPatKind::StructPath(resolved, fields) => {
            HirPatKind::StructPath(resolved.clone(), fields.iter().map(resolve_pat).collect())
        }
    };
    HirPat {
        kind,
        span: pat.span,
    }
}

impl Checker {
    fn item_has_derive(&self, id: ItemId, want: DeriveName) -> bool {
        self.derives
            .get(&id)
            .is_some_and(|list| list.contains(&want))
    }

    /// Whether `ty` is (or, as an unresolved cell, is now obliged to be)
    /// one of the admitted integer types. An unresolved cell becomes an
    /// integer cell, so any later binding must be `i64` or `usize`.
    pub(crate) fn admits_integer(&mut self, ty: &HostTy) -> bool {
        match self.deep(ty) {
            HostTy::I64 | HostTy::Usize => true,
            HostTy::Infer(index) => {
                if !self.int_cells.contains(&index) {
                    self.int_cells.push(index);
                }
                true
            }
            _ => false,
        }
    }

    /// Checks the operator obligations deferred while operand types were
    /// unresolved, now that every body has been checked. No defaulting:
    /// a literal left to Rust's default `i32` inference is host-valid
    /// but outside the subset (grammar §3), so an obligation still
    /// unresolved names no admitted type and the program is rejected.
    /// Deferral (not defaulting) is what closures need: a parameter
    /// compared before its type arrives still checks once the call site
    /// pins it.
    fn discharge_obligations(&self) -> Result<(), Diag> {
        for (ty, span) in &self.i64_obligations {
            if !matches!(self.deep(ty), HostTy::I64 | HostTy::Infer(_)) {
                return Err(Diag::type_error(
                    *span,
                    "`*`, `/`, and `%` need `i64` operands",
                ));
            }
        }
        self.discharge_trait_obligations()
    }
}
