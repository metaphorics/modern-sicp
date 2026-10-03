// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 5.4
//!
//! Section 5.4: the explicit-control evaluator. The machine is a guest
//! data structure and a transition function (grammar §9): typed control
//! states, named registers, and one explicit `Vec` stack over the same
//! checked object AST the direct evaluator runs. Operand evaluation,
//! application, and loop control are explicit states with saved
//! continuations; the direct evaluator shares none of this control
//! code, and agreement between the two is a conformance observation,
//! not a shared path.

use std::collections::VecDeque;

use sicp_runtime::host::check::CheckedProgram;
use sicp_runtime::host::diag::{Diag, Span};
use sicp_runtime::host::hir::{
    BinOp, BindId, CtorOp, FormatKind, FunId, HirBlock, HirExpr, HirExprKind, HirPat, MethodOp,
    PlaceRoot, Proj, Sema, UnOp,
};
use sicp_runtime::host::ops::{self, Flow, RunOutcome, TrapReport};
use sicp_runtime::host::value::{Addr, HostValue, RtProj, Trap};

/// One typed control state of the explicit machine.
#[derive(Debug, Clone)]
pub enum Control {
    /// Evaluate the expression register.
    Eval(HirExpr),
    /// Execute the statements of a block, then its tail.
    Exec(HirBlock),
    /// Write the value register into a binding, then continue with the
    /// rest of the block.
    Bind {
        /// The fresh slot to write.
        binding: BindId,
        /// The tuple destructuring slots.
        destruct: Option<(BindId, BindId)>,
        /// The statements and tail that follow.
        rest: HirBlock,
    },
    /// Evaluate operands left to right; each finished operand lands in
    /// `done`, and `after` runs when none are pending.
    Args {
        /// The operands still unevaluated.
        pending: VecDeque<HirExpr>,
        /// The operands already evaluated, in order.
        done: Vec<HostValue>,
        /// What runs once every operand is evaluated.
        after: Resume,
    },
    /// Evaluate the right operand of a binary operator with the left
    /// operand in the value register.
    Binop {
        /// The operator.
        op: BinOp,
        /// The right operand.
        right: Box<HirExpr>,
    },
    /// Apply one unary operator to the value register.
    Unary(UnOp),
    /// Project one field of the value register.
    Field(u32),
    /// The indexed base is evaluated; evaluate the index.
    IndexBase {
        /// The index expression.
        index: Box<HirExpr>,
    },
    /// The index is evaluated; read the element of the base.
    IndexAt {
        /// The evaluated base (a vector or array).
        base: HostValue,
    },
    /// Store the value register into a resolved place, optionally
    /// through one compound operator.
    Assign {
        /// The destination.
        addr: Addr,
        /// The destination projections.
        projs: Vec<RtProj>,
        /// The compound operator.
        op: Option<BinOp>,
    },
    /// Try each arm against the value register until one binds.
    MatchArms {
        /// The arms, in order.
        arms: Vec<(HirPat, HirExpr)>,
    },
    /// Test one pattern against the value register and branch.
    TestPattern {
        /// The pattern to bind.
        pat: HirPat,
        /// The state on a successful match.
        success: Box<Control>,
        /// The state on a failed match.
        failure: Box<Control>,
    },
    /// Apply the procedure register to the argument register.
    Apply,
    /// Wrap the value register in a `return` signal.
    ReturnValue,
    /// Wrap the value register in a `break` signal.
    BreakValue,
    /// Test the value register for the `Try` early-return lesson.
    TryTest,
    /// The `Range` left endpoint is evaluated; evaluate the right one.
    RangeLeft {
        /// The right endpoint expression.
        right: Box<HirExpr>,
    },
    /// Both `Range` endpoints are evaluated; build the iterator.
    RangeEnd {
        /// The evaluated left endpoint.
        left: HostValue,
    },
    /// The binary left operand is evaluated; evaluate the right one.
    BinaryRight {
        /// The operator.
        op: BinOp,
        /// The evaluated left operand.
        left: HostValue,
    },
    /// A `while` test is evaluated; branch on its boolean value.
    WhileCheck {
        /// The test expression (re-armed on iteration).
        test: Box<HirExpr>,
        /// The loop body.
        body: Box<HirBlock>,
    },
    /// A `while let` scrutinee is evaluated; bind it or exit.
    WhileLetCheck {
        /// The binding pattern.
        pat: HirPat,
        /// The scrutinee expression (re-armed on iteration).
        value: Box<HirExpr>,
        /// The loop body.
        body: Box<HirBlock>,
    },
    /// A `for` iterable is evaluated; build the iterator.
    ForIterable {
        /// The binding pattern.
        pat: HirPat,
        /// The loop body.
        body: Box<HirBlock>,
    },
    /// Resolve one place expression to an address and projections.
    ResolvePlace {
        /// The place to resolve.
        place: sicp_runtime::host::hir::Place,
        /// What runs once the address is resolved.
        then: PlaceCont,
    },
    /// The referent of a place root is evaluated; continue resolution.
    PlaceDeref {
        /// The projections still to resolve.
        proj: Vec<sicp_runtime::host::hir::Proj>,
        /// What runs once the address is resolved.
        then: PlaceCont,
    },
    /// One place index is evaluated; push it and continue.
    PlaceIndex {
        /// The resolved address so far.
        addr: Addr,
        /// The projections resolved so far.
        projs: Vec<RtProj>,
        /// The projections still to resolve.
        rest: Vec<sicp_runtime::host::hir::Proj>,
        /// What runs once the address is resolved.
        then: PlaceCont,
    },
    /// One loop: its recurrence kind and its body.
    Loop {
        /// The recurrence discipline.
        kind: LoopKind,
        /// The loop body.
        body: Box<HirBlock>,
    },
    /// One guest function boundary: `return` and fall-off both exit
    /// the activation and resume the caller; main's boundary halts.
    FunEnd {
        /// The caller's frame register, restored on exit.
        caller_frame: usize,
    },
    /// The machine has halted.
    Halt,
}

