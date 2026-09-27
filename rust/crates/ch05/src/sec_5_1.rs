// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 5.1

//! Section 5.1: Designing register machines.
//!
//! This module carries the hand-transcription model the section's
//! design answers run on. A machine is transcribed line for line from
//! the book's figures into [`Instruction`] data: label lines,
//! register, constant, label, and operation sources, tests, branches,
//! gotos, saves, and restores. There is deliberately no controller
//! parser, no register objects, and no pluggable source of
//! instructions; those are the concerns of the 5.2 simulator, which
//! will build on the same instruction language. Here a run is a
//! reader's hand simulation: resolved labels, one program counter, a
//! register file, a test flag, and a stack of saved register values,
//! stepped at most [`MAX_STEPS`] instructions so a defective
//! controller design fails loudly instead of hanging.
//!
//! Every executed instruction counts, transfers included. Save and
//! restore steps are recorded as [`Event`]s carrying the stack depth
//! and, for a restore, its matching save: the section's exercise 5.5
//! asks for the stack contents at each significant point, and the
//! edition's exercise 5.5a asks for exactly this annotation.

/// The largest number of instructions one run may execute before the
/// model reports [`Fault::OutOfSteps`].
pub const MAX_STEPS: u64 = 100_000;

/// A machine value: the small tower section 5.1 needs.
///
/// Labels are values because the section stores controller labels in
/// registers (`continue`) and jumps to them. Booleans exist only so
/// test operations have a return type; they never enter a register in
/// this section's machines.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Value {
    /// A fixed-width integer, as in `(const 3)`.
    Int(i64),
    /// A floating-point number, as in `(const 1.0)`.
    Real(f64),
    /// A controller label, as in `(label after-fact)`.
    Label(&'static str),
    /// The result of a test operation.
    Bool(bool),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Int(n) => write!(f, "{n}"),
            Self::Real(x) => write!(f, "{x}"),
            Self::Label(name) => write!(f, "{name}"),
            Self::Bool(true) => write!(f, "#t"),
            Self::Bool(false) => write!(f, "#f"),
        }
    }
}

impl Value {
    /// The value as a real number, promoting integers.
    ///
    /// An exact integer outside f64's mantissa loses precision by the
    /// same exactness rule the book's inexact arithmetic applies; the
    /// hand model's arguments stay far below the 2^53 point.
    #[expect(
        clippy::cast_precision_loss,
        reason = "the section's machine values are far below f64's exact range"
    )]
    #[must_use]
    pub fn as_real(self) -> Option<f64> {
        match self {
            Self::Int(n) => Some(n as f64),
            Self::Real(x) => Some(x),
            _ => None,
        }
    }
}

