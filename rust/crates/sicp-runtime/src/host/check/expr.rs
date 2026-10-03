// SPDX-License-Identifier: GPL-3.0-only

//! Expression, statement, and block checking: typing with inference
//! cells, move and borrow discipline (grammar §5), closure-capture
//! classification, and the closed constructor/method allowlist
//! (grammar §4). All checking finishes before any effect exists.

use std::collections::HashMap;

use crate::host::ast::{self};
use crate::host::check::traits::Trait;
use crate::host::check::{Access, Checker, Loan, LoopCtx, table_index};
use crate::host::diag::{Diag, Span};
use crate::host::hir::{
    BinOp, BindId, CaptureMode, ClosureKind, CtorOp, HirBlock, HirExpr, HirExprKind, HirPat,
    HirPatKind, HirStmt, HostTy, ItemKind, MethodOp, Place, PlaceRoot, PlaceUse, Proj, Resolved,
    UnOp,
};

/// The move state at the top of a loop, taken before its header.
struct LoopMark {
    /// The bindings already moved on entry.
    moved: Vec<BindId>,
    /// The first binding id the loop's own code allocates: earlier ids
    /// name bindings that outlive an iteration.
    next_bind: u32,
}

/// Whether a loop's body runs before its condition is known.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LoopEntry {
    /// `loop`: the body always runs first; only `break` leaves it.
    Unconditional,
    /// `while`, `while let`, `for`: the loop may end before or after
    /// any iteration.
    Guarded,
}

/// Whether control cannot reach the end of `block`.
fn block_diverges(block: &HirBlock) -> bool {
    block.tail.as_ref().is_some_and(|tail| tail.diverges)
        || block.stmts.iter().any(|stmt| match stmt {
            HirStmt::Expr(expr) | HirStmt::Let { value: expr, .. } => expr.diverges,
        })
}

/// How an expression uses the place it names (grammar §5). A value use
/// moves a non-`Copy` place out; a place use only reads or borrows it,
/// as comparison operands, format arguments, method receivers,
/// scrutinees, and field, index, and dereference bases do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Use {
    /// The expression's value is consumed.
    Value,
    /// The expression names a place that is only inspected.
    Place,
}

impl Checker {
    /// Checks a block as a function or closure body.
    pub(crate) fn check_block_body(
        &mut self,
        block: &ast::Block,
        expect: Option<&HostTy>,
    ) -> Result<HirBlock, Diag> {
        self.ctx_mut().scopes.push(HashMap::new());
        let mut stmts = Vec::with_capacity(block.stmts.len());
        for stmt in &block.stmts {
            stmts.push(self.check_stmt(stmt)?);
        }
        let tail = if let Some(tail) = &block.tail {
            Some(Box::new(self.check_expr(tail, expect)?))
        } else {
            if let Some(want) = expect {
                self.unify(&HostTy::Unit, want, block.span)?;
            }
            None
        };
        self.pop_scope();
        Ok(HirBlock { stmts, tail })
    }

    fn check_stmt(&mut self, stmt: &ast::Stmt) -> Result<HirStmt, Diag> {
        match stmt {
            ast::Stmt::Let(let_stmt) => self.check_let(let_stmt),
            ast::Stmt::Expr { expr, .. } => {
                let checked = self.check_expr(expr, None)?;
                self.end_temp_loans();
                Ok(HirStmt::Expr(checked))
            }
        }
    }

    fn end_temp_loans(&mut self) {
        let ctx = self.ctx_mut();
        ctx.loans.retain(|loan| loan.holder.is_some());
    }

    fn check_let(&mut self, let_stmt: &ast::LetStmt) -> Result<HirStmt, Diag> {
        let expect = match &let_stmt.annotation {
            Some(ty) => Some(self.resolve_ty(ty, &mut Vec::new())?),
            None => None,
        };
        let value = self.check_expr(&let_stmt.value, expect.as_ref())?;
        match &let_stmt.pat.kind {
            ast::PatKind::Bind(name) => {
                let ty = value.ty.clone();
                let binding = self.fresh_bind(&name.name, ty, let_stmt.mutable);
                self.ctx_mut()
                    .scopes
                    .last_mut()
                    .expect("the block scope")
                    .insert(name.name.clone(), binding);
                self.retire_borrow_into(&value, name.name.clone());
                self.settle_initializer_loans(&value.ty, &[name.name.as_str()]);
                Ok(HirStmt::Let {
                    binding,
                    destruct: None,
                    value,
                })
            }
            ast::PatKind::Tuple(first, second) => {
                let (ast::PatKind::Bind(left), ast::PatKind::Bind(right)) =
                    (&first.kind, &second.kind)
                else {
                    return Err(Diag::type_error(
                        let_stmt.pat.span,
                        "let patterns are identifiers or two-element tuples of identifiers",
                    ));
                };
                let HostTy::Tuple(a, b) = self.deep(&value.ty) else {
                    return Err(Diag::type_error(
                        let_stmt.pat.span,
                        "this pattern requires a two-element tuple",
                    ));
                };
                if left.name == right.name {
                    return Err(Diag::type_error(
                        let_stmt.pat.span,
                        format!("`{}` is bound more than once", left.name),
                    ));
                }
                let left_id = self.fresh_bind(&left.name, *a, let_stmt.mutable);
                let right_id = self.fresh_bind(&right.name, *b, let_stmt.mutable);
                let scope = self.ctx_mut().scopes.last_mut().expect("the block scope");
                scope.insert(left.name.clone(), left_id);
                scope.insert(right.name.clone(), right_id);
                self.settle_initializer_loans(
                    &value.ty,
                    &[left.name.as_str(), right.name.as_str()],
                );
                Ok(HirStmt::Let {
                    binding: BindId(u32::MAX),
                    destruct: Some((left_id, right_id)),
                    value,
                })
            }
            _ => Err(Diag::type_error(
                let_stmt.pat.span,
                "let patterns are identifiers or two-element tuples of identifiers",
            )),
        }
    }

    /// Settles the borrows an initializer took when its statement ends:
    /// a value that holds no reference releases them (like Rust's
    /// temporaries), while a value that holds a reference may be
    /// derived from them through elided lifetimes, so they stay live
    /// as long as the bound names have later uses.
    fn settle_initializer_loans(&mut self, ty: &HostTy, holders: &[&str]) {
        let temporaries: Vec<Loan> = {
            let loans = &mut self.ctx_mut().loans;
            let (temporaries, held) = std::mem::take(loans)
                .into_iter()
                .partition(|loan| loan.holder.is_none());
            *loans = held;
            temporaries
        };
        if !self.holds_reference(ty) {
            return;
        }
        for loan in temporaries {
            for holder in holders {
                let mut held = loan.clone();
                held.holder = Some((*holder).to_owned());
                self.ctx_mut().loans.push(held);
            }
        }
    }

    fn holds_reference(&self, ty: &HostTy) -> bool {
        match self.deep(ty) {
            HostTy::Ref(..)
            | HostTy::Str
            | HostTy::Iter(_)
            | HostTy::IterMut(_)
            | HostTy::Infer(_) => true,
            HostTy::Box(inner)
            | HostTy::Vec(inner)
            | HostTy::Option(inner)
            | HostTy::HashMap(inner)
            | HostTy::Array(inner, _)
            | HostTy::IntoIter(inner)
            | HostTy::Enumerate(inner)
            | HostTy::Range(inner) => self.holds_reference(&inner),
            HostTy::Result(left, right) | HostTy::Tuple(left, right) | HostTy::Zip(left, right) => {
                self.holds_reference(&left) || self.holds_reference(&right)
            }
            HostTy::FnPtr(..)
            | HostTy::DynFn(..)
            | HostTy::Unit
            | HostTy::Bool
            | HostTy::I64
            | HostTy::Usize
            | HostTy::String
            | HostTy::Struct(_)
            | HostTy::Enum(_) => false,
        }
    }

    /// Retargets the temporary loan a `let NAME = &place` initializer
    /// created so the loan lives as long as `NAME` has later uses.
    fn retire_borrow_into(&mut self, value: &HirExpr, holder: String) {
        let HirExprKind::Unary {
            op: UnOp::Ref | UnOp::RefMut,
            operand,
        } = &value.kind
        else {
            return;
        };
        let HirExprKind::Place { place, .. } = &operand.kind else {
            return;
        };
        let PlaceRoot::Local(root) = &place.root else {
            return;
        };
        let root = *root;
        if let Some(loan) = self
            .ctx_mut()
            .loans
            .iter_mut()
            .rev()
            .find(|loan| loan.root == root && loan.holder.is_none())
        {
            loan.holder = Some(holder);
        }
    }

    /// Checks one expression against an optional expectation, using
    /// its value: a non-`Copy` local, field, or element it names moves.
    pub(crate) fn check_expr(
        &mut self,
        expr: &ast::Expr,
        expect: Option<&HostTy>,
    ) -> Result<HirExpr, Diag> {
        self.check_expr_as(expr, expect, Use::Value)
    }

    /// Checks an expression that is only inspected in place, never
    /// consumed: nothing it names moves.
    pub(crate) fn check_expr_place(
        &mut self,
        expr: &ast::Expr,
        expect: Option<&HostTy>,
    ) -> Result<HirExpr, Diag> {
        self.check_expr_as(expr, expect, Use::Place)
    }

    fn check_expr_as(
        &mut self,
        expr: &ast::Expr,
        expect: Option<&HostTy>,
        usage: Use,
    ) -> Result<HirExpr, Diag> {
        self.ctx_mut().here = expr.span;
        let checked = self.check_expr_kind(expr, expect, usage)?;
        self.coerce(checked, expect, expr.span)
    }

    fn coerce(
        &mut self,
        mut expr: HirExpr,
        expect: Option<&HostTy>,
        span: Span,
    ) -> Result<HirExpr, Diag> {
        let Some(want) = expect else {
            return Ok(expr);
        };
        let want = self.deep(want);
        if let HostTy::FnPtr(..) = want
            && let HirExprKind::Closure(closure) = &expr.kind
            && closure.captures.is_empty()
        {
            expr.ty = want;
            return Ok(expr);
        }
        let mut expr = self.deref_coercion(expr, &want);
        // `&mut T` coerces to `&T`: the exclusive borrow is reborrowed
        // as a shared one.
        if let (HostTy::Ref(true, inner), HostTy::Ref(false, _)) = (self.deep(&expr.ty), &want) {
            expr.ty = HostTy::Ref(false, inner);
        }
        expr.ty = self.unify(&expr.ty.clone(), &want, span)?;
        Ok(expr)
    }

    /// Rust's deref coercion at a coercion site: `&Box<T>` (or
    /// `&mut Box<T>`) stands where `&T` is expected by borrowing the
    /// box's contents, the same operation `Box::as_ref`/`as_mut`
    /// performs. Other expressions pass through unchanged.
    fn deref_coercion(&self, expr: HirExpr, want: &HostTy) -> HirExpr {
        if let HostTy::Str = want {
            return self.string_deref_coercion(expr);
        }
        let HostTy::Ref(want_mut, want_inner) = want else {
            return expr;
        };
        let HostTy::Ref(got_mut, got_inner) = self.deep(&expr.ty) else {
            return expr;
        };
        let HostTy::Box(contents) = self.deep(&got_inner) else {
            return expr;
        };
        if *want_mut && !got_mut
            || matches!(self.deep(want_inner), HostTy::Box(_) | HostTy::Infer(_))
        {
            return expr;
        }
        let receiver_place = match &expr.kind {
            HirExprKind::Place { place, .. } => place.clone(),
            HirExprKind::Unary {
                op: UnOp::Ref | UnOp::RefMut,
                operand,
            } => match &operand.kind {
                HirExprKind::Place { place, .. } => place.clone(),
                _ => return expr,
            },
            _ => return expr,
        };
        // `&place` borrows the box itself; its place is the receiver.
        let receiver = match expr.kind {
            HirExprKind::Unary { operand, .. } => *operand,
            _ => expr,
        };
        let op = if *want_mut {
            MethodOp::BoxAsMut
        } else {
            MethodOp::BoxAsRef
        };
        let span = receiver.span;
        node(
            HirExprKind::Method {
                op,
                receiver: Box::new(receiver),
                receiver_place: Some(receiver_place),
                args: Vec::new(),
            },
            HostTy::Ref(*want_mut, contents),
            false,
            span,
        )
    }

