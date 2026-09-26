// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 5.2

//! Section 5.2: A register-machine simulator.
//!
//! This module builds the real simulator on the 5.1 transcription
//! model's instruction language. Where 5.1 stepped hand-transcribed
//! [`Instruction`](crate::sec_5_1::Instruction) data, a machine here is
//! constructed the book's way: controller text in the book's notation
//! is read into datums, the assembler scans the labels out
//! ([`extract_labels`]) and builds one execution procedure per typed
//! instruction, and the machine runs the resolved sequence with a
//! program counter, a test flag, a monitored stack, and the operation
//! table. The machine type is deliberately left open: 5.3's machines
//! will plug in compound operations the way 5.1's `good-enough?` did,
//! and the explicit-control evaluator of 5.4 reuses this simulator
//! whole.
//!
//! The execution procedures are closures built at assembly time
//! (`make_assign`, `make_test`, ...), the direct shape of the book's
//! `make-assign` and friends: an instruction's syntax is analyzed
//! once, and each execution carries only what it needs. Every
//! executed instruction counts ([`Machine::instruction_count`]); the
//! monitoring extensions of 5.2.4 and the exercises ride the same
//! machine: stack statistics, an instruction-count budget
//! ([`Fault::BudgetExceeded`]), instruction tracing, traced labels,
//! traced registers, and breakpoints.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::rc::Rc;

use sicp_runtime::{Value, display_value, read_program};

/// One machine operation: the arguments in, one value or fault, with
/// access to the machine (the stack statistics and print operations
/// need it).
pub type OpHandler = Rc<dyn Fn(&mut Machine, &[Value]) -> Result<Value, Fault>>;

/// Why a simulated machine stops early, at assembly or at run time.
#[derive(Clone, Debug, PartialEq)]
pub enum Fault {
    /// The controller text was not readable as Scheme datums.
    Parse(String),
    /// An instruction named a register outside the machine's table.
    UnknownRegister {
        /// The unknown register name.
        reg: String,
    },
    /// An instruction named an operation outside the machine's table.
    UnknownOperation {
        /// The unknown operation name.
        op: String,
    },
    /// A branch, goto, or label source named an unknown label.
    UnknownLabel {
        /// The unknown label name.
        label: String,
    },
    /// Two label lines in one controller share a name (exercise 5.8).
    DuplicateLabel {
        /// The repeated label name.
        label: String,
    },
    /// Two registers of one machine share a name.
    DuplicateRegister {
        /// The repeated register name.
        reg: String,
    },
    /// An instruction was not one of the seven types, or its parts
    /// had the wrong shape.
    BadInstruction {
        /// The instruction's rendered text.
        text: String,
    },
    /// An operation input was written `(label ...)`: the operand
    /// grammar of exercise 5.9 admits registers and constants only.
    LabelOperand {
        /// The operation the input reached.
        op: String,
        /// The refused label.
        label: String,
    },
    /// A branch executed before any test set the flag: the flag
    /// starts at `*unassigned*` and only a `test` writes it.
    BranchWithoutTest {
        /// The instruction count at the branch.
        step: u64,
    },
    /// A restore reached past the bottom of the stack.
    StackUnderflow {
        /// The restored register.
        reg: String,
        /// The instruction count at the restore.
        step: u64,
    },
    /// A restore under the tagged discipline popped an entry saved
    /// from a different register (exercise 5.11b).
    MismatchedRestore {
        /// The register the restore names.
        reg: String,
        /// The register the popped entry was saved from.
        saved: String,
        /// The instruction count at the restore.
        step: u64,
    },
    /// A `(goto (reg ...))` reached a register that did not hold a
    /// label value.
    GotoNonLabel {
        /// The jumped register.
        reg: String,
        /// The register's rendered contents.
        value: String,
        /// The instruction count at the goto.
        step: u64,
    },
    /// A machine operation rejected its inputs.
    Op {
        /// The operation name.
        op: String,
        /// What the operation objected to.
        message: String,
        /// The instruction count at the call.
        step: u64,
    },
    /// The run reached the instruction budget of exercise 5.15a: the
    /// count equals the budget and one more instruction was due.
    BudgetExceeded {
        /// The instruction count when the machine halted.
        count: u64,
        /// The program counter of the instruction that would have run.
        pc: usize,
    },
    /// A breakpoint was set where no instruction is: past the end of
    /// the controller sequence, or at offset zero.
    BadBreakpoint {
        /// The breakpointed label.
        label: String,
        /// The refused offset.
        n: usize,
    },
}

impl std::fmt::Display for Fault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(message) => write!(f, "parse error: {message}"),
            Self::UnknownRegister { reg } => write!(f, "unknown register {reg:?}"),
            Self::UnknownOperation { op } => write!(f, "unknown operation {op:?}"),
            Self::UnknownLabel { label } => write!(f, "unknown label {label:?}"),
            Self::DuplicateLabel { label } => write!(f, "the label {label:?} is used twice"),
            Self::DuplicateRegister { reg } => {
                write!(f, "multiply defined register {reg:?}")
            }
            Self::BadInstruction { text } => write!(f, "bad instruction {text:?}"),
            Self::LabelOperand { op, label } => write!(
                f,
                "an operation input of {op:?} is a register or a constant, \
                 not the label {label:?}"
            ),
            Self::BranchWithoutTest { step } => {
                write!(f, "branch at step {step} before any test set the flag")
            }
            Self::StackUnderflow { reg, step } => {
                write!(f, "restore {reg} with an empty stack at step {step}")
            }
            Self::MismatchedRestore { reg, saved, step } => {
                write!(
                    f,
                    "restore {reg} but the stack holds {saved} at step {step}"
                )
            }
            Self::GotoNonLabel { reg, value, step } => {
                write!(
                    f,
                    "goto (reg {reg}) with the non-label value {value} at step {step}"
                )
            }
            Self::Op { op, message, step } => {
                write!(f, "operation {op:?} failed: {message} at step {step}")
            }
            Self::BudgetExceeded { count, pc } => {
                write!(f, "instruction count {count} reached the budget at pc {pc}")
            }
            Self::BadBreakpoint { label, n } => {
                write!(f, "no instruction at {label} offset {n}")
            }
        }
    }
}

impl std::error::Error for Fault {}

/// Why [`Machine::start`] or [`Machine::proceed`] stopped.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Stop {
    /// The program counter ran past the last instruction.
    End,
    /// The machine is holding at a breakpoint, named by its
    /// dominating label and the 1-based offset after it.
    Breakpoint {
        /// The breakpoint's dominating label.
        label: String,
        /// The 1-based instruction offset after the label.
        offset: usize,
    },
}

/// Which restore discipline the stack follows (exercise 5.11). The
/// book's base simulator pops whatever is on the stack into the named
/// register; the exercise's tagged and per-register disciplines are
/// the typed alternatives.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RestoreDiscipline {
    /// Pop the top entry into the named register, whatever it is.
    #[default]
    Plain,
    /// Pop the top entry and fault unless it was saved from the
    /// restored register.
    Tagged,
    /// One stack per register; a restore pops that register's stack.
    PerRegister,
}