/// An operation input: a register reference or a constant.
#[derive(Clone, Copy, Debug)]
pub enum Input {
    /// `(reg ⟨name⟩)`
    Reg(&'static str),
    /// `(const ⟨value⟩)`
    Const(Value),
}

/// The right-hand side of an assign, as the 5.1.5 grammar allows.
#[derive(Clone, Copy, Debug)]
pub enum AssignSource {
    /// `(reg ⟨name⟩)`
    Reg(&'static str),
    /// `(const ⟨value⟩)`
    Const(Value),
    /// `(label ⟨name⟩)`: a label as a special kind of constant.
    Label(&'static str),
    /// `((op ⟨name⟩) ⟨input⟩ ...)`: an operation application.
    Op {
        /// The operation name, as `(op ⟨name⟩)`.
        name: &'static str,
        /// The operation's inputs.
        inputs: &'static [Input],
    },
}

/// One controller instruction, transcribed from the book's notation.
#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    /// A label line; it occupies no slot in the instruction sequence.
    Label(&'static str),
    /// `(assign ⟨reg⟩ ⟨source⟩)`
    Assign {
        /// The assigned register.
        reg: &'static str,
        /// The assigned value's source.
        source: AssignSource,
    },
    /// `(test (op ⟨name⟩) ⟨input⟩ ...)`: sets the test flag.
    Test {
        /// The tested operation's name.
        op: &'static str,
        /// The tested operation's inputs.
        inputs: &'static [Input],
    },
    /// `(branch (label ⟨name⟩))`: taken when the flag is true.
    Branch(&'static str),
    /// `(goto (label ⟨name⟩))`
    GotoLabel(&'static str),
    /// `(goto (reg ⟨name⟩))`: jump to the register's label value.
    GotoReg(&'static str),
    /// `(save ⟨reg⟩)`: push the register onto the stack.
    Save(&'static str),
    /// `(restore ⟨reg⟩)`: pop the stack into the register.
    Restore(&'static str),
}

/// A machine description: registers, its operations, and the
/// controller sequence. The operations are part of the machine's
/// data-path description; a machine may supply compound operations of
/// its own (the book's `good-enough?` and `improve` in exercise 5.3)
/// and every machine may also use the shared table [`hand_op`].
#[derive(Clone, Copy)]
pub struct Machine {
    registers: &'static [&'static str],
    ops: &'static [(&'static str, HandOp)],
    code: &'static [Instruction],
}

/// One resolved instruction slot: the instruction plus the label that
/// dominates it, for reading traces.
#[derive(Clone, Copy, Debug)]
pub struct Slot {
    /// The instruction itself.
    pub instruction: Instruction,
    /// The dominating label, or the empty label above the first one.
    pub label: &'static str,
}

/// The type of one machine operation: inputs in, one value or fault.
pub type HandOp = fn(&[Value]) -> Result<Value, Fault>;

/// The section's shared operation table: `= < > + - * / abs`, the
/// operations every machine of this section uses. Comparisons take
/// two numbers. `+ - *` fold two or more numbers, staying in the
/// integers when every argument is an integer. `/` always computes a
/// real quotient. `abs` takes one number. Integer arithmetic is
/// 64-bit and machine-sized arguments never approach its bounds.
#[must_use]
pub fn hand_op(name: &str) -> Option<HandOp> {
    match name {
        "=" => Some(|a| compare("=", a, |x, y| x == y, numeric_eq)),
        "<" => Some(|a| compare("<", a, |x, y| x < y, |x, y| x < y)),
        ">" => Some(|a| compare(">", a, |x, y| x > y, |x, y| x > y)),
        "+" => Some(|a| arith("+", a, |x, y| x + y, |x, y| x + y)),
        "-" => Some(|a| arith("-", a, |x, y| x - y, |x, y| x - y)),
        "*" => Some(|a| arith("*", a, |x, y| x * y, |x, y| x * y)),
        "/" => Some(divide),
        "abs" => Some(abs),
        _ => None,
    }
}

/// Why a hand simulation stops early.
#[derive(Clone, Debug, PartialEq)]
pub enum Fault {
    /// The controller named an operation outside [`hand_op`].
    UnknownOp {
        /// The unknown operation name.
        op: String,
        /// The step that reached it.
        step: u64,
    },
    /// An operation got the wrong number of inputs.
    BadArity {
        /// The operation name.
        op: &'static str,
        /// How many inputs arrived.
        got: usize,
    },
    /// An operation got a non-number where a number was required.
    TypeMismatch {
        /// The operation name.
        op: &'static str,
        /// The step that reached it.
        step: u64,
    },
    /// A branch, goto, or label source named an unknown label.
    UnknownLabel {
        /// The unknown label name.
        label: String,
        /// The step that reached it.
        step: u64,
    },
    /// An instruction named a register outside the machine's table.
    UnknownRegister {
        /// The unknown register name.
        reg: String,
        /// The step that reached it.
        step: u64,
    },
    /// Two label lines in one controller share a name.
    DuplicateLabel {
        /// The repeated label name.
        label: String,
    },
    /// A restore reached past the bottom of the stack.
    StackUnderflow {
        /// The restored register.
        reg: &'static str,
        /// The step that reached it.
        step: u64,
    },
    /// A restore popped a value saved from a different register.
    MismatchedRestore {
        /// The register the restore names.
        reg: &'static str,
        /// The register the popped entry was saved from.
        found: &'static str,
        /// The step that reached it.
        step: u64,
    },
    /// The run exceeded [`MAX_STEPS`] instructions.
    OutOfSteps {
        /// The step budget that ran out.
        step: u64,
    },
}

impl std::fmt::Display for Fault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::UnknownOp { ref op, step } => {
                write!(f, "unknown operation {op:?} at step {step}")
            }
            Self::BadArity { op, got } => {
                write!(f, "operation {op:?} got {got} inputs")
            }
            Self::TypeMismatch { op, step } => {
                write!(f, "operation {op:?} needs numbers at step {step}")
            }
            Self::UnknownLabel { ref label, step } => {
                write!(f, "unknown label {label:?} at step {step}")
            }
            Self::UnknownRegister { ref reg, step } => {
                write!(f, "unknown register {reg:?} at step {step}")
            }
            Self::DuplicateLabel { ref label } => write!(f, "duplicate label {label:?}"),
            Self::StackUnderflow { reg, step } => {
                write!(f, "restore {reg:?} on an empty stack at step {step}")
            }
            Self::MismatchedRestore { reg, found, step } => write!(
                f,
                "restore {reg:?} popped a value saved from {found:?} at step {step}"
            ),
            Self::OutOfSteps { step } => write!(f, "run exceeded {step} steps"),
        }
    }
}

/// One recorded significant point: a save or a restore.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Event {
    /// A `(save ⟨reg⟩)` executed; `depth` is the stack depth after it.
    Save {
        /// The step of the save, the trace's event id.
        step: u64,
        /// The label dominating the save.
        label: &'static str,
        /// The saved register.
        reg: &'static str,
        /// The saved value.
        value: Value,
        /// The stack depth after the push.
        depth: usize,
    },
    /// A `(restore ⟨reg⟩)` executed; `depth` is the depth after it.
    Restore {
        /// The step of the restore, the trace's event id.
        step: u64,
        /// The label dominating the restore.
        label: &'static str,
        /// The restored register.
        reg: &'static str,
        /// The value the restore returned.
        value: Value,
        /// The stack depth after the pop.
        depth: usize,
        /// The step of the save this restore matches.
        matched: u64,
        /// How many instructions passed between that save and here.
        age: u64,
        /// How many older saves of the same register stayed beneath.
        older: usize,
    },
}

/// The outcome of one hand-simulated run.
#[derive(Clone, Debug, PartialEq)]
pub struct Run {
    /// Final register contents, in the machine's register order.
    pub registers: Vec<(&'static str, Value)>,
    /// The recorded save and restore events, in execution order.
    pub events: Vec<Event>,
    /// Every executed instruction, transfers included.
    pub instructions: u64,
    /// How many saves executed.
    pub pushes: u64,
    /// How many restores executed.
    pub pops: u64,
    /// The deepest the stack reached.
    pub max_depth: usize,
}

impl Run {
    /// The final value of one register of the run.
    ///
    /// # Panics
    /// Panics when the machine has no register of that name: a
    /// transcription defect the caller sees immediately.
    #[must_use]
    pub fn value_of(&self, reg: &str) -> Value {
        self.registers
            .iter()
            .find(|(name, _)| *name == reg)
            .map_or_else(
                || panic!("no register {reg} in the run"),
                |(_, value)| *value,
            )
    }
}

impl Machine {
    /// Describes a machine from its registers, its operations, and
    /// its controller sequence. Pass no operations for a machine
    /// built on the shared table alone.
    #[must_use]
    pub fn new(
        registers: &'static [&'static str],
        ops: &'static [(&'static str, HandOp)],
        code: &'static [Instruction],
    ) -> Self {
        Self {
            registers,
            ops,
            code,
        }
    }

    /// Resolves the controller: each instruction slot carries the
    /// label that dominates it. A trailing label binds to the stop
    /// address one past the last instruction, the section's exit
    /// convention, so a jump there ends the run. A label repeated in
    /// one sequence is a defect.
    ///
    /// # Errors
    /// [`Fault::DuplicateLabel`] when a label name repeats.
    pub fn resolve(&self) -> Result<Vec<Slot>, Fault> {
        let mut slots: Vec<Slot> = Vec::with_capacity(self.code.len());
        let mut seen: Vec<&'static str> = Vec::new();
        let mut label = "";
        for instruction in self.code {
            match *instruction {
                Instruction::Label(name) => {
                    if seen.contains(&name) {
                        return Err(Fault::DuplicateLabel {
                            label: name.to_owned(),
                        });
                    }
                    seen.push(name);
                    label = name;
                }
                instruction => slots.push(Slot { instruction, label }),
            }
        }
        Ok(slots)
    }

    /// Hand-simulates the machine from its initial registers,
    /// recording every save and restore as an [`Event`]. The events
    /// are the trace the section's exercises read; recording them
    /// always keeps one honest entry point.
    ///
    /// # Errors
    /// Any [`Fault`] the run meets; see the variant list.
    pub fn run(&self, init: &[(&'static str, Value)]) -> Result<Run, Fault> {
        self.execute(init)
    }

    fn execute(&self, init: &[(&'static str, Value)]) -> Result<Run, Fault> {
        let slots = self.resolve()?;
        let mut cells: Vec<(&'static str, Value)> =
            self.registers.iter().map(|r| (*r, Value::Int(0))).collect();
        for (reg, value) in init {
            let cell = cells
                .iter_mut()
                .find(|(name, _)| name == reg)
                .ok_or_else(|| Fault::UnknownRegister {
                    reg: (*reg).to_owned(),
                    step: 0,
                })?;
            cell.1 = *value;
        }
        let mut pc = 0usize;
        let mut flag = false;
        let mut stack: Vec<(&'static str, Value, u64)> = Vec::new();
        let mut run = Run {
            registers: Vec::new(),
            events: Vec::new(),
            instructions: 0,
            pushes: 0,
            pops: 0,
            max_depth: 0,
        };
        while pc < slots.len() {
            if run.instructions >= MAX_STEPS {
                return Err(Fault::OutOfSteps {
                    step: run.instructions,
                });
            }
            let step = run.instructions;
            let slot = slots[pc];
            let mut next = pc + 1;
            match slot.instruction {
                Instruction::Label(_) => unreachable!("labels occupy no slot"),
                Instruction::Assign { reg, source } => {
                    let value = Self::evaluate(source, self.ops, &cells, step);
                    Self::store(reg, value?, &mut cells, step)?;
                }
                Instruction::Test { op, inputs } => {
                    let out = Self::apply(op, self.ops, inputs, &cells, step)?;
                    flag = out == Value::Bool(true);
                }
                Instruction::Branch(target) => {
                    if flag {
                        next = self.address(target, &slots, step)?;
                    }
                }
                Instruction::GotoLabel(target) => {
                    next = self.address(target, &slots, step)?;
                }
                Instruction::GotoReg(reg) => {
                    let Value::Label(target) = Self::fetch(reg, &cells, step)? else {
                        return Err(Fault::TypeMismatch { op: "goto", step });
                    };
                    next = self.address(target, &slots, step)?;
                }
                Instruction::Save(reg) => {
                    let value = Self::fetch(reg, &cells, step)?;
                    stack.push((reg, value, step));
                    run.pushes += 1;
                    run.max_depth = run.max_depth.max(stack.len());
                    run.events.push(Event::Save {
                        step,
                        label: slot.label,
                        reg,
                        value,
                        depth: stack.len(),
                    });
                }
                Instruction::Restore(reg) => {
                    let (value, matched) = Self::restore(reg, &mut stack, step)?;
                    Self::store(reg, value, &mut cells, step)?;
                    run.pops += 1;
                    let older = stack.iter().filter(|(name, _, _)| *name == reg).count();
                    run.events.push(Event::Restore {
                        step,
                        label: slot.label,
                        reg,
                        value,
                        depth: stack.len(),
                        matched,
                        age: step - matched,
                        older,
                    });
                }
            }
            pc = next;
            run.instructions += 1;
        }
        run.registers = cells;
        Ok(run)
    }

    /// Pops the stack into `reg`, with the checked discipline: the
    /// popped entry must carry the restored register's name. Returns
    /// the value and the step of the save it matches.
    fn restore(
        reg: &'static str,
        stack: &mut Vec<(&'static str, Value, u64)>,
        step: u64,
    ) -> Result<(Value, u64), Fault> {
        let Some((saved_reg, value, saved_step)) = stack.pop() else {
            return Err(Fault::StackUnderflow { reg, step });
        };
        if saved_reg != reg {
            return Err(Fault::MismatchedRestore {
                reg,
                found: saved_reg,
                step,
            });
        }
        Ok((value, saved_step))
    }

    fn evaluate(
        source: AssignSource,
        machine_ops: &[(&'static str, HandOp)],
        cells: &[(&'static str, Value)],
        step: u64,
    ) -> Result<Value, Fault> {
        match source {
            AssignSource::Reg(reg) => Self::fetch(reg, cells, step),
            AssignSource::Const(value) => Ok(value),
            AssignSource::Label(name) => Ok(Value::Label(name)),
            AssignSource::Op { name, inputs } => {
                Self::apply(name, machine_ops, inputs, cells, step)
            }
        }
    }

    fn apply(
        op: &'static str,
        machine_ops: &[(&'static str, HandOp)],
        inputs: &'static [Input],
        cells: &[(&'static str, Value)],
        step: u64,
    ) -> Result<Value, Fault> {
        let mut args: Vec<Value> = Vec::with_capacity(inputs.len());
        for input in inputs {
            let value = match *input {
                Input::Reg(reg) => Self::fetch(reg, cells, step)?,
                Input::Const(value) => value,
            };
            args.push(value);
        }
        let function = Self::lookup(op, machine_ops, step)?;
        function(&args)
    }

    /// Finds an operation: the machine's own table first, then the
    /// section's shared table.
    fn lookup(
        op: &'static str,
        machine_ops: &[(&'static str, HandOp)],
        step: u64,
    ) -> Result<HandOp, Fault> {
        if let Some((_, function)) = machine_ops.iter().find(|(name, _)| *name == op) {
            return Ok(*function);
        }
        hand_op(op).ok_or_else(|| Fault::UnknownOp {
            op: op.to_owned(),
            step,
        })
    }

    fn fetch(
        reg: &'static str,
        cells: &[(&'static str, Value)],
        step: u64,
    ) -> Result<Value, Fault> {
        cells
            .iter()
            .find(|(name, _)| *name == reg)
            .map(|(_, value)| *value)
            .ok_or_else(|| Fault::UnknownRegister {
                reg: reg.to_owned(),
                step,
            })
    }

    fn store(
        reg: &'static str,
        value: Value,
        cells: &mut [(&'static str, Value)],
        step: u64,
    ) -> Result<(), Fault> {
        let cell = cells
            .iter_mut()
            .find(|(name, _)| *name == reg)
            .ok_or_else(|| Fault::UnknownRegister {
                reg: reg.to_owned(),
                step,
            })?;
        cell.1 = value;
        Ok(())
    }

    /// The slot index a label names: the first instruction it
    /// dominates, or the stop address for a trailing label.
    fn address(&self, label: &str, slots: &[Slot], step: u64) -> Result<usize, Fault> {
        if let Some(pc) = slots.iter().position(|slot| slot.label == label) {
            return Ok(pc);
        }
        match self.code.last() {
            Some(Instruction::Label(name)) if *name == label => Ok(slots.len()),
            _ => Err(Fault::UnknownLabel {
                label: label.to_owned(),
                step,
            }),
        }
    }
}

fn two_args(op: &'static str, args: &[Value]) -> Result<(), Fault> {
    if args.len() == 2 {
        Ok(())
    } else {
        Err(Fault::BadArity {
            op,
            got: args.len(),
        })
    }
}

fn compare(
    op: &'static str,
    args: &[Value],
    int_pick: fn(i64, i64) -> bool,
    real_pick: fn(f64, f64) -> bool,
) -> Result<Value, Fault> {
    two_args(op, args)?;
    if let (Value::Int(a), Value::Int(b)) = (args[0], args[1]) {
        return Ok(Value::Bool(int_pick(a, b)));
    }
    let (Some(a), Some(b)) = (args[0].as_real(), args[1].as_real()) else {
        return Err(Fault::TypeMismatch { op, step: 0 });
    };
    Ok(Value::Bool(real_pick(a, b)))
}

/// The machine language's numeric equality: exact comparison of the
/// two real values is the definition, not an approximation to avoid.
fn numeric_eq(a: f64, b: f64) -> bool {
    #[allow(
        clippy::float_cmp,
        reason = "the machine language's = is exact numeric equality"
    )]
    let equal = a == b;
    equal
}

/// Folds two or more numbers, staying in the integers when every
/// argument is an integer and promoting to reals otherwise.
fn arith(
    op: &'static str,
    args: &[Value],
    int: fn(i64, i64) -> i64,
    real: fn(f64, f64) -> f64,
) -> Result<Value, Fault> {
    if args.len() < 2 {
        return Err(Fault::BadArity {
            op,
            got: args.len(),
        });
    }
    if args.iter().all(|v| matches!(v, Value::Int(_))) {
        let Value::Int(mut acc) = args[0] else {
            unreachable!("all integers checked above")
        };
        for value in &args[1..] {
            let Value::Int(n) = *value else {
                unreachable!("all integers checked above")
            };
            acc = int(acc, n);
        }
        return Ok(Value::Int(acc));
    }
    let mut acc = args[0]
        .as_real()
        .ok_or(Fault::TypeMismatch { op, step: 0 })?;
    for value in &args[1..] {
        let next = value.as_real().ok_or(Fault::TypeMismatch { op, step: 0 })?;
        acc = real(acc, next);
    }
    Ok(Value::Real(acc))
}

/// Renders the trace of save and restore events, the answer form of
/// exercise 5.5: one line per significant point, the stack rendered
/// after the event with its top on the left.
#[must_use]
pub fn render_trace(events: &[Event]) -> String {
    render(events, false)
}

/// Renders the trace with exercise 5.5a's annotations: each restore
/// line names the save it matches, the age of the value it returns,
/// and the older saves of the same register left beneath it.
#[must_use]
pub fn render_annotated_trace(events: &[Event]) -> String {
    render(events, true)
}

fn render(events: &[Event], annotate: bool) -> String {
    use std::fmt::Write as _;
    let mut text = String::new();
    let mut stack: Vec<(&'static str, Value)> = Vec::new();
    for event in events {
        match *event {
            Event::Save {
                step,
                label,
                reg,
                value,
                depth,
            } => {
                stack.push((reg, value));
                let _ = writeln!(
                    text,
                    "{}",
                    line(step, label, "save", reg, value, depth, &stack)
                );
            }
            Event::Restore {
                step,
                label,
                reg,
                value,
                depth,
                matched,
                age,
                older,
            } => {
                stack.pop();
                let mut line = line(step, label, "restore", reg, value, depth, &stack);
                if annotate {
                    let _ = write!(line, " matches save #{matched}, age {age}");
                    if older > 0 {
                        let _ = write!(line, ", {older} older save of {reg} beneath");
                    }
                }
                let _ = writeln!(text, "{line}");
            }
        }
    }
    text
}

fn line(
    step: u64,
    label: &'static str,
    kind: &str,
    reg: &'static str,
    value: Value,
    depth: usize,
    stack: &[(&'static str, Value)],
) -> String {
    let shown: Vec<String> = stack
        .iter()
        .rev()
        .map(|(name, value)| format!("{name}={value}"))
        .collect();
    format!(
        "{step:>4} {label:<13} {kind} {reg}={value}  stack [{joined}] depth {depth}",
        joined = shown.join(", ")
    )
}

fn divide(args: &[Value]) -> Result<Value, Fault> {
    two_args("/", args)?;
    let (Some(a), Some(b)) = (args[0].as_real(), args[1].as_real()) else {
        return Err(Fault::TypeMismatch { op: "/", step: 0 });
    };
    Ok(Value::Real(a / b))
}

fn abs(args: &[Value]) -> Result<Value, Fault> {
    if args.len() != 1 {
        return Err(Fault::BadArity {
            op: "abs",
            got: args.len(),
        });
    }
    match args[0] {
        Value::Int(n) => Ok(Value::Int(n.abs())),
        Value::Real(x) => Ok(Value::Real(x.abs())),
        _ => Err(Fault::TypeMismatch { op: "abs", step: 0 }),
    }
}

/// The iterative factorial machine of exercise 5.1: registers `n`,
/// `counter`, and `product`, operations `>`, `*`, and `+`, and no
/// stack. This is also the controller sequence exercise 5.2 asks for.
#[must_use]
pub fn factorial_iterative() -> Machine {
    Machine::new(
        &["n", "counter", "product"],
        &[],
        &[
            Instruction::Assign {
                reg: "product",
                source: AssignSource::Const(Value::Int(1)),
            },
            Instruction::Assign {
                reg: "counter",
                source: AssignSource::Const(Value::Int(1)),
            },
            Instruction::Label("test-counter"),
            Instruction::Test {
                op: ">",
                inputs: &[Input::Reg("counter"), Input::Reg("n")],
            },
            Instruction::Branch("factorial-done"),
            Instruction::Assign {
                reg: "product",
                source: AssignSource::Op {
                    name: "*",
                    inputs: &[Input::Reg("counter"), Input::Reg("product")],
                },
            },
            Instruction::Assign {
                reg: "counter",
                source: AssignSource::Op {
                    name: "+",
                    inputs: &[Input::Reg("counter"), Input::Const(Value::Int(1))],
                },
            },
            Instruction::GotoLabel("test-counter"),
            Instruction::Label("factorial-done"),
        ],
    )
}

/// The square-root machine of exercise 5.3, first stage: the machine
/// treats `good-enough?` and `improve` as primitive operations. The
/// tolerance and the averaging are exactly the book's 1.1.7
/// definitions; `improve` receives `guess` and `x`.
#[must_use]
pub fn sqrt_primitive() -> Machine {
    fn good_enough(args: &[Value]) -> Result<Value, Fault> {
        let (Some(g), Some(x)) = (args[0].as_real(), args[1].as_real()) else {
            return Err(Fault::TypeMismatch {
                op: "good-enough?",
                step: 0,
            });
        };
        Ok(Value::Bool((g * g - x).abs() < 0.001))
    }
    fn improve(args: &[Value]) -> Result<Value, Fault> {
        let (Some(g), Some(x)) = (args[0].as_real(), args[1].as_real()) else {
            return Err(Fault::TypeMismatch {
                op: "improve",
                step: 0,
            });
        };
        // The book's average, and the same (+) then (/) sequence the
        // expanded machine runs, so both stages agree bit for bit.
        #[allow(
            clippy::manual_midpoint,
            reason = "the book's average is literally (a + b) / 2"
        )]
        Ok(Value::Real((g + x / g) / 2.0))
    }
    Machine::new(
        &["x", "guess"],
        &[("good-enough?", good_enough), ("improve", improve)],
        &[
            Instruction::Assign {
                reg: "guess",
                source: AssignSource::Const(Value::Real(1.0)),
            },
            Instruction::Label("test-guess"),
            Instruction::Test {
                op: "good-enough?",
                inputs: &[Input::Reg("guess"), Input::Reg("x")],
            },
            Instruction::Branch("sqrt-done"),
            Instruction::Assign {
                reg: "guess",
                source: AssignSource::Op {
                    name: "improve",
                    inputs: &[Input::Reg("guess"), Input::Reg("x")],
                },
            },
            Instruction::GotoLabel("test-guess"),
            Instruction::Label("sqrt-done"),
        ],
    )
}

/// The square-root machine of exercise 5.3, second stage:
/// `good-enough?` and `improve` expand into the arithmetic
/// operations, with `t` holding each intermediate value.
#[must_use]
pub fn sqrt_expanded() -> Machine {
    Machine::new(
        &["x", "guess", "t"],
        &[],
        &[
            Instruction::Assign {
                reg: "guess",
                source: AssignSource::Const(Value::Real(1.0)),
            },
            Instruction::Label("test-guess"),
            Instruction::Assign {
                reg: "t",
                source: AssignSource::Op {
                    name: "*",
                    inputs: &[Input::Reg("guess"), Input::Reg("guess")],
                },
            },
            Instruction::Assign {
                reg: "t",
                source: AssignSource::Op {
                    name: "-",
                    inputs: &[Input::Reg("t"), Input::Reg("x")],
                },
            },
            Instruction::Assign {
                reg: "t",
                source: AssignSource::Op {
                    name: "abs",
                    inputs: &[Input::Reg("t")],
                },
            },
            Instruction::Test {
                op: "<",
                inputs: &[Input::Reg("t"), Input::Const(Value::Real(0.001))],
            },
            Instruction::Branch("sqrt-done"),
            Instruction::Assign {
                reg: "t",
                source: AssignSource::Op {
                    name: "/",
                    inputs: &[Input::Reg("x"), Input::Reg("guess")],
                },
            },
            Instruction::Assign {
                reg: "t",
                source: AssignSource::Op {
                    name: "+",
                    inputs: &[Input::Reg("guess"), Input::Reg("t")],
                },
            },
            Instruction::Assign {
                reg: "guess",
                source: AssignSource::Op {
                    name: "/",
                    inputs: &[Input::Reg("t"), Input::Const(Value::Real(2.0))],
                },
            },
            Instruction::GotoLabel("test-guess"),
            Instruction::Label("sqrt-done"),
        ],
    )
}

/// The recursive exponentiation machine of exercise 5.4a: the save
/// and restore discipline of Figure 5.11 with the multiplication
/// `b * val` after the recursive call returns.
#[must_use]
pub fn expt_recursive() -> Machine {
    Machine::new(
        &["b", "n", "val", "continue"],
        &[],
        &[
            Instruction::Assign {
                reg: "continue",
                source: AssignSource::Label("expt-done"),
            },
            Instruction::Label("expt-loop"),
            Instruction::Test {
                op: "=",
                inputs: &[Input::Reg("n"), Input::Const(Value::Int(0))],
            },
            Instruction::Branch("base-case"),
            Instruction::Save("continue"),
            Instruction::Save("n"),
            Instruction::Assign {
                reg: "n",
                source: AssignSource::Op {
                    name: "-",
                    inputs: &[Input::Reg("n"), Input::Const(Value::Int(1))],
                },
            },
            Instruction::Assign {
                reg: "continue",
                source: AssignSource::Label("after-expt"),
            },
            Instruction::GotoLabel("expt-loop"),
            Instruction::Label("after-expt"),
            Instruction::Restore("n"),
            Instruction::Restore("continue"),
            Instruction::Assign {
                reg: "val",
                source: AssignSource::Op {
                    name: "*",
                    inputs: &[Input::Reg("b"), Input::Reg("val")],
                },
            },
            Instruction::GotoReg("continue"),
            Instruction::Label("base-case"),
            Instruction::Assign {
                reg: "val",
                source: AssignSource::Const(Value::Int(1)),
            },
            Instruction::GotoReg("continue"),
            Instruction::Label("expt-done"),
        ],
    )
}

/// The iterative exponentiation machine of exercise 5.4b: registers
/// `b`, `counter`, and `product`, and no stack.
#[must_use]
pub fn expt_iterative() -> Machine {
    Machine::new(
        &["b", "n", "counter", "product"],
        &[],
        &[
            Instruction::Assign {
                reg: "counter",
                source: AssignSource::Reg("n"),
            },
            Instruction::Assign {
                reg: "product",
                source: AssignSource::Const(Value::Int(1)),
            },
            Instruction::Label("expt-iter"),
            Instruction::Test {
                op: "=",
                inputs: &[Input::Reg("counter"), Input::Const(Value::Int(0))],
            },
            Instruction::Branch("expt-done"),
            Instruction::Assign {
                reg: "product",
                source: AssignSource::Op {
                    name: "*",
                    inputs: &[Input::Reg("b"), Input::Reg("product")],
                },
            },
            Instruction::Assign {
                reg: "counter",
                source: AssignSource::Op {
                    name: "-",
                    inputs: &[Input::Reg("counter"), Input::Const(Value::Int(1))],
                },
            },
            Instruction::GotoLabel("expt-iter"),
            Instruction::Label("expt-done"),
        ],
    )
}

/// The recursive factorial machine of Figure 5.11: registers `n`,
/// `val`, and `continue`, with `continue` and `n` saved before each
/// recursive call and restored after it.
#[must_use]
pub fn factorial_recursive() -> Machine {
    Machine::new(
        &["n", "val", "continue"],
        &[],
        &[
            Instruction::Assign {
                reg: "continue",
                source: AssignSource::Label("fact-done"),
            },
            Instruction::Label("fact-loop"),
            Instruction::Test {
                op: "=",
                inputs: &[Input::Reg("n"), Input::Const(Value::Int(1))],
            },
            Instruction::Branch("base-case"),
            Instruction::Save("continue"),
            Instruction::Save("n"),
            Instruction::Assign {
                reg: "n",
                source: AssignSource::Op {
                    name: "-",
                    inputs: &[Input::Reg("n"), Input::Const(Value::Int(1))],
                },
            },
            Instruction::Assign {
                reg: "continue",
                source: AssignSource::Label("after-fact"),
            },
            Instruction::GotoLabel("fact-loop"),
            Instruction::Label("after-fact"),
            Instruction::Restore("n"),
            Instruction::Restore("continue"),
            Instruction::Assign {
                reg: "val",
                source: AssignSource::Op {
                    name: "*",
                    inputs: &[Input::Reg("n"), Input::Reg("val")],
                },
            },
            Instruction::GotoReg("continue"),
            Instruction::Label("base-case"),
            Instruction::Assign {
                reg: "val",
                source: AssignSource::Const(Value::Int(1)),
            },
            Instruction::GotoReg("continue"),
            Instruction::Label("fact-done"),
        ],
    )
}

/// The Fibonacci machine of Figure 5.12: two recursive calls per
/// level, the second entered from `afterfib-n-1` with `val` saved.
#[must_use]
pub fn fibonacci() -> Machine {
    Machine::new(
        &["n", "val", "continue"],
        &[],
        &[
            Instruction::Assign {
                reg: "continue",
                source: AssignSource::Label("fib-done"),
            },
            Instruction::Label("fib-loop"),
            Instruction::Test {
                op: "<",
                inputs: &[Input::Reg("n"), Input::Const(Value::Int(2))],
            },
            Instruction::Branch("immediate-answer"),
            // set up to compute Fib(n - 1)
            Instruction::Save("continue"),
            Instruction::Assign {
                reg: "continue",
                source: AssignSource::Label("afterfib-n-1"),
            },
            Instruction::Save("n"),
            Instruction::Assign {
                reg: "n",
                source: AssignSource::Op {
                    name: "-",
                    inputs: &[Input::Reg("n"), Input::Const(Value::Int(1))],
                },
            },
            Instruction::GotoLabel("fib-loop"),
            Instruction::Label("afterfib-n-1"),
            Instruction::Restore("n"),
            Instruction::Restore("continue"),
            // set up to compute Fib(n - 2)
            Instruction::Assign {
                reg: "n",
                source: AssignSource::Op {
                    name: "-",
                    inputs: &[Input::Reg("n"), Input::Const(Value::Int(2))],
                },
            },
            Instruction::Save("continue"),
            Instruction::Assign {
                reg: "continue",
                source: AssignSource::Label("afterfib-n-2"),
            },
            Instruction::Save("val"),
            Instruction::GotoLabel("fib-loop"),
            Instruction::Label("afterfib-n-2"),
            Instruction::Assign {
                reg: "n",
                source: AssignSource::Reg("val"),
            },
            Instruction::Restore("val"),
            Instruction::Restore("continue"),
            Instruction::Assign {
                reg: "val",
                source: AssignSource::Op {
                    name: "+",
                    inputs: &[Input::Reg("val"), Input::Reg("n")],
                },
            },
            Instruction::GotoReg("continue"),
            Instruction::Label("immediate-answer"),
            Instruction::Assign {
                reg: "val",
                source: AssignSource::Reg("n"),
            },
            Instruction::GotoReg("continue"),
            Instruction::Label("fib-done"),
        ],
    )
}

/// The Fibonacci machine with exercise 5.6's answer applied: the
/// `(restore continue)` at `afterfib-n-1` and the `(save continue)`
/// before the second recursive call are removed, because the restore
/// returns the caller's return label only for the very next save to
/// push it back unchanged. The trailing label names the exit.
#[must_use]
pub fn fibonacci_without_redundant_pair() -> Machine {
    Machine::new(
        &["n", "val", "continue"],
        &[],
        &[
            Instruction::Assign {
                reg: "continue",
                source: AssignSource::Label("fib-done"),
            },
            Instruction::Label("fib-loop"),
            Instruction::Test {
                op: "<",
                inputs: &[Input::Reg("n"), Input::Const(Value::Int(2))],
            },
            Instruction::Branch("immediate-answer"),
            Instruction::Save("continue"),
            Instruction::Assign {
                reg: "continue",
                source: AssignSource::Label("afterfib-n-1"),
            },
            Instruction::Save("n"),
            Instruction::Assign {
                reg: "n",
                source: AssignSource::Op {
                    name: "-",
                    inputs: &[Input::Reg("n"), Input::Const(Value::Int(1))],
                },
            },
            Instruction::GotoLabel("fib-loop"),
            Instruction::Label("afterfib-n-1"),
            Instruction::Restore("n"),
            Instruction::Assign {
                reg: "n",
                source: AssignSource::Op {
                    name: "-",
                    inputs: &[Input::Reg("n"), Input::Const(Value::Int(2))],
                },
            },
            Instruction::Assign {
                reg: "continue",
                source: AssignSource::Label("afterfib-n-2"),
            },
            Instruction::Save("val"),
            Instruction::GotoLabel("fib-loop"),
            Instruction::Label("afterfib-n-2"),
            Instruction::Assign {
                reg: "n",
                source: AssignSource::Reg("val"),
            },
            Instruction::Restore("val"),
            Instruction::Restore("continue"),
            Instruction::Assign {
                reg: "val",
                source: AssignSource::Op {
                    name: "+",
                    inputs: &[Input::Reg("val"), Input::Reg("n")],
                },
            },
            Instruction::GotoReg("continue"),
            Instruction::Label("immediate-answer"),
            Instruction::Assign {
                reg: "val",
                source: AssignSource::Reg("n"),
            },
            Instruction::GotoReg("continue"),
            Instruction::Label("fib-done"),
        ],
    )
}