/// How one [`Control::Loop`] re-enters itself.
#[derive(Debug, Clone)]
pub enum LoopKind {
    /// `loop`: the body repeats until a `break` or `return`.
    Forever,
    /// `while TEST BODY`: test before every iteration.
    While {
        /// The condition.
        test: Box<HirExpr>,
    },
    /// `while let PAT = VALUE BODY`.
    WhileLet {
        /// The matched pattern.
        pat: HirPat,
        /// The scrutinee, re-evaluated every iteration.
        value: Box<HirExpr>,
    },
    /// `for PAT in ITERABLE BODY`: the iterator register steps.
    For {
        /// The binding pattern.
        pat: HirPat,
    },
}

/// What runs once a place resolves to an address and projections.
#[derive(Debug, Clone)]
pub enum PlaceCont {
    /// Read the resolved place into the value register.
    Read {
        /// Whether the read moves the value out.
        mode: sicp_runtime::host::hir::PlaceUse,
    },
    /// Evaluate the assigned value, then store it.
    Assign {
        /// The compound operator.
        op: Option<BinOp>,
        /// The assigned value expression.
        value: Box<HirExpr>,
    },
    /// Evaluate the receiver, then run the method over its arguments.
    Method {
        /// The method to run.
        op: MethodOp,
        /// The receiver expression.
        receiver: Box<HirExpr>,
        /// The argument expressions.
        args: Vec<HirExpr>,
    },
    /// Build the reference value for the resolved place without
    /// reading it: borrowing aliases the slot.
    Borrow {
        /// Whether the borrow is exclusive.
        mutable: bool,
    },
}

/// What the machine does after an operand sequence completes.
#[derive(Debug, Clone)]
pub enum Resume {
    /// Call the resolved top-level function.
    Call {
        /// The resolved callee.
        callee: FunId,
    },
    /// Call the evaluated callee value.
    CallValue,
    /// Build a struct or tuple-struct literal.
    Struct {
        /// The item identity.
        item: u32,
    },
    /// Build an enum variant.
    Variant {
        /// The item identity.
        item: u32,
        /// The variant index.
        index: u32,
    },
    /// Build a two-element tuple.
    Tuple,
    /// Build a vector.
    VecBuild,
    /// Run an admitted constructor.
    Ctor(CtorOp),
    /// Run an admitted method over the evaluated receiver.
    Method {
        /// The method to run.
        op: MethodOp,
        /// The receiver's resolved place.
        place: Option<(Addr, Vec<RtProj>)>,
    },
    /// Render one format call.
    Format {
        /// The macro's kind.
        kind: FormatKind,
        /// The parsed specification.
        spec: sicp_runtime::host::hir::FormatSpec,
    },
    /// Repeat one value for `vec![value; count]`.
    VecRepeat,
}

/// One signal a control state raises to its saved continuations.
#[derive(Debug, Clone)]
pub enum Signal {
    /// `return`, carrying its value.
    Return(HostValue),
    /// `break`, carrying its typed value.
    Break(HostValue),
    /// `continue`.
    Continue,
}

/// The explicit-control evaluator: registers, explicit stacks, and the
/// shared leaf engine.
pub struct Eceval {
    /// The shared leaf semantics.
    pub engine: ops::Engine,
    /// The control state register.
    pub control: Control,
    /// The saved control states: the machine's `continue` register.
    pub continues: Vec<Control>,
    /// The explicit save stack.
    pub stack: Vec<HostValue>,
    /// The iterator register, used by `for` loops.
    pub iterator: Option<HostValue>,
    /// The value register.
    pub val: HostValue,
    /// The procedure register.
    pub proc: HostValue,
    /// The argument register.
    pub argl: Vec<HostValue>,
    /// The frame register.
    pub frame: usize,
    /// The pending signal register.
    pub signal: Option<Signal>,
    /// Whether the machine has halted.
    pub halted: bool,
}

impl Eceval {
    /// Builds the machine over one checked program and runs its
    /// `main`, answering the observable outcome.
    #[must_use]
    pub fn run(program: &CheckedProgram) -> RunOutcome {
        let mut machine = Self::new(program.sema.clone());
        match machine.execute_main() {
            Ok(()) => RunOutcome {
                stdout: machine.engine.effects.stdout,
                trap: None,
            },
            Err(report) => RunOutcome {
                stdout: machine.engine.effects.stdout,
                trap: Some(report),
            },
        }
    }

    fn new(sema: Sema) -> Self {
        Self {
            engine: ops::Engine::new(sema),
            control: Control::Halt,
            continues: Vec::new(),
            stack: Vec::new(),
            iterator: None,
            val: HostValue::Unit,
            proc: HostValue::Unit,
            argl: Vec::new(),
            frame: 0,
            signal: None,
            halted: false,
        }
    }

    fn execute_main(&mut self) -> Result<(), TrapReport> {
        let main = self.engine.sema.main;
        let body = self.engine.sema.funs[main.0 as usize].body.clone();
        self.enter_function(main, Vec::new(), &body)?;
        self.drive()
    }