/// The instruction-use summary the assembler collects (exercise 5.12):
/// the instruction types in use with their distinct instruction
/// counts, the registers used as entry points, and the sources each
/// register is assigned from.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct InstructionUse {
    /// Instruction type name to the count of its distinct
    /// instruction texts.
    pub counts: BTreeMap<String, usize>,
    /// Registers named by `(goto (reg ...))`.
    pub entry_points: BTreeSet<String>,
    /// Registers named by `save` or `restore`.
    pub stack_registers: BTreeSet<String>,
    /// Register name to the set of distinct assign sources it
    /// receives, rendered in the book's notation.
    pub sources: BTreeMap<String, BTreeSet<String>>,
}

/// One scanned instruction: its rendered text, its datum, and the
/// labels that immediately precede it (retained for exercise 5.17).
#[derive(Clone, Debug)]
pub struct ScannedInstruction {
    /// The instruction rendered back into the book's notation.
    pub text: String,
    /// The instruction datum as read.
    pub datum: Value,
    /// The labels between this instruction and its predecessor.
    pub labels: Vec<String>,
}

/// A scanned controller: the instructions in order and each label
/// with the index of the instruction it dominates, the assembler's
/// two results from [`extract_labels`].
#[derive(Clone, Debug, Default)]
pub struct ScannedController {
    /// The instructions in controller order.
    pub insts: Vec<ScannedInstruction>,
    /// Each label with the instruction index it names; a trailing
    /// label names one past the last instruction, the section's exit.
    pub labels: Vec<(String, usize)>,
}

/// One assembled instruction: its text, its dominating label with the
/// label's own index (for breakpoint reports), the labels that
/// immediately precede it (for the traced labels of exercise 5.17),
/// and the execution procedure.
struct Slot {
    text: String,
    dominating: Option<(String, usize)>,
    labels: Vec<String>,
    exec: Exec,
}

/// The execution-procedure type: one step over the machine.
type Exec = Rc<dyn Fn(&mut Machine) -> Result<(), Fault>>;

/// The book's `make-new-machine` plus everything `make-machine` and
/// the monitoring exercises install: registers, operations, the
/// assembled instruction sequence with resolved labels, the monitored
/// stack, and the counters.
pub struct Machine {
    pc: usize,
    flag: Value,
    registers: HashMap<String, Value>,
    ops: HashMap<String, OpHandler>,
    insts: Vec<Slot>,
    labels: HashMap<String, usize>,
    stack: Vec<(String, Value)>,
    per_register: BTreeMap<String, Vec<Value>>,
    discipline: RestoreDiscipline,
    depth: u64,
    pushes: u64,
    max_depth: u64,
    instruction_count: u64,
    budget: Option<u64>,
    trace: bool,
    trace_labels: bool,
    traced_registers: BTreeSet<String>,
    breakpoints: BTreeSet<usize>,
    held: Option<usize>,
    transcript: Vec<String>,
    use_summary: InstructionUse,
}

impl std::fmt::Debug for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Machine")
            .field("pc", &self.pc)
            .field("flag", &self.flag)
            .field("registers", &self.registers)
            .field("instruction_count", &self.instruction_count)
            .field("pushes", &self.pushes)
            .field("max_depth", &self.max_depth)
            .finish_non_exhaustive()
    }
}

/// The book's `make-machine`: allocates the named registers,
/// installs the operations, and assembles the controller text.
///
/// The machine's own operations, `initialize-stack`,
/// `print-stack-statistics`, and `print`, are installed first; a user
/// operation of the same name overrides them. An unknown register,
/// operation, or label, a duplicated register or label, or a
/// malformed instruction fails before the machine can start.
///
/// # Errors
/// [`Fault::Parse`] when the controller is unreadable, plus the
/// assembly faults of [`Fault`] when a name or instruction shape is
/// wrong.
pub fn make_machine(
    register_names: &[&str],
    operations: &[(&str, OpHandler)],
    controller: &str,
) -> Result<Machine, Fault> {
    make_machine_from_datums(register_names, operations, &parse_controller(controller)?)
}

/// The seam of exercise 5.10: assembles a controller some other
/// syntax has already translated into the book's datums, so a new
/// surface syntax needs no change to the assembler or the machine.
///
/// # Errors
/// As [`make_machine`].
pub fn make_machine_from_datums(
    register_names: &[&str],
    operations: &[(&str, OpHandler)],
    datums: &[Value],
) -> Result<Machine, Fault> {
    let mut machine = make_new_machine(register_names, operations)?;
    machine.install_program(datums)?;
    Ok(machine)
}

/// The variant of exercise 5.13: derives the register set from the
/// controller text instead of requiring a register list.
///
/// The scan names every register the controller mentions: assign
/// targets, `(reg ...)` sources and inputs, save and restore names,
/// and `(goto (reg ...))` entry points. The machine's own `flag` is
/// never named by a controller and so is never derived.
///
/// # Errors
/// As [`make_machine`], plus [`Fault::Parse`] when the controller is
/// unreadable.
pub fn make_machine_with_derived_registers(
    operations: &[(&str, OpHandler)],
    controller: &str,
) -> Result<Machine, Fault> {
    let datums = parse_controller(controller)?;
    let names: Vec<String> = registers_in(&datums).into_iter().collect();
    let derived: Vec<&str> = names.iter().map(String::as_str).collect();
    make_machine_from_datums(&derived, operations, &datums)
}

/// The book's `make-new-machine`: the container every machine starts
/// from, with the machine operations of 5.2.4 installed.
fn make_new_machine(
    register_names: &[&str],
    operations: &[(&str, OpHandler)],
) -> Result<Machine, Fault> {
    let mut machine = Machine {
        pc: 0,
        flag: Value::sym("*unassigned*"),
        registers: HashMap::new(),
        ops: HashMap::new(),
        insts: Vec::new(),
        labels: HashMap::new(),
        stack: Vec::new(),
        per_register: BTreeMap::new(),
        discipline: RestoreDiscipline::Plain,
        depth: 0,
        pushes: 0,
        max_depth: 0,
        instruction_count: 0,
        budget: None,
        trace: false,
        trace_labels: false,
        traced_registers: BTreeSet::new(),
        breakpoints: BTreeSet::new(),
        held: None,
        transcript: Vec::new(),
        use_summary: InstructionUse::default(),
    };
    machine.allocate_registers(register_names)?;
    machine.ops.insert(
        "initialize-stack".to_owned(),
        Rc::new(|machine, _| {
            machine.initialize_stack();
            Ok(Value::sym("done"))
        }),
    );
    machine.ops.insert(
        "print-stack-statistics".to_owned(),
        Rc::new(|machine, _| {
            machine.print_stack_statistics();
            Ok(Value::sym("done"))
        }),
    );
    machine.ops.insert(
        "print-instruction-count".to_owned(),
        Rc::new(|machine, _| {
            machine.print_instruction_count();
            Ok(Value::sym("done"))
        }),
    );
    machine.ops.insert(
        "print".to_owned(),
        Rc::new(|machine, args| {
            let text = args
                .first()
                .map_or_else(|| "(unspecified)".to_owned(), display_value);
            machine.transcript.push(text);
            Ok(Value::sym("done"))
        }),
    );
    for (name, handler) in operations {
        machine.ops.insert((*name).to_owned(), Rc::clone(handler));
    }
    Ok(machine)
}

impl Machine {
    /// The book's `allocate-register`: adds the named registers,
    /// each initialized to `*unassigned*`; a repeated name is a
    /// defect of the machine description.
    ///
    /// # Errors
    /// [`Fault::DuplicateRegister`] when a name is already taken.
    pub fn allocate_registers(&mut self, names: &[&str]) -> Result<(), Fault> {
        for name in names {
            if self.registers.contains_key(*name) {
                return Err(Fault::DuplicateRegister {
                    reg: (*name).to_owned(),
                });
            }
            self.registers
                .insert((*name).to_owned(), Value::sym("*unassigned*"));
        }
        Ok(())
    }

