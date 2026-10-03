// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 5.5
//!
//! Section 5.5: the compiler. Checked syntax compiles to typed
//! instruction sequences (grammar §9): labels, assignments, branches,
//! saves, restores, performs, and the two purpose-built typed
//! instructions (`Bind` for pattern dispatch, `MakeClosure` for closure
//! creation), with explicit register liveness in each sequence and the
//! `preserving` discipline that protects live registers across nested
//! compiled calls. `compiled_run` executes the sequences on a real
//! instruction VM over guest values; `emit_c` renders the same
//! sequences as a C translation unit over the `mc_*` word runtime
//! (exercise 5.52), linked against the backend with no duplicate main.

use std::collections::{BTreeSet, HashMap};
use std::fmt::Write as _;

use sicp_runtime::host::check::CheckedProgram;
use sicp_runtime::host::diag::Diag;
use sicp_runtime::host::hir::{
    BinOp, ClosureKind, CtorOp, FormatKind, FormatSpec, FunId, HirBlock, HirExpr, HirExprKind,
    HirPat, MethodOp, PlaceRoot, Proj, Sema, UnOp,
};
use sicp_runtime::host::ops::{self, RunOutcome, TrapReport};
use sicp_runtime::host::value::{HostValue, RtProj, Trap};

/// One compiled instruction over the teaching machine's registers.
#[derive(Debug, Clone, PartialEq)]
pub enum Instr {
    /// A label definition.
    Label(String),
    /// `assign REG <- OPERAND`.
    Assign(String, Operand),
    /// `test` one binary relation over two operands, setting `flag`.
    Test(BinOp, Operand, Operand),
    /// `test` one operand for truth, setting `flag`.
    TestBool(Operand),
    /// `branch LABEL` when `flag` is set.
    Branch(String),
    /// `goto OPERAND`.
    Goto(Operand),
    /// `save REG`.
    Save(String),
    /// `restore REG`.
    Restore(String),
    /// Drop the top N stack entries: a `break` or `continue` leaving
    /// the entries its loop body pushed.
    Discard(usize),
    /// Bind the value register against one pattern, falling through on
    /// success and jumping to `fail` otherwise.
    Bind {
        /// The pattern to bind.
        pat: HirPat,
        /// The label to jump to when the pattern does not match.
        fail: String,
    },
    /// Build one closure value with the checked capture discipline.
    MakeClosure {
        /// The closure's kind.
        kind: ClosureKind,
        /// The closure's body label.
        body: String,
        /// The parameter slots.
        params: Vec<u32>,
        /// The capture records: binding slot and capture mode index.
        captures: Vec<(u32, u32)>,
        /// The closure activation's first binding slot.
        bind_base: u32,
        /// The closure activation's local slots.
        frame_slots: u32,
    },
    /// `perform` one typed operation over operands.
    Perform(PerformOp, Vec<Operand>),
}

/// One compiled operand.
#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    /// A constant integer.
    Const(i64),
    /// A constant `usize`.
    ConstU(u64),
    /// A constant `bool`.
    Bool(bool),
    /// A constant string slice.
    Str(String),
    /// A register read.
    Reg(String),
    /// A label address.
    Label(String),
    /// A typed operation whose result the assignment stores: the
    /// compiled form of the repaired machine contract (grammar §7
    /// repair note). `Perform` stays effect-only.
    Op(PerformOp, Vec<Operand>),
}

/// The typed operations `Perform` runs (grammar §9 typed operation
/// values, closed over the shared leaf semantics).
#[derive(Debug, Clone, PartialEq)]
pub enum PerformOp {
    /// Call the compiled function at the labeled entry.
    CallFun,
    /// Call the function or closure value in `val` (args in `argl`).
    CallValue,
    /// Return from the compiled call.
    Return,
    /// An admitted constructor.
    Ctor(CtorOp),
    /// An admitted method (receiver value in `val`, args in `argl`).
    Method(MethodOp),
    /// An admitted method on a place (a reference to the receiver's
    /// place in `val`, args in `argl`): the receiver's update is
    /// written back to the place.
    MethodAt(MethodOp),
    /// The function value of one top-level function.
    FunRef(u32),
    /// A reference to one local binding's place (its own slot, or the
    /// place its capture record names); `through` addresses the
    /// referent when the slot holds a reference.
    LocalRef {
        /// The binding slot.
        bind: u32,
        /// Whether the reference is exclusive.
        mutable: bool,
        /// Whether a borrow held in the slot is seen through.
        through: bool,
    },
    /// Extend the reference in `val` by one field projection.
    RefField(u32),
    /// Extend the reference in `val` by the index in `tmp`.
    RefIndex,
    /// Read the place the reference in `val` names.
    Load,
    /// Move out of the place the reference in `val` names.
    Take,
    /// Store `val` (or, for a compound assignment, the place's value
    /// combined with `val`) into the place the reference in `tmp`
    /// names.
    Store(Option<BinOp>),
    /// One arithmetic/comparison operator.
    Arith(BinOp),
    /// One unary operator.
    Unary(UnOp),
    /// A format call (args in `argl`).
    Format(FormatKind, FormatSpec),
    /// Build a tuple from `val` and `tmp`.
    MakeTuple,
    /// Build a vector from `argl`.
    MakeVec,
    /// Build a struct or tuple-struct from `argl`.
    MakeStruct(u32),
    /// Build an enum variant from `argl`.
    MakeVariant(u32, u32),
    /// Project one field of `val`.
    Project(u32),
    /// Index `val` by `tmp`.
    IndexGet,
    /// Build `val` repeated `tmp` times into one vector.
    MakeRepeat,
    /// Build the half-open range `val..tmp`.
    MakeRange,
    /// Advance the iterator in `val`: the item goes to `item` and
    /// `flag` tells whether there was one.
    IterNext,
    /// Turn the iterable in `val` into its iterator (a range or
    /// iterator as is, a borrowed collection by reference, an owned one
    /// by value).
    IterStart,
    /// No `match` arm matched (the checker's exhaustiveness makes this
    /// unreachable).
    NoMatch,
    /// The postfix `?` control transfer.
    Try,
}

/// One instruction sequence with its register liveness (grammar §5.5).
#[derive(Debug, Clone, PartialEq)]
pub struct Seq {
    /// The registers the sequence reads.
    pub needs: BTreeSet<String>,
    /// The registers the sequence writes.
    pub modifies: BTreeSet<String>,
    /// The instructions, in order.
    pub stmts: Vec<Instr>,
}

impl Seq {
    /// Builds one sequence from its parts.
    #[must_use]
    pub fn new(needs: &[&str], modifies: &[&str], stmts: Vec<Instr>) -> Self {
        Self {
            needs: needs.iter().map(|name| (*name).to_owned()).collect(),
            modifies: modifies.iter().map(|name| (*name).to_owned()).collect(),
            stmts,
        }
    }

    /// The empty sequence.
    #[must_use]
    pub fn empty() -> Self {
        Self::new(&[], &[], Vec::new())
    }

    /// Concatenates two sequences in order.
    #[must_use]
    pub fn append(mut self, other: Self) -> Self {
        self.needs.extend(other.needs);
        self.modifies.extend(other.modifies);
        self.stmts.extend(other.stmts);
        self
    }

    /// Runs `body` while `registers` are saved and restored: the
    /// `preserving` discipline of 5.5.1 that makes nested calls safe.
    #[must_use]
    pub fn preserving(self, registers: &[&str], body: Self) -> Self {
        // Save every register the body modifies and the first
        // sequence needs, run `self`, then `body`, then restore: the
        // first sequence is always present.
        let mut saves = Self::empty();
        let mut restores = Self::empty();
        for register in registers {
            let name = (*register).to_owned();
            if self.needs.contains(&name) && body.modifies.contains(&name) {
                saves = saves.append(Seq::new(&[&name], &[], vec![Instr::Save(name.clone())]));
                restores = restores.append(Seq::new(&[], &[], vec![Instr::Restore(name)]));
            }
        }
        saves.append(self).append(body).append(restores)
    }
}

/// The label supply one compilation draws on.
#[derive(Debug, Default)]
pub struct Labels {
    next: usize,
}

impl Labels {
    /// Builds one label supply.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Draws one fresh label with the given prefix.
    #[must_use]
    pub fn fresh(&mut self, prefix: &str) -> String {
        let label = format!("{prefix}#{}", self.next);
        self.next += 1;
        label
    }
}