    /// `&String` stands where `&str` is expected by borrowing the
    /// string's contents, the operation `String::as_str` performs.
    fn string_deref_coercion(&self, expr: HirExpr) -> HirExpr {
        let HostTy::Ref(_, inner) = self.deep(&expr.ty) else {
            return expr;
        };
        if self.deep(&inner) != HostTy::String {
            return expr;
        }
        let span = expr.span;
        node(
            HirExprKind::Method {
                op: MethodOp::AsStr,
                receiver: Box::new(expr),
                receiver_place: None,
                args: Vec::new(),
            },
            HostTy::Str,
            false,
            span,
        )
    }

    fn check_expr_kind(
        &mut self,
        expr: &ast::Expr,
        expect: Option<&HostTy>,
        usage: Use,
    ) -> Result<HirExpr, Diag> {
        let span = expr.span;
        match &expr.kind {
            ast::ExprKind::IntLit { text, suffix } => {
                self.check_int_lit(text, *suffix, expect, span)
            }
            ast::ExprKind::StrLit(value) => Ok(node(
                HirExprKind::Str(value.clone()),
                HostTy::Str,
                false,
                span,
            )),
            ast::ExprKind::BoolLit(value) => {
                Ok(node(HirExprKind::Bool(*value), HostTy::Bool, false, span))
            }
            ast::ExprKind::UnitLit => Ok(node(HirExprKind::Unit, HostTy::Unit, false, span)),
            ast::ExprKind::Path(path) => self.check_path_value(path, expect, usage, span),
            ast::ExprKind::Tuple(left, right) => self.check_tuple(left, right, span),
            ast::ExprKind::Array(items) => {
                let (checked, elem) = self.check_elements(items)?;
                let count = u64::try_from(checked.len())
                    .expect("array lengths fit u64 in the teaching subset");
                let ty = HostTy::Array(Box::new(elem), count);
                Ok(node(HirExprKind::Array(checked), ty, false, span))
            }
            ast::ExprKind::VecList(items) => {
                let (checked, elem) = self.check_elements(items)?;
                let ty = HostTy::Vec(Box::new(elem));
                Ok(node(HirExprKind::VecList(checked), ty, false, span))
            }
            ast::ExprKind::VecRepeat(value, count) => self.check_vec_repeat(value, count, span),
            ast::ExprKind::Format { kind, fmt, args } => self.check_format(*kind, fmt, args, span),
            ast::ExprKind::StructLit { path, fields } => self.check_struct_lit(path, fields, span),
            ast::ExprKind::Field { base, name } => self.check_field(base, &name.name, usage, span),
            ast::ExprKind::Index { base, index } => self.check_index(base, index, usage, span),
            ast::ExprKind::Call { callee, args } => self.check_call(callee, args, expect, span),
            ast::ExprKind::MethodCall {
                receiver,
                name,
                args,
            } => self.check_method(receiver, &name.name, args, span),
            ast::ExprKind::Unary { op, operand } => self.check_unary(*op, operand, usage, span),
            ast::ExprKind::Binary { op, left, right } => self.check_binary(*op, left, right, span),
            ast::ExprKind::Assign { op, target, value } => {
                self.check_assign(*op, target, value, span)
            }
            ast::ExprKind::If {
                test,
                then,
                else_branch,
            } => self.check_if(test, then, else_branch.as_deref(), expect, span),
            ast::ExprKind::IfLet {
                pat,
                value,
                then,
                else_branch,
            } => self.check_if_let(pat, value, then, else_branch.as_deref(), expect, span),
            ast::ExprKind::Match { scrutinee, arms } => {
                self.check_match(scrutinee, arms, expect, span)
            }
            ast::ExprKind::Block(block) => self.check_block_expr(block, expect, span),
            ast::ExprKind::Loop(body) => self.check_loop(body, span),
            ast::ExprKind::While { test, body } => self.check_while(test, body, span),
            ast::ExprKind::WhileLet { pat, value, body } => {
                self.check_while_let(pat, value, body, span)
            }
            ast::ExprKind::For {
                pat,
                iterable,
                body,
            } => self.check_for(pat, iterable, body, span),
            ast::ExprKind::Closure { mov, params, body } => {
                self.check_closure(*mov, params, body, expect, span)
            }
            ast::ExprKind::Return(value) => self.check_return(value.as_deref(), span),
            ast::ExprKind::Break(value) => self.check_break(value.as_deref(), span),
            ast::ExprKind::Continue => self.check_continue(span),
            ast::ExprKind::Try(inner) => self.check_try(inner, span),
            ast::ExprKind::Range(left, right) => self.check_range(left, right, span),
        }
    }

    fn check_tuple(
        &mut self,
        left: &ast::Expr,
        right: &ast::Expr,
        span: Span,
    ) -> Result<HirExpr, Diag> {
        let first = self.check_expr(left, None)?;
        let second = self.check_expr(right, None)?;
        let ty = HostTy::Tuple(Box::new(first.ty.clone()), Box::new(second.ty.clone()));
        Ok(node(
            HirExprKind::Tuple(Box::new(first), Box::new(second)),
            ty,
            false,
            span,
        ))
    }

    fn check_vec_repeat(
        &mut self,
        value: &ast::Expr,
        count: &ast::Expr,
        span: Span,
    ) -> Result<HirExpr, Diag> {
        let elem = self.fresh_cell(false);
        let checked = self.check_expr(value, Some(&elem))?;
        self.require_trait(&elem, Trait::Clone, value.span)?;
        let times = self.check_expr(count, Some(&HostTy::Usize))?;
        let ty = HostTy::Vec(Box::new(elem));
        Ok(node(
            HirExprKind::VecRepeat(Box::new(checked), Box::new(times)),
            ty,
            false,
            span,
        ))
    }

    fn check_block_expr(
        &mut self,
        block: &ast::Block,
        expect: Option<&HostTy>,
        span: Span,
    ) -> Result<HirExpr, Diag> {
        let body = self.check_block_body(block, expect)?;
        let ty = body
            .tail
            .as_ref()
            .map_or(HostTy::Unit, |tail| tail.ty.clone());
        Ok(node(HirExprKind::Block(body), ty, false, span))
    }

    fn check_continue(&mut self, span: Span) -> Result<HirExpr, Diag> {
        let moved = self.ctx().uninit.clone();
        let Some(current) = self.ctx_mut().loops.last_mut() else {
            return Err(Diag::type_error(span, "`continue` outside a loop"));
        };
        current.continue_moves.push(moved);
        let cell = self.fresh_never_cell();
        Ok(node(HirExprKind::Continue, cell, true, span))
    }

    fn check_range(
        &mut self,
        left: &ast::Expr,
        right: &ast::Expr,
        span: Span,
    ) -> Result<HirExpr, Diag> {
        let elem = self.fresh_cell(true);
        let start = self.check_expr(left, Some(&elem))?;
        let end = self.check_expr(right, Some(&elem))?;
        Ok(node(
            HirExprKind::Range(Box::new(start), Box::new(end)),
            HostTy::Range(Box::new(elem)),
            false,
            span,
        ))
    }

    /// Checks the items of an array or `vec![...]` literal against one
    /// fresh element type.
    fn check_elements(&mut self, items: &[ast::Expr]) -> Result<(Vec<HirExpr>, HostTy), Diag> {
        let elem = self.fresh_cell(false);
        let mut checked = Vec::with_capacity(items.len());
        for item in items {
            checked.push(self.check_expr(item, Some(&elem))?);
        }
        Ok((checked, elem))
    }

    fn check_while(
        &mut self,
        test: &ast::Expr,
        body: &ast::Block,
        span: Span,
    ) -> Result<HirExpr, Diag> {
        let mark = self.loop_mark();
        let checked_test = self.check_expr(test, Some(&HostTy::Bool))?;
        let (checked_body, _) =
            self.check_loop_block(body, HostTy::Unit, mark, LoopEntry::Guarded)?;
        Ok(node(
            HirExprKind::While {
                test: Box::new(checked_test),
                body: checked_body,
            },
            HostTy::Unit,
            false,
            span,
        ))
    }

    fn check_while_let(
        &mut self,
        pat: &ast::Pat,
        value: &ast::Expr,
        body: &ast::Block,
        span: Span,
    ) -> Result<HirExpr, Diag> {
        let mark = self.loop_mark();
        let checked_value = self.check_expr_place(value, None)?;
        let wanted = checked_value.ty.clone();
        self.ctx_mut().scopes.push(HashMap::new());
        let checked_pat = self.check_pattern(pat, &wanted)?;
        self.move_bound_by_value(&checked_value, &checked_pat, value.span)?;
        let (checked_body, _) =
            self.check_loop_block(body, HostTy::Unit, mark, LoopEntry::Guarded)?;
        self.pop_scope();
        Ok(node(
            HirExprKind::WhileLet {
                pat: checked_pat,
                value: Box::new(checked_value),
                body: checked_body,
            },
            HostTy::Unit,
            false,
            span,
        ))
    }

    fn check_return(&mut self, value: Option<&ast::Expr>, span: Span) -> Result<HirExpr, Diag> {
        let ret = self.ctx().ret.clone();
        let checked = self.check_jump_value(value, &ret, span)?;
        let cell = self.fresh_never_cell();
        Ok(node(HirExprKind::Return(checked), cell, true, span))
    }

    fn check_break(&mut self, value: Option<&ast::Expr>, span: Span) -> Result<HirExpr, Diag> {
        let Some(current) = self.ctx_mut().loops.last_mut() else {
            return Err(Diag::type_error(span, "`break` outside a loop"));
        };
        current.saw_break = true;
        let cell = current.break_ty.clone();
        let checked = self.check_jump_value(value, &cell, span)?;
        let moved = self.ctx().uninit.clone();
        if let Some(current) = self.ctx_mut().loops.last_mut() {
            current.break_moves.push(moved);
        }
        let cell = self.fresh_never_cell();
        Ok(node(HirExprKind::Break(checked), cell, true, span))
    }

    /// Checks the optional value a `return` or `break` carries against
    /// its target type; a bare jump carries `()`.
    fn check_jump_value(
        &mut self,
        value: Option<&ast::Expr>,
        target: &HostTy,
        span: Span,
    ) -> Result<Option<Box<HirExpr>>, Diag> {
        let Some(value) = value else {
            self.unify(&HostTy::Unit, target, span)?;
            return Ok(None);
        };
        Ok(Some(Box::new(self.check_expr(value, Some(target))?)))
    }

    fn check_int_lit(
        &mut self,
        text: &str,
        suffix: Option<crate::host::lexer::IntSuffix>,
        expect: Option<&HostTy>,
        span: Span,
    ) -> Result<HirExpr, Diag> {
        let clean: String = text.chars().filter(|c| *c != '_').collect();
        let raw = clean
            .parse::<u64>()
            .map_err(|_| Diag::syntax(span, "integer literal out of range"))?;
        let declared = match suffix {
            Some(crate::host::lexer::IntSuffix::I64) => Some(HostTy::I64),
            Some(crate::host::lexer::IntSuffix::Usize) => Some(HostTy::Usize),
            None => None,
        };
        let ty = match (declared, expect.map(|want| self.deep(want))) {
            (Some(declared), Some(want)) => self.unify(&declared, &want, span)?,
            (Some(declared), None) => declared,
            (None, Some(want @ (HostTy::I64 | HostTy::Usize))) => want,
            (None, Some(HostTy::Infer(_)) | None) => self.fresh_cell(true),
            (None, Some(want)) => {
                return Err(Diag::type_error(
                    span,
                    format!("integer literal where `{want:?}` is required"),
                ));
            }
        };
        // The literal node carries its full admitted range: `usize`
        // spans `u64`, and `i64` spans the signed range. The final
        // resolution pass normalizes the node kind against the type.
        let kind = match i64::try_from(raw) {
            Ok(signed) => HirExprKind::I64(signed),
            Err(_) => HirExprKind::Usize(raw),
        };
        Ok(node(kind, ty, false, span))
    }