    /// Scans, resolves, and installs the controller: the book's
    /// `assemble` with `update-insts!` folded in.
    fn install_program(&mut self, datums: &[Value]) -> Result<(), Fault> {
        let scanned = extract_labels(datums)?;
        let mut labels = HashMap::new();
        for (name, index) in &scanned.labels {
            labels.insert(name.clone(), *index);
        }
        let register_names: HashSet<String> = self.registers.keys().cloned().collect();
        let mut insts = Vec::with_capacity(scanned.insts.len());
        let mut dominating: Option<(String, usize)> = None;
        let mut next_label = 0;
        for (index, scanned_inst) in scanned.insts.iter().enumerate() {
            while let Some((name, at)) = scanned.labels.get(next_label) {
                if *at > index {
                    break;
                }
                dominating = Some((name.clone(), *at));
                next_label += 1;
            }
            let exec =
                make_execution_procedure(&scanned_inst.datum, &labels, &self.ops, &register_names)?;
            insts.push(Slot {
                text: scanned_inst.text.clone(),
                dominating: dominating.clone(),
                labels: scanned_inst.labels.clone(),
                exec,
            });
        }
        self.insts = insts;
        self.labels = labels;
        self.use_summary = summarize(&scanned);
        Ok(())
    }

    /// The book's `get-register-contents`: reads a register's
    /// contents.
    ///
    /// # Errors
    /// [`Fault::UnknownRegister`] when the machine has no such
    /// register.
    pub fn get_register(&self, name: &str) -> Result<Value, Fault> {
        self.registers
            .get(name)
            .cloned()
            .ok_or_else(|| Fault::UnknownRegister {
                reg: name.to_owned(),
            })
    }

    /// The book's `set-register-contents!`: stores a value through
    /// the single store path of exercise 5.18, so a traced register
    /// reports the change.
    ///
    /// # Errors
    /// [`Fault::UnknownRegister`] when the machine has no such
    /// register.
    pub fn set_register(&mut self, name: &str, value: Value) -> Result<(), Fault> {
        self.store(name, value)
    }

    /// The store itself: traces the change when the register is
    /// traced, then writes.
    fn store(&mut self, name: &str, value: Value) -> Result<(), Fault> {
        let old = self.get_register(name)?;
        if self.traced_registers.contains(name) {
            let line = format!(
                "{name}: {} -> {}",
                display_value(&old),
                display_value(&value)
            );
            self.transcript.push(line);
        }
        self.registers.insert(name.to_owned(), value);
        Ok(())
    }

    /// The book's `start`: sets the program counter to the beginning
    /// and executes until the sequence ends or a breakpoint is
    /// reached.
    ///
    /// # Errors
    /// Any run-time [`Fault`] an instruction or operation raises.
    pub fn start(&mut self) -> Result<Stop, Fault> {
        self.pc = 0;
        self.held = None;
        self.execute()
    }

    /// The book's `proceed-machine`: resumes execution from where the
    /// machine is holding, without re-stopping at the breakpoint it
    /// is holding at.
    ///
    /// # Errors
    /// As [`Machine::start`].
    pub fn proceed(&mut self) -> Result<Stop, Fault> {
        self.execute()
    }

    /// The book's `execute`: the machine's driver loop, one
    /// instruction execution procedure per step, counting every
    /// executed instruction.
    fn execute(&mut self) -> Result<Stop, Fault> {
        loop {
            if self.pc >= self.insts.len() {
                return Ok(Stop::End);
            }
            if self.at_breakpoint() {
                let (label, offset) = self.breakpoint_report();
                let line = format!("breakpoint at {label}: {offset}");
                self.transcript.push(line);
                self.held = Some(self.pc);
                return Ok(Stop::Breakpoint { label, offset });
            }
            if let Some(budget) = self.budget
                && self.instruction_count >= budget
            {
                return Err(Fault::BudgetExceeded {
                    count: self.instruction_count,
                    pc: self.pc,
                });
            }
            self.instruction_count += 1;
            self.trace_step();
            self.held = None;
            let exec = Rc::clone(&self.insts[self.pc].exec);
            exec(self)?;
        }
    }

    /// Whether the machine is about to execute a breakpointed
    /// instruction it is not already holding at.
    fn at_breakpoint(&self) -> bool {
        self.breakpoints.contains(&self.pc) && self.held != Some(self.pc)
    }

    /// The label and 1-based offset of the instruction at the
    /// program counter.
    fn breakpoint_report(&self) -> (String, usize) {
        match &self.insts[self.pc].dominating {
            Some((label, index)) => (label.clone(), self.pc - index + 1),
            None => (String::new(), self.pc + 1),
        }
    }

    /// Prints the traced labels and the instruction text of the step
    /// about to execute.
    fn trace_step(&mut self) {
        if !self.trace {
            return;
        }
        if self.trace_labels {
            for label in self.insts[self.pc].labels.clone() {
                let line = format!("{label}:");
                self.transcript.push(line);
            }
        }
        let text = self.insts[self.pc].text.clone();
        self.transcript.push(text);
    }

    /// The book's `advance-pc`: the next instruction in sequence.
    fn advance_pc(&mut self) {
        self.pc += 1;
    }

    /// Reads a register for an instruction source.
    fn read(&self, name: &str) -> Result<Value, Fault> {
        self.get_register(name)
    }

    /// Pushes one `(register, value)` entry by the current
    /// discipline, updating the counters.
    fn push(&mut self, reg: &str, value: Value) {
        match self.discipline {
            RestoreDiscipline::PerRegister => {
                self.per_register
                    .entry(reg.to_owned())
                    .or_default()
                    .push(value);
            }
            _ => self.stack.push((reg.to_owned(), value)),
        }
        self.depth += 1;
        self.pushes += 1;
        if self.depth > self.max_depth {
            self.max_depth = self.depth;
        }
    }

    /// Pops one entry by the current discipline: the entry saved from
    /// `reg`, or a typed fault.
    fn pop(&mut self, reg: &str) -> Result<(String, Value), Fault> {
        let step = self.instruction_count;
        let popped = match self.discipline {
            RestoreDiscipline::PerRegister => self
                .per_register
                .get_mut(reg)
                .and_then(Vec::pop)
                .map(|value| (reg.to_owned(), value)),
            _ => self.stack.pop(),
        };
        self.depth = self.depth.saturating_sub(1);
        popped.ok_or_else(|| Fault::StackUnderflow {
            reg: reg.to_owned(),
            step,
        })
    }

    /// The book's stack `initialize`: empties the stack and its
    /// counters.
    fn initialize_stack(&mut self) {
        self.stack.clear();
        self.per_register.clear();
        self.depth = 0;
        self.pushes = 0;
        self.max_depth = 0;
    }

    /// The machine's stack counters `(total-pushes, maximum-depth)`,
    /// of exercise 5.14.
    #[must_use]
    pub fn stack_statistics(&self) -> (u64, u64) {
        (self.pushes, self.max_depth)
    }