/// Compiles one checked program to one flat instruction stream: every
/// function and every closure body is pre-compiled and labeled, so the
/// VM executes instructions only and never sees syntax.
#[must_use]
pub fn compile_program(program: &CheckedProgram) -> CompiledProgram {
    let mut labels = Labels::new();
    let mut closures: Vec<(String, Seq)> = Vec::new();
    let mut instrs: Vec<Instr> = Vec::new();
    for def in &program.sema.funs {
        let seq = compile_block(&def.body, &mut labels, &mut closures);
        instrs.push(Instr::Label(format!("fun{}", def.id.0)));
        instrs.extend(seq.stmts);
        instrs.push(Instr::Perform(PerformOp::Return, Vec::new()));
    }
    for (label, seq) in closures {
        instrs.push(Instr::Label(label));
        instrs.extend(seq.stmts);
        instrs.push(Instr::Perform(PerformOp::Return, Vec::new()));
    }
    CompiledProgram {
        sema: program.sema.clone(),
        instrs,
        entry: format!("fun{}", program.sema.main.0),
        main: program.sema.main,
    }
}

/// One compiled program: one flat, labeled instruction stream.
#[derive(Debug, Clone)]
pub struct CompiledProgram {
    /// The typed program the sequences came from.
    pub sema: Sema,
    /// Every function and closure body, pre-compiled in one stream.
    pub instrs: Vec<Instr>,
    /// The entry label.
    pub entry: String,
    /// The entry point.
    pub main: FunId,
}

/// The compile-time context one body compiles under: the label supply,
/// the pre-compiled closure bodies, the enclosing loops' exit and
/// continue targets, and how many stack entries the body has pushed
/// above its activation's base at the current point. A `break` or
/// `continue` discards exactly the entries pushed since its loop's
/// target point, so an early exit leaves the stack balanced.
struct Ctx<'a> {
    labels: &'a mut Labels,
    closures: &'a mut Vec<(String, Seq)>,
    loops: Vec<LoopTarget>,
    depth: usize,
}

/// One enclosing loop's control targets.
struct LoopTarget {
    /// Where `break` jumps; the loop's value is in `val` there.
    exit: String,
    /// The stack depth live at `exit`.
    exit_depth: usize,
    /// Where `continue` jumps.
    next: String,
    /// The stack depth live at `next`.
    next_depth: usize,
}

impl<'a> Ctx<'a> {
    fn new(labels: &'a mut Labels, closures: &'a mut Vec<(String, Seq)>) -> Self {
        Self {
            labels,
            closures,
            loops: Vec::new(),
            depth: 0,
        }
    }

    /// Compiles `body` with `val` saved around it: `val` holds what it
    /// held before (restored), and `tmp` holds the body's result.
    fn around_val(&mut self, body: impl FnOnce(&mut Self) -> Seq) -> Seq {
        self.depth += 1;
        let compiled = body(self);
        self.depth -= 1;
        seq(vec![Instr::Save(VAL.to_owned())])
            .append(compiled)
            .append(seq(vec![
                Instr::Assign(TMP.to_owned(), reg(VAL)),
                Instr::Restore(VAL.to_owned()),
            ]))
    }

    /// Compiles one loop body with its control targets installed.
    fn in_loop(&mut self, target: LoopTarget, body: &HirBlock) -> Seq {
        self.loops.push(target);
        let compiled = compile_block_in(self, body);
        self.loops.pop();
        compiled
    }
}

const VAL: &str = "val";
const TMP: &str = "tmp";
const ARGL: &str = "argl";
const ITEM: &str = "item";

fn reg(name: &str) -> Operand {
    Operand::Reg(name.to_owned())
}

/// Builds one sequence from its instructions, deriving the registers
/// it reads and writes.
fn seq(stmts: Vec<Instr>) -> Seq {
    let mut needs = BTreeSet::new();
    let mut modifies = BTreeSet::new();
    let read = |operand: &Operand, needs: &mut BTreeSet<String>| {
        if let Operand::Reg(name) = operand {
            needs.insert(name.clone());
        }
    };
    for stmt in &stmts {
        match stmt {
            Instr::Assign(target, operand) => {
                read(operand, &mut needs);
                modifies.insert(target.clone());
            }
            Instr::Test(_, left, right) => {
                read(left, &mut needs);
                read(right, &mut needs);
                modifies.insert("flag".to_owned());
            }
            Instr::TestBool(operand) => {
                read(operand, &mut needs);
                modifies.insert("flag".to_owned());
            }
            Instr::Branch(_) => {
                needs.insert("flag".to_owned());
            }
            Instr::Goto(operand) => read(operand, &mut needs),
            Instr::Save(name) => {
                needs.insert(name.clone());
            }
            Instr::Restore(name) => {
                modifies.insert(name.clone());
            }
            Instr::Bind { .. } => {
                needs.insert(VAL.to_owned());
            }
            Instr::MakeClosure { .. } => {
                modifies.insert(VAL.to_owned());
            }
            Instr::Perform(_, operands) => {
                for operand in operands {
                    read(operand, &mut needs);
                }
                needs.insert(VAL.to_owned());
                modifies.insert(VAL.to_owned());
            }
            Instr::Label(_) | Instr::Discard(_) => {}
        }
    }
    Seq {
        needs,
        modifies,
        stmts,
    }
}

fn perform(op: PerformOp) -> Seq {
    seq(vec![Instr::Perform(op, Vec::new())])
}

fn set_val(operand: Operand) -> Seq {
    seq(vec![Instr::Assign(VAL.to_owned(), operand)])
}

fn label(name: &str) -> Seq {
    seq(vec![Instr::Label(name.to_owned())])
}

fn goto(name: &str) -> Seq {
    seq(vec![Instr::Goto(Operand::Label(name.to_owned()))])
}

/// Compiles one block into one sequence producing its tail in `val`.
#[must_use]
pub fn compile_block(
    block: &HirBlock,
    labels: &mut Labels,
    closures: &mut Vec<(String, Seq)>,
) -> Seq {
    compile_block_in(&mut Ctx::new(labels, closures), block)
}

/// Compiles one expression into one sequence producing its value in
/// `val`. Every admitted form lowers here; nothing falls through.
#[must_use]
pub fn compile_expr(expr: &HirExpr, labels: &mut Labels, closures: &mut Vec<(String, Seq)>) -> Seq {
    compile_expr_in(&mut Ctx::new(labels, closures), expr)
}

fn compile_block_in(ctx: &mut Ctx<'_>, block: &HirBlock) -> Seq {
    let mut out = Seq::empty();
    for stmt in &block.stmts {
        match stmt {
            sicp_runtime::host::hir::HirStmt::Let {
                binding,
                destruct,
                value,
            } => {
                out = out.append(compile_expr_in(ctx, value));
                out = out.append(match destruct {
                    Some((left, right)) => seq(vec![
                        Instr::Assign(TMP.to_owned(), reg(VAL)),
                        Instr::Perform(PerformOp::Project(0), Vec::new()),
                        Instr::Assign(slot_name(*left), reg(VAL)),
                        Instr::Assign(VAL.to_owned(), reg(TMP)),
                        Instr::Perform(PerformOp::Project(1), Vec::new()),
                        Instr::Assign(slot_name(*right), reg(VAL)),
                    ]),
                    None => seq(vec![Instr::Assign(slot_name(*binding), reg(VAL))]),
                });
            }
            sicp_runtime::host::hir::HirStmt::Expr(expr) => {
                out = out.append(compile_expr_in(ctx, expr));
            }
        }
    }
    match &block.tail {
        Some(tail) => out.append(compile_expr_in(ctx, tail)),
        None => out.append(set_val(Operand::Const(0))),
    }
}

/// Compiles the operands of a call, constructor, or collection into
/// `argl` (one vector, in source order); `val` is not preserved.
fn compile_operands(ctx: &mut Ctx<'_>, args: &[HirExpr]) -> Seq {
    let mut out = Seq::empty();
    let base = ctx.depth;
    for arg in args {
        out = out
            .append(compile_expr_in(ctx, arg))
            .append(seq(vec![Instr::Save(VAL.to_owned())]));
        ctx.depth += 1;
    }
    ctx.depth = base;
    let count = i64::try_from(args.len()).expect("operand lists are small");
    out.append(seq(vec![
        Instr::Perform(PerformOp::MakeVec, vec![Operand::Const(count)]),
        Instr::Assign(ARGL.to_owned(), reg(VAL)),
    ]))
}

/// Compiles one place to its address: `val` holds a reference to the
/// place. `through` sees through a borrow held in a local root (a
/// projection or method addresses the referent, like the native autoref
/// adjustment); a bare read, store, or borrow keeps the slot itself.
fn compile_address(
    ctx: &mut Ctx<'_>,
    place: &sicp_runtime::host::hir::Place,
    through: bool,
    mutable: bool,
) -> Seq {
    let mut out = match &place.root {
        PlaceRoot::Local(bind) => perform(PerformOp::LocalRef {
            bind: bind.0,
            mutable,
            through,
        }),
        PlaceRoot::Deref(inner) => compile_expr_in(ctx, inner),
    };
    for step in &place.proj {
        out = out.append(match step {
            Proj::Field(index) => perform(PerformOp::RefField(*index)),
            Proj::Index(index) => ctx
                .around_val(|ctx| compile_expr_in(ctx, index))
                .append(perform(PerformOp::RefIndex)),
        });
    }
    out
}