    /// A local named as an operand. A value use of a non-`Copy` local
    /// moves it (grammar §5), except that a `&mut` local passed where a
    /// reference is expected is reborrowed, as Rust does at a coercion
    /// site whose type is known.
    fn use_local(
        &mut self,
        binding: BindId,
        expect: Option<&HostTy>,
        usage: Use,
        span: Span,
    ) -> Result<HirExpr, Diag> {
        let ty = self.sema.bindings[binding.0 as usize].ty.clone();
        let reborrows = matches!(self.deep(&ty), HostTy::Ref(true, _))
            && expect.is_some_and(|want| matches!(self.deep(want), HostTy::Ref(..)));
        let moves = usage == Use::Value && !reborrows && !self.is_copy_ty(&ty);
        let access = if moves { Access::Move } else { Access::Read };
        self.check_access(binding, access, span)?;
        self.note_capture_use(
            binding,
            if reborrows && usage == Use::Value {
                Access::BorrowMut
            } else {
                access
            },
        );
        Ok(node(
            HirExprKind::Place {
                place: Place {
                    root: PlaceRoot::Local(binding),
                    proj: Vec::new(),
                    span,
                },
                mode: if moves {
                    PlaceUse::Move
                } else {
                    PlaceUse::Read
                },
            },
            ty,
            false,
            span,
        ))
    }

    fn check_path_value(
        &mut self,
        path: &[ast::Ident],
        expect: Option<&HostTy>,
        usage: Use,
        span: Span,
    ) -> Result<HirExpr, Diag> {
        match self.resolve_path(path, span)? {
            Resolved::Local(binding) => self.use_local(binding, expect, usage, span),
            Resolved::Fun(id) => {
                let params = self.fun_params[id.0 as usize].clone();
                let ret = self.sema.funs[id.0 as usize].ret.clone();
                Ok(node(
                    HirExprKind::FunRef(id),
                    HostTy::FnPtr(params, Box::new(ret)),
                    false,
                    span,
                ))
            }
            Resolved::UnitStruct(id) => Ok(node(
                HirExprKind::StructLit(id, Vec::new()),
                HostTy::Struct(id),
                false,
                span,
            )),
            Resolved::Variant(id, index) => Ok(node(
                HirExprKind::VariantLit(id, index, Vec::new()),
                HostTy::Enum(id),
                false,
                span,
            )),
            Resolved::Ctor(CtorOp::OptNone) => {
                let cell = self.fresh_cell(false);
                Ok(node(
                    HirExprKind::Ctor(CtorOp::OptNone, Vec::new()),
                    HostTy::Option(Box::new(cell)),
                    false,
                    span,
                ))
            }
            Resolved::Ctor(op) => Err(Diag::type_error(span, format!("{op:?} must be called"))),
            Resolved::TupleStruct(_) => Err(Diag::type_error(
                span,
                "a tuple struct is constructed by calling it",
            )),
            Resolved::TypeName(name) => Err(Diag::type_error(
                span,
                format!("`{name}` is a type, not a value"),
            )),
        }
    }

    fn resolve_path(&mut self, path: &[ast::Ident], span: Span) -> Result<Resolved, Diag> {
        match path.len() {
            1 => self.resolve_single_segment(&path[0].name, span),
            2 => self.resolve_two_segment(path, span),
            _ => Err(Diag::type_error(span, "unsupported path")),
        }
    }

    fn resolve_single_segment(&mut self, name: &str, span: Span) -> Result<Resolved, Diag> {
        if let Some(binding) = self.resolve_local(name) {
            return Ok(Resolved::Local(binding));
        }
        if let Some(id) = self.fun_env.get(name).copied() {
            return Ok(Resolved::Fun(id));
        }
        if let Some(item) = self.item_env.get(name).copied() {
            return match &self.sema.items[item.0 as usize].kind {
                ItemKind::Struct(fields) if fields.is_empty() => Ok(Resolved::UnitStruct(item)),
                ItemKind::Struct(fields)
                    if fields
                        .iter()
                        .all(|(field, _)| field.parse::<usize>().is_ok()) =>
                {
                    Ok(Resolved::TupleStruct(item))
                }
                _ => Err(Diag::type_error(
                    span,
                    format!("`{name}` names an item, not a value"),
                )),
            };
        }
        let ctor = match name {
            "Some" => Some(CtorOp::OptSome),
            "None" => Some(CtorOp::OptNone),
            "Ok" => Some(CtorOp::ResOk),
            "Err" => Some(CtorOp::ResErr),
            _ => None,
        };
        ctor.map_or_else(
            || {
                Err(Diag::type_error(
                    span,
                    format!("cannot find value `{name}` in this scope"),
                ))
            },
            |op| Ok(Resolved::Ctor(op)),
        )
    }

    fn resolve_two_segment(&mut self, path: &[ast::Ident], span: Span) -> Result<Resolved, Diag> {
        let head = &path[0].name;
        let tail = &path[1].name;
        if let Some(item) = self.item_env.get(head).copied()
            && let ItemKind::Enum(variants) = &self.sema.items[item.0 as usize].kind
            && let Some(index) = variants.iter().position(|(name, _)| name == tail)
        {
            return Ok(Resolved::Variant(item, table_index(index, path[1].span)?));
        }
        let ctor = match (head.as_str(), tail.as_str()) {
            ("String", "from") => Some(CtorOp::StringFrom),
            ("Vec", "new") => Some(CtorOp::VecNew),
            ("Vec", "with_capacity") => Some(CtorOp::VecWithCapacity),
            ("Box", "new") => Some(CtorOp::BoxNew),
            (map, "new") if self.map_alias.as_deref() == Some(map) => Some(CtorOp::MapNew),
            _ => None,
        };
        ctor.map_or_else(
            || {
                Err(Diag::type_error(
                    span,
                    format!("cannot find `{head}::{tail}` in this scope"),
                ))
            },
            |op| Ok(Resolved::Ctor(op)),
        )
    }

    fn check_struct_lit(
        &mut self,
        path: &[ast::Ident],
        fields: &[(ast::Ident, Option<ast::Expr>)],
        span: Span,
    ) -> Result<HirExpr, Diag> {
        // A struct literal names its struct as a type: a named-field
        // struct is not a value path, so it resolves here, not through
        // `resolve_path`.
        if let [name] = path {
            let item = self.item_env.get(&name.name).copied();
            if let Some(id) =
                item.filter(|id| matches!(self.sema.items[id.0 as usize].kind, ItemKind::Struct(_)))
            {
                let field_names = self.struct_field_names(id);
                return self.build_struct_lit(id, &field_names, fields, span);
            }
        }
        let resolved = self.resolve_path(path, span)?;
        match resolved {
            Resolved::Variant(id, index) => {
                let field_names = self.variant_field_names(id, index);
                self.build_variant_lit(id, index, &field_names, fields, span)
            }
            _ => Err(Diag::type_error(span, "this path does not name a struct")),
        }
    }

    fn struct_field_names(&self, id: crate::host::hir::ItemId) -> Vec<String> {
        match &self.sema.items[id.0 as usize].kind {
            ItemKind::Struct(fields) => fields.iter().map(|(name, _)| name.clone()).collect(),
            _ => Vec::new(),
        }
    }

    fn variant_field_names(&self, id: crate::host::hir::ItemId, index: u32) -> Vec<String> {
        match &self.sema.items[id.0 as usize].kind {
            ItemKind::Enum(variants) => variants[index as usize]
                .1
                .iter()
                .map(|(name, _)| name.clone())
                .collect(),
            _ => Vec::new(),
        }
    }

    fn build_struct_lit(
        &mut self,
        id: crate::host::hir::ItemId,
        field_names: &[String],
        fields: &[(ast::Ident, Option<ast::Expr>)],
        span: Span,
    ) -> Result<HirExpr, Diag> {
        let want_types = match &self.sema.items[id.0 as usize].kind {
            ItemKind::Struct(fields) => fields.iter().map(|(_, ty)| ty.clone()).collect(),
            _ => Vec::new(),
        };
        let checked = self.fill_fields(field_names, &want_types, fields, span)?;
        Ok(node(
            HirExprKind::StructLit(id, checked),
            HostTy::Struct(id),
            false,
            span,
        ))
    }

    fn build_variant_lit(
        &mut self,
        id: crate::host::hir::ItemId,
        index: u32,
        field_names: &[String],
        fields: &[(ast::Ident, Option<ast::Expr>)],
        span: Span,
    ) -> Result<HirExpr, Diag> {
        let want_types = match &self.sema.items[id.0 as usize].kind {
            ItemKind::Enum(variants) => variants[index as usize]
                .1
                .iter()
                .map(|(_, ty)| ty.clone())
                .collect(),
            _ => Vec::new(),
        };
        let checked = self.fill_fields(field_names, &want_types, fields, span)?;
        Ok(node(
            HirExprKind::VariantLit(id, index, checked),
            HostTy::Enum(id),
            false,
            span,
        ))
    }

    fn fill_fields(
        &mut self,
        field_names: &[String],
        want_types: &[HostTy],
        fields: &[(ast::Ident, Option<ast::Expr>)],
        span: Span,
    ) -> Result<Vec<HirExpr>, Diag> {
        let mut checked: Vec<Option<HirExpr>> = vec![None; field_names.len()];
        for (field_name, value) in fields {
            let Some(position) = field_names.iter().position(|name| *name == field_name.name)
            else {
                return Err(Diag::type_error(
                    field_name.span,
                    format!("no field `{}` here", field_name.name),
                ));
            };
            if checked[position].is_some() {
                return Err(Diag::type_error(
                    field_name.span,
                    format!("duplicate field `{}`", field_name.name),
                ));
            }
            let want = want_types[position].clone();
            let expr = match value {
                Some(value) => self.check_expr(value, Some(&want))?,
                None => self.check_shorthand(&field_name.name, &want, field_name.span)?,
            };
            checked[position] = Some(expr);
        }
        let mut filled = Vec::with_capacity(checked.len());
        for (slot, name) in checked.into_iter().zip(field_names) {
            match slot {
                Some(expr) => filled.push(expr),
                None => return Err(Diag::type_error(span, format!("missing field `{name}`"))),
            }
        }
        Ok(filled)
    }

    fn check_shorthand(&mut self, name: &str, want: &HostTy, span: Span) -> Result<HirExpr, Diag> {
        let Some(binding) = self.resolve_local(name) else {
            return Err(Diag::type_error(
                span,
                format!("cannot find value `{name}` in this scope"),
            ));
        };
        let mut checked = self.use_local(binding, Some(want), Use::Value, span)?;
        checked.ty = self.unify(&checked.ty, want, span)?;
        Ok(checked)
    }