    /// The book's `print-stack-statistics` message: appends one
    /// `(total-pushes = ... maximum-depth = ...)` line to the
    /// transcript.
    pub fn print_stack_statistics(&mut self) {
        let line = format!(
            "(total-pushes = {} maximum-depth = {})",
            self.pushes, self.max_depth
        );
        self.transcript.push(line);
    }

    /// The instruction count of exercise 5.15: every executed
    /// instruction, transfers included.
    #[must_use]
    pub fn instruction_count(&self) -> u64 {
        self.instruction_count
    }

    /// The book's `print-instruction-count` message: appends the
    /// count to the transcript and resets it to zero, returning the
    /// count it printed.
    pub fn print_instruction_count(&mut self) -> u64 {
        let count = self.instruction_count;
        self.transcript.push(count.to_string());
        self.instruction_count = 0;
        count
    }

    /// The budget of exercise 5.15a: the largest instruction count a
    /// run may reach; `None` runs unbounded. A run due to execute one
    /// more instruction past the budget halts with
    /// [`Fault::BudgetExceeded`], carrying the count and the program
    /// counter.
    pub fn set_instruction_budget(&mut self, budget: Option<u64>) {
        self.budget = budget;
    }

    /// The `trace-on` and `trace-off` messages of exercise 5.16:
    /// before each executed instruction, its text.
    pub fn set_trace(&mut self, on: bool) {
        self.trace = on;
    }

    /// The extension of exercise 5.17: when tracing, print the
    /// labels that immediately precede each executed instruction
    /// before the instruction itself.
    pub fn set_label_trace(&mut self, on: bool) {
        self.trace_labels = on;
    }

    /// Traces one named register (exercise 5.18): every store
    /// reports `name: old -> new`.
    ///
    /// # Errors
    /// [`Fault::UnknownRegister`] when the machine has no such
    /// register.
    pub fn trace_register(&mut self, name: &str) -> Result<(), Fault> {
        self.get_register(name)?;
        self.traced_registers.insert(name.to_owned());
        Ok(())
    }

    /// Stops tracing one named register (exercise 5.18).
    ///
    /// # Errors
    /// [`Fault::UnknownRegister`] when the machine has no such
    /// register.
    pub fn untrace_register(&mut self, name: &str) -> Result<(), Fault> {
        self.get_register(name)?;
        self.traced_registers.remove(name);
        Ok(())
    }

    /// Installs the save-and-restore discipline of exercise 5.11.
    /// Changing the discipline leaves any saved entries stranded; set
    /// it before the machine starts.
    pub fn set_restore_discipline(&mut self, discipline: RestoreDiscipline) {
        self.discipline = discipline;
    }

    /// The book's `set-breakpoint`: stops the machine just before the
    /// `n`th instruction (1-based) after `label`, printing the label
    /// and offset each time it is reached.
    ///
    /// # Errors
    /// [`Fault::UnknownLabel`] when the label is unknown,
    /// [`Fault::BadBreakpoint`] when the offset names no instruction.
    pub fn set_breakpoint(&mut self, label: &str, n: usize) -> Result<(), Fault> {
        let target = self.breakpoint_target(label, n)?;
        self.breakpoints.insert(target);
        Ok(())
    }

    /// The book's `cancel-breakpoint`: removes one breakpoint.
    ///
    /// # Errors
    /// As [`Machine::set_breakpoint`].
    pub fn cancel_breakpoint(&mut self, label: &str, n: usize) -> Result<(), Fault> {
        let target = self.breakpoint_target(label, n)?;
        self.breakpoints.remove(&target);
        if self.held == Some(target) {
            self.held = None;
        }
        Ok(())
    }

    /// The book's `cancel-all-breakpoints`.
    pub fn cancel_all_breakpoints(&mut self) {
        self.breakpoints.clear();
        self.held = None;
    }

    /// Resolves a `(label, offset)` pair to an instruction index.
    fn breakpoint_target(&self, label: &str, n: usize) -> Result<usize, Fault> {
        let index = *self.labels.get(label).ok_or_else(|| Fault::UnknownLabel {
            label: label.to_owned(),
        })?;
        let target = index + n.saturating_sub(1);
        if n == 0 || target >= self.insts.len() {
            return Err(Fault::BadBreakpoint {
                label: label.to_owned(),
                n,
            });
        }
        Ok(target)
    }

    /// The lines the machine has printed: trace lines, register-trace
    /// lines, stack statistics, instruction counts, `print` outputs,
    /// and breakpoint reports, in order.
    #[must_use]
    pub fn transcript(&self) -> &[String] {
        &self.transcript
    }

    /// The instruction-use summary of exercise 5.12.
    #[must_use]
    pub fn instruction_use(&self) -> &InstructionUse {
        &self.use_summary
    }

    /// The rendered text of every assembled instruction, in
    /// controller order.
    #[must_use]
    pub fn instruction_texts(&self) -> Vec<String> {
        self.insts.iter().map(|slot| slot.text.clone()).collect()
    }

    /// Each label with the instruction index it names.
    #[must_use]
    pub fn labels(&self) -> &HashMap<String, usize> {
        &self.labels
    }

    /// The machine's register names, in sorted order: the answer the
    /// derived-register machine of exercise 5.13 assembled from the
    /// controller text alone.
    #[must_use]
    pub fn register_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.registers.keys().cloned().collect();
        names.sort();
        names
    }
}

/// Reads the controller text into datums, accepting the book's
/// optional `(controller ...)` wrapper.
///
/// # Errors
/// [`Fault::Parse`] when the text is unreadable.
pub fn parse_controller(controller: &str) -> Result<Vec<Value>, Fault> {
    let datums = read_program(controller).map_err(|error| Fault::Parse(error.to_string()))?;
    if let [only] = datums.as_slice()
        && let Ok(items) = only.list_items()
        && matches!(items.first(), Some(Value::Sym(tag)) if tag.as_ref() == "controller")
    {
        return Ok(items.into_iter().skip(1).collect());
    }
    Ok(datums)
}

/// The book's `extract-labels`: scans the datums, separating labels
/// from instructions and attaching each label to the instruction it
/// dominates; a repeated label is the defect of exercise 5.8 and
/// fails the assembly. A trailing label names one past the last
/// instruction.
///
/// # Errors
/// [`Fault::DuplicateLabel`] when a label name is used twice.
pub fn extract_labels(datums: &[Value]) -> Result<ScannedController, Fault> {
    let mut scanned = ScannedController::default();
    let mut pending: Vec<String> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    for datum in datums {
        let Value::Sym(name) = datum else {
            let index = scanned.insts.len();
            for label in pending.drain(..) {
                scanned.labels.push((label, index));
            }
            let labels: Vec<String> = scanned
                .labels
                .iter()
                .filter(|(_, at)| *at == index)
                .map(|(name, _)| name.clone())
                .collect();
            scanned.insts.push(ScannedInstruction {
                text: render_datum(datum),
                datum: datum.clone(),
                labels,
            });
            continue;
        };
        if !seen.insert(name.to_string()) {
            return Err(Fault::DuplicateLabel {
                label: name.to_string(),
            });
        }
        pending.push(name.to_string());
    }
    for label in pending.drain(..) {
        scanned.labels.push((label, scanned.insts.len()));
    }
    Ok(scanned)
}

/// The book's `lookup-label`: the instruction index a label names.
///
/// # Errors
/// [`Fault::UnknownLabel`] when no label of that name was scanned.
pub fn lookup_label(labels: &[(String, usize)], name: &str) -> Result<usize, Fault> {
    labels.iter().find(|(seen, _)| seen == name).map_or(
        Err(Fault::UnknownLabel {
            label: name.to_owned(),
        }),
        |(_, index)| Ok(*index),
    )
}