/// Compiles `left` into `val` and `right` into `tmp`, in that order.
fn compile_pair(ctx: &mut Ctx<'_>, left: &HirExpr, right: &HirExpr) -> Seq {
    compile_expr_in(ctx, left).append(ctx.around_val(|ctx| compile_expr_in(ctx, right)))
}

// Preserve one exhaustive dispatch from checked HIR forms to instruction sequences.
#[allow(clippy::too_many_lines)]
fn compile_expr_in(ctx: &mut Ctx<'_>, expr: &HirExpr) -> Seq {
    match &expr.kind {
        HirExprKind::I64(value) => set_val(Operand::Const(*value)),
        HirExprKind::Usize(value) => set_val(Operand::ConstU(*value)),
        HirExprKind::Bool(value) => set_val(Operand::Bool(*value)),
        HirExprKind::Unit => set_val(Operand::Const(0)),
        HirExprKind::Str(text) => set_val(Operand::Str(text.clone())),
        HirExprKind::Place { place, mode } => {
            let through = !place.proj.is_empty();
            let load = if *mode == sicp_runtime::host::hir::PlaceUse::Move {
                PerformOp::Take
            } else {
                PerformOp::Load
            };
            compile_address(ctx, place, through, false).append(perform(load))
        }
        HirExprKind::FunRef(fun) => perform(PerformOp::FunRef(fun.0)),
        HirExprKind::StructLit(id, fields) | HirExprKind::TupleStructLit(id, fields) => {
            compile_operands(ctx, fields).append(perform(PerformOp::MakeStruct(id.0)))
        }
        HirExprKind::VariantLit(id, index, payload) => {
            compile_operands(ctx, payload).append(perform(PerformOp::MakeVariant(id.0, *index)))
        }
        HirExprKind::Tuple(left, right) => {
            compile_pair(ctx, left, right).append(perform(PerformOp::MakeTuple))
        }
        HirExprKind::Array(items) | HirExprKind::VecList(items) => {
            compile_operands(ctx, items).append(set_val(reg(ARGL)))
        }
        HirExprKind::VecRepeat(value, count) => {
            compile_pair(ctx, value, count).append(perform(PerformOp::MakeRepeat))
        }
        HirExprKind::Format { kind, spec, args } => {
            compile_operands(ctx, args).append(perform(PerformOp::Format(*kind, spec.clone())))
        }
        HirExprKind::Field { base, index } => {
            compile_expr_in(ctx, base).append(perform(PerformOp::Project(*index)))
        }
        HirExprKind::Index { base, index } => {
            compile_pair(ctx, base, index).append(perform(PerformOp::IndexGet))
        }
        HirExprKind::Call { callee, args } => {
            compile_operands(ctx, args).append(seq(vec![Instr::Perform(
                PerformOp::CallFun,
                vec![Operand::Label(format!("fun{}", callee.0))],
            )]))
        }
        HirExprKind::Ctor(op, args) => {
            compile_operands(ctx, args).append(perform(PerformOp::Ctor(*op)))
        }
        HirExprKind::IndirectCall { callee, args } => {
            let callee_seq = compile_expr_in(ctx, callee);
            ctx.depth += 1;
            let operands = compile_operands(ctx, args);
            ctx.depth -= 1;
            callee_seq
                .append(seq(vec![Instr::Save(VAL.to_owned())]))
                .append(operands)
                .append(seq(vec![Instr::Restore(VAL.to_owned())]))
                .append(perform(PerformOp::CallValue))
        }
        HirExprKind::Method {
            op,
            receiver,
            receiver_place,
            args,
        } => {
            // The receiver is evaluated first, as in Rust: its place
            // when the method reads or updates it in place, its value
            // otherwise.
            let (receiver_seq, apply) = match receiver_place {
                Some(place) => (
                    compile_address(ctx, place, true, true),
                    PerformOp::MethodAt(*op),
                ),
                None => (compile_expr_in(ctx, receiver), PerformOp::Method(*op)),
            };
            ctx.depth += 1;
            let operands = compile_operands(ctx, args);
            ctx.depth -= 1;
            receiver_seq
                .append(seq(vec![Instr::Save(VAL.to_owned())]))
                .append(operands)
                .append(seq(vec![Instr::Restore(VAL.to_owned())]))
                .append(perform(apply))
        }
        HirExprKind::Unary { op, operand } => match (op, &operand.kind) {
            (UnOp::Ref | UnOp::RefMut, HirExprKind::Place { place, .. }) => {
                compile_address(ctx, place, !place.proj.is_empty(), *op == UnOp::RefMut)
            }
            _ => compile_expr_in(ctx, operand).append(perform(PerformOp::Unary(*op))),
        },
        HirExprKind::Binary { op, left, right } => match op {
            BinOp::And | BinOp::Or => {
                // Short-circuit: the right operand runs only when the
                // left one does not decide the result.
                let done = ctx.labels.fresh("logic-done");
                let rhs = ctx.labels.fresh("logic-rhs");
                let left_seq = compile_expr_in(ctx, left);
                let right_seq = compile_expr_in(ctx, right);
                let test = if *op == BinOp::And {
                    seq(vec![
                        Instr::TestBool(reg(VAL)),
                        Instr::Branch(rhs.clone()),
                        Instr::Goto(Operand::Label(done.clone())),
                    ])
                } else {
                    seq(vec![
                        Instr::TestBool(reg(VAL)),
                        Instr::Branch(done.clone()),
                        Instr::Goto(Operand::Label(rhs.clone())),
                    ])
                };
                left_seq
                    .append(test)
                    .append(label(&rhs))
                    .append(right_seq)
                    .append(label(&done))
            }
            _ => compile_pair(ctx, left, right).append(perform(PerformOp::Arith(*op))),
        },
        HirExprKind::Assign { op, target, value } => {
            // The assigned value is evaluated before the place, as in
            // Rust; the place's address lands in `tmp`.
            let value_seq = compile_expr_in(ctx, value);
            let address =
                ctx.around_val(|ctx| compile_address(ctx, target, !target.proj.is_empty(), true));
            value_seq
                .append(address)
                .append(perform(PerformOp::Store(*op)))
        }
        HirExprKind::If {
            test,
            then,
            else_branch,
        } => {
            let then_label = ctx.labels.fresh("then");
            let end = ctx.labels.fresh("endif");
            let test_seq = compile_expr_in(ctx, test);
            let then_seq = compile_expr_in(ctx, then);
            let else_seq = compile_expr_in(ctx, else_branch);
            test_seq
                .append(seq(vec![
                    Instr::TestBool(reg(VAL)),
                    Instr::Branch(then_label.clone()),
                ]))
                .append(else_seq)
                .append(goto(&end))
                .append(label(&then_label))
                .append(then_seq)
                .append(label(&end))
        }
        HirExprKind::IfLet {
            pat,
            value,
            then,
            else_branch,
        } => {
            let otherwise = ctx.labels.fresh("iflet-else");
            let end = ctx.labels.fresh("iflet-end");
            let value_seq = compile_expr_in(ctx, value);
            let then_seq = compile_expr_in(ctx, then);
            let else_seq = compile_expr_in(ctx, else_branch);
            value_seq
                .append(seq(vec![Instr::Bind {
                    pat: pat.clone(),
                    fail: otherwise.clone(),
                }]))
                .append(then_seq)
                .append(goto(&end))
                .append(label(&otherwise))
                .append(else_seq)
                .append(label(&end))
        }
        HirExprKind::Match { scrutinee, arms } => {
            // A failed `Bind` leaves `val` intact, so every arm tests
            // the same scrutinee value.
            let end = ctx.labels.fresh("match-end");
            let mut out = compile_expr_in(ctx, scrutinee);
            for (pat, body) in arms {
                let next = ctx.labels.fresh("match-next");
                out = out
                    .append(seq(vec![Instr::Bind {
                        pat: pat.clone(),
                        fail: next.clone(),
                    }]))
                    .append(compile_expr_in(ctx, body))
                    .append(goto(&end))
                    .append(label(&next));
            }
            out.append(perform(PerformOp::NoMatch)).append(label(&end))
        }
        HirExprKind::Block(block) => compile_block_in(ctx, block),
        HirExprKind::Loop { body, .. } => {
            let top = ctx.labels.fresh("loop");
            let done = ctx.labels.fresh("loop-done");
            let target = LoopTarget {
                exit: done.clone(),
                exit_depth: ctx.depth,
                next: top.clone(),
                next_depth: ctx.depth,
            };
            let body_seq = ctx.in_loop(target, body);
            label(&top)
                .append(body_seq)
                .append(goto(&top))
                .append(label(&done))
        }
        HirExprKind::While { test, body } => {
            let top = ctx.labels.fresh("while");
            let run = ctx.labels.fresh("while-body");
            let done = ctx.labels.fresh("while-done");
            let test_seq = compile_expr_in(ctx, test);
            let target = LoopTarget {
                exit: done.clone(),
                exit_depth: ctx.depth,
                next: top.clone(),
                next_depth: ctx.depth,
            };
            let body_seq = ctx.in_loop(target, body);
            label(&top)
                .append(test_seq)
                .append(seq(vec![
                    Instr::TestBool(reg(VAL)),
                    Instr::Branch(run.clone()),
                    Instr::Goto(Operand::Label(done.clone())),
                ]))
                .append(label(&run))
                .append(body_seq)
                .append(goto(&top))
                .append(label(&done))
                .append(set_val(Operand::Const(0)))
        }
        HirExprKind::WhileLet { pat, value, body } => {
            let top = ctx.labels.fresh("whilelet");
            let done = ctx.labels.fresh("whilelet-done");
            let value_seq = compile_expr_in(ctx, value);
            let target = LoopTarget {
                exit: done.clone(),
                exit_depth: ctx.depth,
                next: top.clone(),
                next_depth: ctx.depth,
            };
            let body_seq = ctx.in_loop(target, body);
            label(&top)
                .append(value_seq)
                .append(seq(vec![Instr::Bind {
                    pat: pat.clone(),
                    fail: done.clone(),
                }]))
                .append(body_seq)
                .append(goto(&top))
                .append(label(&done))
                .append(set_val(Operand::Const(0)))
        }
        HirExprKind::For {
            pat,
            iterable,
            body,
        } => {
            // The iterator lives on the stack for the whole loop: each
            // step restores it, advances it, and saves it back.
            let top = ctx.labels.fresh("for");
            let run = ctx.labels.fresh("for-body");
            let done = ctx.labels.fresh("for-done");
            let iterable_seq = compile_expr_in(ctx, iterable);
            let outer = ctx.depth;
            ctx.depth += 1;
            let target = LoopTarget {
                exit: done.clone(),
                exit_depth: outer,
                next: top.clone(),
                next_depth: ctx.depth,
            };
            let body_seq = ctx.in_loop(target, body);
            ctx.depth = outer;
            let finish = ctx.labels.fresh("for-finish");
            iterable_seq
                .append(perform(PerformOp::IterStart))
                .append(seq(vec![Instr::Save(VAL.to_owned())]))
                .append(label(&top))
                .append(seq(vec![
                    Instr::Restore(VAL.to_owned()),
                    Instr::Perform(PerformOp::IterNext, Vec::new()),
                    Instr::Save(VAL.to_owned()),
                    Instr::Branch(run.clone()),
                    Instr::Goto(Operand::Label(finish.clone())),
                ]))
                .append(label(&run))
                .append(seq(vec![
                    Instr::Assign(VAL.to_owned(), reg(ITEM)),
                    Instr::Bind {
                        pat: pat.clone(),
                        fail: top.clone(),
                    },
                ]))
                .append(body_seq)
                .append(goto(&top))
                .append(label(&finish))
                .append(seq(vec![Instr::Discard(1)]))
                .append(label(&done))
                .append(set_val(Operand::Const(0)))
        }
        HirExprKind::Closure(closure) => {
            let body_label = ctx.labels.fresh("closure-body");
            let params: Vec<u32> = closure.params.iter().map(|(bind, _)| bind.0).collect();
            let captures: Vec<(u32, u32)> = closure
                .captures
                .iter()
                .map(|capture| (capture.binding.0, capture.mode as u32))
                .collect();
            // The body is pre-compiled into the flat stream under its
            // own activation: no enclosing loop or stack entry reaches
            // into it.
            let body_seq = compile_block_in(
                &mut Ctx::new(&mut *ctx.labels, &mut *ctx.closures),
                &closure.body,
            );
            ctx.closures.push((body_label.clone(), body_seq));
            seq(vec![Instr::MakeClosure {
                kind: closure.kind,
                body: body_label,
                params,
                captures,
                bind_base: closure.bind_base,
                frame_slots: closure.frame_slots,
            }])
        }
        HirExprKind::Return(value) => {
            let value_seq = match value {
                Some(value) => compile_expr_in(ctx, value),
                None => set_val(Operand::Const(0)),
            };
            value_seq.append(perform(PerformOp::Return))
        }
        HirExprKind::Break(value) => {
            let value_seq = match value {
                Some(value) => compile_expr_in(ctx, value),
                None => set_val(Operand::Const(0)),
            };
            let target = ctx
                .loops
                .last()
                .expect("the checker admits `break` only inside a loop");
            value_seq.append(seq(vec![
                Instr::Discard(ctx.depth - target.exit_depth),
                Instr::Goto(Operand::Label(target.exit.clone())),
            ]))
        }
        HirExprKind::Continue => {
            let target = ctx
                .loops
                .last()
                .expect("the checker admits `continue` only inside a loop");
            seq(vec![
                Instr::Discard(ctx.depth - target.next_depth),
                Instr::Goto(Operand::Label(target.next.clone())),
            ])
        }
        HirExprKind::Try(inner) => compile_expr_in(ctx, inner).append(perform(PerformOp::Try)),
        HirExprKind::Range(left, right) => {
            compile_pair(ctx, left, right).append(perform(PerformOp::MakeRange))
        }
    }
}

