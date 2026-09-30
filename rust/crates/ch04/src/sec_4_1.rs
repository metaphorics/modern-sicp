// SPDX-License-Identifier: GPL-3.0-only

//! Section 4.1: direct evaluation of the checked Rust host subset,
//! and the analyzer of 4.1.7 that precomputes execution procedures.
//!
//! Both engines run the one typed representation of
//! [`sicp_runtime::host::hir`] through the shared leaf semantics of
//! [`sicp_runtime::host::ops`], so values, effect order, and traps are
//! identical by construction and the cross-engine agreement tests can
//! hold every path to the same byte transcript. The direct evaluator
//! dispatches on the checked tree; the analyzer walks a precomputed
//! [`Plan`] tree whose construction resolves every dispatch decision
//! once, which is the book's "execution procedure" discipline in
//! Rust's own terms.
//!
//! # Entry points
//!
//! [`admit`] checks source before any effect; [`run`] and
//! [`run_analyzed`] execute one checked program; [`run_source`] is the
//! check-then-run composition; [`run_program`] is the corpus
//! convenience that answers the stdout transcript.

use std::sync::Arc;

use sicp_runtime::host::check::CheckedProgram;
use sicp_runtime::host::diag::Diag;
use sicp_runtime::host::hir::{
    BinOp, BindId, CtorOp, FormatKind, FormatSpec, FunId, HirBlock, HirClosure, HirExpr,
    HirExprKind, HirPat, HirStmt, MethodOp, Place, PlaceRoot, Proj, Sema, UnOp,
};
use sicp_runtime::host::ops::{self, Engine, Flow, RunOutcome, TrapReport};
use sicp_runtime::host::value::{Addr, HostValue, RtProj, Trap};

pub use sicp_runtime::host::admit;

/// The direct evaluator: one walker over the checked tree.
pub struct Direct {
    /// The shared engine state.
    pub engine: Engine,
}

impl Direct {
    /// Builds the evaluator over one checked program.
    #[must_use]
    pub fn new(sema: Sema) -> Self {
        Self {
            engine: Engine::new(sema),
        }
    }

    /// Runs `main`, answering its final flow.
    ///
    /// # Errors
    /// The first [`TrapReport`] the run raises.
    pub fn run_main(&mut self) -> Result<Flow, TrapReport> {
        let main = self.engine.sema.main;
        let value = self.call_fun(main, Vec::new())?;
        Ok(Flow::Value(value))
    }

    /// Calls one top-level function.
    ///
    /// # Errors
    /// The first [`TrapReport`] the call raises.
    pub fn call_fun(&mut self, fun: FunId, args: Vec<HostValue>) -> Result<HostValue, TrapReport> {
        let def = self.engine.sema.funs[fun.0 as usize].clone();
        let frame = self
            .engine
            .push_activation(def.bind_base, def.frame_slots, Vec::new());
        for ((binding, _), value) in def.params.iter().zip(args) {
            self.write(*binding, value, frame)?;
        }
        let outcome = self.eval_block(&def.body, frame);
        self.engine.pop_activation();
        match outcome? {
            Flow::Value(value) | Flow::Return(value) => Ok(value),
            Flow::Break(_) | Flow::Continue => Err(Self::trap(
                Trap::Dangling,
                sicp_runtime::host::diag::Span::default(),
            )),
        }
    }

    fn write(&mut self, bind: BindId, value: HostValue, frame: usize) -> Result<(), TrapReport> {
        let _ = frame;
        self.engine
            .write_local(bind, value)
            .map_err(|trap| Self::trap(trap, default_span()))
    }

    fn trap(trap: Trap, span: sicp_runtime::host::diag::Span) -> TrapReport {
        TrapReport { trap, span }
    }

    /// Evaluates one block, threading the control flows of grammar §4.
    ///
    /// # Errors
    /// The first [`TrapReport`] the block raises.
    pub fn eval_block(&mut self, block: &HirBlock, frame: usize) -> Result<Flow, TrapReport> {
        for stmt in &block.stmts {
            let flow = match stmt {
                HirStmt::Let {
                    binding,
                    destruct,
                    value,
                } => {
                    let produced = self.eval_expr(value, frame)?;
                    let Flow::Value(produced) = produced else {
                        return Ok(produced);
                    };
                    match destruct {
                        Some((left, right)) => {
                            if let HostValue::Tuple(a, b) = produced {
                                self.write(*left, *a, frame)?;
                                self.write(*right, *b, frame)?;
                            }
                        }
                        None => self.write(*binding, produced, frame)?,
                    }
                    Flow::Value(HostValue::Unit)
                }
                HirStmt::Expr(expr) => self.eval_expr(expr, frame)?,
            };
            if !matches!(flow, Flow::Value(_)) {
                return Ok(flow);
            }
        }
        match &block.tail {
            Some(tail) => self.eval_expr(tail, frame),
            None => Ok(Flow::Value(HostValue::Unit)),
        }
    }