/// Renders a datum back into the book's notation, the instruction
/// text the tracer prints.
#[must_use]
pub fn render_datum(datum: &Value) -> String {
    match datum {
        Value::Pair(_) => {
            let items = datum.list_items().unwrap_or_default();
            let parts: Vec<String> = items.iter().map(render_datum).collect();
            format!("({})", parts.join(" "))
        }
        other => display_value(other),
    }
}

/// The book's `make-execution-procedure`: dispatches on the
/// instruction type to build the one execution procedure this
/// instruction will ever need, resolving labels, registers, and
/// operations at assembly time.
fn make_execution_procedure(
    datum: &Value,
    labels: &HashMap<String, usize>,
    ops: &HashMap<String, OpHandler>,
    registers: &HashSet<String>,
) -> Result<Exec, Fault> {
    let text = render_datum(datum);
    let Ok(items) = datum.list_items() else {
        return Err(Fault::BadInstruction { text });
    };
    let Some(Value::Sym(kind)) = items.first() else {
        return Err(Fault::BadInstruction { text });
    };
    let rest = items.get(1..).unwrap_or_default();
    match kind.as_ref() {
        "assign" => make_assign(&text, rest, labels, ops, registers),
        "test" => make_test(&text, rest, ops, registers),
        "branch" => make_branch(&text, rest, labels),
        "goto" => make_goto(&text, rest, labels, registers),
        "save" => make_save(&text, rest, registers),
        "restore" => make_restore(&text, rest, registers),
        "perform" => make_perform(&text, rest, ops, registers),
        _ => Err(Fault::BadInstruction { text }),
    }
}

/// One analyzed operation input: a register reference or a constant.
#[derive(Clone, Debug)]
enum Input {
    Reg(String),
    Const(Value),
}

impl Input {
    fn evaluate(&self, machine: &mut Machine) -> Result<Value, Fault> {
        match self {
            Self::Reg(name) => machine.read(name),
            Self::Const(value) => Ok(value.clone()),
        }
    }
}

/// One analyzed operation application: the resolved handler and its
/// inputs, ready to run.
#[derive(Clone)]
struct Operation {
    name: String,
    handler: OpHandler,
    inputs: Vec<Input>,
}

impl Operation {
    /// Evaluates the application against the machine.
    fn evaluate(&self, machine: &mut Machine) -> Result<Value, Fault> {
        let args: Vec<Value> = self
            .inputs
            .iter()
            .map(|input| input.evaluate(machine))
            .collect::<Result<_, _>>()?;
        call_op(&self.name, &self.handler, machine, &args)
    }
}

/// The right-hand side of an assign, analyzed at assembly time.
enum Source {
    Reg(String),
    Const(Value),
    Label(String),
    Op(Operation),
}

impl Source {
    /// Evaluates the source against the machine.
    fn evaluate(&self, machine: &mut Machine) -> Result<Value, Fault> {
        match self {
            Self::Reg(name) => machine.read(name),
            Self::Const(value) => Ok(value.clone()),
            Self::Label(name) => Ok(Value::sym(name)),
            Self::Op(operation) => operation.evaluate(machine),
        }
    }
}

/// Calls one resolved operation handler, decorating its failures with
/// the operation name and the step.
fn call_op(
    name: &str,
    handler: &OpHandler,
    machine: &mut Machine,
    args: &[Value],
) -> Result<Value, Fault> {
    let step = machine.instruction_count;
    handler(machine, args).map_err(|fault| match fault {
        Fault::Op { message, .. } => Fault::Op {
            op: name.to_owned(),
            message,
            step,
        },
        other => other,
    })
}

/// Reads one `(reg ...)` or `(const ...)` operand of an operation,
/// the grammar of exercise 5.9: labels are refused here, while assign
/// sources keep their own label case.
fn make_input(datum: &Value, op: &str, registers: &HashSet<String>) -> Result<Input, Fault> {
    let bad = || Fault::BadInstruction {
        text: render_datum(datum),
    };
    let Ok(items) = datum.list_items() else {
        return Err(bad());
    };
    let [Value::Sym(tag), argument] = items.as_slice() else {
        return Err(bad());
    };
    match tag.as_ref() {
        "reg" => {
            let name = sym_text(argument).ok_or_else(bad)?;
            check_register(registers, &name)?;
            Ok(Input::Reg(name))
        }
        "const" => Ok(Input::Const(const_value(argument)?)),
        "label" => Err(Fault::LabelOperand {
            op: op.to_owned(),
            label: sym_text(argument).unwrap_or_default(),
        }),
        _ => Err(bad()),
    }
}

/// Validates a register name against the machine's table, the check
/// the book spreads over its `get-register` calls at assembly time.
fn check_register(registers: &HashSet<String>, name: &str) -> Result<(), Fault> {
    if registers.contains(name) {
        return Ok(());
    }
    Err(Fault::UnknownRegister {
        reg: name.to_owned(),
    })
}

/// Converts a `(const ...)` argument into a machine value: numbers,
/// booleans, strings, symbols (quoted or bare), and lists.
fn const_value(datum: &Value) -> Result<Value, Fault> {
    match datum {
        Value::Pair(_) => {
            let items = datum
                .list_items()
                .map_err(|error| Fault::Parse(error.to_string()))?;
            if matches!(items.first(), Some(Value::Sym(tag)) if tag.as_ref() == "quote") {
                return items.get(1).cloned().ok_or_else(|| Fault::BadInstruction {
                    text: render_datum(datum),
                });
            }
            let values = items
                .iter()
                .map(const_value)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Value::list(values))
        }
        Value::Sym(name) if matches!(name.as_ref(), "unquote" | "quasiquote") => {
            Err(Fault::BadInstruction {
                text: render_datum(datum),
            })
        }
        other => Ok(other.clone()),
    }
}

/// A datum that must be a symbol: register, label, and type names.
fn sym_text(datum: &Value) -> Option<String> {
    match datum {
        Value::Sym(name) => Some(name.to_string()),
        _ => None,
    }
}

/// The book's `make-assign`: evaluates the analyzed source against
/// the machine, stores through the single store path, and advances
/// the program counter.
fn make_assign(
    text: &str,
    rest: &[Value],
    labels: &HashMap<String, usize>,
    ops: &HashMap<String, OpHandler>,
    registers: &HashSet<String>,
) -> Result<Exec, Fault> {
    let bad = || Fault::BadInstruction {
        text: text.to_owned(),
    };
    let reg = sym_text(rest.first().ok_or_else(bad)?).ok_or_else(bad)?;
    check_register(registers, &reg)?;
    let source = assign_source(rest.get(1..).ok_or_else(bad)?, labels, ops, registers)?;
    Ok(Rc::new(move |machine| {
        let value = source.evaluate(machine)?;
        machine.store(&reg, value)?;
        machine.advance_pc();
        Ok(())
    }))
}

/// Analyzes an assign's value expression: `(reg r)`, `(const v)`,
/// `(label l)`, or `((op name) inputs...)` in the book's flat
/// surface form.
fn assign_source(
    value_exp: &[Value],
    labels: &HashMap<String, usize>,
    ops: &HashMap<String, OpHandler>,
    registers: &HashSet<String>,
) -> Result<Source, Fault> {
    if let [single] = value_exp
        && !is_operation_form(single)
    {
        return simple_source(single, labels, registers);
    }
    let (name, inputs) = operation_parts(value_exp, registers)?;
    let handler = resolve_operation(ops, &name)?;
    Ok(Source::Op(Operation {
        name,
        handler,
        inputs,
    }))
}