    fn check_field(
        &mut self,
        base: &ast::Expr,
        name: &str,
        usage: Use,
        span: Span,
    ) -> Result<HirExpr, Diag> {
        let checked_base = self.check_expr_place(base, None)?;
        let base_ty = self.peel_refs(&checked_base.ty);
        let field_types = self.field_table(&base_ty, span)?;
        let Some(position) = field_types.iter().position(|(field, _)| *field == name) else {
            return Err(Diag::type_error(
                span,
                format!("no field `{name}` on this value"),
            ));
        };
        let ty = field_types[position].1.clone();
        if usage == Use::Value && !self.is_copy_ty(&ty) {
            self.move_out_of(&checked_base, span)?;
        }
        Ok(node(
            HirExprKind::Field {
                base: Box::new(checked_base),
                index: table_index(position, span)?,
            },
            ty,
            false,
            span,
        ))
    }

    fn field_table(&self, base_ty: &HostTy, span: Span) -> Result<Vec<(String, HostTy)>, Diag> {
        match base_ty {
            HostTy::Struct(id) => match &self.sema.items[id.0 as usize].kind {
                ItemKind::Struct(fields) => Ok(fields.clone()),
                _ => Err(Diag::type_error(span, "field access needs a struct")),
            },
            HostTy::Tuple(left, right) => Ok(vec![
                ("0".to_owned(), (**left).clone()),
                ("1".to_owned(), (**right).clone()),
            ]),
            HostTy::Enum(id) => match &self.sema.items[id.0 as usize].kind {
                ItemKind::Enum(variants) if variants.len() == 1 => Ok(variants[0].1.clone()),
                _ => Err(Diag::type_error(
                    span,
                    "field access on an enum needs a single-variant value",
                )),
            },
            _ => Err(Diag::type_error(span, "field access needs a product type")),
        }
    }

    /// Moves a non-`Copy` field out of `base` (a value use of `base.f`):
    /// legal out of an owned local or a temporary, and out of nothing
    /// borrowed. The whole local counts as moved.
    fn move_out_of(&mut self, base: &HirExpr, span: Span) -> Result<(), Diag> {
        let Some(binding) = self.moved_root(base, span)? else {
            return Ok(());
        };
        self.check_access(binding, Access::Move, span)?;
        self.note_capture_use(binding, Access::Move);
        Ok(())
    }

    /// The local an owning place expression is rooted in, or `None` for
    /// a temporary.
    ///
    /// # Errors
    /// An ownership error when the place lies behind a reference or in
    /// an indexed element, which cannot be moved out of (Rust E0507).
    fn moved_root(&self, expr: &HirExpr, span: Span) -> Result<Option<BindId>, Diag> {
        match &expr.kind {
            HirExprKind::Place { place, .. } => match &place.root {
                PlaceRoot::Local(binding) => Ok(Some(*binding)),
                PlaceRoot::Deref(_) => Err(Diag::ownership(
                    span,
                    "cannot move out of a value behind a reference",
                )),
            },
            HirExprKind::Field { base, .. } => {
                if matches!(self.deep(&base.ty), HostTy::Ref(..)) {
                    return Err(Diag::ownership(
                        span,
                        "cannot move out of a field behind a reference",
                    ));
                }
                self.moved_root(base, span)
            }
            HirExprKind::Index { .. } => Err(Diag::ownership(
                span,
                "cannot move out of an indexed element",
            )),
            _ => Ok(None),
        }
    }

    fn check_index(
        &mut self,
        base: &ast::Expr,
        index: &ast::Expr,
        usage: Use,
        span: Span,
    ) -> Result<HirExpr, Diag> {
        let checked_base = self.check_expr_place(base, None)?;
        let checked_index = self.check_expr(index, Some(&HostTy::Usize))?;
        let base_ty = self.peel_refs(&checked_base.ty);
        let elem = match base_ty {
            HostTy::Vec(inner) | HostTy::Array(inner, _) => *inner,
            _ => return Err(Diag::type_error(span, "indexing needs a vector or array")),
        };
        if usage == Use::Value && !self.is_copy_ty(&elem) {
            return Err(Diag::ownership(
                span,
                "cannot move out of an indexed element",
            ));
        }
        Ok(node(
            HirExprKind::Index {
                base: Box::new(checked_base),
                index: Box::new(checked_index),
            },
            elem,
            false,
            span,
        ))
    }

    fn check_call(
        &mut self,
        callee: &ast::Expr,
        args: &[ast::Expr],
        expect: Option<&HostTy>,
        span: Span,
    ) -> Result<HirExpr, Diag> {
        if let ast::ExprKind::Path(path) = &callee.kind {
            let resolved = self.resolve_path(path, callee.span)?;
            // A local names a function-pointer or closure value: it is
            // called indirectly, like any other callee expression.
            if !matches!(resolved, Resolved::Local(_)) {
                return self.check_resolved_call(resolved, args, expect, span);
            }
        }
        let checked_callee = self.check_expr_place(callee, None)?;
        // A boxed function or closure calls through the box, as the
        // standard library's `Fn` implementations for `Box<F>` do.
        let mut callee_ty = self.deep(&checked_callee.ty);
        while let HostTy::Box(inner) = callee_ty {
            callee_ty = self.deep(&inner);
        }
        let (params, ret, kind) = match callee_ty {
            HostTy::FnPtr(params, ret) => (params, *ret, None),
            HostTy::DynFn(kind, params, ret) => (params, *ret, Some(kind)),
            other => {
                return Err(Diag::type_error(
                    span,
                    format!("`{other:?}` is not callable"),
                ));
            }
        };
        if args.len() != params.len() {
            return Err(Diag::type_error(
                span,
                "this call has the wrong number of arguments",
            ));
        }
        let mut checked_args = Vec::with_capacity(args.len());
        for (arg, want) in args.iter().zip(&params) {
            checked_args.push(self.check_expr(arg, Some(want))?);
        }
        if kind == Some(ClosureKind::FnMut) {
            self.require_mutable_callee(callee, span)?;
        }
        if kind == Some(ClosureKind::FnOnce) {
            self.consume_callee(callee, span)?;
        }
        Ok(node(
            HirExprKind::IndirectCall {
                callee: Box::new(checked_callee),
                args: checked_args,
            },
            ret,
            false,
            span,
        ))
    }

    fn require_mutable_callee(&mut self, callee: &ast::Expr, span: Span) -> Result<(), Diag> {
        let Ok(place) = self.place_of(callee) else {
            return Ok(());
        };
        let PlaceRoot::Local(binding) = place.root else {
            return Ok(());
        };
        if !self.sema.bindings[binding.0 as usize].mutable {
            return Err(Diag::ownership(
                span,
                "calling an `FnMut` closure needs a mutable binding",
            ));
        }
        Ok(())
    }

    fn consume_callee(&mut self, callee: &ast::Expr, span: Span) -> Result<(), Diag> {
        if let Ok(place) = self.place_of(callee)
            && let PlaceRoot::Local(binding) = place.root
        {
            self.check_access(binding, Access::Move, span)?;
            self.note_capture_use(binding, Access::Move);
        }
        Ok(())
    }

    fn check_resolved_call(
        &mut self,
        resolved: Resolved,
        args: &[ast::Expr],
        expect: Option<&HostTy>,
        span: Span,
    ) -> Result<HirExpr, Diag> {
        match resolved {
            Resolved::Fun(id) => {
                let wants = self.fun_params[id.0 as usize].clone();
                if args.len() != wants.len() {
                    return Err(Diag::type_error(
                        span,
                        "this call has the wrong number of arguments",
                    ));
                }
                let mut checked = Vec::with_capacity(args.len());
                for (arg, want) in args.iter().zip(&wants) {
                    checked.push(self.check_expr(arg, Some(want))?);
                }
                let ret = self.sema.funs[id.0 as usize].ret.clone();
                Ok(node(
                    HirExprKind::Call {
                        callee: id,
                        args: checked,
                    },
                    ret,
                    false,
                    span,
                ))
            }
            Resolved::Ctor(op) => self.check_ctor(op, args, expect, span),
            Resolved::Variant(id, index) => self.build_variant_call(id, index, args, span),
            Resolved::TupleStruct(id) => self.build_tuple_struct_call(id, args, span),
            Resolved::UnitStruct(id) => {
                if !args.is_empty() {
                    return Err(Diag::type_error(span, "this struct takes no arguments"));
                }
                Ok(node(
                    HirExprKind::StructLit(id, Vec::new()),
                    HostTy::Struct(id),
                    false,
                    span,
                ))
            }
            other => Err(Diag::type_error(span, format!("{other:?} is not callable"))),
        }
    }

    fn build_tuple_struct_call(
        &mut self,
        id: crate::host::hir::ItemId,
        args: &[ast::Expr],
        span: Span,
    ) -> Result<HirExpr, Diag> {
        let want_types: Vec<HostTy> = match &self.sema.items[id.0 as usize].kind {
            ItemKind::Struct(fields) => fields.iter().map(|(_, ty)| ty.clone()).collect(),
            _ => Vec::new(),
        };
        if args.len() != want_types.len() {
            return Err(Diag::type_error(
                span,
                "this call has the wrong number of arguments",
            ));
        }
        let mut checked = Vec::with_capacity(args.len());
        for (arg, want) in args.iter().zip(&want_types) {
            checked.push(self.check_expr(arg, Some(want))?);
        }
        Ok(node(
            HirExprKind::StructLit(id, checked),
            HostTy::Struct(id),
            false,
            span,
        ))
    }

    fn build_variant_call(
        &mut self,
        id: crate::host::hir::ItemId,
        index: u32,
        args: &[ast::Expr],
        span: Span,
    ) -> Result<HirExpr, Diag> {
        let want_types: Vec<HostTy> = match &self.sema.items[id.0 as usize].kind {
            ItemKind::Enum(variants) => variants[index as usize]
                .1
                .iter()
                .map(|(_, ty)| ty.clone())
                .collect(),
            _ => Vec::new(),
        };
        if args.len() != want_types.len() {
            return Err(Diag::type_error(
                span,
                "this call has the wrong number of arguments",
            ));
        }
        let mut checked = Vec::with_capacity(args.len());
        for (arg, want) in args.iter().zip(&want_types) {
            checked.push(self.check_expr(arg, Some(want))?);
        }
        Ok(node(
            HirExprKind::VariantLit(id, index, checked),
            HostTy::Enum(id),
            false,
            span,
        ))
    }

    fn check_ctor(
        &mut self,
        op: CtorOp,
        args: &[ast::Expr],
        expect: Option<&HostTy>,
        span: Span,
    ) -> Result<HirExpr, Diag> {
        let (checked, ty) = match op {
            CtorOp::StringFrom => {
                let arg = expect_arg_count(args, 1, span)?;
                let checked = self.check_expr(&arg[0], Some(&HostTy::Str))?;
                (vec![checked], HostTy::String)
            }
            CtorOp::VecNew => {
                expect_arg_count(args, 0, span)?;
                let elem = self.fresh_cell(false);
                (Vec::new(), HostTy::Vec(Box::new(elem)))
            }
            CtorOp::VecWithCapacity => {
                let arg = expect_arg_count(args, 1, span)?;
                let checked = self.check_expr(&arg[0], Some(&HostTy::Usize))?;
                let elem = self.fresh_cell(false);
                (vec![checked], HostTy::Vec(Box::new(elem)))
            }
            CtorOp::MapNew => {
                expect_arg_count(args, 0, span)?;
                let value = self.fresh_cell(false);
                (Vec::new(), HostTy::HashMap(Box::new(value)))
            }
            CtorOp::BoxNew => {
                let arg = expect_arg_count(args, 1, span)?;
                // `Box<dyn Fn..>` is the boxed-closure type itself: a
                // closure boxed where one is expected unsizes into it.
                if let Some(want @ HostTy::DynFn(..)) = expect.map(|want| self.deep(want)) {
                    let checked = self.check_expr(&arg[0], Some(&want))?;
                    (vec![checked], want)
                } else {
                    let checked = self.check_expr(&arg[0], None)?;
                    let inner = checked.ty.clone();
                    (vec![checked], HostTy::Box(Box::new(inner)))
                }
            }
            CtorOp::OptSome | CtorOp::ResOk => {
                let arg = expect_arg_count(args, 1, span)?;
                let checked = self.check_expr(&arg[0], None)?;
                let inner = checked.ty.clone();
                let ty = if op == CtorOp::OptSome {
                    HostTy::Option(Box::new(inner))
                } else {
                    HostTy::Result(Box::new(inner), Box::new(self.fresh_cell(false)))
                };
                (vec![checked], ty)
            }
            CtorOp::OptNone => {
                expect_arg_count(args, 0, span)?;
                let ty = HostTy::Option(Box::new(self.fresh_cell(false)));
                (Vec::new(), ty)
            }
            CtorOp::ResErr => {
                let arg = expect_arg_count(args, 1, span)?;
                let checked = self.check_expr(&arg[0], None)?;
                let inner = checked.ty.clone();
                let ty = HostTy::Result(Box::new(self.fresh_cell(false)), Box::new(inner));
                (vec![checked], ty)
            }
        };
        Ok(node(HirExprKind::Ctor(op, checked), ty, false, span))
    }