    fn enter_function(
        &mut self,
        fun: FunId,
        args: Vec<HostValue>,
        body: &HirBlock,
    ) -> Result<(), TrapReport> {
        let def = self.engine.sema.funs[fun.0 as usize].clone();
        let frame = self
            .engine
            .push_activation(def.bind_base, def.frame_slots, Vec::new());
        for ((binding, _), value) in def.params.iter().zip(args) {
            self.engine
                .write_local(*binding, value)
                .map_err(Self::trap)?;
        }
        let caller_frame = self.frame;
        self.frame = frame;
        self.continues.push(Control::FunEnd { caller_frame });
        self.control = Control::Exec(body.clone());
        Ok(())
    }

    /// Runs the machine until it halts.
    ///
    /// # Errors
    /// The first [`TrapReport`] the machine raises.
    pub fn drive(&mut self) -> Result<(), TrapReport> {
        while !self.halted {
            self.step()?;
        }
        Ok(())
    }

    /// Executes one transition of the machine.
    ///
    /// # Errors
    /// The first [`TrapReport`] the transition raises.
    // Keep this exhaustive transition dispatch aligned with `Control`.
    #[allow(clippy::too_many_lines)]
    pub fn step(&mut self) -> Result<(), TrapReport> {
        let control = std::mem::replace(&mut self.control, Control::Halt);
        match control {
            Control::Eval(expr) => self.step_eval(expr),
            Control::Exec(block) => {
                self.step_exec(block);
                Ok(())
            }
            Control::Bind {
                binding,
                destruct,
                rest,
            } => self.step_bind(binding, destruct, rest),
            Control::Args {
                pending,
                done,
                after,
            } => self.step_args(pending, done, after),
            Control::Binop { op, right } => self.step_binop(op, *right),
            Control::Unary(op) => self.step_unary(op),
            Control::Field(index) => self.step_field(index),
            Control::Assign { addr, projs, op } => self.step_assign(addr, &projs, op),
            Control::MatchArms { arms } => self.step_match(arms),
            Control::TestPattern {
                pat,
                success,
                failure,
            } => self.step_test_pattern(&pat, *success, *failure),
            Control::Apply => self.step_apply(),
            Control::ReturnValue => {
                self.signal = Some(Signal::Return(std::mem::replace(
                    &mut self.val,
                    HostValue::Unit,
                )));
                self.unwind();
                Ok(())
            }
            Control::BreakValue => {
                self.signal = Some(Signal::Break(std::mem::replace(
                    &mut self.val,
                    HostValue::Unit,
                )));
                self.unwind();
                Ok(())
            }
            Control::TryTest => {
                let tested = std::mem::replace(&mut self.val, HostValue::Unit);
                match ops::builtin_variant(&tested) {
                    Some((0, payload)) if payload.len() == 1 => {
                        self.val = payload[0].clone();
                        self.unwind();
                    }
                    Some((1, _)) => {
                        self.signal = Some(Signal::Return(tested));
                        self.unwind();
                    }
                    _ => return Err(Self::trap(Trap::Dangling)),
                }
                Ok(())
            }
            Control::RangeLeft { right } => {
                let left = std::mem::replace(&mut self.val, HostValue::Unit);
                self.continues.push(Control::RangeEnd { left });
                self.control = Control::Eval(*right);
                Ok(())
            }
            Control::RangeEnd { left } => {
                let end = std::mem::replace(&mut self.val, HostValue::Unit);
                self.val = ops::range_of(&left, &end).map_err(Self::trap)?;
                self.unwind();
                Ok(())
            }
            Control::IndexBase { index } => {
                let base = std::mem::replace(&mut self.val, HostValue::Unit);
                let base = self.through_ref(base)?;
                self.continues.push(Control::IndexAt { base });
                self.control = Control::Eval(*index);
                Ok(())
            }
            Control::IndexAt { base } => {
                let index = std::mem::replace(&mut self.val, HostValue::Unit);
                let at = ops::index_position(&index).map_err(Self::trap)?;
                self.val = index_value(&base, at).map_err(Self::trap)?;
                self.unwind();
                Ok(())
            }
            Control::BinaryRight { op, left } => {
                let right = std::mem::replace(&mut self.val, HostValue::Unit);
                self.val = ops::checked_binary(op, &left, &right).map_err(Self::trap)?;
                self.unwind();
                Ok(())
            }
            Control::WhileCheck { test, body } => {
                let HostValue::Bool(decision) = std::mem::replace(&mut self.val, HostValue::Unit)
                else {
                    return Err(Self::trap(Trap::Dangling));
                };
                if decision {
                    self.continues.push(Control::Loop {
                        kind: LoopKind::While { test },
                        body: body.clone(),
                    });
                    self.control = Control::Exec(*body);
                } else {
                    self.val = HostValue::Unit;
                    self.unwind();
                }
                Ok(())
            }
            Control::WhileLetCheck { pat, value, body } => {
                let tested = std::mem::replace(&mut self.val, HostValue::Unit);
                if self.bind_pattern(&pat, &tested)? {
                    self.continues.push(Control::Loop {
                        kind: LoopKind::WhileLet { pat, value },
                        body: body.clone(),
                    });
                    self.control = Control::Exec(*body);
                } else {
                    self.val = HostValue::Unit;
                    self.unwind();
                }
                Ok(())
            }
            Control::ForIterable { pat, body } => {
                let value = std::mem::replace(&mut self.val, HostValue::Unit);
                match value {
                    HostValue::Iter(_) => {
                        self.iterator = Some(value);
                    }
                    HostValue::Ref {
                        addr,
                        projs,
                        mutable,
                        ..
                    } => {
                        let collection = self.engine.read_at(addr, &projs).map_err(Self::trap)?;
                        let len = match collection {
                            HostValue::Vec(items) | HostValue::Array(items) => items.len(),
                            _ => return Err(Self::trap(Trap::Dangling)),
                        };
                        self.iterator = Some(ops::refs_iterator(addr, projs, len, mutable));
                    }
                    HostValue::Vec(items) | HostValue::Array(items) => {
                        self.iterator = Some(ops::items_iterator(items));
                    }
                    _ => return Err(Self::trap(Trap::Dangling)),
                }
                self.control = Control::Loop {
                    kind: LoopKind::For { pat },
                    body,
                };
                Ok(())
            }
            Control::ResolvePlace { place, then } => self.step_resolve_place(place, then),
            Control::PlaceDeref { proj, then } => {
                let HostValue::Ref { addr, projs, .. } =
                    std::mem::replace(&mut self.val, HostValue::Unit)
                else {
                    return Err(Self::trap(Trap::Dangling));
                };
                self.fold_place_projs(addr, projs, proj.into_iter(), then)
            }
            Control::PlaceIndex {
                addr,
                mut projs,
                rest,
                then,
            } => {
                let index = std::mem::replace(&mut self.val, HostValue::Unit);
                let at = ops::index_position(&index).map_err(Self::trap)?;
                projs.push(RtProj::Index(at));
                self.fold_place_projs(addr, projs, rest.into_iter(), then)
            }
            Control::Loop { kind, body } => self.step_loop(kind, *body),
            Control::FunEnd { caller_frame } => {
                self.engine.pop_activation();
                self.frame = caller_frame;
                // The tail value (or Unit from a tail-less block) is
                // already in the value register: it becomes the
                // caller's result.
                match self.continues.pop() {
                    Some(next) => self.control = next,
                    None => self.halted = true,
                }
                Ok(())
            }
            Control::Halt => {
                self.halted = true;
                Ok(())
            }
        }
    }