/// Whether the source is the head of an operation application,
/// `(op ...)`: the operation path handles it whatever its input
/// count, including the zero-input `(assign exp (op read))` of the
/// section 5.4 driver.
fn is_operation_form(single: &Value) -> bool {
    single
        .list_items()
        .is_ok_and(|items| matches!(items.first(), Some(Value::Sym(tag)) if tag.as_ref() == "op"))
}

/// Analyzes a one-element assign source: `(reg r)`, `(const v)`, or
/// `(label l)`.
fn simple_source(
    single: &Value,
    labels: &HashMap<String, usize>,
    registers: &HashSet<String>,
) -> Result<Source, Fault> {
    let bad = || Fault::BadInstruction {
        text: render_datum(single),
    };
    let Ok(items) = single.list_items() else {
        return Err(bad());
    };
    let [Value::Sym(tag), argument] = items.as_slice() else {
        return Err(bad());
    };
    match tag.as_ref() {
        "reg" => {
            let name = sym_text(argument).ok_or_else(bad)?;
            check_register(registers, &name)?;
            Ok(Source::Reg(name))
        }
        "const" => Ok(Source::Const(const_value(argument)?)),
        "label" => {
            let name = sym_text(argument).ok_or_else(bad)?;
            if !labels.contains_key(&name) {
                return Err(Fault::UnknownLabel { label: name });
            }
            Ok(Source::Label(name))
        }
        _ => Err(bad()),
    }
}

/// Analyzes an operation application: the head is `(op name)`, the
/// tail the inputs.
fn operation_parts(
    value_exp: &[Value],
    registers: &HashSet<String>,
) -> Result<(String, Vec<Input>), Fault> {
    let bad = || Fault::BadInstruction {
        text: value_exp.first().map_or_else(String::new, render_datum),
    };
    let head = value_exp.first().ok_or_else(bad)?;
    let Ok(head_items) = head.list_items() else {
        return Err(bad());
    };
    let [Value::Sym(tag), op_name] = head_items.as_slice() else {
        return Err(bad());
    };
    if tag.as_ref() != "op" {
        return Err(bad());
    }
    let name = sym_text(op_name).ok_or_else(bad)?;
    let inputs = value_exp[1..]
        .iter()
        .map(|datum| make_input(datum, &name, registers))
        .collect::<Result<Vec<_>, _>>()?;
    Ok((name, inputs))
}

fn resolve_operation(ops: &HashMap<String, OpHandler>, name: &str) -> Result<OpHandler, Fault> {
    ops.get(name)
        .cloned()
        .ok_or_else(|| Fault::UnknownOperation {
            op: name.to_owned(),
        })
}

/// The book's `make-test`: evaluates the operation into the flag.
fn make_test(
    text: &str,
    rest: &[Value],
    ops: &HashMap<String, OpHandler>,
    registers: &HashSet<String>,
) -> Result<Exec, Fault> {
    let (name, inputs) = operation_parts(rest, registers).map_err(|_| Fault::BadInstruction {
        text: text.to_owned(),
    })?;
    let handler = resolve_operation(ops, &name)?;
    let operation = Operation {
        name,
        handler,
        inputs,
    };
    Ok(Rc::new(move |machine| {
        machine.flag = operation.evaluate(machine)?;
        machine.advance_pc();
        Ok(())
    }))
}

/// The book's `make-branch`: consults the flag, jumps or advances.
/// The flag starts at `*unassigned*` and only a test writes it, so a
/// branch before any test is a typed fault.
fn make_branch(text: &str, rest: &[Value], labels: &HashMap<String, usize>) -> Result<Exec, Fault> {
    let destination = label_destination(text, rest, labels)?;
    Ok(Rc::new(move |machine| match machine.flag {
        Value::Bool(true) => {
            machine.pc = destination;
            Ok(())
        }
        Value::Bool(false) => {
            machine.advance_pc();
            Ok(())
        }
        _ => Err(Fault::BranchWithoutTest {
            step: machine.instruction_count,
        }),
    }))
}

/// The book's `make-goto`: a label destination resolved at assembly
/// time, or a register holding a label value.
fn make_goto(
    text: &str,
    rest: &[Value],
    labels: &HashMap<String, usize>,
    registers: &HashSet<String>,
) -> Result<Exec, Fault> {
    let bad = || Fault::BadInstruction {
        text: text.to_owned(),
    };
    let destination = rest.first().ok_or_else(bad)?;
    let Ok(items) = destination.list_items() else {
        return Err(bad());
    };
    let [Value::Sym(tag), argument] = items.as_slice() else {
        return Err(bad());
    };
    match tag.as_ref() {
        "label" => {
            let name = sym_text(argument).ok_or_else(bad)?;
            let target = *labels.get(&name).ok_or_else(|| Fault::UnknownLabel {
                label: name.clone(),
            })?;
            Ok(Rc::new(move |machine| {
                machine.pc = target;
                Ok(())
            }))
        }
        "reg" => {
            let name = sym_text(argument).ok_or_else(bad)?;
            check_register(registers, &name)?;
            Ok(Rc::new(move |machine| {
                let value = machine.read(&name)?;
                let Some(label) = sym_text(&value) else {
                    return Err(Fault::GotoNonLabel {
                        reg: name.clone(),
                        value: display_value(&value),
                        step: machine.instruction_count,
                    });
                };
                machine.pc = *machine
                    .labels
                    .get(&label)
                    .ok_or(Fault::UnknownLabel { label })?;
                Ok(())
            }))
        }
        _ => Err(bad()),
    }
}

/// The book's `make-save`: pushes the register by the machine's
/// discipline and advances.
fn make_save(text: &str, rest: &[Value], registers: &HashSet<String>) -> Result<Exec, Fault> {
    let reg = register_argument(text, rest, registers)?;
    Ok(Rc::new(move |machine| {
        let value = machine.read(&reg)?;
        machine.push(&reg, value);
        machine.advance_pc();
        Ok(())
    }))
}

/// The book's `make-restore`: pops by the machine's discipline,
/// stores through the single store path, and advances.
fn make_restore(text: &str, rest: &[Value], registers: &HashSet<String>) -> Result<Exec, Fault> {
    let reg = register_argument(text, rest, registers)?;
    Ok(Rc::new(move |machine| {
        let step = machine.instruction_count;
        let (saved, value) = machine.pop(&reg)?;
        if machine.discipline == RestoreDiscipline::Tagged && saved != reg {
            return Err(Fault::MismatchedRestore {
                reg: reg.clone(),
                saved,
                step,
            });
        }
        machine.store(&reg, value)?;
        machine.advance_pc();
        Ok(())
    }))
}

/// The book's `make-perform`: evaluates the operation for effect.
fn make_perform(
    text: &str,
    rest: &[Value],
    ops: &HashMap<String, OpHandler>,
    registers: &HashSet<String>,
) -> Result<Exec, Fault> {
    let (name, inputs) = operation_parts(rest, registers).map_err(|_| Fault::BadInstruction {
        text: text.to_owned(),
    })?;
    let handler = resolve_operation(ops, &name)?;
    let operation = Operation {
        name,
        handler,
        inputs,
    };
    Ok(Rc::new(move |machine| {
        operation.evaluate(machine)?;
        machine.advance_pc();
        Ok(())
    }))
}