    /// Evaluates one expression.
    ///
    /// # Errors
    /// The first [`TrapReport`] the expression raises.
    // Keep the exhaustive HIR dispatch in one match to make evaluation order visible.
    #[allow(clippy::too_many_lines)]
    pub fn eval_expr(&mut self, expr: &HirExpr, frame: usize) -> Result<Flow, TrapReport> {
        let span = expr.span;
        let value = match &expr.kind {
            HirExprKind::I64(value) => HostValue::Int(*value),
            HirExprKind::Usize(value) => HostValue::Usize(*value),
            HirExprKind::Bool(value) => HostValue::Bool(*value),
            HirExprKind::Unit => HostValue::Unit,
            HirExprKind::Str(text) => HostValue::Text(text.clone()),
            HirExprKind::Place { place, mode } => {
                let (addr, projs) = self.eval_place(place, frame)?;
                // A projected read through a borrowed slot addresses
                // the referent, never the reference word itself.
                let (addr, projs) = self.see_through_root(addr, projs, false, span)?;
                let moved = *mode == sicp_runtime::host::hir::PlaceUse::Move;
                let result = if moved {
                    self.engine.take_at(addr, &projs)
                } else {
                    self.engine.read_at(addr, &projs)
                };
                result.map_err(|trap| Self::trap(trap, span))?
            }
            HirExprKind::FunRef(fun) => HostValue::FnPtr(*fun),
            HirExprKind::StructLit(id, fields) | HirExprKind::TupleStructLit(id, fields) => {
                let values = self.eval_args(fields, frame)?;
                HostValue::Struct(id.0, values)
            }
            HirExprKind::VariantLit(id, index, payload) => {
                let values = self.eval_args(payload, frame)?;
                HostValue::Variant(id.0, *index, values)
            }
            HirExprKind::Tuple(left, right) => {
                let a = self.eval_value(left, frame)?;
                let b = self.eval_value(right, frame)?;
                HostValue::Tuple(Box::new(a), Box::new(b))
            }
            HirExprKind::Array(items) | HirExprKind::VecList(items) => {
                HostValue::Vec(self.eval_args(items, frame)?)
            }
            HirExprKind::VecRepeat(value, count) => {
                let item = self.eval_value(value, frame)?;
                let HostValue::Int(times) = self.eval_value(count, frame)? else {
                    return Err(Self::trap(Trap::Dangling, span));
                };
                let times = usize::try_from(times)
                    .map_err(|_| Self::trap(Trap::Overflow("repeat"), span))?;
                HostValue::Vec(vec![item; times])
            }
            HirExprKind::Format { kind, spec, args } => {
                let values = self.eval_args(args, frame)?;
                let rendered = ops::render_format(spec, &values, &self.engine.store)
                    .map_err(|trap| Self::trap(trap, span))?;
                match kind {
                    FormatKind::Format => HostValue::Text(rendered),
                    FormatKind::Print => {
                        self.engine.effects.push_text(&rendered);
                        HostValue::Unit
                    }
                    FormatKind::Println => {
                        self.engine.effects.push_line(&rendered);
                        HostValue::Unit
                    }
                }
            }
            HirExprKind::Field { base, index } => {
                let value = self.eval_value(base, frame)?;
                // Field reads see through a borrowed base, like the
                // native autoref adjustment.
                let value = self.deref_scrutinee(value, span)?;
                project_value(&value, *index).map_err(|trap| Self::trap(trap, span))?
            }
            HirExprKind::Index { base, index } => {
                let value = self.eval_value(base, frame)?;
                let value = self.deref_scrutinee(value, span)?;
                let index = self.eval_value(index, frame)?;
                let at = ops::index_position(&index).map_err(|trap| Self::trap(trap, span))?;
                index_value(&value, at).map_err(|trap| Self::trap(trap, span))?
            }
            HirExprKind::Call { callee, args } => {
                let values = self.eval_args(args, frame)?;
                self.call_fun(*callee, values)?
            }
            HirExprKind::Ctor(op, args) => {
                let values = self.eval_args(args, frame)?;
                ops::construct(*op, &values).map_err(|trap| Self::trap(trap, span))?
            }
            HirExprKind::IndirectCall { callee, args } => {
                let function = self.eval_value(callee, frame)?;
                let values = self.eval_args(args, frame)?;
                self.call_value(function, values)?
            }
            HirExprKind::Method {
                op,
                receiver,
                receiver_place,
                args,
            } => {
                let values = self.eval_args(args, frame)?;
                self.eval_method(*op, receiver, receiver_place.as_ref(), &values, frame, span)?
            }
            HirExprKind::Unary { op, operand } => {
                if let (UnOp::Ref | UnOp::RefMut, HirExprKind::Place { place, .. }) =
                    (op, &operand.kind)
                {
                    // Borrowing resolves the place to its address without
                    // reading it: the reference aliases the slot, so later
                    // writes through it stay visible.
                    let (addr, projs) = self.eval_place(place, frame)?;
                    let (addr, projs) = self.see_through_root(addr, projs, false, span)?;
                    HostValue::Ref {
                        addr,
                        projs,
                        mutable: *op == UnOp::RefMut,
                    }
                } else {
                    let value = self.eval_value(operand, frame)?;
                    ops::checked_unary(*op, &value).map_err(|trap| Self::trap(trap, span))?
                }
            }
            HirExprKind::Binary { op, left, right } => {
                self.eval_binary(*op, left, right, frame, span)?
            }
            HirExprKind::Assign { op, target, value } => {
                let (addr, projs) = self.eval_place(target, frame)?;
                let (addr, projs) = self.see_through_root(addr, projs, false, span)?;
                let produced = self.eval_value(value, frame)?;
                let final_value = match op {
                    None => produced,
                    Some(binop) => {
                        let current = self
                            .engine
                            .read_at(addr, &projs)
                            .map_err(|trap| Self::trap(trap, span))?;
                        ops::checked_binary(*binop, &current, &produced)
                            .map_err(|trap| Self::trap(trap, span))?
                    }
                };
                self.engine
                    .write_at(addr, &projs, final_value)
                    .map_err(|trap| Self::trap(trap, span))?;
                HostValue::Unit
            }
            HirExprKind::If {
                test,
                then,
                else_branch,
            } => {
                let HostValue::Bool(decision) = self.eval_value(test, frame)? else {
                    return Err(Self::trap(Trap::Dangling, span));
                };
                let branch = if decision { then } else { else_branch };
                return self.eval_expr(branch, frame);
            }
            HirExprKind::IfLet {
                pat,
                value,
                then,
                else_branch,
            } => {
                let tested = self.eval_value(value, frame)?;
                if self.bind_pattern(pat, &tested, frame)? {
                    return self.eval_expr(then, frame);
                }
                return self.eval_expr(else_branch, frame);
            }
            HirExprKind::Match { scrutinee, arms } => {
                let tested = self.eval_value(scrutinee, frame)?;
                for (pat, body) in arms {
                    if self.bind_pattern(pat, &tested, frame)? {
                        return self.eval_expr(body, frame);
                    }
                }
                return Err(Self::trap(Trap::Dangling, span));
            }
            HirExprKind::Block(block) => return self.eval_block(block, frame),
            HirExprKind::Loop { body, .. } => loop {
                match self.eval_block(body, frame)? {
                    Flow::Value(_) | Flow::Continue => {}
                    Flow::Break(value) => break value,
                    flow @ Flow::Return(_) => return Ok(flow),
                }
            },
            HirExprKind::While { test, body } => loop {
                let HostValue::Bool(decision) = self.eval_value(test, frame)? else {
                    return Err(Self::trap(Trap::Dangling, span));
                };
                if !decision {
                    break HostValue::Unit;
                }
                match self.eval_block(body, frame)? {
                    Flow::Value(_) | Flow::Continue => {}
                    Flow::Break(value) => break value,
                    flow @ Flow::Return(_) => return Ok(flow),
                }
            },
            HirExprKind::WhileLet { pat, value, body } => loop {
                let tested = self.eval_value(value, frame)?;
                if !self.bind_pattern(pat, &tested, frame)? {
                    break HostValue::Unit;
                }
                match self.eval_block(body, frame)? {
                    Flow::Value(_) | Flow::Continue => {}
                    Flow::Break(value) => break value,
                    flow @ Flow::Return(_) => return Ok(flow),
                }
            },
            HirExprKind::For {
                pat,
                iterable,
                body,
            } => {
                let mut iterator = self.eval_iterable(iterable, frame)?;
                loop {
                    let step =
                        ops::iterator_next(&mut iterator).map_err(|trap| Self::trap(trap, span))?;
                    let item = match ops::builtin_variant(&step) {
                        Some((0, payload)) if payload.len() == 1 => payload[0].clone(),
                        _ => break HostValue::Unit,
                    };
                    if self.bind_pattern(pat, &item, frame)? {
                        match self.eval_block(body, frame)? {
                            Flow::Value(_) | Flow::Continue => {}
                            Flow::Break(value) => break value,
                            flow @ Flow::Return(_) => return Ok(flow),
                        }
                    }
                }
            }
            HirExprKind::Closure(closure) => self.make_closure(closure, frame, span)?,
            HirExprKind::Return(value) => {
                let produced = match value {
                    Some(value) => self.eval_value(value, frame)?,
                    None => HostValue::Unit,
                };
                return Ok(Flow::Return(produced));
            }
            HirExprKind::Break(value) => {
                let produced = match value {
                    Some(value) => self.eval_value(value, frame)?,
                    None => HostValue::Unit,
                };
                return Ok(Flow::Break(produced));
            }
            HirExprKind::Continue => return Ok(Flow::Continue),
            HirExprKind::Try(inner) => {
                let tested = self.eval_value(inner, frame)?;
                match ops::builtin_variant(&tested) {
                    Some((0, payload)) if payload.len() == 1 => payload[0].clone(),
                    Some((1, _)) => return Ok(Flow::Return(tested)),
                    _ => return Err(Self::trap(Trap::Dangling, span)),
                }
            }
            HirExprKind::Range(left, right) => {
                let start = self.eval_value(left, frame)?;
                let end = self.eval_value(right, frame)?;
                ops::range_of(&start, &end).map_err(|trap| Self::trap(trap, span))?
            }
        };
        Ok(Flow::Value(value))
    }