    /// Resumes the saved continuations, absorbing loop signals at
    /// their loop boundaries.
    fn unwind(&mut self) {
        loop {
            match (&self.signal, self.continues.last()) {
                (Some(Signal::Return(_)), _) => {
                    // Pop to this activation's function boundary, then
                    // resume the caller with the returned value.
                    loop {
                        match self.continues.pop() {
                            Some(Control::FunEnd { caller_frame }) => {
                                self.frame = caller_frame;
                                break;
                            }
                            Some(_) => {}
                            None => {
                                self.halted = true;
                                return;
                            }
                        }
                    }
                    self.engine.pop_activation();
                    if let Some(Signal::Return(value)) = self.signal.take() {
                        self.val = value;
                    }
                    match self.continues.pop() {
                        Some(next) => self.control = next,
                        None => self.halted = true,
                    }
                    return;
                }
                (Some(Signal::Break(value)), Some(Control::Loop { .. })) => {
                    self.val = value.clone();
                    self.signal = None;
                    self.continues.pop();
                }
                (Some(Signal::Continue), Some(Control::Loop { .. })) => {
                    self.signal = None;
                    if let Some(loop_state) = self.continues.pop() {
                        self.control = loop_state;
                    }
                    return;
                }
                (Some(_), Some(_)) => {
                    self.continues.pop();
                }
                (Some(_) | None, None) => {
                    self.halted = true;
                    return;
                }
                (None, Some(_)) => {
                    if let Some(next) = self.continues.pop() {
                        self.control = next;
                    }
                    return;
                }
            }
        }
    }

    fn step_exec(&mut self, block: HirBlock) {
        let HirBlock { stmts, tail } = block;
        let mut stmts = stmts.into_iter();
        match stmts.next() {
            Some(sicp_runtime::host::hir::HirStmt::Let {
                binding,
                destruct,
                value,
            }) => {
                let rest = HirBlock {
                    stmts: stmts.collect(),
                    tail,
                };
                self.continues.push(Control::Bind {
                    binding,
                    destruct,
                    rest,
                });
                self.control = Control::Eval(value);
            }
            Some(sicp_runtime::host::hir::HirStmt::Expr(expr)) => {
                let rest = HirBlock {
                    stmts: stmts.collect(),
                    tail,
                };
                self.continues.push(Control::Exec(rest));
                self.control = Control::Eval(expr);
            }
            None => {
                if let Some(tail) = tail {
                    self.control = Control::Eval(*tail);
                } else {
                    self.val = HostValue::Unit;
                    self.unwind();
                }
            }
        }
    }

    fn step_bind(
        &mut self,
        binding: BindId,
        destruct: Option<(BindId, BindId)>,
        rest: HirBlock,
    ) -> Result<(), TrapReport> {
        if let Some((left, right)) = destruct {
            if let HostValue::Tuple(a, b) = self.val.clone() {
                self.engine.write_local(left, *a).map_err(Self::trap)?;
                self.engine.write_local(right, *b).map_err(Self::trap)?;
            }
        } else {
            let value = std::mem::replace(&mut self.val, HostValue::Unit);
            self.engine
                .write_local(binding, value)
                .map_err(Self::trap)?;
        }
        self.control = Control::Exec(rest);
        Ok(())
    }