fn slot_name(binding: sicp_runtime::host::hir::BindId) -> String {
    format!("slot{}", binding.0)
}

/// The compiled-code runner: one instruction VM over guest values.
#[must_use]
pub fn compiled_run(program: &CheckedProgram) -> RunOutcome {
    let compiled = compile_program(program);
    let mut vm = Vm::new(compiled);
    match vm.run() {
        Ok(()) => RunOutcome {
            stdout: vm.engine.effects.stdout,
            trap: None,
        },
        Err(report) => RunOutcome {
            stdout: vm.engine.effects.stdout,
            trap: Some(report),
        },
    }
}

/// Runs one compiled program and reports the counted statistics the
/// compiled-versus-interpreted stack lessons (exercises 5.27 through
/// 5.29 and 5.45, 5.46, 5.50) compare: save/restore totals, the
/// control-stack depth over the save stack and the call frames, and
/// the executed-instruction count.
#[must_use]
pub fn compiled_run_counted(program: &CheckedProgram) -> (RunOutcome, crate::sec_5_2::StackStats) {
    let compiled = compile_program(program);
    let mut vm = Vm::new(compiled);
    let outcome = match vm.run() {
        Ok(()) => RunOutcome {
            stdout: vm.engine.effects.stdout,
            trap: None,
        },
        Err(report) => RunOutcome {
            stdout: vm.engine.effects.stdout,
            trap: Some(report),
        },
    };
    (outcome, vm.stats)
}

/// The compiled entry point the conformance gates name.
///
/// # Errors
/// The admission [`Diag`] when the source is rejected before any
/// effect.
pub fn run_compiled(source: &str) -> Result<RunOutcome, Diag> {
    let program = sicp_runtime::host::admit(source)?;
    Ok(compiled_run(&program))
}

/// One compiled call in progress: where to resume and the stack height
/// the callee started at, which `return` (and `?`) restores however
/// many entries the callee's early exit left pushed.
struct CallRecord {
    return_pc: usize,
    stack_base: usize,
}

struct Vm {
    program: CompiledProgram,
    engine: ops::Engine,
    regs: HashMap<String, HostValue>,
    stack: Vec<(String, HostValue)>,
    calls: Vec<CallRecord>,
    labels: HashMap<String, usize>,
    pc: usize,
    stats: crate::sec_5_2::StackStats,
}

impl Vm {
    fn new(program: CompiledProgram) -> Self {
        let mut engine = ops::Engine::new(program.sema.clone());
        // Main gets its real frame: local writes must resolve.
        let main_def = &program.sema.funs[program.main.0 as usize];
        engine.push_activation(main_def.bind_base, main_def.frame_slots, Vec::new());
        let labels = program
            .instrs
            .iter()
            .enumerate()
            .filter_map(|(index, instr)| match instr {
                Instr::Label(name) => Some((name.clone(), index)),
                _ => None,
            })
            .collect();
        Self {
            program,
            engine,
            regs: HashMap::new(),
            stack: Vec::new(),
            calls: Vec::new(),
            labels,
            pc: 0,
            stats: crate::sec_5_2::StackStats::default(),
        }
    }

    fn run(&mut self) -> Result<(), TrapReport> {
        let entry = self.program.entry.clone();
        self.jump(&entry)?;
        while self.pc < self.program.instrs.len() {
            let instr = self.program.instrs[self.pc].clone();
            self.stats.steps += 1;
            self.step(instr)?;
        }
        Ok(())
    }

    fn trap(trap: Trap) -> TrapReport {
        TrapReport {
            trap,
            span: sicp_runtime::host::diag::Span::default(),
        }
    }

    fn reg(&self, name: &str) -> HostValue {
        self.regs.get(name).cloned().unwrap_or(HostValue::Unit)
    }