/// Reads one `(label ...)` branch/goto destination, resolved to its
/// instruction index at assembly time.
fn label_destination(
    text: &str,
    rest: &[Value],
    labels: &HashMap<String, usize>,
) -> Result<usize, Fault> {
    let bad = || Fault::BadInstruction {
        text: text.to_owned(),
    };
    let destination = rest.first().ok_or_else(bad)?;
    let Ok(items) = destination.list_items() else {
        return Err(bad());
    };
    let [Value::Sym(tag), argument] = items.as_slice() else {
        return Err(bad());
    };
    if tag.as_ref() != "label" {
        return Err(bad());
    }
    let name = sym_text(argument).ok_or_else(bad)?;
    labels
        .get(&name)
        .copied()
        .ok_or(Fault::UnknownLabel { label: name })
}

/// Reads the register name of a `(save ...)`/`(restore ...)`
/// instruction, checked against the machine's table.
fn register_argument(
    text: &str,
    rest: &[Value],
    registers: &HashSet<String>,
) -> Result<String, Fault> {
    let bad = || Fault::BadInstruction {
        text: text.to_owned(),
    };
    let name = sym_text(rest.first().ok_or_else(bad)?).ok_or_else(bad)?;
    check_register(registers, &name)?;
    Ok(name)
}

/// The instruction-use summary of exercise 5.12, computed from the
/// scanned controller once the assembly has succeeded: distinct
/// instruction texts per type, `(goto (reg ...))` entry points, and
/// the distinct assign sources per register.
fn summarize(scanned: &ScannedController) -> InstructionUse {
    let mut summary = InstructionUse::default();
    let mut texts: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for inst in &scanned.insts {
        let kind = instruction_type(&inst.datum);
        texts.entry(kind).or_default().insert(inst.text.clone());
        collect_summary(&inst.datum, &mut summary);
    }
    for (kind, distinct) in texts {
        summary.counts.insert(kind, distinct.len());
    }
    summary
}

fn instruction_type(datum: &Value) -> String {
    match datum.list_items() {
        Ok(items) => match items.first() {
            Some(Value::Sym(name)) => name.to_string(),
            _ => "bad".to_owned(),
        },
        Err(_) => "bad".to_owned(),
    }
}

fn collect_summary(datum: &Value, summary: &mut InstructionUse) {
    let Ok(items) = datum.list_items() else {
        return;
    };
    let Some(Value::Sym(kind)) = items.first() else {
        return;
    };
    match kind.as_ref() {
        "assign" => {
            let Some(target) = items.get(1).and_then(sym_text) else {
                return;
            };
            let Some(value_exp) = items.get(2..) else {
                return;
            };
            if let Ok(source) = render_source(value_exp) {
                summary.sources.entry(target).or_default().insert(source);
            }
        }
        "goto" => {
            let Some(destination) = items.get(1) else {
                return;
            };
            let Ok(parts) = destination.list_items() else {
                return;
            };
            if matches!(parts.first(), Some(Value::Sym(tag)) if tag.as_ref() == "reg")
                && let Some(name) = parts.get(1).and_then(sym_text)
            {
                summary.entry_points.insert(name);
            }
        }
        "save" | "restore" => {
            if let Some(name) = items.get(1).and_then(sym_text) {
                summary.stack_registers.insert(name);
            }
        }
        _ => {}
    }
}

/// Renders an assign source at the book's operand level without
/// resolving handlers, for the 5.12 summary.
fn render_source(value_exp: &[Value]) -> Result<String, Fault> {
    let bad = || Fault::BadInstruction {
        text: value_exp.first().map_or_else(String::new, render_datum),
    };
    if let [single] = value_exp {
        let Ok(items) = single.list_items() else {
            return Err(bad());
        };
        let [Value::Sym(tag), argument] = items.as_slice() else {
            return Err(bad());
        };
        return match tag.as_ref() {
            "reg" => Ok(format!("(reg {})", sym_text(argument).ok_or_else(bad)?)),
            "const" => Ok(format!("(const {})", render_datum(argument))),
            "label" => Ok(format!("(label {})", sym_text(argument).ok_or_else(bad)?)),
            _ => Err(bad()),
        };
    }
    let head = value_exp.first().ok_or_else(bad)?;
    let Ok(head_items) = head.list_items() else {
        return Err(bad());
    };
    let [Value::Sym(tag), op_name] = head_items.as_slice() else {
        return Err(bad());
    };
    if tag.as_ref() != "op" {
        return Err(bad());
    }
    let name = sym_text(op_name).ok_or_else(bad)?;
    let parts: Vec<String> = value_exp[1..]
        .iter()
        .map(render_input_datum)
        .collect::<Result<_, _>>()?;
    Ok(format!("(op {name} {})", parts.join(" ")))
}

/// Renders one operation input at the book's operand level.
fn render_input_datum(datum: &Value) -> Result<String, Fault> {
    let bad = || Fault::BadInstruction {
        text: render_datum(datum),
    };
    let Ok(items) = datum.list_items() else {
        return Err(bad());
    };
    let [Value::Sym(tag), argument] = items.as_slice() else {
        return Err(bad());
    };
    match tag.as_ref() {
        "reg" => Ok(format!("(reg {})", sym_text(argument).ok_or_else(bad)?)),
        "const" => Ok(format!("(const {})", render_datum(argument))),
        _ => Err(bad()),
    }
}

/// Derives the register set of a controller (exercise 5.13): every
/// register name the controller mentions, in first-seen order.
#[must_use]
pub fn registers_in(datums: &[Value]) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    for datum in datums {
        collect_registers(datum, &mut names);
    }
    names
}

fn collect_registers(datum: &Value, names: &mut BTreeSet<String>) {
    let Ok(items) = datum.list_items() else {
        return;
    };
    let Some(Value::Sym(kind)) = items.first() else {
        return;
    };
    match kind.as_ref() {
        "assign" => {
            if let Some(target) = items.get(1).and_then(sym_text) {
                names.insert(target);
            }
            for part in items.iter().skip(2) {
                collect_registers(part, names);
            }
        }
        "test" | "perform" | "goto" => {
            for part in items.iter().skip(1) {
                collect_registers(part, names);
            }
        }
        "save" | "restore" | "reg" => {
            if let Some(name) = items.get(1).and_then(sym_text) {
                names.insert(name);
            }
        }
        _ => {}
    }
}