    // Keep HIR expression routing in one exhaustive transition dispatch.
    #[allow(clippy::too_many_lines)]
    fn step_eval(&mut self, expr: HirExpr) -> Result<(), TrapReport> {
        let span = expr.span;
        match expr.kind {
            HirExprKind::I64(value) => {
                self.val = HostValue::Int(value);
                self.unwind();
            }
            HirExprKind::Usize(value) => {
                self.val = HostValue::Usize(value);
                self.unwind();
            }
            HirExprKind::Bool(value) => {
                self.val = HostValue::Bool(value);
                self.unwind();
            }
            HirExprKind::Unit => {
                self.val = HostValue::Unit;
                self.unwind();
            }
            HirExprKind::Str(text) => {
                self.val = HostValue::Text(text);
                self.unwind();
            }
            HirExprKind::FunRef(fun) => {
                self.val = HostValue::FnPtr(fun);
                self.unwind();
            }
            HirExprKind::Place { place, mode } => {
                self.control = Control::ResolvePlace {
                    place,
                    then: PlaceCont::Read { mode },
                };
            }
            HirExprKind::Call { callee, args } => {
                self.start_args(args, Resume::Call { callee })?;
            }
            HirExprKind::IndirectCall { callee, args } => {
                self.continues.push(Control::Args {
                    pending: args.into(),
                    done: Vec::new(),
                    after: Resume::CallValue,
                });
                self.control = Control::Eval(*callee);
            }
            HirExprKind::Ctor(op, args) => self.start_args(args, Resume::Ctor(op))?,
            HirExprKind::StructLit(id, fields) | HirExprKind::TupleStructLit(id, fields) => {
                self.start_args(fields, Resume::Struct { item: id.0 })?;
            }
            HirExprKind::VariantLit(id, index, payload) => {
                self.start_args(payload, Resume::Variant { item: id.0, index })?;
            }
            HirExprKind::Tuple(left, right) => {
                self.start_args(vec![*left, *right], Resume::Tuple)?;
            }
            HirExprKind::Array(items) | HirExprKind::VecList(items) => {
                self.start_args(items, Resume::VecBuild)?;
            }
            HirExprKind::VecRepeat(value, count) => {
                self.start_args(vec![*value, *count], Resume::VecRepeat)?;
            }
            HirExprKind::Format { kind, spec, args } => {
                self.start_args(args, Resume::Format { kind, spec })?;
            }
            HirExprKind::Field { base, index } => {
                self.continues.push(Control::Field(index));
                self.control = Control::Eval(*base);
            }
            HirExprKind::Index { base, index } => {
                self.continues.push(Control::IndexBase { index });
                self.control = Control::Eval(*base);
            }
            HirExprKind::Binary { op, left, right } => {
                self.continues.push(Control::Binop { op, right });
                self.control = Control::Eval(*left);
            }
            HirExprKind::Unary { op, operand } => {
                if let (UnOp::Ref | UnOp::RefMut, HirExprKind::Place { place, .. }) =
                    (op, &operand.kind)
                {
                    // Borrowing names the place's address without
                    // reading it, so the reference aliases the slot.
                    self.control = Control::ResolvePlace {
                        place: place.clone(),
                        then: PlaceCont::Borrow {
                            mutable: op == UnOp::RefMut,
                        },
                    };
                    return Ok(());
                }
                self.continues.push(Control::Unary(op));
                self.control = Control::Eval(*operand);
            }
            HirExprKind::Method {
                op,
                receiver,
                receiver_place,
                args,
            } => {
                // The receiver's place resolves through its own
                // control frames before the receiver evaluates.
                if let Some(place) = receiver_place {
                    self.control = Control::ResolvePlace {
                        place,
                        then: PlaceCont::Method { op, receiver, args },
                    };
                } else {
                    self.continues.push(Control::Args {
                        pending: args.into(),
                        done: Vec::new(),
                        after: Resume::Method { op, place: None },
                    });
                    self.control = Control::Eval(*receiver);
                }
            }
            HirExprKind::Assign { op, target, value } => {
                self.control = Control::ResolvePlace {
                    place: target,
                    then: PlaceCont::Assign { op, value },
                };
            }
            HirExprKind::If {
                test,
                then,
                else_branch,
            } => {
                self.continues.push(Control::TestPattern {
                    pat: HirPat {
                        kind: sicp_runtime::host::hir::HirPatKind::Bool(true),
                        span,
                    },
                    success: Box::new(Control::Eval(*then)),
                    failure: Box::new(Control::Eval(*else_branch)),
                });
                self.control = Control::Eval(*test);
            }
            HirExprKind::IfLet {
                pat,
                value,
                then,
                else_branch,
            } => {
                self.continues.push(Control::TestPattern {
                    pat,
                    success: Box::new(Control::Eval(*then)),
                    failure: Box::new(Control::Eval(*else_branch)),
                });
                self.control = Control::Eval(*value);
            }
            HirExprKind::Match { scrutinee, arms } => {
                self.continues.push(Control::MatchArms { arms });
                self.control = Control::Eval(*scrutinee);
            }
            HirExprKind::Block(block) => self.control = Control::Exec(block),
            HirExprKind::Loop { body, .. } => {
                self.control = Control::Loop {
                    kind: LoopKind::Forever,
                    body: Box::new(body),
                };
            }
            HirExprKind::While { test, body } => {
                self.control = Control::Loop {
                    kind: LoopKind::While { test },
                    body: Box::new(body),
                };
            }
            HirExprKind::WhileLet { pat, value, body } => {
                self.control = Control::Loop {
                    kind: LoopKind::WhileLet { pat, value },
                    body: Box::new(body),
                };
            }
            HirExprKind::For {
                pat,
                iterable,
                body,
            } => {
                self.continues.push(Control::ForIterable {
                    pat,
                    body: Box::new(body),
                });
                self.control = Control::Eval(*iterable);
            }
            HirExprKind::Closure(closure) => {
                let mut captures = Vec::with_capacity(closure.captures.len());
                for capture in &closure.captures {
                    let value = self
                        .engine
                        .capture_value(capture.binding, capture.mode)
                        .map_err(Self::trap)?;
                    captures.push((capture.binding, capture.mode, value));
                }
                self.val = ops::closure_value(
                    closure.kind,
                    std::sync::Arc::new(closure.body.clone()),
                    closure.params.clone(),
                    captures,
                    closure.ret.clone(),
                    closure.frame_slots,
                    closure.bind_base,
                );
                self.unwind();
            }
            HirExprKind::Return(value) => {
                if let Some(value) = value {
                    self.continues.push(Control::ReturnValue);
                    self.control = Control::Eval(*value);
                } else {
                    self.signal = Some(Signal::Return(HostValue::Unit));
                    self.unwind();
                }
            }
            HirExprKind::Break(value) => {
                if let Some(value) = value {
                    self.continues.push(Control::BreakValue);
                    self.control = Control::Eval(*value);
                } else {
                    self.signal = Some(Signal::Break(HostValue::Unit));
                    self.unwind();
                }
            }
            HirExprKind::Continue => {
                self.signal = Some(Signal::Continue);
                self.unwind();
            }
            HirExprKind::Try(inner) => {
                self.continues.push(Control::TryTest);
                self.control = Control::Eval(*inner);
            }
            HirExprKind::Range(left, right) => {
                self.continues.push(Control::RangeLeft { right });
                self.control = Control::Eval(*left);
            }
        }
        Ok(())
    }