    fn set(&mut self, name: &str, value: HostValue) {
        self.regs.insert(name.to_owned(), value);
    }

    /// Reads through a borrowed value, like the native autoref
    /// adjustment for field, index, and method bases.
    fn through_ref(&self, value: HostValue) -> Result<HostValue, TrapReport> {
        if matches!(value, HostValue::Ref { .. }) {
            return self.engine.deref_value(&value).map_err(Self::trap);
        }
        Ok(value)
    }

    /// The place a reference register addresses.
    fn place_of(
        &self,
        name: &str,
    ) -> Result<(sicp_runtime::host::value::Addr, Vec<RtProj>, bool), TrapReport> {
        ops::Engine::ref_target(&self.reg(name)).map_err(Self::trap)
    }

    fn operand(&mut self, operand: &Operand) -> Result<HostValue, TrapReport> {
        Ok(match operand {
            Operand::Const(value) => HostValue::Int(*value),
            Operand::ConstU(value) => HostValue::Usize(*value),
            Operand::Bool(value) => HostValue::Bool(*value),
            Operand::Str(text) => HostValue::Text(text.clone()),
            Operand::Reg(name) => match name
                .strip_prefix("slot")
                .and_then(|digits| digits.parse::<u32>().ok())
            {
                // Locals live in per-call activation frames: a
                // recursive call cannot clobber its caller's values.
                Some(slot) => self
                    .engine
                    .read_local(sicp_runtime::host::hir::BindId(slot))
                    .map_err(Self::trap)?,
                None => self.reg(name),
            },
            Operand::Op(op, operands) => {
                // Computation results flow through the operation
                // path: the operation computes into `val`, and the
                // surrounding assignment stores the target. Control
                // operations are not operand-shaped.
                if matches!(
                    op,
                    PerformOp::Try
                        | PerformOp::CallFun
                        | PerformOp::CallValue
                        | PerformOp::Return
                        | PerformOp::NoMatch
                ) {
                    return Err(Self::trap(Trap::Dangling));
                }
                let saved_pc = self.pc;
                self.perform(op, operands)?;
                self.pc = saved_pc;
                self.reg(VAL)
            }
            Operand::Label(name) => HostValue::Text(name.clone()),
        })
    }

    // Keep each instruction's machine transition in this exhaustive dispatch.
    #[allow(clippy::too_many_lines)]
    fn step(&mut self, instr: Instr) -> Result<(), TrapReport> {
        match instr {
            Instr::Label(_) => self.pc += 1,
            Instr::Assign(target, operand) => {
                let value = self.operand(&operand)?;
                match target
                    .strip_prefix("slot")
                    .and_then(|digits| digits.parse::<u32>().ok())
                {
                    Some(slot) => self
                        .engine
                        .write_local(sicp_runtime::host::hir::BindId(slot), value)
                        .map_err(Self::trap)?,
                    None => self.set(&target, value),
                }
                self.pc += 1;
            }
            Instr::Test(op, left, right) => {
                let a = self.operand(&left)?;
                let b = self.operand(&right)?;
                let result = ops::checked_binary(op, &a, &b).map_err(Self::trap)?;
                self.set("flag", result);
                self.pc += 1;
            }
            Instr::TestBool(operand) => {
                let value = self.operand(&operand)?;
                self.set("flag", value);
                self.pc += 1;
            }
            Instr::Branch(label) => {
                if matches!(self.reg("flag"), HostValue::Bool(true)) {
                    self.jump(&label)?;
                } else {
                    self.pc += 1;
                }
            }
            Instr::Goto(operand) => {
                let HostValue::Text(label) = self.operand(&operand)? else {
                    return Err(Self::trap(Trap::Dangling));
                };
                self.jump(&label)?;
            }
            Instr::Save(name) => {
                let value = self.reg(&name);
                self.stack.push((name, value));
                self.stats.pushes += 1;
                self.note_depth();
                self.pc += 1;
            }
            Instr::Restore(name) => {
                let (saved, value) = self.stack.pop().ok_or_else(|| Self::trap(Trap::Dangling))?;
                if saved != name {
                    return Err(Self::trap(Trap::Dangling));
                }
                self.stats.pops += 1;
                self.set(&name, value);
                self.pc += 1;
            }
            Instr::Discard(count) => {
                let keep = self
                    .stack
                    .len()
                    .checked_sub(count)
                    .ok_or_else(|| Self::trap(Trap::Dangling))?;
                self.stack.truncate(keep);
                self.pc += 1;
            }
            Instr::Bind { pat, fail } => {
                let bound =
                    ops::bind_pattern(&self.engine, &pat, &self.reg(VAL)).map_err(Self::trap)?;
                match bound {
                    Some(bindings) => {
                        for (binding, bound) in bindings {
                            self.engine
                                .write_local(binding, bound)
                                .map_err(Self::trap)?;
                        }
                        self.pc += 1;
                    }
                    None => self.jump(&fail)?,
                }
            }
            Instr::MakeClosure {
                kind,
                body,
                params,
                captures,
                bind_base,
                frame_slots,
            } => {
                let mut capture_values = Vec::with_capacity(captures.len());
                for (slot, mode) in captures {
                    let binding = sicp_runtime::host::hir::BindId(slot);
                    let mode = match mode {
                        1 => sicp_runtime::host::hir::CaptureMode::Mut,
                        2 => sicp_runtime::host::hir::CaptureMode::Owned,
                        _ => sicp_runtime::host::hir::CaptureMode::Shared,
                    };
                    let value = self
                        .engine
                        .capture_value(binding, mode)
                        .map_err(Self::trap)?;
                    capture_values.push((binding, mode, value));
                }
                let param_types: Vec<_> = params
                    .iter()
                    .map(|slot| {
                        (
                            sicp_runtime::host::hir::BindId(*slot),
                            sicp_runtime::host::hir::HostTy::Unit,
                        )
                    })
                    .collect();
                // The closure value carries its compiled entry label:
                // application jumps to the pre-compiled instructions.
                let closure = ops::compiled_closure_value(
                    kind,
                    param_types,
                    capture_values,
                    frame_slots,
                    bind_base,
                    body,
                );
                self.set(VAL, closure);
                self.pc += 1;
            }
            Instr::Perform(op, operands) => self.perform(&op, &operands)?,
        }
        Ok(())
    }

    fn jump(&mut self, label: &str) -> Result<(), TrapReport> {
        self.pc = *self
            .labels
            .get(label)
            .ok_or_else(|| Self::trap(Trap::Dangling))?;
        Ok(())
    }

    /// Leaves the current compiled call with `val` as its result: the
    /// activation is popped, the stack returns to the height the call
    /// started at, and control resumes after the call site. Leaving
    /// main halts the machine.
    fn leave_call(&mut self) {
        self.engine.pop_activation();
        match self.calls.pop() {
            Some(record) => {
                self.stack.truncate(record.stack_base);
                self.pc = record.return_pc;
            }
            None => self.pc = self.program.instrs.len(),
        }
    }

    /// Notes the control-stack depth: the saved-register stack and
    /// the active call frames share the machine's one stack.
    fn note_depth(&mut self) {
        let depth = self.stack.len() + self.calls.len();
        self.stats.max_depth = self.stats.max_depth.max(depth);
    }

    /// Enters one compiled body: pushes its activation, writes its
    /// parameters, records the return point, and jumps to its entry.
    fn enter(
        &mut self,
        entry: &str,
        activation: (
            u32,
            u32,
            Vec<(
                sicp_runtime::host::hir::BindId,
                sicp_runtime::host::hir::CaptureMode,
                HostValue,
            )>,
        ),
        params: &[sicp_runtime::host::hir::BindId],
        args: Vec<HostValue>,
    ) -> Result<(), TrapReport> {
        let (bind_base, frame_slots, captures) = activation;
        self.engine
            .push_activation(bind_base, frame_slots, captures);
        for (binding, value) in params.iter().zip(args) {
            self.engine
                .write_local(*binding, value)
                .map_err(Self::trap)?;
        }
        self.calls.push(CallRecord {
            return_pc: self.pc + 1,
            stack_base: self.stack.len(),
        });
        self.note_depth();
        self.jump(entry)
    }

    fn call_fun(&mut self, fun: FunId, args: Vec<HostValue>) -> Result<(), TrapReport> {
        let def = self
            .program
            .sema
            .funs
            .get(fun.0 as usize)
            .ok_or_else(|| Self::trap(Trap::Dangling))?;
        let params: Vec<_> = def.params.iter().map(|(binding, _)| *binding).collect();
        let activation = (def.bind_base, def.frame_slots, Vec::new());
        self.enter(&format!("fun{}", fun.0), activation, &params, args)
    }