    fn check_method(
        &mut self,
        receiver: &ast::Expr,
        name: &str,
        args: &[ast::Expr],
        span: Span,
    ) -> Result<HirExpr, Diag> {
        // A receiver is a place the method borrows (even a dereference
        // root reads without moving, so `(*slot).clone()` leaves the
        // referent live like the native borrow); only the by-value
        // iterator methods consume it, and the engines take the
        // receiver of `into_iter` explicitly.
        let checked_receiver = self.check_expr_place(receiver, None)?;
        let recv_ty = self.deep(&checked_receiver.ty);
        if name == "zip" {
            self.consume_receiver(&checked_receiver, span)?;
            return self.check_zip(receiver, checked_receiver, args, span);
        }
        let op = self.method_op(&recv_ty, name, span)?;
        let receiver_place = self.place_of(receiver).ok();
        self.enforce_receiver_mutability(op, receiver_place.as_ref(), span)?;
        if matches!(op, MethodOp::IntoIter | MethodOp::Enumerate) {
            self.consume_receiver(&checked_receiver, span)?;
        }
        // Methods returning references or iterator adaptors borrow their
        // receiver: the loan keeps mutation out while the result is live.
        let borrow = match op {
            MethodOp::Iter | MethodOp::VecGet | MethodOp::MapGet | MethodOp::BoxAsRef => Some(false),
            MethodOp::IterMut | MethodOp::VecGetMut | MethodOp::MapGetMut | MethodOp::BoxAsMut => {
                Some(true)
            }
            _ => None,
        };
        if let Some(mutable) = borrow
            && let Some(place) = &receiver_place
            && let PlaceRoot::Local(binding) = place.root
        {
            let access = if mutable {
                Access::BorrowMut
            } else {
                Access::BorrowShared
            };
            self.check_access(binding, access, span)?;
            self.begin_loan(binding, mutable, None);
        }
        let (params, ret) = self.method_signature(op, &recv_ty, span)?;
        if args.len() != params.len() {
            return Err(Diag::type_error(
                span,
                "this method has the wrong number of arguments",
            ));
        }
        let mut checked_args = Vec::with_capacity(args.len());
        for (arg, want) in args.iter().zip(&params) {
            checked_args.push(self.check_expr(arg, Some(want))?);
        }
        Ok(node(
            HirExprKind::Method {
                op,
                receiver: Box::new(checked_receiver),
                receiver_place,
                args: checked_args,
            },
            ret,
            false,
            span,
        ))
    }

    /// A by-value method (`into_iter`, `enumerate`, `zip`) takes its
    /// receiver: a non-`Copy` one moves out of its place.
    fn consume_receiver(&mut self, receiver: &HirExpr, span: Span) -> Result<(), Diag> {
        if self.is_copy_ty(&receiver.ty) {
            return Ok(());
        }
        self.move_out_of(receiver, span)
    }

    fn check_zip(
        &mut self,
        receiver: &ast::Expr,
        checked_receiver: HirExpr,
        args: &[ast::Expr],
        span: Span,
    ) -> Result<HirExpr, Diag> {
        let arg = expect_arg_count(args, 1, span)?;
        let checked_arg = self.check_expr(&arg[0], None)?;
        if self.iter_item(&checked_arg.ty, arg[0].span).is_err() {
            return Err(Diag::type_error(
                arg[0].span,
                "`zip` needs an iterable argument",
            ));
        }
        if self.iter_item(&checked_receiver.ty, receiver.span).is_err() {
            return Err(Diag::type_error(
                receiver.span,
                "`zip` needs an iterable receiver",
            ));
        }
        let ty = HostTy::Zip(
            Box::new(checked_receiver.ty.clone()),
            Box::new(checked_arg.ty.clone()),
        );
        Ok(node(
            HirExprKind::Method {
                op: MethodOp::Zip,
                receiver: Box::new(checked_receiver),
                receiver_place: None,
                args: vec![checked_arg],
            },
            ty,
            false,
            span,
        ))
    }

    fn enforce_receiver_mutability(
        &mut self,
        op: MethodOp,
        receiver_place: Option<&Place>,
        span: Span,
    ) -> Result<(), Diag> {
        let needs_place = matches!(
            op,
            MethodOp::VecPush
                | MethodOp::VecPop
                | MethodOp::StrPushStr
                | MethodOp::MapInsert
                | MethodOp::MapRemove
                | MethodOp::VecGet
                | MethodOp::VecGetMut
                | MethodOp::MapGet
                | MethodOp::MapGetMut
                | MethodOp::BoxAsRef
                | MethodOp::BoxAsMut
                | MethodOp::Iter
                | MethodOp::IterMut
                | MethodOp::Next
        );
        if !needs_place {
            return Ok(());
        }
        let Some(place) = receiver_place else {
            return Err(Diag::ownership(span, "this method needs a place receiver"));
        };
        let needs_mut = matches!(
            op,
            MethodOp::VecPush
                | MethodOp::VecPop
                | MethodOp::StrPushStr
                | MethodOp::MapInsert
                | MethodOp::MapRemove
                | MethodOp::VecGetMut
                | MethodOp::MapGetMut
                | MethodOp::BoxAsMut
                | MethodOp::IterMut
                | MethodOp::Next
        );
        if needs_mut {
            return self.check_mutation_through(place, span);
        }
        Ok(())
    }

    fn method_op(&mut self, recv: &HostTy, name: &str, span: Span) -> Result<MethodOp, Diag> {
        let peeled = match recv {
            HostTy::Ref(_, inner) => self.deep(inner),
            other => other.clone(),
        };
        let op = match (kind_of(&peeled), name) {
            (TypeKind::String, "push_str") => MethodOp::StrPushStr,
            (TypeKind::String, "push") => {
                return Err(Diag::unsupported(
                    span,
                    "`String::push` takes `char`, which is outside the subset; \
                     use `push_str`",
                ));
            }
            (TypeKind::String, "as_str") => MethodOp::AsStr,
            (TypeKind::String, "len") => MethodOp::StrLen,
            (TypeKind::Vec, "push") => MethodOp::VecPush,
            (TypeKind::Vec, "pop") => MethodOp::VecPop,
            (TypeKind::Vec, "len") => MethodOp::VecLen,
            (TypeKind::Vec, "is_empty") => MethodOp::VecIsEmpty,
            (TypeKind::Vec, "get") => MethodOp::VecGet,
            (TypeKind::Vec, "get_mut") => MethodOp::VecGetMut,
            (TypeKind::Vec | TypeKind::Map | TypeKind::Array, "iter") => MethodOp::Iter,
            (TypeKind::Vec | TypeKind::Array, "iter_mut") => MethodOp::IterMut,
            (TypeKind::Vec | TypeKind::Array, "into_iter") => MethodOp::IntoIter,
            (TypeKind::Map, "insert") => MethodOp::MapInsert,
            (TypeKind::Map, "get") => MethodOp::MapGet,
            (TypeKind::Map, "get_mut") => MethodOp::MapGetMut,
            (TypeKind::Map, "contains_key") => MethodOp::MapContainsKey,
            (TypeKind::Map, "remove") => MethodOp::MapRemove,
            (TypeKind::Map, "len") => MethodOp::MapLen,
            (TypeKind::Map, "is_empty") => MethodOp::MapIsEmpty,
            (TypeKind::Box, "as_ref") => MethodOp::BoxAsRef,
            (TypeKind::Box, "as_mut") => MethodOp::BoxAsMut,
            (_, "clone") => {
                self.require_clone_receiver(recv, span)?;
                MethodOp::Clone
            }
            (
                TypeKind::Iter
                | TypeKind::IterMut
                | TypeKind::IntoIter
                | TypeKind::Range
                | TypeKind::Enumerate,
                "next",
            ) => MethodOp::Next,
            (
                TypeKind::Iter
                | TypeKind::IterMut
                | TypeKind::IntoIter
                | TypeKind::Range
                | TypeKind::Enumerate,
                "enumerate",
            ) => MethodOp::Enumerate,
            _ => {
                return Err(Diag::unsupported(
                    span,
                    format!("method `{name}` is outside the admitted allowlist"),
                ));
            }
        };
        Ok(op)
    }

    fn method_signature(
        &mut self,
        op: MethodOp,
        recv: &HostTy,
        span: Span,
    ) -> Result<(Vec<HostTy>, HostTy), Diag> {
        let peeled = match recv {
            HostTy::Ref(_, inner) => self.deep(inner),
            other => other.clone(),
        };
        Ok(match op {
            MethodOp::StrPushStr => (vec![HostTy::Str], HostTy::Unit),
            MethodOp::AsStr => (Vec::new(), HostTy::Str),
            MethodOp::StrLen | MethodOp::VecLen | MethodOp::MapLen => (Vec::new(), HostTy::Usize),
            MethodOp::VecIsEmpty | MethodOp::MapIsEmpty => (Vec::new(), HostTy::Bool),
            MethodOp::VecPush => (vec![*vec_elem(peeled, "push", span)?], HostTy::Unit),
            MethodOp::VecPop => (Vec::new(), HostTy::Option(vec_elem(peeled, "pop", span)?)),
            MethodOp::VecGet | MethodOp::VecGetMut => {
                let mutable = op == MethodOp::VecGetMut;
                let method = if mutable { "get_mut" } else { "get" };
                let elem = vec_elem(peeled, method, span)?;
                (
                    vec![HostTy::Usize],
                    HostTy::Option(Box::new(HostTy::Ref(mutable, elem))),
                )
            }
            MethodOp::MapInsert => {
                let value = map_value(peeled, "insert", span)?;
                (
                    vec![HostTy::String, value.as_ref().clone()],
                    HostTy::Option(value),
                )
            }
            MethodOp::MapGet | MethodOp::MapGetMut => {
                let mutable = op == MethodOp::MapGetMut;
                let method = if mutable { "get_mut" } else { "get" };
                let value = map_value(peeled, method, span)?;
                (
                    vec![HostTy::Str],
                    HostTy::Option(Box::new(HostTy::Ref(mutable, value))),
                )
            }
            MethodOp::MapContainsKey => (vec![HostTy::Str], HostTy::Bool),
            MethodOp::MapRemove => (
                vec![HostTy::Str],
                HostTy::Option(map_value(peeled, "remove", span)?),
            ),
            MethodOp::BoxAsRef | MethodOp::BoxAsMut => {
                let mutable = op == MethodOp::BoxAsMut;
                let HostTy::Box(inner) = peeled else {
                    let method = if mutable { "as_mut" } else { "as_ref" };
                    return Err(Diag::type_error(span, format!("`{method}` needs a `Box`")));
                };
                (Vec::new(), HostTy::Ref(mutable, inner))
            }
            // Method resolution finds `T::clone` for a `&T` receiver
            // first (auto-ref), so `(&T).clone()` yields an owned `T`.
            MethodOp::Clone => match self.deep(recv) {
                HostTy::Ref(_, inner) if self.implements(&inner, Trait::Clone) => {
                    (Vec::new(), *inner)
                }
                other => (Vec::new(), other),
            },
            MethodOp::Iter => (Vec::new(), iter_type(peeled, span)?),
            MethodOp::IterMut => (
                Vec::new(),
                HostTy::IterMut(collection_elem(peeled, "iter_mut", span)?),
            ),
            MethodOp::IntoIter => (
                Vec::new(),
                HostTy::IntoIter(collection_elem(peeled, "into_iter", span)?),
            ),
            MethodOp::Next => {
                let item = self.iter_item(recv, span)?;
                (Vec::new(), HostTy::Option(Box::new(item)))
            }
            MethodOp::Enumerate => (Vec::new(), HostTy::Enumerate(Box::new(recv.clone()))),
            MethodOp::Zip => unreachable!("zip is checked through check_zip"),
        })
    }