    fn start_args(&mut self, args: Vec<HirExpr>, after: Resume) -> Result<(), TrapReport> {
        let mut pending: VecDeque<HirExpr> = args.into();
        match pending.pop_front() {
            Some(first) => {
                self.continues.push(Control::Args {
                    pending,
                    done: Vec::new(),
                    after,
                });
                self.control = Control::Eval(first);
            }
            None => self.finish_args(Vec::new(), after)?,
        }
        Ok(())
    }

    fn step_args(
        &mut self,
        mut pending: VecDeque<HirExpr>,
        mut done: Vec<HostValue>,
        after: Resume,
    ) -> Result<(), TrapReport> {
        done.push(std::mem::replace(&mut self.val, HostValue::Unit));
        match pending.pop_front() {
            Some(next) => {
                self.continues.push(Control::Args {
                    pending,
                    done,
                    after,
                });
                self.control = Control::Eval(next);
            }
            None => self.finish_args(done, after)?,
        }
        Ok(())
    }

    fn finish_args(&mut self, values: Vec<HostValue>, after: Resume) -> Result<(), TrapReport> {
        match after {
            Resume::Call { callee } => {
                let def = self.engine.sema.funs[callee.0 as usize].clone();
                let body = def.body.clone();
                self.enter_function(callee, values, &body)?;
            }
            Resume::CallValue => {
                // The callee was evaluated first, like a method
                // receiver: it leads the evaluated values.
                let mut values = values.into_iter();
                self.proc = values.next().unwrap_or(HostValue::Unit);
                self.argl = values.collect();
                self.control = Control::Apply;
            }
            Resume::Struct { item } => {
                self.val = HostValue::Struct(item, values);
                self.unwind();
            }
            Resume::Variant { item, index } => {
                self.val = HostValue::Variant(item, index, values);
                self.unwind();
            }
            Resume::Tuple => {
                if let [a, b] = &values[..] {
                    self.val = HostValue::Tuple(Box::new(a.clone()), Box::new(b.clone()));
                }
                self.unwind();
            }
            Resume::VecBuild => {
                self.val = HostValue::Vec(values);
                self.unwind();
            }
            Resume::Ctor(op) => {
                self.val = ops::construct(op, &values).map_err(Self::trap)?;
                self.unwind();
            }
            Resume::Method { op, place } => {
                let mut values = values.into_iter();
                // Method bodies match on owned shapes: a borrowed
                // receiver reads through to its referent first.
                let receiver = self.through_ref(values.next().unwrap_or(HostValue::Unit))?;
                let args: Vec<HostValue> = values.collect();
                let (result, updated) =
                    ops::apply_method(op, receiver, place.clone(), &args).map_err(Self::trap)?;
                if let (Some(updated), Some((addr, projs))) = (updated, place) {
                    self.engine
                        .write_at(addr, &projs, updated)
                        .map_err(Self::trap)?;
                }
                self.val = result;
                self.unwind();
            }
            Resume::Format { kind, spec } => {
                let rendered =
                    ops::render_format(&spec, &values, &self.engine.store, &self.engine.sema.items)
                        .map_err(Self::trap)?;
                self.val = match kind {
                    FormatKind::Format => HostValue::Text(rendered),
                    FormatKind::Print => {
                        self.engine.effects.push_text(&rendered);
                        HostValue::Unit
                    }
                    FormatKind::Println => {
                        self.engine.effects.push_line(&rendered);
                        HostValue::Unit
                    }
                };
                self.unwind();
            }
            Resume::VecRepeat => {
                if let [item, times] = &values[..] {
                    let count = match times {
                        HostValue::Int(n) => usize::try_from(*n).unwrap_or(0),
                        HostValue::Usize(n) => usize::try_from(*n).unwrap_or(0),
                        _ => 0,
                    };
                    self.val = HostValue::Vec(vec![item.clone(); count]);
                }
                self.unwind();
            }
        }
        Ok(())
    }