    fn eval_value(&mut self, expr: &HirExpr, frame: usize) -> Result<HostValue, TrapReport> {
        match self.eval_expr(expr, frame)? {
            Flow::Value(value) => Ok(value),
            Flow::Return(_) | Flow::Break(_) | Flow::Continue => {
                Err(Self::trap(Trap::Dangling, expr.span))
            }
        }
    }

    fn eval_args(&mut self, args: &[HirExpr], frame: usize) -> Result<Vec<HostValue>, TrapReport> {
        let mut values = Vec::with_capacity(args.len());
        for arg in args {
            values.push(self.eval_value(arg, frame)?);
        }
        Ok(values)
    }

    fn eval_binary(
        &mut self,
        op: BinOp,
        left: &HirExpr,
        right: &HirExpr,
        frame: usize,
        span: sicp_runtime::host::diag::Span,
    ) -> Result<HostValue, TrapReport> {
        if op == BinOp::And || op == BinOp::Or {
            let HostValue::Bool(first) = self.eval_value(left, frame)? else {
                return Err(Self::trap(Trap::Dangling, span));
            };
            if op == BinOp::And && !first {
                return Ok(HostValue::Bool(false));
            }
            if op == BinOp::Or && first {
                return Ok(HostValue::Bool(true));
            }
            let HostValue::Bool(second) = self.eval_value(right, frame)? else {
                return Err(Self::trap(Trap::Dangling, span));
            };
            return Ok(HostValue::Bool(second));
        }
        let a = self.eval_value(left, frame)?;
        let b = self.eval_value(right, frame)?;
        ops::checked_binary(op, &a, &b).map_err(|trap| Self::trap(trap, span))
    }

    fn eval_method(
        &mut self,
        op: MethodOp,
        receiver: &HirExpr,
        receiver_place: Option<&Place>,
        args: &[HostValue],
        frame: usize,
        span: sicp_runtime::host::diag::Span,
    ) -> Result<HostValue, TrapReport> {
        let place = match receiver_place {
            Some(place) => {
                let (addr, projs) = self.eval_place(place, frame)?;
                // Methods consume the referent and write their updates
                // back to it, so a borrowed root always resolves to
                // the referent's address.
                Some(self.see_through_root(addr, projs, true, span)?)
            }
            None => None,
        };
        let taken = if let (MethodOp::IntoIter, Some((addr, projs))) = (op, place.as_ref()) {
            self.engine
                .take_at(*addr, projs)
                .map_err(|trap| Self::trap(trap, span))?
        } else {
            let taken = self.eval_value(receiver, frame)?;
            // Method bodies match on owned shapes: a borrowed receiver
            // reads through to its referent first.
            self.deref_scrutinee(taken, span)?
        };
        let (result, updated) = ops::apply_method(op, taken, place.clone(), args)
            .map_err(|trap| Self::trap(trap, span))?;
        if let (Some(updated), Some((addr, projs))) = (updated, place) {
            self.engine
                .write_at(addr, &projs, updated)
                .map_err(|trap| Self::trap(trap, span))?;
        }
        Ok(result)
    }

    fn eval_iterable(&mut self, iterable: &HirExpr, frame: usize) -> Result<HostValue, TrapReport> {
        if let HirExprKind::Range(left, right) = &iterable.kind {
            let start = self.eval_value(left, frame)?;
            let end = self.eval_value(right, frame)?;
            ops::range_of(&start, &end).map_err(|trap| Self::trap(trap, iterable.span))
        } else {
            let value = self.eval_value(iterable, frame)?;
            match value {
                HostValue::Iter(_) => Ok(value),
                HostValue::Ref {
                    addr,
                    projs,
                    mutable,
                } => {
                    let collection = self
                        .engine
                        .read_at(addr, &projs)
                        .map_err(|trap| Self::trap(trap, iterable.span))?;
                    let len = match collection {
                        HostValue::Vec(items) | HostValue::Array(items) => items.len(),
                        _ => return Err(Self::trap(Trap::Dangling, iterable.span)),
                    };
                    Ok(ops::refs_iterator(addr, projs, len, mutable))
                }
                HostValue::Vec(items) | HostValue::Array(items) => Ok(ops::items_iterator(items)),
                _ => Err(Self::trap(Trap::Dangling, iterable.span)),
            }
        }
    }

    fn make_closure(
        &mut self,
        closure: &HirClosure,
        frame: usize,
        span: sicp_runtime::host::diag::Span,
    ) -> Result<HostValue, TrapReport> {
        let _ = frame;
        let mut captures = Vec::with_capacity(closure.captures.len());
        for capture in &closure.captures {
            let value = self
                .engine
                .capture_value(capture.binding, capture.mode)
                .map_err(|trap| Self::trap(trap, span))?;
            captures.push((capture.binding, capture.mode, value));
        }
        Ok(ops::closure_value(
            closure.kind,
            Arc::new(closure.body.clone()),
            closure.params.clone(),
            captures,
            closure.ret.clone(),
            closure.frame_slots,
            closure.bind_base,
        ))
    }

    fn bind_pattern(
        &mut self,
        pat: &HirPat,
        value: &HostValue,
        frame: usize,
    ) -> Result<bool, TrapReport> {
        let bound = ops::bind_pattern(&self.engine, pat, value)
            .map_err(|trap| Self::trap(trap, pat.span))?;
        let Some(bound) = bound else {
            return Ok(false);
        };
        for (binding, bound_value) in bound {
            self.write(binding, bound_value, frame)?;
        }
        Ok(true)
    }

    /// Follows one borrowed field base, index base, or method receiver
    /// to its referent, like the native autoref adjustment; a
    /// non-reference value passes through unchanged. Pattern matching
    /// does not use this: [`ops::bind_pattern`] reads through a
    /// reference scrutinee itself so bindings stay borrows.
    ///
    /// # Errors
    /// The first [`TrapReport`] the dereference raises.
    fn deref_scrutinee(
        &self,
        value: HostValue,
        span: sicp_runtime::host::diag::Span,
    ) -> Result<HostValue, TrapReport> {
        if matches!(value, HostValue::Ref { .. }) {
            return self
                .engine
                .deref_value(&value)
                .map_err(|trap| Self::trap(trap, span));
        }
        Ok(value)
    }