    fn iter_item(&self, ty: &HostTy, span: Span) -> Result<HostTy, Diag> {
        match self.deep(ty) {
            HostTy::Range(inner) | HostTy::IntoIter(inner) => Ok(*inner),
            HostTy::Iter(inner) => Ok(HostTy::Ref(false, inner)),
            HostTy::IterMut(inner) => Ok(HostTy::Ref(true, inner)),
            HostTy::Enumerate(inner) => {
                let item = self.iter_item(&inner, span)?;
                Ok(HostTy::Tuple(Box::new(HostTy::Usize), Box::new(item)))
            }
            HostTy::Zip(left, right) => {
                let a = self.iter_item(&left, span)?;
                let b = self.iter_item(&right, span)?;
                Ok(HostTy::Tuple(Box::new(a), Box::new(b)))
            }
            other => Err(Diag::type_error(
                span,
                format!("`{other:?}` is not iterable"),
            )),
        }
    }

    /// Whether `ty` is a reference and which kind: `Some(shared)`
    /// for `&T`, `Some(exclusive)` for `&mut T`, `None` otherwise.
    fn borrowed_as(&self, ty: &HostTy) -> (HostTy, Option<bool>) {
        match self.deep(ty) {
            HostTy::Ref(shared, inner) => ((*inner).clone(), Some(shared)),
            other => (other, None),
        }
    }

    fn check_unary(
        &mut self,
        op: UnOp,
        operand: &ast::Expr,
        usage: Use,
        span: Span,
    ) -> Result<HirExpr, Diag> {
        match op {
            UnOp::Neg => {
                let checked = self.check_expr(operand, Some(&HostTy::I64))?;
                Ok(node(
                    HirExprKind::Unary {
                        op,
                        operand: Box::new(checked),
                    },
                    HostTy::I64,
                    false,
                    span,
                ))
            }
            UnOp::Not => {
                let checked = self.check_expr(operand, Some(&HostTy::Bool))?;
                Ok(node(
                    HirExprKind::Unary {
                        op,
                        operand: Box::new(checked),
                    },
                    HostTy::Bool,
                    false,
                    span,
                ))
            }
            UnOp::Deref => {
                let checked = self.check_expr_place(operand, None)?;
                let deep = self.deep(&checked.ty);
                let (_, referent) = deep
                    .referent()
                    .ok_or_else(|| Diag::type_error(span, "dereference needs a reference"))?;
                let referent = referent.clone();
                if usage == Use::Value && !self.is_copy_ty(&referent) {
                    return Err(Diag::ownership(
                        span,
                        "cannot move out of a value behind a reference",
                    ));
                }
                Ok(node(
                    HirExprKind::Place {
                        place: Place {
                            root: PlaceRoot::Deref(Box::new(checked)),
                            proj: Vec::new(),
                            span,
                        },
                        mode: PlaceUse::Read,
                    },
                    referent,
                    false,
                    span,
                ))
            }
            UnOp::Ref | UnOp::RefMut => {
                let mutable = op == UnOp::RefMut;
                if !self.is_place_expr(operand) {
                    return self.borrow_temporary(op, operand, span);
                }
                let place = self.place_of(operand)?;
                self.check_place_borrow(&place, mutable, span)?;
                let inner = self.place_ty(&place)?;
                let checked_operand = node(
                    HirExprKind::Place {
                        place: place.clone(),
                        mode: PlaceUse::Read,
                    },
                    inner.clone(),
                    false,
                    operand.span,
                );
                Ok(node(
                    HirExprKind::Unary {
                        op,
                        operand: Box::new(checked_operand),
                    },
                    HostTy::Ref(mutable, Box::new(inner)),
                    false,
                    span,
                ))
            }
        }
    }

    /// Whether `expr` is a place expression (a local, a field or index
    /// of a place, or a dereference); any other operand of `&` is a
    /// value expression borrowed through a temporary.
    fn is_place_expr(&self, expr: &ast::Expr) -> bool {
        match &expr.kind {
            ast::ExprKind::Path(path) => {
                path.len() == 1 && self.lookup_depth(&path[0].name).is_some()
            }
            ast::ExprKind::Field { base, .. } | ast::ExprKind::Index { base, .. } => {
                self.is_place_expr(base)
            }
            ast::ExprKind::Unary {
                op: UnOp::Deref, ..
            } => true,
            _ => false,
        }
    }

    /// `&value` of a value expression: Rust materializes the value in
    /// a temporary and borrows it. The temporary is an anonymous slot
    /// of the current frame, initialized and then borrowed.
    fn borrow_temporary(
        &mut self,
        op: UnOp,
        operand: &ast::Expr,
        span: Span,
    ) -> Result<HirExpr, Diag> {
        let mutable = op == UnOp::RefMut;
        let value = self.check_expr(operand, None)?;
        let ty = value.ty.clone();
        let binding = self.fresh_bind("<temporary>", ty.clone(), mutable);
        let read = node(
            HirExprKind::Place {
                place: Place {
                    root: PlaceRoot::Local(binding),
                    proj: Vec::new(),
                    span: operand.span,
                },
                mode: PlaceUse::Read,
            },
            ty.clone(),
            false,
            operand.span,
        );
        let borrow = node(
            HirExprKind::Unary {
                op,
                operand: Box::new(read),
            },
            HostTy::Ref(mutable, Box::new(ty.clone())),
            false,
            span,
        );
        let block = HirBlock {
            stmts: vec![HirStmt::Let {
                binding,
                destruct: None,
                value,
            }],
            tail: Some(Box::new(borrow)),
        };
        Ok(node(
            HirExprKind::Block(block),
            HostTy::Ref(mutable, Box::new(ty)),
            false,
            span,
        ))
    }

    fn check_place_borrow(&mut self, place: &Place, mutable: bool, span: Span) -> Result<(), Diag> {
        match &place.root {
            PlaceRoot::Local(binding) => {
                let binding = *binding;
                let access = if mutable {
                    Access::BorrowMut
                } else {
                    Access::BorrowShared
                };
                self.check_access(binding, access, span)?;
                self.begin_loan(binding, mutable, None);
                Ok(())
            }
            PlaceRoot::Deref(_) => Ok(()),
        }
    }

    fn place_ty(&self, place: &Place) -> Result<HostTy, Diag> {
        let mut ty = match &place.root {
            PlaceRoot::Local(binding) => self.sema.bindings[binding.0 as usize].ty.clone(),
            PlaceRoot::Deref(expr) => {
                let base_ty = self.deep(&expr.ty);
                let (_, referent) = base_ty
                    .referent()
                    .ok_or_else(|| Diag::type_error(expr.span, "dereference needs a reference"))?;
                referent.clone()
            }
        };
        for proj in &place.proj {
            ty = match proj {
                Proj::Field(index) => {
                    let fields = self.field_table(&self.peel_refs(&ty), place.root_span())?;
                    fields[*index as usize].1.clone()
                }
                Proj::Index(_) => match self.peel_refs(&ty) {
                    HostTy::Vec(elem) | HostTy::Array(elem, _) => *elem,
                    _ => return Err(Diag::type_error(place.root_span(), "not indexable")),
                },
            };
        }
        Ok(ty)
    }

    fn place_of(&mut self, expr: &ast::Expr) -> Result<Place, Diag> {
        match &expr.kind {
            ast::ExprKind::Path(path) if path.len() == 1 => {
                let name = &path[0].name;
                let Some(binding) = self.resolve_local(name) else {
                    return Err(Diag::type_error(
                        expr.span,
                        format!("`{name}` is not a place"),
                    ));
                };
                Ok(Place {
                    root: PlaceRoot::Local(binding),
                    proj: Vec::new(),
                    span: expr.span,
                })
            }
            ast::ExprKind::Field { base, name } => {
                let mut place = self.place_of(base)?;
                let base_ty = self.place_ty(&place)?;
                let fields = self.field_table(&self.peel_refs(&base_ty), expr.span)?;
                let Some(position) = fields.iter().position(|(field, _)| *field == name.name)
                else {
                    return Err(Diag::type_error(
                        name.span,
                        format!("no field `{}` on this value", name.name),
                    ));
                };
                place
                    .proj
                    .push(Proj::Field(table_index(position, name.span)?));
                Ok(place)
            }
            ast::ExprKind::Index { base, index } => {
                let mut place = self.place_of(base)?;
                let checked_index = self.check_expr(index, Some(&HostTy::Usize))?;
                place.proj.push(Proj::Index(Box::new(checked_index)));
                Ok(place)
            }
            ast::ExprKind::Unary {
                op: UnOp::Deref,
                operand,
            } => {
                let checked = self.check_expr(operand, None)?;
                if self.deep(&checked.ty).referent().is_none() {
                    return Err(Diag::type_error(expr.span, "dereference needs a reference"));
                }
                Ok(Place {
                    root: PlaceRoot::Deref(Box::new(checked)),
                    proj: Vec::new(),
                    span: expr.span,
                })
            }
            _ => Err(Diag::type_error(
                expr.span,
                "this expression is not a place",
            )),
        }
    }

    fn check_binary(
        &mut self,
        op: BinOp,
        left: &ast::Expr,
        right: &ast::Expr,
        span: Span,
    ) -> Result<HirExpr, Diag> {
        if matches!(op, BinOp::And | BinOp::Or) {
            let l = self.check_expr(left, Some(&HostTy::Bool))?;
            let r = self.check_expr(right, Some(&HostTy::Bool))?;
            return Ok(node(
                HirExprKind::Binary {
                    op,
                    left: Box::new(l),
                    right: Box::new(r),
                },
                HostTy::Bool,
                false,
                span,
            ));
        }
        // Operators borrow their operands (`a == b` is `PartialEq::eq(&a, &b)`).
        let l = self.check_expr_place(left, None)?;
        let r = self.check_expr_place(right, Some(&l.ty))?;
        let ty = self.deep(&l.ty);
        match op {
            BinOp::Add | BinOp::Sub => {
                if !self.admits_integer(&ty) {
                    return Err(Diag::type_error(
                        span,
                        "arithmetic needs `i64` or `usize` operands",
                    ));
                }
            }
            BinOp::Mul | BinOp::Div | BinOp::Rem => {
                if !self.admits_integer(&ty) || matches!(ty, HostTy::Usize) {
                    return Err(Diag::type_error(
                        span,
                        "`*`, `/`, and `%` need `i64` operands",
                    ));
                }
                if let HostTy::Infer(_) = ty {
                    self.i64_obligations.push((ty.clone(), span));
                }
            }
            BinOp::Eq | BinOp::Ne => {
                self.require_trait(&ty, Trait::PartialEq, span)?;
                return Ok(bool_binary(op, l, r, span));
            }
            BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge => {
                let ordered = match ty {
                    HostTy::String => true,
                    _ => self.admits_integer(&ty),
                };
                if !ordered {
                    return Err(Diag::type_error(
                        span,
                        format!("`{ty:?}` does not admit ordering"),
                    ));
                }
                return Ok(bool_binary(op, l, r, span));
            }
            BinOp::And | BinOp::Or => {}
        }
        Ok(node(
            HirExprKind::Binary {
                op,
                left: Box::new(l),
                right: Box::new(r),
            },
            ty,
            false,
            span,
        ))
    }