    fn step_binop(&mut self, op: BinOp, right: HirExpr) -> Result<(), TrapReport> {
        if matches!(op, BinOp::And | BinOp::Or) {
            let HostValue::Bool(first) = self.val.clone() else {
                return Err(Self::trap(Trap::Dangling));
            };
            let short = (op == BinOp::And && !first) || (op == BinOp::Or && first);
            if short {
                self.val = HostValue::Bool(op == BinOp::Or);
                self.unwind();
                return Ok(());
            }
            self.control = Control::Eval(right);
            return Ok(());
        }
        let left = std::mem::replace(&mut self.val, HostValue::Unit);
        self.continues.push(Control::BinaryRight { op, left });
        self.control = Control::Eval(right);
        Ok(())
    }

    fn step_unary(&mut self, op: UnOp) -> Result<(), TrapReport> {
        let operand = std::mem::replace(&mut self.val, HostValue::Unit);
        self.val = ops::checked_unary(op, &operand).map_err(Self::trap)?;
        self.unwind();
        Ok(())
    }

    fn step_field(&mut self, index: u32) -> Result<(), TrapReport> {
        let base = std::mem::replace(&mut self.val, HostValue::Unit);
        let base = self.through_ref(base)?;
        self.val = project_value(&base, index).map_err(Self::trap)?;
        self.unwind();
        Ok(())
    }

    fn step_assign(
        &mut self,
        addr: Addr,
        projs: &[RtProj],
        op: Option<BinOp>,
    ) -> Result<(), TrapReport> {
        let produced = std::mem::replace(&mut self.val, HostValue::Unit);
        let final_value = match op {
            None => produced,
            Some(binop) => {
                let current = self.engine.read_at(addr, projs).map_err(Self::trap)?;
                ops::checked_binary(binop, &current, &produced).map_err(Self::trap)?
            }
        };
        self.engine
            .write_at(addr, projs, final_value)
            .map_err(Self::trap)?;
        self.val = HostValue::Unit;
        self.unwind();
        Ok(())
    }

    fn step_match(&mut self, arms: Vec<(HirPat, HirExpr)>) -> Result<(), TrapReport> {
        let tested = std::mem::replace(&mut self.val, HostValue::Unit);
        for (pat, body) in arms {
            if self.bind_pattern(&pat, &tested)? {
                self.control = Control::Eval(body);
                return Ok(());
            }
        }
        Err(Self::trap(Trap::Dangling))
    }

    fn step_test_pattern(
        &mut self,
        pat: &HirPat,
        success: Control,
        failure: Control,
    ) -> Result<(), TrapReport> {
        let tested = std::mem::replace(&mut self.val, HostValue::Unit);
        if self.bind_pattern(pat, &tested)? {
            self.control = success;
        } else {
            self.control = failure;
        }
        Ok(())
    }

    fn step_loop(&mut self, kind: LoopKind, body: HirBlock) -> Result<(), TrapReport> {
        match kind {
            LoopKind::Forever => {
                self.continues.push(Control::Loop {
                    kind: LoopKind::Forever,
                    body: Box::new(body.clone()),
                });
                self.control = Control::Exec(body);
            }
            LoopKind::While { test } => {
                self.continues.push(Control::WhileCheck {
                    test: test.clone(),
                    body: Box::new(body),
                });
                self.control = Control::Eval(*test);
            }
            LoopKind::WhileLet { pat, value } => {
                self.continues.push(Control::WhileLetCheck {
                    pat,
                    value: value.clone(),
                    body: Box::new(body),
                });
                self.control = Control::Eval(*value);
            }
            LoopKind::For { pat } => {
                let Some(mut iterator) = self.iterator.take() else {
                    self.val = HostValue::Unit;
                    self.unwind();
                    return Ok(());
                };
                let step = ops::iterator_next(&mut iterator).map_err(Self::trap)?;
                self.iterator = Some(iterator);
                let item = match ops::builtin_variant(&step) {
                    Some((0, payload)) if payload.len() == 1 => payload[0].clone(),
                    _ => {
                        self.val = HostValue::Unit;
                        self.unwind();
                        return Ok(());
                    }
                };
                let recur = Control::Loop {
                    kind: LoopKind::For { pat: pat.clone() },
                    body: Box::new(body.clone()),
                };
                self.continues.push(recur);
                if self.bind_pattern(&pat, &item)? {
                    self.control = Control::Exec(body);
                } else {
                    self.unwind();
                }
            }
        }
        Ok(())
    }

    fn step_apply(&mut self) -> Result<(), TrapReport> {
        let callee = std::mem::replace(&mut self.proc, HostValue::Unit);
        let args = std::mem::take(&mut self.argl);
        match callee {
            HostValue::FnPtr(fun) => {
                let def = self.engine.sema.funs[fun.0 as usize].clone();
                let body = def.body.clone();
                self.enter_function(fun, args, &body)?;
            }
            HostValue::Closure(closure) => {
                let body = closure.body.clone();
                let captures = closure.captures.clone();
                let params = closure.params.clone();
                let frame =
                    self.engine
                        .push_activation(closure.bind_base, closure.frame_slots, captures);
                for ((binding, _), value) in params.iter().zip(args) {
                    self.engine
                        .write_local(*binding, value)
                        .map_err(Self::trap)?;
                }
                let caller_frame = self.frame;
                self.frame = frame;
                self.continues.push(Control::FunEnd { caller_frame });
                self.control = Control::Exec((*body).clone());
            }
            HostValue::Box(inner) => {
                self.proc = *inner;
                self.argl = args;
                self.control = Control::Apply;
            }
            _ => return Err(Self::trap(Trap::Dangling)),
        }
        Ok(())
    }