    /// Applies one function or closure value in the flat stream: the
    /// activation is pushed and control jumps to the pre-compiled
    /// entry, so no syntax is interpreted here.
    fn enter_value(&mut self, callee: HostValue, args: Vec<HostValue>) -> Result<(), TrapReport> {
        match callee {
            HostValue::Closure(closure) => {
                let entry = closure
                    .compiled_entry
                    .clone()
                    .ok_or_else(|| Self::trap(Trap::Dangling))?;
                let params: Vec<_> = closure.params.iter().map(|(binding, _)| *binding).collect();
                let activation = (
                    closure.bind_base,
                    closure.frame_slots,
                    closure.captures.clone(),
                );
                self.enter(&entry, activation, &params, args)
            }
            HostValue::FnPtr(fun) => self.call_fun(fun, args),
            HostValue::Box(inner) => self.enter_value(*inner, args),
            HostValue::Ref { .. } => {
                let callee = self.through_ref(callee)?;
                self.enter_value(callee, args)
            }
            _ => Err(Self::trap(Trap::Dangling)),
        }
    }

    fn take_argl(&mut self) -> Vec<HostValue> {
        match self.regs.remove(ARGL) {
            Some(HostValue::Vec(items)) => items,
            _ => Vec::new(),
        }
    }

    // Keep each typed machine operation's register effects together here.
    #[allow(clippy::too_many_lines)]
    fn perform(&mut self, op: &PerformOp, operands: &[Operand]) -> Result<(), TrapReport> {
        let value = match op {
            PerformOp::CallFun => {
                let Some(Operand::Label(label)) = operands.first() else {
                    return Err(Self::trap(Trap::Dangling));
                };
                let fun = label
                    .strip_prefix("fun")
                    .and_then(|digits| digits.parse().ok())
                    .ok_or_else(|| Self::trap(Trap::Dangling))?;
                let args = self.take_argl();
                return self.call_fun(FunId(fun), args);
            }
            PerformOp::CallValue => {
                let callee = self.reg(VAL);
                let args = self.take_argl();
                return self.enter_value(callee, args);
            }
            PerformOp::Return => {
                self.leave_call();
                return Ok(());
            }
            PerformOp::Try => {
                let value = self.reg(VAL);
                match ops::builtin_variant(&value) {
                    Some((0, payload)) if payload.len() == 1 => payload[0].clone(),
                    // `Err(e)` or `None` leaves the function with the
                    // residual as its value.
                    Some((1, _)) => {
                        self.leave_call();
                        return Ok(());
                    }
                    _ => return Err(Self::trap(Trap::Dangling)),
                }
            }
            PerformOp::NoMatch => return Err(Self::trap(Trap::Dangling)),
            PerformOp::FunRef(fun) => HostValue::FnPtr(FunId(*fun)),
            PerformOp::Ctor(ctor) => {
                let args = self.take_argl();
                ops::construct(*ctor, &args).map_err(Self::trap)?
            }
            PerformOp::Method(method) => {
                let receiver = self.through_ref(self.reg(VAL))?;
                let args = self.take_argl();
                let (result, _) =
                    ops::apply_method(*method, receiver, None, &args).map_err(Self::trap)?;
                result
            }
            PerformOp::MethodAt(method) => {
                let (addr, projs, _) = self.place_of(VAL)?;
                let args = self.take_argl();
                let receiver = if *method == MethodOp::IntoIter {
                    self.engine.take_at(addr, &projs)
                } else {
                    self.engine.read_at(addr, &projs)
                }
                .map_err(Self::trap)?;
                let receiver = self.through_ref(receiver)?;
                let (result, updated) =
                    ops::apply_method(*method, receiver, Some((addr, projs.clone())), &args)
                        .map_err(Self::trap)?;
                if let Some(updated) = updated {
                    self.engine
                        .write_at(addr, &projs, updated)
                        .map_err(Self::trap)?;
                }
                result
            }
            PerformOp::LocalRef {
                bind,
                mutable,
                through,
            } => {
                let (addr, projs) = self
                    .engine
                    .local_place(sicp_runtime::host::hir::BindId(*bind))
                    .map_err(Self::trap)?;
                let (addr, projs) = if *through {
                    match self.engine.read_at(addr, &projs).map_err(Self::trap)? {
                        HostValue::Ref { addr, projs, .. } => (addr, projs),
                        _ => (addr, projs),
                    }
                } else {
                    (addr, projs)
                };
                HostValue::Ref {
                    addr,
                    projs,
                    mutable: *mutable,
                }
            }
            PerformOp::RefField(index) => {
                let (addr, mut projs, mutable) = self.place_of(VAL)?;
                projs.push(RtProj::Field(*index));
                HostValue::Ref {
                    addr,
                    projs,
                    mutable,
                }
            }
            PerformOp::RefIndex => {
                let (addr, mut projs, mutable) = self.place_of(VAL)?;
                let at = ops::index_position(&self.reg(TMP)).map_err(Self::trap)?;
                projs.push(RtProj::Index(at));
                HostValue::Ref {
                    addr,
                    projs,
                    mutable,
                }
            }
            PerformOp::Load => {
                let (addr, projs, _) = self.place_of(VAL)?;
                self.engine.read_at(addr, &projs).map_err(Self::trap)?
            }
            PerformOp::Take => {
                let (addr, projs, _) = self.place_of(VAL)?;
                self.engine.take_at(addr, &projs).map_err(Self::trap)?
            }
            PerformOp::Store(binop) => {
                let (addr, projs, _) = self.place_of(TMP)?;
                let produced = self.reg(VAL);
                let stored = match *binop {
                    None => produced,
                    Some(binop) => {
                        let current = self.engine.read_at(addr, &projs).map_err(Self::trap)?;
                        ops::checked_binary(binop, &current, &produced).map_err(Self::trap)?
                    }
                };
                self.engine
                    .write_at(addr, &projs, stored)
                    .map_err(Self::trap)?;
                HostValue::Unit
            }
            PerformOp::Arith(binop) => {
                ops::checked_binary(*binop, &self.reg(VAL), &self.reg(TMP)).map_err(Self::trap)?
            }
            PerformOp::Unary(unop) => {
                ops::checked_unary(*unop, &self.reg(VAL)).map_err(Self::trap)?
            }
            PerformOp::Format(kind, spec) => {
                let args = self.take_argl();
                let rendered =
                    ops::render_format(spec, &args, &self.engine.store).map_err(Self::trap)?;
                match *kind {
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
            PerformOp::MakeTuple => {
                HostValue::Tuple(Box::new(self.reg(VAL)), Box::new(self.reg(TMP)))
            }
            PerformOp::MakeVec => {
                // The stack holds the operands bottom-first; they leave
                // it in source order.
                let count = match operands.first() {
                    Some(Operand::Const(count)) => usize::try_from(*count).unwrap_or(0),
                    _ => 0,
                };
                let keep = self
                    .stack
                    .len()
                    .checked_sub(count)
                    .ok_or_else(|| Self::trap(Trap::Dangling))?;
                self.stats.pops += u64::try_from(count).expect("operand lists are small");
                HostValue::Vec(
                    self.stack
                        .split_off(keep)
                        .into_iter()
                        .map(|(_, value)| value)
                        .collect(),
                )
            }
            PerformOp::MakeRepeat => {
                let times = ops::index_position(&self.reg(TMP)).map_err(Self::trap)?;
                let times =
                    usize::try_from(times).map_err(|_| Self::trap(Trap::Overflow("repeat")))?;
                HostValue::Vec(vec![self.reg(VAL); times])
            }
            PerformOp::MakeStruct(item) => HostValue::Struct(*item, self.take_argl()),
            PerformOp::MakeVariant(item, index) => {
                HostValue::Variant(*item, *index, self.take_argl())
            }
            PerformOp::Project(index) => {
                let base = self.through_ref(self.reg(VAL))?;
                project_value(&base, *index).map_err(Self::trap)?
            }
            PerformOp::IndexGet => {
                let base = self.through_ref(self.reg(VAL))?;
                let at = ops::index_position(&self.reg(TMP)).map_err(Self::trap)?;
                let (HostValue::Vec(items) | HostValue::Array(items)) = base else {
                    return Err(Self::trap(Trap::Dangling));
                };
                let at = usize::try_from(at).map_err(|_| Self::trap(Trap::IndexOutOfBounds))?;
                items
                    .get(at)
                    .cloned()
                    .ok_or_else(|| Self::trap(Trap::IndexOutOfBounds))?
            }
            PerformOp::MakeRange => {
                ops::range_of(&self.reg(VAL), &self.reg(TMP)).map_err(Self::trap)?
            }
            PerformOp::IterStart => match self.reg(VAL) {
                iterator @ HostValue::Iter(_) => iterator,
                HostValue::Ref {
                    addr,
                    projs,
                    mutable,
                } => {
                    let collection = self.engine.read_at(addr, &projs).map_err(Self::trap)?;
                    let (HostValue::Vec(items) | HostValue::Array(items)) = collection else {
                        return Err(Self::trap(Trap::Dangling));
                    };
                    ops::refs_iterator(addr, projs, items.len(), mutable)
                }
                HostValue::Vec(items) | HostValue::Array(items) => ops::items_iterator(items),
                _ => return Err(Self::trap(Trap::Dangling)),
            },
            PerformOp::IterNext => {
                let mut iterator = self.reg(VAL);
                let step = ops::iterator_next(&mut iterator).map_err(Self::trap)?;
                match ops::builtin_variant(&step) {
                    Some((0, payload)) if payload.len() == 1 => {
                        let item = payload[0].clone();
                        self.set(ITEM, item);
                        self.set("flag", HostValue::Bool(true));
                    }
                    _ => self.set("flag", HostValue::Bool(false)),
                }
                iterator
            }
        };
        self.set(VAL, value);
        self.pc += 1;
        Ok(())
    }
}

fn project_value(value: &HostValue, index: u32) -> Result<HostValue, Trap> {
    match value {
        HostValue::Struct(_, fields) | HostValue::Variant(_, _, fields) => fields
            .get(index as usize)
            .cloned()
            .ok_or(Trap::IndexOutOfBounds),
        HostValue::Tuple(left, right) => match index {
            0 => Ok((**left).clone()),
            1 => Ok((**right).clone()),
            _ => Err(Trap::IndexOutOfBounds),
        },
        _ => Err(Trap::Dangling),
    }
}

/// The C translation of one compiled program (exercise 5.52's
/// artifact): a translation unit over the `mc_*` word runtime that the
/// backend links with its own `main`. The generated `mc_main` executes
/// the compiled instruction sequence; nothing here is a placeholder.
#[must_use]
pub fn emit_c(program: &CompiledProgram) -> String {
    let mut out = String::new();
    out.push_str(
        "/* Generated from the typed compiler representation (exercise 5.52). */\n#include <stdio.h>\n#include <stdlib.h>\n#include <string.h>\n\n/* The tagged-word runtime the backend provides. */\nextern long mc_alloc_pair(long car, long cdr);\nextern long mc_car(long pair);\nextern long mc_cdr(long pair);\nextern long mc_set_car(long pair, long value);\nextern long mc_set_cdr(long pair, long value);\nextern long mc_alloc_int(long value);\nextern long mc_int_value(long word);\nextern long mc_is_pair(long word);\nextern long mc_is_null(long word);\nextern void mc_print(long word);\nextern long mc_make_closure(long body_id, long arity);\nextern long mc_apply_closure(long closure, long args);\nextern long mc_alloc_vec(long capacity);\nextern long mc_vec_push(long vec, long value);\nextern long mc_vec_get(long vec, long index);\nextern long mc_text(const char *text);\n\nvoid mc_main(void);\n\n",
    );
    out.push_str("void mc_main(void) {\n");
    out.push_str("    long val = 0, tmp = 0, argl = 0, flag = 0;\n");
    out.push_str("    long save_stack[4096]; long save_top = 0;\n");
    out.push_str("    long call_stack[2048]; long call_top = 0;\n");
    out.push_str("    static int labels_indexed = 0;\n    (void)labels_indexed;\n");
    let mut label_index: HashMap<String, usize> = HashMap::new();
    for (instruction_index, instr) in program.instrs.iter().enumerate() {
        if let Instr::Label(name) = instr {
            label_index.insert(name.clone(), instruction_index);
        }
    }
    out.push_str("    goto ");
    write_c_label(&mut out, &program.entry);
    out.push_str(";\n");
    // Formatting into a `String` cannot fail.
    for instr in &program.instrs {
        match instr {
            Instr::Label(name) => {
                write_c_label(&mut out, name);
                out.push_str(": ;\n");
            }
            Instr::Assign(target, operand) => {
                if let Operand::Op(op, args) = operand
                    && is_statement_op(op)
                {
                    out.push_str(&c_perform(op, args));
                    let _ = writeln!(&mut out, "    {} = val;", c_reg(target));
                    continue;
                }
                let _ = writeln!(&mut out, "    {} = {};", c_reg(target), c_operand(operand));
            }
            Instr::Test(op, left, right) => {
                let _ = writeln!(
                    &mut out,
                    "    flag = (mc_int_value({}) {} mc_int_value({}));",
                    c_operand(left),
                    c_op(*op),
                    c_operand(right)
                );
            }
            Instr::TestBool(operand) => {
                let _ = writeln!(&mut out, "    flag = ({} != 0);", c_operand(operand));
            }
            Instr::Branch(label) => {
                out.push_str("    if (flag) goto ");
                write_c_label(&mut out, label);
                out.push_str(";\n");
            }
            Instr::Goto(operand) => match operand {
                Operand::Label(label) => {
                    out.push_str("    goto ");
                    write_c_label(&mut out, label);
                    out.push_str(";\n");
                }
                other => {
                    let _ = writeln!(&mut out, "    goto *dispatch[{}];", c_operand(other));
                }
            },
            Instr::Save(name) => {
                let _ = writeln!(&mut out, "    save_stack[save_top++] = {};", c_reg(name));
            }
            Instr::Restore(name) => {
                let _ = writeln!(&mut out, "    {} = save_stack[--save_top];", c_reg(name));
            }
            Instr::Discard(count) => {
                let _ = writeln!(&mut out, "    save_top -= {count};");
            }
            Instr::Bind { fail, .. } => {
                out.push_str("    if (!mc_bind(val)) goto ");
                write_c_label(&mut out, fail);
                out.push_str(";\n");
            }
            Instr::MakeClosure { body, params, .. } => {
                let _ = writeln!(
                    &mut out,
                    "    val = mc_make_closure({}, {});",
                    label_index.get(body).copied().unwrap_or(0),
                    params.len()
                );
            }
            Instr::Perform(op, operands) => {
                out.push_str(&c_perform(op, operands));
            }
        }
    }
    out.push_str("}\n");
    let _ = label_index;
    out
}
fn write_c_label(out: &mut String, label: &str) {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    out.push_str("L_");
    for byte in label.bytes() {
        if byte.is_ascii_alphanumeric() {
            out.push(char::from(byte));
        } else {
            out.push('_');
            out.push(char::from(HEX[usize::from(byte >> 4)]));
            out.push(char::from(HEX[usize::from(byte & 0x0f)]));
        }
    }
}

fn c_reg(name: &str) -> String {
    match name {
        "val" | "tmp" | "argl" | "flag" => name.to_owned(),
        other => format!("reg_{other}"),
    }
}

fn c_operand(operand: &Operand) -> String {
    match operand {
        Operand::Const(value) => format!("mc_alloc_int({value})"),
        Operand::ConstU(value) => format!("mc_alloc_int({value})"),
        Operand::Bool(value) => format!("mc_alloc_int({})", i64::from(*value)),
        Operand::Str(text) => format!("mc_text({text:?})"),
        Operand::Reg(name) => c_reg(name),
        Operand::Label(name) => format!("/* label {name} */ 0"),
        Operand::Op(op, args) => c_expr_op(op, args),
    }
}

/// Whether an operation materializes through statements rather than
/// an expression: those reach C only through `c_perform`.
fn is_statement_op(op: &PerformOp) -> bool {
    matches!(
        op,
        PerformOp::Ctor(_)
            | PerformOp::MakeVec
            | PerformOp::MakeRepeat
            | PerformOp::MakeStruct(_)
            | PerformOp::MakeVariant(_, _)
            | PerformOp::CallFun
            | PerformOp::CallValue
            | PerformOp::Return
            | PerformOp::Format(_, _)
            | PerformOp::Method(_)
    )
}

/// Lowers expression-shaped operations inline; statement-shaped
/// operations reach C only through `c_perform` at statement level.
fn c_expr_op(op: &PerformOp, args: &[Operand]) -> String {
    let arg = |index: usize| -> String {
        args.get(index)
            .map_or_else(|| "mc_alloc_int(0)".to_owned(), c_operand)
    };
    match op {
        PerformOp::Arith(binop) => format!(
            "mc_alloc_int(mc_int_value({}) {} mc_int_value({}))",
            arg(0),
            c_op(*binop),
            arg(1)
        ),
        PerformOp::Unary(UnOp::Neg) => format!("mc_alloc_int(-mc_int_value({}))", arg(0)),
        PerformOp::Unary(UnOp::Not) => format!("mc_alloc_int(!mc_int_value({}))", arg(0)),
        PerformOp::Unary(_) | PerformOp::IterStart => arg(0),
        PerformOp::Project(index) => {
            format!("({index} == 0 ? mc_car({}) : mc_cdr({}))", arg(0), arg(0))
        }
        PerformOp::IndexGet => format!("mc_vec_get({}, mc_int_value({}))", arg(0), arg(1)),
        PerformOp::MakeRange | PerformOp::MakeTuple => {
            format!("mc_alloc_pair({}, {})", arg(0), arg(1))
        }
        PerformOp::Try => format!("mc_car({})", arg(0)),
        PerformOp::IterNext => format!("mc_cdr({})", arg(0)),
        // Statement-shaped operations appear only as Assign sources:
        // the instruction loop emits them through `c_perform`. A
        // nested statement operation in an expression position cannot
        // be a C expression; fail loudly rather than emit wrong code.
        _ => String::from("mc_compile_error_statement_operation_in_expression_position"),
    }
}

fn c_op(op: BinOp) -> &'static str {
    match op {
        BinOp::Add | BinOp::And | BinOp::Or => "+",
        BinOp::Sub => "-",
        BinOp::Mul => "*",
        BinOp::Div => "/",
        BinOp::Rem => "%",
        BinOp::Eq => "==",
        BinOp::Ne => "!=",
        BinOp::Lt => "<",
        BinOp::Le => "<=",
        BinOp::Gt => ">",
        BinOp::Ge => ">=",
    }
}
fn c_string_literal(text: &str) -> String {
    let mut out = String::new();
    out.push('"');
    for character in text.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\0' => out.push_str("\\000"),
            character if character.is_ascii_control() => {
                let byte =
                    u8::try_from(u32::from(character)).expect("ASCII control characters fit in u8");
                out.push('\\');
                out.push(char::from(b'0' + (byte >> 6)));
                out.push(char::from(b'0' + ((byte >> 3) & 7)));
                out.push(char::from(b'0' + (byte & 7)));
            }
            other => out.push(other),
        }
    }
    out.push('"');
    out
}