    fn check_assign(
        &mut self,
        op: Option<BinOp>,
        target: &ast::Expr,
        value: &ast::Expr,
        span: Span,
    ) -> Result<HirExpr, Diag> {
        let place = self.place_of(target)?;
        let target_ty = self.place_ty(&place)?;
        if op.is_some() && !self.admits_integer(&self.deep(&target_ty)) {
            return Err(Diag::type_error(
                span,
                "`+=` and `-=` need `i64` or `usize` places",
            ));
        }
        // Rust evaluates the value first, then writes the place: the
        // write re-initializes a place the value moved out of
        // (`list = Cons(1, Box::new(list))`), and the borrows the value
        // took (`v = f(&v)`) end before it.
        let loans_before = self.ctx().loans.len();
        let checked = self.check_expr(value, Some(&target_ty))?;
        let loans = &mut self.ctx_mut().loans;
        let taken = loans.split_off(loans_before);
        loans.extend(taken.into_iter().filter(|loan| loan.holder.is_some()));
        self.check_assign_place(&place, span)?;
        Ok(node(
            HirExprKind::Assign {
                op,
                target: place,
                value: Box::new(checked),
            },
            HostTy::Unit,
            false,
            span,
        ))
    }

    fn check_assign_place(&mut self, place: &Place, span: Span) -> Result<(), Diag> {
        if let PlaceRoot::Local(binding) = place.root
            && place.proj.is_empty()
        {
            self.check_access(binding, Access::Write, span)?;
            self.reinit(binding);
            return Ok(());
        }
        self.check_mutation_through(place, span)
    }

    /// Mutating a projection of a place, or calling a mutating method
    /// on it, writes through the place's root: a local root that holds
    /// a reference must hold `&mut` (the binding itself need not be
    /// `mut`), any other local root must be a `mut` binding, and a
    /// dereference root must dereference `&mut`.
    fn check_mutation_through(&mut self, place: &Place, span: Span) -> Result<(), Diag> {
        let (root_ty, binding) = match &place.root {
            PlaceRoot::Local(binding) => {
                (self.sema.bindings[binding.0 as usize].ty.clone(), *binding)
            }
            PlaceRoot::Deref(expr) => {
                let HirExprKind::Place { place: inner, .. } = &expr.kind else {
                    return self.require_mut_ref(&expr.ty, span);
                };
                let PlaceRoot::Local(binding) = inner.root else {
                    return self.require_mut_ref(&expr.ty, span);
                };
                self.require_mut_ref(&expr.ty, span)?;
                self.note_capture_use(binding, Access::Write);
                return Ok(());
            }
        };
        if let HostTy::Ref(..) = self.deep(&root_ty) {
            self.require_mut_ref(&root_ty, span)?;
            self.note_capture_use(binding, Access::Write);
            return Ok(());
        }
        self.check_access(binding, Access::Write, span)
    }

    fn require_mut_ref(&self, ty: &HostTy, span: Span) -> Result<(), Diag> {
        match self.deep(ty) {
            HostTy::Ref(false, _) => Err(Diag::ownership(
                span,
                "cannot mutate through a shared reference",
            )),
            _ => Ok(()),
        }
    }

    fn check_if(
        &mut self,
        test: &ast::Expr,
        then: &ast::Block,
        else_branch: Option<&ast::Expr>,
        expect: Option<&HostTy>,
        span: Span,
    ) -> Result<HirExpr, Diag> {
        let checked_test = self.check_expr(test, Some(&HostTy::Bool))?;
        let want = expect.cloned().unwrap_or(HostTy::Unit);
        let mut branches = BranchMoves::start(self);
        let then_block = self.check_block_body(then, Some(&want))?;
        let then_diverges = then_block.tail.as_ref().is_some_and(|tail| tail.diverges);
        let then_expr = node(
            HirExprKind::Block(then_block),
            want.clone(),
            then_diverges,
            then.span,
        );
        branches.end_branch(self, then_expr.diverges);
        let else_expr = if let Some(branch) = else_branch {
            self.check_expr(branch, Some(&want))?
        } else {
            if want != HostTy::Unit {
                return Err(Diag::type_error(
                    span,
                    "an `if` used as a value needs an `else`",
                ));
            }
            node(HirExprKind::Unit, HostTy::Unit, false, span)
        };
        branches.end_branch(self, else_expr.diverges);
        branches.finish(self);
        let diverges = then_expr.diverges && else_expr.diverges;
        Ok(node(
            HirExprKind::If {
                test: Box::new(checked_test),
                then: Box::new(then_expr),
                else_branch: Box::new(else_expr),
            },
            want,
            diverges,
            span,
        ))
    }

    fn check_if_let(
        &mut self,
        pat: &ast::Pat,
        value: &ast::Expr,
        then: &ast::Block,
        else_branch: Option<&ast::Expr>,
        expect: Option<&HostTy>,
        span: Span,
    ) -> Result<HirExpr, Diag> {
        let checked_value = self.check_expr_place(value, None)?;
        let wanted = checked_value.ty.clone();
        let want = expect.cloned().unwrap_or(HostTy::Unit);
        let mut branches = BranchMoves::start(self);
        self.ctx_mut().scopes.push(HashMap::new());
        let checked_pat = self.check_pattern(pat, &wanted)?;
        self.move_bound_by_value(&checked_value, &checked_pat, value.span)?;
        let then_block = self.check_block_body(then, Some(&want))?;
        self.pop_scope();
        let then_diverges = then_block.tail.as_ref().is_some_and(|tail| tail.diverges);
        let then_expr = node(
            HirExprKind::Block(then_block),
            want.clone(),
            then_diverges,
            then.span,
        );
        branches.end_branch(self, then_expr.diverges);
        let else_expr = if let Some(branch) = else_branch {
            self.check_expr(branch, Some(&want))?
        } else {
            if self.deep(&want) != HostTy::Unit {
                return Err(Diag::type_error(
                    span,
                    "an `if let` used as a value needs an `else`",
                ));
            }
            node(HirExprKind::Unit, HostTy::Unit, false, span)
        };
        branches.end_branch(self, else_expr.diverges);
        branches.finish(self);
        let diverges = then_expr.diverges && else_expr.diverges;
        Ok(node(
            HirExprKind::IfLet {
                pat: checked_pat,
                value: Box::new(checked_value),
                then: Box::new(then_expr),
                else_branch: Box::new(else_expr),
            },
            want,
            diverges,
            span,
        ))
    }

    fn check_match(
        &mut self,
        scrutinee: &ast::Expr,
        arms: &[ast::MatchArm],
        expect: Option<&HostTy>,
        span: Span,
    ) -> Result<HirExpr, Diag> {
        let checked_value = self.check_expr_place(scrutinee, None)?;
        let wanted = checked_value.ty.clone();
        let want = expect.cloned().unwrap_or_else(|| self.fresh_cell(false));
        let mut branches = BranchMoves::start(self);
        let mut checked_arms = Vec::with_capacity(arms.len());
        for arm in arms {
            self.ctx_mut().scopes.push(HashMap::new());
            let pat = self.check_pattern(&arm.pat, &wanted)?;
            self.move_bound_by_value(&checked_value, &pat, scrutinee.span)?;
            let body = self.check_expr(&arm.body, Some(&want))?;
            self.pop_scope();
            branches.end_branch(self, body.diverges);
            checked_arms.push((pat, body));
        }
        branches.finish(self);
        self.check_exhaustive(&checked_arms, &wanted, span)?;
        let diverges = checked_arms.iter().all(|(_, body)| body.diverges);
        Ok(node(
            HirExprKind::Match {
                scrutinee: Box::new(checked_value),
                arms: checked_arms,
            },
            want,
            diverges,
            span,
        ))
    }

    /// A pattern that binds a non-`Copy` part of the scrutinee by value
    /// moves it out of the matched place (Rust binds by move unless the
    /// scrutinee is a reference, whose bindings borrow); the place is
    /// then unusable in the arm and after it. A scrutinee that is a
    /// temporary has nothing left to use.
    fn move_bound_by_value(
        &mut self,
        scrutinee: &HirExpr,
        pat: &HirPat,
        span: Span,
    ) -> Result<(), Diag> {
        if !self.binds_by_move(pat) {
            return Ok(());
        }
        self.move_out_of(scrutinee, span)
    }

    fn binds_by_move(&self, pat: &HirPat) -> bool {
        match &pat.kind {
            HirPatKind::Bind(binding) => {
                let ty = &self.sema.bindings[binding.0 as usize].ty;
                !matches!(self.deep(ty), HostTy::Ref(..)) && !self.is_copy_ty(ty)
            }
            HirPatKind::Tuple(left, right) => self.binds_by_move(left) || self.binds_by_move(right),
            HirPatKind::TuplePath(_, subs) | HirPatKind::StructPath(_, subs) => {
                subs.iter().any(|sub| self.binds_by_move(sub))
            }
            HirPatKind::Wild
            | HirPatKind::I64(_)
            | HirPatKind::Usize(_)
            | HirPatKind::Bool(_)
            | HirPatKind::UnitPath(_) => false,
        }
    }

    fn check_loop(&mut self, body: &ast::Block, span: Span) -> Result<HirExpr, Diag> {
        let cell = self.fresh_cell(false);
        let mark = self.loop_mark();
        let (checked, saw_break) =
            self.check_loop_block(body, cell.clone(), mark, LoopEntry::Unconditional)?;
        if saw_break {
            Ok(node(
                HirExprKind::Loop {
                    body: checked,
                    break_ty: cell.clone(),
                },
                cell,
                false,
                span,
            ))
        } else {
            Ok(node(
                HirExprKind::Loop {
                    body: checked,
                    break_ty: HostTy::Unit,
                },
                HostTy::Unit,
                true,
                span,
            ))
        }
    }

    /// The move state at the top of a loop, taken before its header.
    fn loop_mark(&self) -> LoopMark {
        LoopMark {
            moved: self.ctx().uninit.clone(),
            next_bind: self.next_bind,
        }
    }

    /// Checks a loop body and settles the loop's move analysis: a value
    /// moved out of a binding declared before the loop, and not moved
    /// on entry, is gone at the next iteration (Rust E0382), and the
    /// code after the loop starts from the states its exits leave.
    fn check_loop_block(
        &mut self,
        body: &ast::Block,
        break_ty: HostTy,
        mark: LoopMark,
        entry: LoopEntry,
    ) -> Result<(HirBlock, bool), Diag> {
        self.ctx_mut().loops.push(LoopCtx::new(break_ty));
        let checked = self.check_block_body(body, Some(&HostTy::Unit))?;
        let Some(done) = self.ctx_mut().loops.pop() else {
            return Err(Diag::type_error(body.span, "loop state underflow"));
        };
        let mut iteration_ends = done.continue_moves;
        if !block_diverges(&checked) {
            iteration_ends.push(self.ctx().uninit.clone());
        }
        for state in &iteration_ends {
            let recurring = state
                .iter()
                .find(|binding| binding.0 < mark.next_bind && !mark.moved.contains(binding));
            if let Some(binding) = recurring {
                return Err(Diag::ownership(
                    body.span,
                    format!(
                        "value `{}` is moved in a previous iteration of this loop",
                        self.sema.bindings[binding.0 as usize].name
                    ),
                ));
            }
        }
        let mut exits = done.break_moves;
        if entry == LoopEntry::Guarded {
            exits.push(mark.moved);
            exits.extend(iteration_ends);
        }
        if !exits.is_empty() {
            let mut joined: Vec<BindId> = Vec::new();
            for binding in exits.into_iter().flatten() {
                if !joined.contains(&binding) {
                    joined.push(binding);
                }
            }
            self.ctx_mut().uninit = joined;
        }
        Ok((checked, done.saw_break))
    }