    fn bind_pattern(&mut self, pat: &HirPat, value: &HostValue) -> Result<bool, TrapReport> {
        let bound = ops::bind_pattern(&self.engine, pat, value).map_err(Self::trap)?;
        let Some(bound) = bound else {
            return Ok(false);
        };
        for (binding, bound_value) in bound {
            self.engine
                .write_local(binding, bound_value)
                .map_err(Self::trap)?;
        }
        Ok(true)
    }

    fn step_resolve_place(
        &mut self,
        place: sicp_runtime::host::hir::Place,
        then: PlaceCont,
    ) -> Result<(), TrapReport> {
        match place.root {
            PlaceRoot::Local(bind) => {
                let (addr, projs) = self.engine.local_place(bind).map_err(Self::trap)?;
                // A projection or method through a borrowed slot addresses
                // the referent, like the native autoref adjustment; a bare
                // read, store, or borrow keeps the slot itself.
                let see_through =
                    !place.proj.is_empty() || matches!(then, PlaceCont::Method { .. });
                let (addr, projs) = match self.engine.read_at(addr, &projs) {
                    Ok(HostValue::Ref {
                        addr: base,
                        projs: mut base_projs,
                        ..
                    }) if see_through => {
                        base_projs.extend(projs);
                        (base, base_projs)
                    }
                    Err(trap) if see_through => return Err(Self::trap(trap)),
                    _ => (addr, projs),
                };
                self.fold_place_projs(addr, projs, place.proj.into_iter(), then)
            }
            PlaceRoot::Deref(inner) => {
                self.continues.push(Control::PlaceDeref {
                    proj: place.proj,
                    then,
                });
                self.control = Control::Eval(*inner);
                Ok(())
            }
        }
    }

    fn fold_place_projs(
        &mut self,
        addr: Addr,
        mut projs: Vec<RtProj>,
        mut rest: std::vec::IntoIter<sicp_runtime::host::hir::Proj>,
        then: PlaceCont,
    ) -> Result<(), TrapReport> {
        loop {
            match rest.next() {
                None => return self.finish_place(addr, projs, then),
                Some(Proj::Field(index)) => projs.push(RtProj::Field(index)),
                Some(Proj::BoxDeref) => projs.push(RtProj::BoxDeref),
                Some(Proj::Index(expr)) => {
                    self.continues.push(Control::PlaceIndex {
                        addr,
                        projs,
                        rest: rest.collect(),
                        then,
                    });
                    self.control = Control::Eval(*expr);
                    return Ok(());
                }
            }
        }
    }

    fn finish_place(
        &mut self,
        addr: Addr,
        projs: Vec<RtProj>,
        then: PlaceCont,
    ) -> Result<(), TrapReport> {
        match then {
            PlaceCont::Read { mode } => {
                let moved = mode == sicp_runtime::host::hir::PlaceUse::Move;
                self.val = if moved {
                    self.engine.take_at(addr, &projs)
                } else {
                    self.engine.read_at(addr, &projs)
                }
                .map_err(Self::trap)?;
                self.unwind();
            }
            PlaceCont::Assign { op, value } => {
                self.continues.push(Control::Assign { addr, projs, op });
                self.control = Control::Eval(*value);
            }
            PlaceCont::Method { op, receiver, args } => {
                let place = Some((addr, projs));
                self.continues.push(Control::Args {
                    pending: args.into(),
                    done: Vec::new(),
                    after: Resume::Method { op, place },
                });
                self.control = Control::Eval(*receiver);
            }
            PlaceCont::Borrow { mutable } => {
                self.val = HostValue::Ref {
                    addr,
                    projs,
                    mutable,
                };
                self.unwind();
            }
        }
        Ok(())
    }
    /// Reads through a borrowed field or index base, like the native
    /// autoref adjustment; other values pass through unchanged.
    fn through_ref(&self, value: HostValue) -> Result<HostValue, TrapReport> {
        if matches!(value, HostValue::Ref { .. }) {
            return self.engine.deref_value(&value).map_err(Self::trap);
        }
        Ok(value)
    }

    fn trap(trap: Trap) -> TrapReport {
        TrapReport {
            trap,
            span: Span::default(),
        }
    }
}

fn index_value(value: &HostValue, at: i64) -> Result<HostValue, Trap> {
    let (HostValue::Vec(items) | HostValue::Array(items)) = value else {
        return Err(Trap::Dangling);
    };
    let at = usize::try_from(at).map_err(|_| Trap::IndexOutOfBounds)?;
    items.get(at).cloned().ok_or(Trap::IndexOutOfBounds)
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

/// The explicit-control entry point every conformance gate names.
///
/// # Errors
/// The admission [`Diag`] when the source is rejected before any
/// effect.
pub fn run_session(source: &str) -> Result<RunOutcome, Diag> {
    let program = sicp_runtime::host::admit(source)?;
    Ok(Eceval::run(&program))
}

/// The flow vocabulary shared with the other engines.
pub type MachineFlow = Flow;