    /// Sees a resolved place through a borrow held in its root slot:
    /// when the root slot itself holds a reference, the referent's
    /// address plus the reference's projections becomes the base.
    /// Reads, borrows, and stores keep a bare borrowed slot as-is
    /// (only projections force the substitution); method places
    /// always substitute, because methods consume the referent and
    /// write their updates back to it.
    ///
    /// # Errors
    /// The first [`TrapReport`] the peek raises.
    fn see_through_root(
        &self,
        addr: Addr,
        projs: Vec<RtProj>,
        for_method: bool,
        span: sicp_runtime::host::diag::Span,
    ) -> Result<(Addr, Vec<RtProj>), TrapReport> {
        if projs.is_empty() && !for_method {
            return Ok((addr, projs));
        }
        match self
            .engine
            .read_at(addr, &[])
            .map_err(|trap| Self::trap(trap, span))?
        {
            HostValue::Ref {
                addr: base,
                projs: mut base_projs,
                ..
            } => {
                base_projs.extend(projs);
                Ok((base, base_projs))
            }
            _ => Ok((addr, projs)),
        }
    }

    /// Resolves one checked place to its arena address and
    /// projections, evaluating dereference roots and index operands.
    ///
    /// # Errors
    /// The first [`TrapReport`] the resolution raises.
    pub fn eval_place(
        &mut self,
        place: &Place,
        frame: usize,
    ) -> Result<(Addr, Vec<RtProj>), TrapReport> {
        let (addr, mut projs) = match &place.root {
            PlaceRoot::Local(bind) => self
                .engine
                .local_place(*bind)
                .map_err(|trap| Self::trap(trap, place.span))?,
            PlaceRoot::Deref(inner) => {
                let HostValue::Ref { addr, projs, .. } = self.eval_value(inner, frame)? else {
                    return Err(Self::trap(Trap::Dangling, place.span));
                };
                (addr, projs)
            }
        };
        for step in &place.proj {
            match step {
                Proj::Field(index) => projs.push(RtProj::Field(*index)),
                Proj::Index(index_expr) => {
                    let index = self.eval_value(index_expr, frame)?;
                    let at =
                        ops::index_position(&index).map_err(|trap| Self::trap(trap, place.span))?;
                    projs.push(RtProj::Index(at));
                }
            }
        }
        Ok((addr, projs))
    }

    /// Calls one function or closure value.
    ///
    /// # Errors
    /// The first [`TrapReport`] the call raises.
    pub fn call_value(
        &mut self,
        callee: HostValue,
        args: Vec<HostValue>,
    ) -> Result<HostValue, TrapReport> {
        match callee {
            HostValue::FnPtr(fun) => self.call_fun(fun, args),
            HostValue::Closure(closure) => self.apply_closure(&closure, args),
            HostValue::Box(inner) => self.call_value(*inner, args),
            _ => Err(TrapReport {
                trap: Trap::Dangling,
                span: sicp_runtime::host::diag::Span::new(1, 1, 1),
            }),
        }
    }

    fn apply_closure(
        &mut self,
        closure: &sicp_runtime::host::value::ClosureVal,
        args: Vec<HostValue>,
    ) -> Result<HostValue, TrapReport> {
        let frame = self.engine.push_activation(
            closure.bind_base,
            closure.frame_slots,
            closure.captures.clone(),
        );
        for ((binding, _), value) in closure.params.iter().zip(args) {
            self.write(*binding, value, frame)?;
        }
        let body = closure.body.clone();
        let outcome = self.eval_block(&body, frame);
        self.engine.pop_activation();
        match outcome? {
            Flow::Value(value) | Flow::Return(value) => Ok(value),
            Flow::Break(_) | Flow::Continue => Err(TrapReport {
                trap: Trap::Dangling,
                span: sicp_runtime::host::diag::Span::new(1, 1, 1),
            }),
        }
    }
}

fn project_value(value: &HostValue, index: u32) -> Result<HostValue, Trap> {
    match value {
        HostValue::Struct(_, fields) | HostValue::Variant(_, _, fields) => fields
            .get(index as usize)
            .cloned()
            .ok_or(Trap::IndexOutOfBounds),
        HostValue::Tuple(left, right) => {
            if index == 0 {
                Ok((**left).clone())
            } else {
                Ok((**right).clone())
            }
        }
        _ => Err(Trap::Dangling),
    }
}

fn index_value(value: &HostValue, at: i64) -> Result<HostValue, Trap> {
    let (HostValue::Vec(items) | HostValue::Array(items)) = value else {
        return Err(Trap::Dangling);
    };
    let at = usize::try_from(at).map_err(|_| Trap::IndexOutOfBounds)?;
    items.get(at).cloned().ok_or(Trap::IndexOutOfBounds)
}

/// Runs one checked program on the direct evaluator.
#[must_use]
pub fn run(program: &CheckedProgram) -> RunOutcome {
    let mut direct = Direct::new(program.sema.clone());
    match direct.run_main() {
        Ok(_) => RunOutcome {
            stdout: direct.engine.effects.stdout,
            trap: None,
        },
        Err(report) => RunOutcome {
            stdout: direct.engine.effects.stdout,
            trap: Some(report),
        },
    }
}

/// Checks one guest source and runs it on the direct evaluator. No
/// effect can occur while the source is rejected.
///
/// # Errors
/// The admission [`Diag`] when the source is rejected.
pub fn run_source(source: &str) -> Result<RunOutcome, Diag> {
    let program = admit(source)?;
    Ok(run(&program))
}

/// The corpus convenience: one program's stdout transcript.
#[must_use]
pub fn run_program(source: &str) -> String {
    run_source(source).map_or_else(
        |diag| format!("rejected: {}", diag.message),
        |out| out.stdout,
    )
}