/// The section's shared machine operations, the arithmetic the
/// section's machines use: `= < > + - * / rem abs`. Two arguments
/// each except `abs`; `+ - *` stay in the integers when both
/// arguments are integers and promote to reals otherwise; `/` always
/// computes a real quotient; `rem` is the integer remainder with the
/// dividend's sign. Every machine also carries `initialize-stack`,
/// `print-stack-statistics`, and `print`.
#[must_use]
pub fn op(name: &str) -> Option<OpHandler> {
    match name {
        "=" => Some(Rc::new(|_, args| {
            let (a, b) = numbers(args, "=")?;
            Ok(Value::boolean(real_eq(args, a, b)))
        })),
        "<" => Some(Rc::new(|_, args| {
            compare(args, "<", |x, y| x < y, |x, y| x < y).map(Value::boolean)
        })),
        ">" => Some(Rc::new(|_, args| {
            compare(args, ">", |x, y| x > y, |x, y| x > y).map(Value::boolean)
        })),
        "+" => Some(Rc::new(|_, args| {
            arith(args, "+", i128::checked_add, |x, y| x + y)
        })),
        "-" => Some(Rc::new(|_, args| {
            arith(args, "-", i128::checked_sub, |x, y| x - y)
        })),
        "*" => Some(Rc::new(|_, args| {
            arith(args, "*", i128::checked_mul, |x, y| x * y)
        })),
        "/" => Some(Rc::new(|_, args| {
            let (a, b) = numbers(args, "/")?;
            if b == 0.0 {
                return Err(op_error("/", "division by zero"));
            }
            Ok(Value::Real(a / b))
        })),
        "rem" => Some(Rc::new(|_, args| {
            let (a, b) = integers(args, "rem")?;
            if b == 0 {
                return Err(op_error("rem", "division by zero"));
            }
            Ok(Value::Int(a.wrapping_rem(b)))
        })),
        "abs" => Some(Rc::new(|_, args| match args {
            [Value::Int(n)] => Ok(Value::Int(n.abs())),
            [Value::Real(x)] => Ok(Value::Real(x.abs())),
            _ => Err(op_error("abs", "needs one number")),
        })),
        _ => None,
    }
}

/// The pair of exact integers when both arguments are integers.
fn both_ints(args: &[Value]) -> Option<(i128, i128)> {
    match args {
        [Value::Int(a), Value::Int(b)] => Some((*a, *b)),
        _ => None,
    }
}

/// The machine's numeric comparison: exact on two integers, real
/// comparison otherwise.
fn compare(
    args: &[Value],
    op: &str,
    int_pick: fn(i128, i128) -> bool,
    real_pick: fn(f64, f64) -> bool,
) -> Result<bool, Fault> {
    if let Some((x, y)) = both_ints(args) {
        return Ok(int_pick(x, y));
    }
    let (a, b) = numbers(args, op)?;
    Ok(real_pick(a, b))
}

/// The machine's numeric equality: exact on two integers, real
/// comparison otherwise.
fn real_eq(args: &[Value], a: f64, b: f64) -> bool {
    match both_ints(args) {
        Some((x, y)) => x == y,
        None => a == b,
    }
}

fn op_error(op: &str, message: &str) -> Fault {
    Fault::Op {
        op: op.to_owned(),
        message: message.to_owned(),
        step: 0,
    }
}

fn numbers(args: &[Value], op: &str) -> Result<(f64, f64), Fault> {
    let [a, b] = args else {
        return Err(op_error(op, "needs two arguments"));
    };
    let (Some(a), Some(b)) = (as_real(a), as_real(b)) else {
        return Err(op_error(op, "needs numbers"));
    };
    Ok((a, b))
}

fn integers(args: &[Value], op: &str) -> Result<(i128, i128), Fault> {
    both_ints(args).ok_or_else(|| op_error(op, "needs two integers"))
}

#[expect(
    clippy::cast_precision_loss,
    reason = "the section's machine values are far below f64's exact range, \
              so the lossy widening is wanted here"
)]
fn as_real(value: &Value) -> Option<f64> {
    match value {
        Value::Int(n) => Some(*n as f64),
        Value::Real(x) => Some(*x),
        _ => None,
    }
}

/// Folds one two-argument arithmetic operation, staying in the
/// integers when both arguments are integers and promoting to reals
/// otherwise.
fn arith(
    args: &[Value],
    op: &str,
    int_op: fn(i128, i128) -> Option<i128>,
    real_op: fn(f64, f64) -> f64,
) -> Result<Value, Fault> {
    if let Some((a, b)) = both_ints(args) {
        let folded = int_op(a, b).ok_or_else(|| op_error(op, "integer overflow"))?;
        return Ok(Value::Int(folded));
    }
    let (a, b) = numbers(args, op)?;
    Ok(Value::Real(real_op(a, b)))
}

/// Builds an operation table from the section's shared arithmetic:
/// `(installed name, operation slug)` pairs. Every slug is one
/// [`op`] operation, so the lookup cannot fail.
fn table(ops: &[(&'static str, &'static str)]) -> Vec<(&'static str, OpHandler)> {
    ops.iter()
        .map(|(name, slug)| (*name, op(slug).expect("a shared operation")))
        .collect()
}

/// The section's worked GCD machine: the controller of Figure 5.4
/// without the read-and-print loop, driven through `set_register`,
/// the way the text's session uses it.
///
/// # Panics
/// Only if the figure's transcription stops assembling, which would
/// be a defect of this module and not of a caller.
#[must_use]
pub fn gcd_machine() -> Machine {
    const CONTROLLER: &str = "
test-b
  (test (op =) (reg b) (const 0))
  (branch (label gcd-done))
  (assign t (op rem) (reg a) (reg b))
  (assign a (reg b))
  (assign b (reg t))
  (goto (label test-b))
gcd-done";
    make_machine(
        &["a", "b", "t"],
        &table(&[("=", "="), ("rem", "rem")]),
        CONTROLLER,
    )
    .expect("assembles")
}

/// The recursive factorial machine of Figure 5.11, the machine
/// exercise 5.14 measures.
///
/// # Panics
/// Only if the figure's transcription stops assembling, which would
/// be a defect of this module and not of a caller.
#[must_use]
pub fn factorial_machine() -> Machine {
    const CONTROLLER: &str = "
  (assign continue (label fact-done))
fact-loop
  (test (op =) (reg n) (const 1))
  (branch (label base-case))
  (save continue)
  (save n)
  (assign n (op -) (reg n) (const 1))
  (assign continue (label after-fact))
  (goto (label fact-loop))
after-fact
  (restore n)
  (restore continue)
  (assign val (op *) (reg n) (reg val))
  (goto (reg continue))
base-case
  (assign val (const 1))
  (goto (reg continue))
fact-done";
    make_machine(
        &["n", "val", "continue"],
        &table(&[("=", "="), ("*", "*"), ("-", "-")]),
        CONTROLLER,
    )
    .expect("assembles")
}

/// The tree-recursive Fibonacci machine of Figure 5.12, the machine
/// exercises 5.11, 5.12, and 5.15 through 5.17 run.
///
/// # Panics
/// Only if the figure's transcription stops assembling, which would
/// be a defect of this module and not of a caller.
#[must_use]
pub fn fibonacci_machine() -> Machine {
    const CONTROLLER: &str = "
  (assign continue (label fib-done))
fib-loop
  (test (op <) (reg n) (const 2))
  (branch (label immediate-answer))
  (save continue)
  (assign continue (label afterfib-n-1))
  (save n)
  (assign n (op -) (reg n) (const 1))
  (goto (label fib-loop))
afterfib-n-1
  (restore n)
  (restore continue)
  (assign n (op -) (reg n) (const 2))
  (save continue)
  (assign continue (label afterfib-n-2))
  (save val)
  (goto (label fib-loop))
afterfib-n-2
  (assign n (reg val))
  (restore val)
  (restore continue)
  (assign val (op +) (reg val) (reg n))
  (goto (reg continue))
immediate-answer
  (assign val (reg n))
  (goto (reg continue))
fib-done";
    make_machine(
        &["n", "val", "continue"],
        &table(&[("<", "<"), ("+", "+"), ("-", "-")]),
        CONTROLLER,
    )
    .expect("assembles")
}