    fn check_for(
        &mut self,
        pat: &ast::Pat,
        iterable: &ast::Expr,
        body: &ast::Block,
        span: Span,
    ) -> Result<HirExpr, Diag> {
        let checked_iterable = self.check_expr(iterable, None)?;
        // The grammar's `for` accepts a range, an owned array or `Vec`
        // (by value), a borrowed one (by `&`/`&mut`), or an admitted
        // iterator value.
        let item_ty = match self.peel_refs(&checked_iterable.ty) {
            HostTy::Vec(elem) | HostTy::Array(elem, _) => {
                let (_, mutable) = self.borrowed_as(&checked_iterable.ty);
                let elem = *elem;
                match mutable {
                    None => elem,
                    Some(shared) => HostTy::Ref(shared, Box::new(elem)),
                }
            }
            _ => self.iter_item(&checked_iterable.ty, iterable.span)?,
        };
        let mark = self.loop_mark();
        self.ctx_mut().scopes.push(HashMap::new());
        let checked_pat = self.check_pattern(pat, &item_ty)?;
        let (checked_body, _) =
            self.check_loop_block(body, HostTy::Unit, mark, LoopEntry::Guarded)?;
        self.pop_scope();
        Ok(node(
            HirExprKind::For {
                pat: checked_pat,
                iterable: Box::new(checked_iterable),
                body: checked_body,
            },
            HostTy::Unit,
            false,
            span,
        ))
    }

    fn check_closure(
        &mut self,
        mov: bool,
        params: &[(ast::Ident, Option<ast::Ty>)],
        body: &ast::ClosureBody,
        expect: Option<&HostTy>,
        span: Span,
    ) -> Result<HirExpr, Diag> {
        let expected = expect.map(|want| self.deep(want));
        let (want_params, want_ret, want_kind) = match &expected {
            Some(HostTy::DynFn(kind, param_tys, ret)) => {
                (Some(param_tys.clone()), Some((**ret).clone()), Some(*kind))
            }
            Some(HostTy::FnPtr(param_tys, ret)) => (
                Some(param_tys.clone()),
                Some((**ret).clone()),
                Some(ClosureKind::Fn),
            ),
            _ => (None, None, None),
        };
        if let Some(wants) = &want_params
            && wants.len() != params.len()
        {
            return Err(Diag::type_error(
                span,
                "this closure has the wrong number of parameters",
            ));
        }
        let mut param_specs = Vec::with_capacity(params.len());
        for (index, (name, annotation)) in params.iter().enumerate() {
            let ty = match annotation {
                Some(ty) => self.resolve_ty(ty, &mut Vec::new())?,
                None => match &want_params {
                    Some(wants) => wants[index].clone(),
                    None => self.fresh_cell(false),
                },
            };
            param_specs.push((name.name.clone(), ty));
        }
        let (mut closure, ret) =
            self.check_closure_body(&block_of(body), want_ret.as_ref(), &param_specs)?;
        closure.params = param_specs
            .iter()
            .zip(closure.params.iter())
            .map(|((_, ty), (binding, _))| (*binding, ty.clone()))
            .collect();
        // The kind follows what the body does to its captures (a
        // mutated capture makes `FnMut` even when `move` owns it).
        closure.kind = closure_kind_of_captures(&closure.captures);
        if mov {
            for capture in &mut closure.captures {
                capture.mode = CaptureMode::Owned;
            }
        }
        for capture in &closure.captures {
            let ty = self.sema.bindings[capture.binding.0 as usize].ty.clone();
            match capture.mode {
                CaptureMode::Shared => self.begin_loan(capture.binding, false, None),
                CaptureMode::Mut => self.begin_loan(capture.binding, true, None),
                CaptureMode::Owned => {
                    if !self.is_copy_ty(&ty) {
                        let binding = capture.binding;
                        self.check_access(binding, Access::Move, span)?;
                    }
                }
            }
        }
        if let Some(expected_kind) = want_kind
            && !crate::host::check::closure_kind_fits(closure.kind, expected_kind)
        {
            return Err(Diag::type_error(
                span,
                format!(
                    "closure kind {:?} cannot stand where {expected_kind:?} is required",
                    closure.kind
                ),
            ));
        }
        if matches!(expected, Some(HostTy::DynFn(..))) {
            for capture in &closure.captures {
                if capture.mode != CaptureMode::Owned {
                    return Err(Diag::ownership(
                        span,
                        "an escaping closure must own its captures: use `move`",
                    ));
                }
            }
        }
        let ty = HostTy::DynFn(
            closure.kind,
            closure.params.iter().map(|(_, ty)| ty.clone()).collect(),
            Box::new(ret),
        );
        let diverges = closure.body.tail.as_ref().is_some_and(|tail| tail.diverges);
        Ok(node(HirExprKind::Closure(closure), ty, diverges, span))
    }

    fn check_try(&mut self, inner: &ast::Expr, span: Span) -> Result<HirExpr, Diag> {
        let checked = self.check_expr(inner, None)?;
        let ret = self.ctx().ret.clone();
        let inner_ty = self.deep(&checked.ty);
        let ty = match (inner_ty, self.deep(&ret)) {
            (HostTy::Result(ok, err), HostTy::Result(_, want_err)) => {
                self.unify(&err, &want_err, span)?;
                *ok
            }
            (HostTy::Option(ok), HostTy::Option(_)) => *ok,
            (other, _) => {
                return Err(Diag::type_error(
                    span,
                    format!("`?` needs a `Result` or `Option`, found `{other:?}`"),
                ));
            }
        };
        Ok(node(HirExprKind::Try(Box::new(checked)), ty, false, span))
    }
}

/// The element type of a `Vec` receiver of `method`.
fn vec_elem(receiver: HostTy, method: &str, span: Span) -> Result<Box<HostTy>, Diag> {
    let HostTy::Vec(elem) = receiver else {
        return Err(Diag::type_error(span, format!("`{method}` needs a vector")));
    };
    Ok(elem)
}

/// The value type of a `HashMap` receiver of `method`.
fn map_value(receiver: HostTy, method: &str, span: Span) -> Result<Box<HostTy>, Diag> {
    let HostTy::HashMap(value) = receiver else {
        return Err(Diag::type_error(
            span,
            format!("`{method}` needs a `HashMap`"),
        ));
    };
    Ok(value)
}

/// The type of `receiver.iter()`: a `Vec` or array yields element
/// references, and a `HashMap<String, V>` yields `(&String, &V)` pairs,
/// modeled as an owning iterator whose items are those reference pairs.
fn iter_type(receiver: HostTy, span: Span) -> Result<HostTy, Diag> {
    match receiver {
        HostTy::HashMap(value) => Ok(HostTy::IntoIter(Box::new(HostTy::Tuple(
            Box::new(HostTy::Ref(false, Box::new(HostTy::String))),
            Box::new(HostTy::Ref(false, value)),
        )))),
        other => Ok(HostTy::Iter(collection_elem(other, "iter", span)?)),
    }
}

/// The element type of a `Vec` or array receiver of an iterator method.
fn collection_elem(receiver: HostTy, method: &str, span: Span) -> Result<Box<HostTy>, Diag> {
    let (HostTy::Vec(elem) | HostTy::Array(elem, _)) = receiver else {
        return Err(Diag::type_error(
            span,
            format!("`{method}` needs a collection"),
        ));
    };
    Ok(elem)
}

fn expect_arg_count(args: &[ast::Expr], want: usize, span: Span) -> Result<&[ast::Expr], Diag> {
    if args.len() != want {
        return Err(Diag::type_error(
            span,
            "this call has the wrong number of arguments",
        ));
    }
    Ok(args)
}

fn kind_of(ty: &HostTy) -> TypeKind {
    match ty {
        HostTy::String => TypeKind::String,
        HostTy::Vec(_) => TypeKind::Vec,
        HostTy::HashMap(_) => TypeKind::Map,
        HostTy::Box(_) => TypeKind::Box,
        HostTy::Array(..) => TypeKind::Array,
        HostTy::Iter(_) => TypeKind::Iter,
        HostTy::IterMut(_) => TypeKind::IterMut,
        HostTy::IntoIter(_) => TypeKind::IntoIter,
        HostTy::Range(_) => TypeKind::Range,
        HostTy::Enumerate(_) | HostTy::Zip(..) => TypeKind::Enumerate,
        _ => TypeKind::Other,
    }
}

fn bool_binary(op: BinOp, l: HirExpr, r: HirExpr, span: Span) -> HirExpr {
    node(
        HirExprKind::Binary {
            op,
            left: Box::new(l),
            right: Box::new(r),
        },
        HostTy::Bool,
        false,
        span,
    )
}

fn node(kind: HirExprKind, ty: HostTy, diverges: bool, span: Span) -> HirExpr {
    HirExpr {
        kind,
        ty,
        diverges,
        span,
    }
}

/// The closure kind Rust's capture rules determine from the capture
/// records (grammar §5).
#[must_use]
pub fn closure_kind_of_captures(captures: &[crate::host::hir::Capture]) -> ClosureKind {
    if captures
        .iter()
        .any(|capture| capture.mode == CaptureMode::Owned && capture.consumed)
    {
        return ClosureKind::FnOnce;
    }
    if captures
        .iter()
        .any(|capture| capture.mode == CaptureMode::Mut)
    {
        return ClosureKind::FnMut;
    }
    ClosureKind::Fn
}

/// The coarse receiver classes the method allowlist dispatches on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TypeKind {
    /// `String`.
    String,
    /// `Vec<T>`.
    Vec,
    /// `HashMap<String, T>`.
    Map,
    /// `Box<T>`.
    Box,
    /// `[T; N]`.
    Array,
    /// Iterator shapes.
    Iter,
    /// Mutable iterator shapes.
    IterMut,
    /// Consuming iterator shapes.
    IntoIter,
    /// `START..END`.
    Range,
    /// `enumerate` and `zip` wrappers.
    Enumerate,
    /// Everything else.
    Other,
}

fn block_of(body: &ast::ClosureBody) -> ast::Block {
    match body {
        ast::ClosureBody::Block(block) => block.clone(),
        ast::ClosureBody::Expr(expr) => ast::Block {
            stmts: Vec::new(),
            tail: Some(expr.clone()),
            span: expr.span,
        },
    }
}

/// The move state across alternative branches (`if`/`else`, `if let`,
/// `match` arms): every branch starts from the state before the
/// construct, and afterwards a binding is moved when any branch that
/// can complete moved it. A diverging branch never reaches the join.
struct BranchMoves {
    start: Vec<crate::host::hir::BindId>,
    joined: Option<Vec<crate::host::hir::BindId>>,
}

impl BranchMoves {
    fn start(checker: &Checker) -> Self {
        Self {
            start: checker.ctx().uninit.clone(),
            joined: None,
        }
    }

    /// Records one finished branch and resets the state for the next.
    fn end_branch(&mut self, checker: &mut Checker, diverges: bool) {
        let ended = std::mem::replace(&mut checker.ctx_mut().uninit, self.start.clone());
        if diverges {
            return;
        }
        let joined = self.joined.get_or_insert_with(Vec::new);
        for binding in ended {
            if !joined.contains(&binding) {
                joined.push(binding);
            }
        }
    }

    /// Installs the joined state after the last branch.
    fn finish(self, checker: &mut Checker) {
        checker.ctx_mut().uninit = self.joined.unwrap_or(self.start);
    }
}