/// The analyzer of 4.1.7: execution procedures as a precomputed plan
/// tree. Building a [`Plan`] resolves every dispatch decision once;
/// running it never re-dispatches on syntax, which is the book's
/// "execution procedure" discipline in Rust's own terms. The observable
/// values, effects, and traps match the direct walker by construction,
/// because both call the same leaf semantics.
#[derive(Debug, Clone)]
pub enum Plan {
    /// A precomputed value.
    Value(HostValue),
    /// A place read or move.
    Place {
        /// The place's precomputed root.
        root: PlanRoot,
        /// The projections, index operands pre-analyzed.
        proj: Vec<PlanProj>,
        /// Whether the read moves.
        mov: bool,
    },
    /// A struct or tuple-struct literal.
    StructLit(u32, Vec<Plan>),
    /// An enum variant construction.
    VariantLit(u32, u32, Vec<Plan>),
    /// `(A, B)`.
    Tuple(Box<Plan>, Box<Plan>),
    /// `[A, ...]` and `vec![A, ...]`.
    Array(Vec<Plan>),
    /// `vec![VALUE; COUNT]`.
    VecRepeat(Box<Plan>, Box<Plan>),
    /// A format macro with its parsed specification.
    Format {
        /// The macro's kind.
        kind: FormatKind,
        /// The parsed format specification.
        spec: FormatSpec,
        /// The analyzed arguments.
        args: Vec<Plan>,
    },
    /// A field read.
    Field {
        /// The analyzed base.
        base: Box<Plan>,
        /// The resolved field index.
        index: u32,
    },
    /// An index read.
    Index {
        /// The analyzed base.
        base: Box<Plan>,
        /// The analyzed index.
        index: Box<Plan>,
    },
    /// A top-level call.
    Call {
        /// The resolved callee.
        callee: FunId,
        /// The analyzed arguments.
        args: Vec<Plan>,
    },
    /// An admitted constructor.
    Ctor(CtorOp, Vec<Plan>),
    /// An indirect call through a value.
    IndirectCall {
        /// The analyzed callee.
        callee: Box<Plan>,
        /// The analyzed arguments.
        args: Vec<Plan>,
    },
    /// An admitted method.
    Method {
        /// The method to run.
        op: MethodOp,
        /// The analyzed receiver.
        receiver: Box<Plan>,
        /// The receiver's place, pre-analyzed.
        place: Option<(PlanRoot, Vec<PlanProj>)>,
        /// The analyzed arguments.
        args: Vec<Plan>,
    },
    /// A unary operator.
    Unary(UnOp, Box<Plan>),
    /// A binary operator.
    Binary(BinOp, Box<Plan>, Box<Plan>),
    /// An assignment.
    Assign {
        /// The compound operator.
        op: Option<BinOp>,
        /// The target place root.
        root: PlanRoot,
        /// The target projections.
        proj: Vec<PlanProj>,
        /// The assigned value.
        value: Box<Plan>,
    },
    /// An `if`.
    If {
        /// The analyzed test.
        test: Box<Plan>,
        /// The then-branch.
        then: Box<Plan>,
        /// The else branch.
        els: Box<Plan>,
    },
    /// An `if let`.
    IfLet {
        /// The checked pattern.
        pat: HirPat,
        /// The analyzed scrutinee.
        value: Box<Plan>,
        /// The then-branch.
        then: Box<Plan>,
        /// The else branch.
        els: Box<Plan>,
    },
    /// A `match`.
    Match {
        /// The analyzed scrutinee.
        scrutinee: Box<Plan>,
        /// The arms, in order.
        arms: Vec<(HirPat, Plan)>,
    },
    /// A block flattened into an ordered statement sequence plus tail.
    Seq {
        /// The ordered statements.
        stmts: Vec<PlanStmt>,
        /// The tail plan.
        tail: Option<Box<Plan>>,
    },
    /// A loop with an optional condition; `None` is `loop`.
    Loop {
        /// The condition plan, when the loop has one.
        test: Option<Box<Plan>>,
        /// The analyzed body.
        body: Box<Plan>,
    },
    /// A `while let`.
    WhileLet {
        /// The checked pattern.
        pat: HirPat,
        /// The analyzed scrutinee.
        value: Box<Plan>,
        /// The analyzed body.
        body: Box<Plan>,
    },
    /// A `for`.
    For {
        /// The binding pattern.
        pat: HirPat,
        /// The analyzed iterable.
        iterable: Box<Plan>,
        /// The analyzed body.
        body: Box<Plan>,
    },
    /// A `return`.
    Return(Option<Box<Plan>>),
    /// A `break`.
    Break(Option<Box<Plan>>),
    /// A `continue`.
    Continue,
    /// The postfix `?`.
    Try(Box<Plan>),
    /// `START..END`.
    Range(Box<Plan>, Box<Plan>),
    /// A closure value with its checked record.
    ClosureValue {
        /// The closure's checked record.
        closure: HirClosure,
    },
}

/// A precomputed place root.
#[derive(Debug, Clone)]
pub enum PlanRoot {
    /// A local binding.
    Local(BindId),
    /// A dereference of an analyzed expression.
    Deref(Box<Plan>),
}

/// A precomputed place projection.
#[derive(Debug, Clone)]
pub enum PlanProj {
    /// `.field`.
    Field(u32),
    /// `[index]`.
    Index(Box<Plan>),
}

/// One precomputed statement of a [`Plan::Seq`].
#[derive(Debug, Clone)]
pub enum PlanStmt {
    /// A binding with its analyzed initializer.
    Let {
        /// The fresh slot.
        binding: BindId,
        /// The tuple destructuring slots.
        destruct: Option<(BindId, BindId)>,
        /// The analyzed initializer.
        value: Plan,
    },
    /// An analyzed expression statement.
    Expr(Plan),
}

/// An analyzed program: one plan per function plus the entry point.
#[derive(Debug, Clone)]
pub struct AnalyzedProgram {
    /// The typed program the plans came from.
    pub sema: Sema,
    /// The per-function analyzed bodies, in function order.
    pub bodies: Vec<Plan>,
}

/// Analyzes one checked program into execution procedures.
#[must_use]
pub fn analyze(program: &CheckedProgram) -> AnalyzedProgram {
    let sema = program.sema.clone();
    let bodies = sema
        .funs
        .iter()
        .map(|def| analyze_block(&def.body))
        .collect();
    AnalyzedProgram { sema, bodies }
}

fn analyze_block(block: &HirBlock) -> Plan {
    let stmts = block.stmts.iter().map(analyze_stmt).collect();
    Plan::Seq {
        stmts,
        tail: block.tail.as_ref().map(|tail| Box::new(analyze_expr(tail))),
    }
}

fn analyze_stmt(stmt: &HirStmt) -> PlanStmt {
    match stmt {
        HirStmt::Let {
            binding,
            destruct,
            value,
        } => PlanStmt::Let {
            binding: *binding,
            destruct: *destruct,
            value: analyze_expr(value),
        },
        HirStmt::Expr(expr) => PlanStmt::Expr(analyze_expr(expr)),
    }
}