fn c_print_format(kind: FormatKind, spec: &FormatSpec) -> String {
    if kind == FormatKind::Format {
        return String::from("#error \"the C backend does not support format! values\"\n");
    }
    if spec.debug.iter().any(|debug| *debug) {
        return String::from("#error \"the C backend does not support debug formatting\"\n");
    }
    let mut out = String::new();
    for (index, piece) in spec.pieces.iter().enumerate() {
        if !piece.is_empty() {
            out.push_str("    fputs(");
            out.push_str(&c_string_literal(piece));
            out.push_str(", stdout);\n");
        }
        if index < spec.debug.len() {
            out.push_str("    mc_print(mc_vec_get(argl, ");
            out.push_str(&index.to_string());
            out.push_str("));\n");
        }
    }
    if kind == FormatKind::Println {
        out.push_str("    putchar('\\n');\n");
    }
    out
}

// Keep one exhaustive lowering from typed operations to C statements.
#[allow(clippy::too_many_lines)]
fn c_perform(op: &PerformOp, operands: &[Operand]) -> String {
    match op {
        PerformOp::CallFun => {
            let name = match operands.first() {
                Some(Operand::Label(name)) => name.as_str(),
                _ => "0",
            };
            let mut out = String::from("    call_stack[call_top++] = 0; goto ");
            write_c_label(&mut out, name);
            out.push_str(";\n");
            out
        }
        PerformOp::Return => "    if (call_top > 0) { call_top--; return; }\n".to_owned(),
        PerformOp::Ctor(CtorOp::StringFrom) => "    val = mc_text(\"\");\n".to_owned(),
        PerformOp::Ctor(CtorOp::VecNew) => "    val = mc_alloc_vec(0);\n".to_owned(),
        // Option and result constructors go through named externs:
        // the backend defines each one it supports, and a missing
        // one fails the artifact at link with the exact missing
        // surface instead of silently wrong code.
        PerformOp::Ctor(CtorOp::OptSome) => {
            let payload = operands
                .first()
                .map_or_else(|| "mc_vec_get(argl, 0)".to_owned(), c_operand);
            format!("    val = mc_compile_error_ctor_opt_some({payload});\n")
        }
        PerformOp::Ctor(CtorOp::OptNone) => {
            "    val = mc_compile_error_ctor_opt_none();\n".to_owned()
        }
        PerformOp::Ctor(CtorOp::ResOk) => {
            let payload = operands
                .first()
                .map_or_else(|| "mc_vec_get(argl, 0)".to_owned(), c_operand);
            format!("    val = mc_compile_error_ctor_res_ok({payload});\n")
        }
        PerformOp::Ctor(CtorOp::ResErr) => {
            let payload = operands
                .first()
                .map_or_else(|| "mc_vec_get(argl, 0)".to_owned(), c_operand);
            format!("    val = mc_compile_error_ctor_res_err({payload});\n")
        }
        PerformOp::Format(kind, spec) => c_print_format(*kind, spec),
        PerformOp::MakeVec => {
            // Counted construction: the save stack holds the pushed
            // values bottom-first; pop them in source order.
            let count = match operands.first() {
                Some(Operand::Const(n)) => usize::try_from(*n).unwrap_or(0),
                _ => 0,
            };
            let mut out = String::from("    val = mc_alloc_vec(0);\n");
            for index in (0..count).rev() {
                let _ = writeln!(
                    &mut out,
                    "    val = mc_vec_push(val, save_stack[save_top - {index} - 1]);"
                );
            }
            let _ = writeln!(&mut out, "    save_top -= {count};");
            out
        }
        PerformOp::MakeRepeat => String::from(
            "    { long _item = val; long _n = mc_int_value(tmp); \
val = mc_alloc_vec(0); \
while (_n > 0) { val = mc_vec_push(val, _item); _n--; } }\n",
        ),
        PerformOp::MakeStruct(_) | PerformOp::MakeVariant(_, _) => "    val = argl;\n".to_owned(),
        PerformOp::MakeTuple | PerformOp::MakeRange => {
            "    val = mc_alloc_pair(val, tmp);\n".to_owned()
        }
        PerformOp::Project(index) => {
            format!("    val = {index} == 0 ? mc_car(val) : mc_cdr(val);\n")
        }
        PerformOp::IndexGet => "    val = mc_vec_get(val, mc_int_value(tmp));\n".to_owned(),
        PerformOp::Arith(op) => format!(
            "    val = mc_alloc_int(mc_int_value(val) {} mc_int_value(tmp));\n",
            c_op(*op)
        ),
        PerformOp::Unary(UnOp::Neg) => "    val = mc_alloc_int(-mc_int_value(val));\n".to_owned(),
        PerformOp::Unary(UnOp::Not) => "    val = mc_alloc_int(!mc_int_value(val));\n".to_owned(),
        PerformOp::Unary(_) | PerformOp::IterStart => "    val = val;\n".to_owned(),
        PerformOp::IterNext => "    val = mc_cdr(val);\n".to_owned(),
        // The mc_* word runtime has no place (reference) model or
        // function-value table: emit named externs so the artifact
        // fails at link with the exact missing surface instead of
        // silently wrong code.
        PerformOp::FunRef(fun) => format!("    val = mc_compile_error_fun_ref({fun});\n"),
        PerformOp::LocalRef { bind, .. } => {
            format!("    val = mc_compile_error_local_ref({bind});\n")
        }
        PerformOp::RefField(index) => {
            format!("    val = mc_compile_error_ref_field(val, {index});\n")
        }
        PerformOp::RefIndex => "    val = mc_compile_error_ref_index(val, tmp);\n".to_owned(),
        PerformOp::Load | PerformOp::Take => "    val = mc_compile_error_load(val);\n".to_owned(),
        PerformOp::Store(_) => "    val = mc_compile_error_store(tmp, val);\n".to_owned(),
        PerformOp::MethodAt(_) => "    val = mc_compile_error_method_at(val, argl);\n".to_owned(),
        PerformOp::NoMatch => "    mc_compile_error_no_match();\n".to_owned(),
        PerformOp::Try => "    val = mc_car(val);\n".to_owned(),
        PerformOp::CallValue | PerformOp::Method(_) => {
            "    val = mc_apply_closure(val, argl);\n".to_owned()
        }
        PerformOp::Ctor(CtorOp::VecWithCapacity) => {
            let cap = operands
                .first()
                .map_or_else(|| "mc_vec_get(argl, 0)".to_owned(), c_operand);
            format!("    val = mc_alloc_vec(mc_int_value({cap}));\n")
        }
        PerformOp::Ctor(CtorOp::MapNew) => {
            "    val = mc_compile_error_ctor_map_new();\n".to_owned()
        }
        PerformOp::Ctor(CtorOp::BoxNew) => {
            let payload = operands
                .first()
                .map_or_else(|| "mc_vec_get(argl, 0)".to_owned(), c_operand);
            format!("    val = mc_compile_error_ctor_box_new({payload});\n")
        }
    }
}

/// The register names the compiler targets, exported for the
/// section's exercises.
#[must_use]
pub fn register_names() -> &'static [&'static str] {
    &["val", "tmp", "argl", "flag"]
}

/// The liveness merge two sequences share when stacked.
#[must_use]
pub fn parallel_merge(left: &Seq, right: &Seq) -> Seq {
    let mut merged = left.clone();
    merged.needs.extend(right.needs.iter().cloned());
    merged.modifies.extend(right.modifies.iter().cloned());
    merged
}