// Keep the exhaustive HIR-to-plan mapping together; each arm is one lowering rule.
#[allow(clippy::too_many_lines)]
fn analyze_expr(expr: &HirExpr) -> Plan {
    match &expr.kind {
        HirExprKind::I64(value) => Plan::Value(HostValue::Int(*value)),
        HirExprKind::Usize(value) => Plan::Value(HostValue::Usize(*value)),
        HirExprKind::Bool(value) => Plan::Value(HostValue::Bool(*value)),
        HirExprKind::Unit => Plan::Value(HostValue::Unit),
        HirExprKind::Str(text) => Plan::Value(HostValue::Text(text.clone())),
        HirExprKind::Place { place, mode } => {
            let (root, proj) = analyze_place(place);
            Plan::Place {
                root,
                proj,
                mov: *mode == sicp_runtime::host::hir::PlaceUse::Move,
            }
        }
        HirExprKind::FunRef(fun) => Plan::Value(HostValue::FnPtr(*fun)),
        HirExprKind::StructLit(id, fields) | HirExprKind::TupleStructLit(id, fields) => {
            Plan::StructLit(id.0, fields.iter().map(analyze_expr).collect())
        }
        HirExprKind::VariantLit(id, index, payload) => {
            Plan::VariantLit(id.0, *index, payload.iter().map(analyze_expr).collect())
        }
        HirExprKind::Tuple(left, right) => {
            Plan::Tuple(Box::new(analyze_expr(left)), Box::new(analyze_expr(right)))
        }
        HirExprKind::Array(items) | HirExprKind::VecList(items) => {
            Plan::Array(items.iter().map(analyze_expr).collect())
        }
        HirExprKind::VecRepeat(value, count) => {
            Plan::VecRepeat(Box::new(analyze_expr(value)), Box::new(analyze_expr(count)))
        }
        HirExprKind::Format { kind, spec, args } => Plan::Format {
            kind: *kind,
            spec: spec.clone(),
            args: args.iter().map(analyze_expr).collect(),
        },
        HirExprKind::Field { base, index } => Plan::Field {
            base: Box::new(analyze_expr(base)),
            index: *index,
        },
        HirExprKind::Index { base, index } => Plan::Index {
            base: Box::new(analyze_expr(base)),
            index: Box::new(analyze_expr(index)),
        },
        HirExprKind::Call { callee, args } => Plan::Call {
            callee: *callee,
            args: args.iter().map(analyze_expr).collect(),
        },
        HirExprKind::Ctor(op, args) => Plan::Ctor(*op, args.iter().map(analyze_expr).collect()),
        HirExprKind::IndirectCall { callee, args } => Plan::IndirectCall {
            callee: Box::new(analyze_expr(callee)),
            args: args.iter().map(analyze_expr).collect(),
        },
        HirExprKind::Method {
            op,
            receiver,
            receiver_place,
            args,
        } => Plan::Method {
            op: *op,
            receiver: Box::new(analyze_expr(receiver)),
            place: receiver_place.as_ref().map(analyze_place),
            args: args.iter().map(analyze_expr).collect(),
        },
        HirExprKind::Unary { op, operand } => Plan::Unary(*op, Box::new(analyze_expr(operand))),
        HirExprKind::Binary { op, left, right } => Plan::Binary(
            *op,
            Box::new(analyze_expr(left)),
            Box::new(analyze_expr(right)),
        ),
        HirExprKind::Assign { op, target, value } => {
            let (root, proj) = analyze_place(target);
            Plan::Assign {
                op: *op,
                root,
                proj,
                value: Box::new(analyze_expr(value)),
            }
        }
        HirExprKind::If {
            test,
            then,
            else_branch,
        } => Plan::If {
            test: Box::new(analyze_expr(test)),
            then: Box::new(analyze_expr(then)),
            els: Box::new(analyze_expr(else_branch)),
        },
        HirExprKind::IfLet {
            pat,
            value,
            then,
            else_branch,
        } => Plan::IfLet {
            pat: pat.clone(),
            value: Box::new(analyze_expr(value)),
            then: Box::new(analyze_expr(then)),
            els: Box::new(analyze_expr(else_branch)),
        },
        HirExprKind::Match { scrutinee, arms } => Plan::Match {
            scrutinee: Box::new(analyze_expr(scrutinee)),
            arms: arms
                .iter()
                .map(|(pat, body)| (pat.clone(), analyze_expr(body)))
                .collect(),
        },
        HirExprKind::Block(block) => analyze_block(block),
        HirExprKind::Loop { body, .. } => Plan::Loop {
            test: None,
            body: Box::new(analyze_block(body)),
        },
        HirExprKind::While { test, body } => Plan::Loop {
            test: Some(Box::new(analyze_expr(test))),
            body: Box::new(analyze_block(body)),
        },
        HirExprKind::WhileLet { pat, value, body } => Plan::WhileLet {
            pat: pat.clone(),
            value: Box::new(analyze_expr(value)),
            body: Box::new(analyze_block(body)),
        },
        HirExprKind::For {
            pat,
            iterable,
            body,
        } => Plan::For {
            pat: pat.clone(),
            iterable: Box::new(analyze_expr(iterable)),
            body: Box::new(analyze_block(body)),
        },
        HirExprKind::Closure(closure) => Plan::ClosureValue {
            closure: closure.clone(),
        },
        HirExprKind::Return(value) => {
            Plan::Return(value.as_ref().map(|inner| Box::new(analyze_expr(inner))))
        }
        HirExprKind::Break(value) => {
            Plan::Break(value.as_ref().map(|inner| Box::new(analyze_expr(inner))))
        }
        HirExprKind::Continue => Plan::Continue,
        HirExprKind::Try(inner) => Plan::Try(Box::new(analyze_expr(inner))),
        HirExprKind::Range(left, right) => {
            Plan::Range(Box::new(analyze_expr(left)), Box::new(analyze_expr(right)))
        }
    }
}

fn analyze_place(place: &Place) -> (PlanRoot, Vec<PlanProj>) {
    let root = match &place.root {
        PlaceRoot::Local(bind) => PlanRoot::Local(*bind),
        PlaceRoot::Deref(inner) => PlanRoot::Deref(Box::new(analyze_expr(inner))),
    };
    let proj = place
        .proj
        .iter()
        .map(|step| match step {
            Proj::Field(index) => PlanProj::Field(*index),
            Proj::Index(index) => PlanProj::Index(Box::new(analyze_expr(index))),
        })
        .collect();
    (root, proj)
}

/// Runs one retained analyzed program without re-analysis.
#[must_use]
pub fn run_analyzed_program(program: &AnalyzedProgram) -> RunOutcome {
    let mut direct = Direct::new(program.sema.clone());
    let main = direct.engine.sema.main;
    let plan = program.bodies[main.0 as usize].clone();
    let outcome = run_plan(&mut direct, &plan, main);
    match outcome {
        Ok(_) => RunOutcome {
            stdout: direct.engine.effects.stdout,
            trap: None,
        },
        Err(report) => RunOutcome {
            stdout: direct.engine.effects.stdout,
            trap: Some(report),
        },
    }
}

/// Runs one analyzed program.
#[must_use]
pub fn run_analyzed(program: &CheckedProgram) -> RunOutcome {
    run_analyzed_program(&analyze(program))
}

fn run_plan(direct: &mut Direct, plan: &Plan, fun: FunId) -> Result<Flow, TrapReport> {
    let def = direct.engine.sema.funs[fun.0 as usize].clone();
    let frame = direct
        .engine
        .push_activation(def.bind_base, def.frame_slots, Vec::new());
    let outcome = eval_plan(direct, plan, frame);
    direct.engine.pop_activation();
    outcome
}

// Keep the executable plan's exhaustive control-flow dispatch in one place.
#[allow(clippy::too_many_lines)]
fn eval_plan(direct: &mut Direct, plan: &Plan, frame: usize) -> Result<Flow, TrapReport> {
    match plan {
        Plan::Value(value) => Ok(Flow::Value(value.clone())),
        Plan::Place { root, proj, mov } => {
            let (addr, projs) = eval_plan_place(direct, root, proj, frame)?;
            let (addr, projs) = direct.see_through_root(addr, projs, false, default_span())?;
            let result = if *mov {
                direct.engine.take_at(addr, &projs)
            } else {
                direct.engine.read_at(addr, &projs)
            };
            result
                .map(Flow::Value)
                .map_err(|trap| Direct::trap(trap, default_span()))
        }
        Plan::StructLit(id, fields) => {
            let values = eval_plan_args(direct, fields, frame)?;
            Ok(Flow::Value(HostValue::Struct(*id, values)))
        }
        Plan::VariantLit(id, index, payload) => {
            let values = eval_plan_args(direct, payload, frame)?;
            Ok(Flow::Value(HostValue::Variant(*id, *index, values)))
        }
        Plan::Tuple(left, right) => {
            let a = eval_plan_value(direct, left, frame)?;
            let b = eval_plan_value(direct, right, frame)?;
            Ok(Flow::Value(HostValue::Tuple(Box::new(a), Box::new(b))))
        }
        Plan::Array(items) => {
            let values = eval_plan_args(direct, items, frame)?;
            Ok(Flow::Value(HostValue::Vec(values)))
        }
        Plan::VecRepeat(value, count) => {
            let item = eval_plan_value(direct, value, frame)?;
            let HostValue::Int(times) = eval_plan_value(direct, count, frame)? else {
                return Err(Direct::trap(Trap::Dangling, default_span()));
            };
            let times = usize::try_from(times)
                .map_err(|_| Direct::trap(Trap::Overflow("repeat"), default_span()))?;
            Ok(Flow::Value(HostValue::Vec(vec![item; times])))
        }
        Plan::Format { kind, spec, args } => {
            let values = eval_plan_args(direct, args, frame)?;
            let rendered = ops::render_format(spec, &values, &direct.engine.store)
                .map_err(|trap| Direct::trap(trap, default_span()))?;
            match kind {
                FormatKind::Format => Ok(Flow::Value(HostValue::Text(rendered))),
                FormatKind::Print => {
                    direct.engine.effects.push_text(&rendered);
                    Ok(Flow::Value(HostValue::Unit))
                }
                FormatKind::Println => {
                    direct.engine.effects.push_line(&rendered);
                    Ok(Flow::Value(HostValue::Unit))
                }
            }
        }
        Plan::Field { base, index } => {
            let value = eval_plan_value(direct, base, frame)?;
            let value = direct.deref_scrutinee(value, default_span())?;
            project_value(&value, *index)
                .map(Flow::Value)
                .map_err(|trap| Direct::trap(trap, default_span()))
        }
        Plan::Index { base, index } => {
            let value = eval_plan_value(direct, base, frame)?;
            let value = direct.deref_scrutinee(value, default_span())?;
            let index = eval_plan_value(direct, index, frame)?;
            let at =
                ops::index_position(&index).map_err(|trap| Direct::trap(trap, default_span()))?;
            index_value(&value, at)
                .map(Flow::Value)
                .map_err(|trap| Direct::trap(trap, default_span()))
        }
        Plan::Call { callee, args } => {
            let values = eval_plan_args(direct, args, frame)?;
            direct.call_fun(*callee, values).map(Flow::Value)
        }
        Plan::Ctor(op, args) => {
            let values = eval_plan_args(direct, args, frame)?;
            ops::construct(*op, &values)
                .map(Flow::Value)
                .map_err(|trap| Direct::trap(trap, default_span()))
        }
        Plan::IndirectCall { callee, args } => {
            let function = eval_plan_value(direct, callee, frame)?;
            let values = eval_plan_args(direct, args, frame)?;
            direct.call_value(function, values).map(Flow::Value)
        }
        Plan::Method {
            op,
            receiver,
            place,
            args,
        } => {
            let values = eval_plan_args(direct, args, frame)?;
            let resolved = match place {
                Some((root, proj)) => {
                    let (addr, projs) = eval_plan_place(direct, root, proj, frame)?;
                    Some(direct.see_through_root(addr, projs, true, default_span())?)
                }
                None => None,
            };
            let taken = if let (MethodOp::IntoIter, Some((addr, projs))) = (*op, resolved.as_ref())
            {
                direct
                    .engine
                    .take_at(*addr, projs)
                    .map_err(|trap| Direct::trap(trap, default_span()))?
            } else {
                let taken = eval_plan_value(direct, receiver, frame)?;
                direct.deref_scrutinee(taken, default_span())?
            };
            let (result, updated) = ops::apply_method(*op, taken, resolved.clone(), &values)
                .map_err(|trap| Direct::trap(trap, default_span()))?;
            if let (Some(updated), Some((addr, projs))) = (updated, resolved) {
                direct
                    .engine
                    .write_at(addr, &projs, updated)
                    .map_err(|trap| Direct::trap(trap, default_span()))?;
            }
            Ok(Flow::Value(result))
        }
        Plan::Unary(op, operand) => {
            if matches!(op, UnOp::Ref | UnOp::RefMut)
                && let Plan::Place { root, proj, .. } = operand.as_ref()
            {
                // Borrowing resolves the analyzed place without
                // reading it, mirroring the direct evaluator.
                let (addr, projs) = eval_plan_place(direct, root, proj, frame)?;
                let (addr, projs) = direct.see_through_root(addr, projs, false, default_span())?;
                return Ok(Flow::Value(HostValue::Ref {
                    addr,
                    projs,
                    mutable: *op == UnOp::RefMut,
                }));
            }
            let value = eval_plan_value(direct, operand, frame)?;
            ops::checked_unary(*op, &value)
                .map(Flow::Value)
                .map_err(|trap| Direct::trap(trap, default_span()))
        }
        Plan::Binary(op, left, right) => {
            if *op == BinOp::And || *op == BinOp::Or {
                let HostValue::Bool(first) = eval_plan_value(direct, left, frame)? else {
                    return Err(Direct::trap(Trap::Dangling, default_span()));
                };
                if (*op == BinOp::And && !first) || (*op == BinOp::Or && first) {
                    return Ok(Flow::Value(HostValue::Bool(*op == BinOp::Or)));
                }
                let HostValue::Bool(second) = eval_plan_value(direct, right, frame)? else {
                    return Err(Direct::trap(Trap::Dangling, default_span()));
                };
                return Ok(Flow::Value(HostValue::Bool(second)));
            }
            let a = eval_plan_value(direct, left, frame)?;
            let b = eval_plan_value(direct, right, frame)?;
            ops::checked_binary(*op, &a, &b)
                .map(Flow::Value)
                .map_err(|trap| Direct::trap(trap, default_span()))
        }
        Plan::Assign {
            op,
            root,
            proj,
            value,
        } => {
            let (addr, projs) = eval_plan_place(direct, root, proj, frame)?;
            let (addr, projs) = direct.see_through_root(addr, projs, false, default_span())?;
            let produced = eval_plan_value(direct, value, frame)?;
            let final_value = match op {
                None => produced,
                Some(binop) => {
                    let current = direct
                        .engine
                        .read_at(addr, &projs)
                        .map_err(|trap| Direct::trap(trap, default_span()))?;
                    ops::checked_binary(*binop, &current, &produced)
                        .map_err(|trap| Direct::trap(trap, default_span()))?
                }
            };
            direct
                .engine
                .write_at(addr, &projs, final_value)
                .map_err(|trap| Direct::trap(trap, default_span()))?;
            Ok(Flow::Value(HostValue::Unit))
        }
        Plan::If { test, then, els } => {
            let HostValue::Bool(decision) = eval_plan_value(direct, test, frame)? else {
                return Err(Direct::trap(Trap::Dangling, default_span()));
            };
            if decision {
                eval_plan(direct, then, frame)
            } else {
                eval_plan(direct, els, frame)
            }
        }
        Plan::IfLet {
            pat,
            value,
            then,
            els,
        } => {
            let tested = eval_plan_value(direct, value, frame)?;
            if direct.bind_pattern(pat, &tested, frame)? {
                eval_plan(direct, then, frame)
            } else {
                eval_plan(direct, els, frame)
            }
        }
        Plan::Match { scrutinee, arms } => {
            let tested = eval_plan_value(direct, scrutinee, frame)?;
            for (pat, body) in arms {
                if direct.bind_pattern(pat, &tested, frame)? {
                    return eval_plan(direct, body, frame);
                }
            }
            Err(Direct::trap(Trap::Dangling, default_span()))
        }
        Plan::Seq { stmts, tail } => {
            for stmt in stmts {
                let flow = match stmt {
                    PlanStmt::Let {
                        binding,
                        destruct,
                        value,
                    } => {
                        let produced = eval_plan(direct, value, frame)?;
                        let Flow::Value(produced) = produced else {
                            return Ok(produced);
                        };
                        match destruct {
                            Some((left, right)) => {
                                if let HostValue::Tuple(a, b) = produced {
                                    direct.write(*left, *a, frame)?;
                                    direct.write(*right, *b, frame)?;
                                }
                            }
                            None => direct.write(*binding, produced, frame)?,
                        }
                        Flow::Value(HostValue::Unit)
                    }
                    PlanStmt::Expr(plan) => eval_plan(direct, plan, frame)?,
                };
                if !matches!(flow, Flow::Value(_)) {
                    return Ok(flow);
                }
            }
            match tail {
                Some(tail) => eval_plan(direct, tail, frame),
                None => Ok(Flow::Value(HostValue::Unit)),
            }
        }
        Plan::Loop { test, body } => loop {
            if let Some(test) = test {
                let HostValue::Bool(decision) = eval_plan_value(direct, test, frame)? else {
                    return Err(Direct::trap(Trap::Dangling, default_span()));
                };
                if !decision {
                    break Ok(Flow::Value(HostValue::Unit));
                }
            }
            match eval_plan(direct, body, frame)? {
                Flow::Value(_) | Flow::Continue => {}
                Flow::Break(value) => break Ok(Flow::Value(value)),
                flow @ Flow::Return(_) => break Ok(flow),
            }
        },
        Plan::WhileLet { pat, value, body } => loop {
            let tested = eval_plan_value(direct, value, frame)?;
            if !direct.bind_pattern(pat, &tested, frame)? {
                break Ok(Flow::Value(HostValue::Unit));
            }
            match eval_plan(direct, body, frame)? {
                Flow::Value(_) | Flow::Continue => {}
                Flow::Break(value) => break Ok(Flow::Value(value)),
                flow @ Flow::Return(_) => break Ok(flow),
            }
        },
        Plan::For {
            pat,
            iterable,
            body,
        } => {
            let mut iterator = eval_plan_iterable(direct, iterable, frame)?;
            loop {
                let step = ops::iterator_next(&mut iterator)
                    .map_err(|trap| Direct::trap(trap, default_span()))?;
                let item = match ops::builtin_variant(&step) {
                    Some((0, payload)) if payload.len() == 1 => payload[0].clone(),
                    _ => break Ok(Flow::Value(HostValue::Unit)),
                };
                if direct.bind_pattern(pat, &item, frame)? {
                    match eval_plan(direct, body, frame)? {
                        Flow::Value(_) | Flow::Continue => {}
                        Flow::Break(value) => break Ok(Flow::Value(value)),
                        flow @ Flow::Return(_) => break Ok(flow),
                    }
                }
            }
        }
        Plan::Return(value) => {
            let produced = match value {
                Some(plan) => eval_plan_value(direct, plan, frame)?,
                None => HostValue::Unit,
            };
            Ok(Flow::Return(produced))
        }
        Plan::Break(value) => {
            let produced = match value {
                Some(plan) => eval_plan_value(direct, plan, frame)?,
                None => HostValue::Unit,
            };
            Ok(Flow::Break(produced))
        }
        Plan::Continue => Ok(Flow::Continue),
        Plan::Try(inner) => {
            let tested = eval_plan_value(direct, inner, frame)?;
            match ops::builtin_variant(&tested) {
                Some((0, payload)) if payload.len() == 1 => Ok(Flow::Value(payload[0].clone())),
                Some((1, _)) => Ok(Flow::Return(tested)),
                _ => Err(Direct::trap(Trap::Dangling, default_span())),
            }
        }
        Plan::Range(left, right) => {
            let start = eval_plan_value(direct, left, frame)?;
            let end = eval_plan_value(direct, right, frame)?;
            let iterator =
                ops::range_of(&start, &end).map_err(|trap| Direct::trap(trap, default_span()))?;
            Ok(Flow::Value(iterator))
        }
        Plan::ClosureValue { closure } => {
            let mut resolved = Vec::with_capacity(closure.captures.len());
            for capture in &closure.captures {
                let value = direct
                    .engine
                    .capture_value(capture.binding, capture.mode)
                    .map_err(|trap| Direct::trap(trap, default_span()))?;
                resolved.push((capture.binding, capture.mode, value));
            }
            Ok(Flow::Value(ops::closure_value(
                closure.kind,
                Arc::new(closure.body.clone()),
                closure.params.clone(),
                resolved,
                closure.ret.clone(),
                closure.frame_slots,
                closure.bind_base,
            )))
        }
    }
}

fn eval_plan_iterable(
    direct: &mut Direct,
    plan: &Plan,
    frame: usize,
) -> Result<HostValue, TrapReport> {
    if let Plan::Range(left, right) = plan {
        let start = eval_plan_value(direct, left, frame)?;
        let end = eval_plan_value(direct, right, frame)?;
        return ops::range_of(&start, &end).map_err(|trap| Direct::trap(trap, default_span()));
    }
    let value = eval_plan_value(direct, plan, frame)?;
    match value {
        HostValue::Iter(_) => Ok(value),
        HostValue::Ref {
            addr,
            projs,
            mutable,
        } => {
            let collection = direct
                .engine
                .read_at(addr, &projs)
                .map_err(|trap| Direct::trap(trap, default_span()))?;
            let len = match collection {
                HostValue::Vec(items) | HostValue::Array(items) => items.len(),
                _ => return Err(Direct::trap(Trap::Dangling, default_span())),
            };
            Ok(ops::refs_iterator(addr, projs, len, mutable))
        }
        HostValue::Vec(items) | HostValue::Array(items) => Ok(ops::items_iterator(items)),
        _ => Err(Direct::trap(Trap::Dangling, default_span())),
    }
}

fn eval_plan_value(
    direct: &mut Direct,
    plan: &Plan,
    frame: usize,
) -> Result<HostValue, TrapReport> {
    match eval_plan(direct, plan, frame)? {
        Flow::Value(value) => Ok(value),
        _ => Err(Direct::trap(Trap::Dangling, default_span())),
    }
}

fn eval_plan_args(
    direct: &mut Direct,
    args: &[Plan],
    frame: usize,
) -> Result<Vec<HostValue>, TrapReport> {
    let mut values = Vec::with_capacity(args.len());
    for arg in args {
        values.push(eval_plan_value(direct, arg, frame)?);
    }
    Ok(values)
}

fn eval_plan_place(
    direct: &mut Direct,
    root: &PlanRoot,
    proj: &[PlanProj],
    frame: usize,
) -> Result<(Addr, Vec<RtProj>), TrapReport> {
    let (addr, mut projs) = match root {
        PlanRoot::Local(bind) => direct
            .engine
            .local_place(*bind)
            .map_err(|trap| Direct::trap(trap, default_span()))?,
        PlanRoot::Deref(inner) => match eval_plan_value(direct, inner, frame)? {
            HostValue::Ref { addr, projs, .. } => (addr, projs),
            _ => return Err(Direct::trap(Trap::Dangling, default_span())),
        },
    };
    for step in proj {
        match step {
            PlanProj::Field(index) => projs.push(RtProj::Field(*index)),
            PlanProj::Index(index) => {
                let index = eval_plan_value(direct, index, frame)?;
                let at = ops::index_position(&index)
                    .map_err(|trap| Direct::trap(trap, default_span()))?;
                projs.push(RtProj::Index(at));
            }
        }
    }
    Ok((addr, projs))
}

fn default_span() -> sicp_runtime::host::diag::Span {
    sicp_runtime::host::diag::Span::new(1, 1, 1)
}
