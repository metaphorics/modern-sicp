// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 5.2

//! Section 5.2: the assembler and simulator over the machine data
//! language of grammar §7. Assembly validates the program and resolves
//! labels; step execution updates one typed register file, the program
//! counter, and one explicit stack. An unbound label or register, an
//! invalid restore, or an undefined operation is a typed machine
//! fault, never an invented source-language feature.

use std::collections::HashMap;

pub use crate::sec_5_1::{Instruction, Label, MachineProgram, Operand, Register};

/// The one machine value the integer machines move: a register holds
/// an integer (memory-machine registers hold heap addresses in the
/// same cells), and `perform` operations record their printed text.
pub type MachineValue = i64;

/// An installed operation: any callable the lessons wire in with
/// `Machine::install_operation`.
pub type OpHandler = std::rc::Rc<dyn Fn(&[MachineValue]) -> Result<MachineValue, Fault>>;

/// The counted observations of one run: the save/restore totals of
/// exercises 5.14 and 5.19, and the executed-instruction count.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct StackStats {
    /// Total pushes performed.
    pub pushes: u64,
    /// Total pops performed.
    pub pops: u64,
    /// The deepest stack reached.
    pub max_depth: usize,
    /// The instructions executed so far.
    pub steps: u64,
}

/// A typed machine fault (grammar §7).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Fault {
    /// A label operand names no label.
    UnboundLabel(String),
    /// A register names no declared register.
    UnboundRegister(String),
    /// `restore` with no matching `save`.
    InvalidRestore,
    /// A `test` or `perform` names no operation.
    UndefinedOperation(String),
    /// The instruction budget expired.
    BudgetExhausted,
    /// Division or remainder by zero inside an operation.
    DivByZero,
    /// Checked-build overflow inside an operation.
    Overflow(&'static str),
}

impl std::fmt::Display for Fault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnboundLabel(label) => write!(f, "unbound label `{label}`"),
            Self::UnboundRegister(register) => write!(f, "unbound register `{register}`"),
            Self::InvalidRestore => f.write_str("`restore` with no matching `save`"),
            Self::UndefinedOperation(name) => write!(f, "undefined operation `{name}`"),
            Self::BudgetExhausted => f.write_str("the instruction budget expired"),
            Self::DivByZero => f.write_str("division or remainder by zero"),
            Self::Overflow(operation) => write!(f, "`{operation}` overflowed"),
        }
    }
}

impl std::error::Error for Fault {}

/// One assembled machine: resolved labels and validated instructions.
#[derive(Debug, Clone)]
pub struct Assembled {
    /// The resolved label table: label name to instruction index.
    pub labels: HashMap<String, usize>,
    /// The instructions in execution order.
    pub instructions: Vec<Instruction>,
    /// The declared registers.
    pub registers: Vec<String>,
}

/// Assembles one machine program: validates register names and
/// resolves every label.
///
/// # Errors
/// [`Fault::UnboundLabel`] or [`Fault::UnboundRegister`].
pub fn assemble(program: &MachineProgram) -> Result<Assembled, Fault> {
    let registers: Vec<String> = program
        .registers
        .iter()
        .map(|Register(name)| name.clone())
        .collect();
    let mut labels = HashMap::new();
    let mut instructions = Vec::with_capacity(program.instructions.len());
    for (label, instruction) in &program.instructions {
        if let Some(Label(name)) = label {
            labels.insert(name.clone(), instructions.len());
        }
        instructions.push(instruction.clone());
    }
    for instruction in &instructions {
        for operand in operands_of(instruction) {
            match operand {
                Operand::Register(Register(name)) => {
                    if !registers.contains(&name) {
                        return Err(Fault::UnboundRegister(name.clone()));
                    }
                }
                Operand::Label(Label(name)) => {
                    if !labels.contains_key(&name) {
                        return Err(Fault::UnboundLabel(name.clone()));
                    }
                }
                // Constants carry no names, and operation arguments were
                // flattened by `collect_operand` into this validation loop.
                Operand::Constant(_) | Operand::Operation { .. } => {}
            }
        }
    }
    Ok(Assembled {
        labels,
        instructions,
        registers,
    })
}

fn operands_of(instruction: &Instruction) -> Vec<Operand> {
    let mut found = Vec::new();
    collect_operands(instruction, &mut found);
    found
}

fn collect_operands(instruction: &Instruction, out: &mut Vec<Operand>) {
    match instruction {
        Instruction::Assign { value, .. } => collect_operand(value, out),
        Instruction::Test { arguments, .. } | Instruction::Perform { arguments, .. } => {
            for argument in arguments {
                collect_operand(argument, out);
            }
        }
        Instruction::Branch(Label(name)) => out.push(Operand::Label(Label(name.clone()))),
        Instruction::Goto(operand) => collect_operand(operand, out),
        Instruction::Save(_) | Instruction::Restore(_) => {}
    }
}

fn collect_operand(operand: &Operand, out: &mut Vec<Operand>) {
    match operand {
        Operand::Operation {
            operation,
            arguments,
        } => {
            out.push(Operand::Operation {
                operation: operation.clone(),
                arguments: Vec::new(),
            });
            for argument in arguments {
                collect_operand(argument, out);
            }
        }
        other => out.push(other.clone()),
    }
}

/// One simulator run's observable outcome.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Run {
    /// The final register file.
    pub registers: HashMap<String, MachineValue>,
    /// The ordered transcript `print` operations produced.
    pub output: Vec<String>,
    /// The number of instructions executed.
    pub steps: u64,
    /// The rendered instruction trace, when tracing was on.
    pub trace: Vec<String>,
    /// The breakpoint label the run stopped at, when one fired.
    pub at_breakpoint: Option<String>,
}

/// The register-machine simulator: one register file, one program
/// counter, one explicit stack, and an optional instruction trace.
pub struct Machine {
    assembled: Assembled,
    registers: HashMap<String, MachineValue>,
    pc: usize,
    stack: Vec<(String, MachineValue)>,
    ops: HashMap<String, OpHandler>,
    output: Vec<String>,
    trace: Vec<String>,
    tracing: bool,
    steps: u64,
    budget: u64,
    stats: StackStats,
    breakpoints: Vec<(String, usize, usize)>,
}

impl std::fmt::Debug for Machine {
    /// The machine's observable state; operation handlers are host
    /// closures and appear by name only.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut operations: Vec<&String> = self.ops.keys().collect();
        operations.sort();
        f.debug_struct("Machine")
            .field("pc", &self.pc)
            .field("registers", &self.registers)
            .field("stack", &self.stack)
            .field("operations", &operations)
            .field("steps", &self.steps)
            .field("stats", &self.stats)
            .finish_non_exhaustive()
    }
}

impl Machine {
    /// Builds one machine over one assembled program.
    #[must_use]
    pub fn new(assembled: Assembled) -> Self {
        let mut registers = HashMap::new();
        for name in &assembled.registers {
            registers.insert(name.clone(), 0);
        }
        Self {
            assembled,
            registers,
            pc: 0,
            stack: Vec::new(),
            ops: standard_operations(),
            output: Vec::new(),
            trace: Vec::new(),
            tracing: false,
            steps: 0,
            budget: 1_000_000,
            stats: StackStats::default(),
            breakpoints: Vec::new(),
        }
    }

    /// Installs one named operation, replacing any earlier install of
    /// the same name.
    pub fn install_operation(&mut self, name: &str, op: OpHandler) {
        self.ops.insert(name.to_owned(), op);
    }

    /// The current register file.
    #[must_use]
    pub fn registers(&self) -> &HashMap<String, MachineValue> {
        &self.registers
    }

    /// Reads one register, including mid-run through installed
    /// operations.
    ///
    /// # Errors
    /// [`Fault::UnboundRegister`] when the register is undeclared.
    pub fn get_register(&self, name: &str) -> Result<MachineValue, Fault> {
        self.read_register(name)
    }

    /// The current stack depth.
    #[must_use]
    pub fn stack_depth(&self) -> usize {
        self.stack.len()
    }

    /// The save/restore statistics of the run so far, with the
    /// executed-instruction count.
    #[must_use]
    pub fn stack_statistics(&self) -> StackStats {
        StackStats {
            steps: self.steps,
            ..self.stats
        }
    }

    /// Sets a breakpoint before the `n`-th visit of one label
    /// (one-based visits).
    ///
    /// # Errors
    /// [`Fault::UnboundLabel`] when the label is undeclared.
    pub fn set_breakpoint(&mut self, label: &str, n: usize) -> Result<(), Fault> {
        self.lookup_label(label)?;
        self.breakpoints.push((label.to_owned(), n, 0));
        Ok(())
    }

    /// Removes every breakpoint.
    pub fn clear_breakpoints(&mut self) {
        self.breakpoints.clear();
    }

    /// The rendered trace lines so far.
    #[must_use]
    pub fn trace(&self) -> &[String] {
        &self.trace
    }

    /// The current program counter.
    #[must_use]
    pub fn pc(&self) -> usize {
        self.pc
    }

    /// The assembled controller.
    #[must_use]
    pub fn assembled(&self) -> &Assembled {
        &self.assembled
    }

    /// Turns instruction tracing on or off.
    pub fn set_trace(&mut self, on: bool) {
        self.tracing = on;
    }

    /// Sets one register's initial value.
    ///
    /// # Errors
    /// [`Fault::UnboundRegister`] when the register is undeclared.
    pub fn set_register(&mut self, name: &str, value: MachineValue) -> Result<(), Fault> {
        if !self.registers.contains_key(name) {
            return Err(Fault::UnboundRegister(name.to_owned()));
        }
        self.registers.insert(name.to_owned(), value);
        Ok(())
    }

    /// Runs the machine from its first instruction to its last.
    ///
    /// # Errors
    /// The first [`Fault`] the run raises.
    pub fn run(&mut self) -> Result<Run, Fault> {
        self.pc = 0;
        self.run_loop()
    }

    /// Continues a run stopped at a breakpoint.
    ///
    /// # Errors
    /// The first [`Fault`] the run raises.
    pub fn resume(&mut self) -> Result<Run, Fault> {
        self.run_loop()
    }

    fn run_loop(&mut self) -> Result<Run, Fault> {
        while self.pc < self.assembled.instructions.len() {
            if self.steps >= self.budget {
                return Err(Fault::BudgetExhausted);
            }
            if let Some(label) = self.breakpoint_here() {
                return Ok(self.finish_at(Some(label)));
            }
            self.step()?;
        }
        Ok(self.finish())
    }

    fn breakpoint_here(&mut self) -> Option<String> {
        for (label, target, visits) in &mut self.breakpoints {
            if self.assembled.labels.get(label).copied() == Some(self.pc) {
                *visits += 1;
                if *visits == *target {
                    return Some(label.clone());
                }
            }
        }
        None
    }

    /// Executes one instruction.
    ///
    /// # Errors
    /// The first [`Fault`] the instruction raises.
    pub fn step(&mut self) -> Result<(), Fault> {
        let Some(instruction) = self.assembled.instructions.get(self.pc).cloned() else {
            return Ok(());
        };
        if self.tracing {
            self.trace.push(format!("{:4}: {:?}", self.pc, instruction));
        }
        self.steps += 1;
        match instruction {
            Instruction::Assign { target, value } => {
                let produced = self.read_operand(&value)?;
                self.write_register(&target, produced)?;
                self.pc += 1;
            }
            Instruction::Test {
                predicate,
                arguments,
            } => {
                let values = self.read_arguments(&arguments)?;
                let result = self.apply_operation(&predicate, &values)?;
                // The flag lives as register `flag`; branch reads it.
                self.registers.insert("flag".to_owned(), result);
                self.pc += 1;
            }
            Instruction::Branch(Label(name)) => {
                let flag = self.registers.get("flag").copied().unwrap_or(0);
                if flag != 0 {
                    self.pc = self.lookup_label(&name)?;
                } else {
                    self.pc += 1;
                }
            }
            Instruction::Goto(operand) => {
                self.pc = match operand {
                    Operand::Label(Label(name)) => self.lookup_label(&name)?,
                    Operand::Register(Register(name)) => {
                        let value = self.read_register(&name)?;
                        usize::try_from(value).map_err(|_| Fault::Overflow("goto"))?
                    }
                    Operand::Constant(value) => {
                        usize::try_from(value).map_err(|_| Fault::Overflow("goto"))?
                    }
                    Operand::Operation { .. } => {
                        // A computed jump target: evaluate the operation
                        // and continue at the resulting address.
                        let value = self.read_operand(&operand)?;
                        usize::try_from(value).map_err(|_| Fault::Overflow("goto"))?
                    }
                };
            }
            Instruction::Save(Register(name)) => {
                let value = self.read_register(&name)?;
                self.stack.push((name, value));
                self.stats.pushes += 1;
                self.stats.max_depth = self.stats.max_depth.max(self.stack.len());
                self.pc += 1;
            }
            Instruction::Restore(Register(name)) => {
                let (saved, value) = self.stack.pop().ok_or(Fault::InvalidRestore)?;
                if saved != name {
                    return Err(Fault::InvalidRestore);
                }
                self.stats.pops += 1;
                self.write_register(&Register(name), value)?;
                self.pc += 1;
            }
            Instruction::Perform {
                operation,
                arguments,
            } => {
                // Perform is effect-only: it never writes a register.
                // Computation results flow through
                // `Assign { target, value: Operation { .. } }`.
                let values = self.read_arguments(&arguments)?;
                if operation == "print" {
                    // A built-in transcript effect: it needs no
                    // installed operation.
                    let rendered = values.first().map_or_else(String::new, ToString::to_string);
                    self.output.push(rendered);
                    self.pc += 1;
                    return Ok(());
                }
                let produced = self.apply_operation(&operation, &values)?;
                let _ = produced;
                self.pc += 1;
            }
        }
        Ok(())
    }

    fn finish(&self) -> Run {
        self.finish_at(None)
    }

    fn finish_at(&self, at_breakpoint: Option<String>) -> Run {
        Run {
            registers: self.registers.clone(),
            output: self.output.clone(),
            steps: self.steps,
            trace: self.trace.clone(),
            at_breakpoint,
        }
    }

    fn lookup_label(&self, name: &str) -> Result<usize, Fault> {
        self.assembled
            .labels
            .get(name)
            .copied()
            .ok_or_else(|| Fault::UnboundLabel(name.to_owned()))
    }

    fn read_register(&self, name: &str) -> Result<MachineValue, Fault> {
        self.registers
            .get(name)
            .copied()
            .ok_or_else(|| Fault::UnboundRegister(name.to_owned()))
    }

    fn write_register(
        &mut self,
        Register(name): &Register,
        value: MachineValue,
    ) -> Result<(), Fault> {
        if !self.registers.contains_key(name) {
            return Err(Fault::UnboundRegister(name.clone()));
        }
        self.registers.insert(name.clone(), value);
        Ok(())
    }

    fn read_operand(&self, operand: &Operand) -> Result<MachineValue, Fault> {
        match operand {
            Operand::Constant(value) => Ok(*value),
            Operand::Register(Register(name)) => self.read_register(name),
            Operand::Label(Label(name)) => {
                let index = self.lookup_label(name)?;
                let value = i64::try_from(index).map_err(|_| Fault::Overflow("label"))?;
                Ok(value)
            }
            Operand::Operation {
                operation,
                arguments,
            } => {
                let values = self.read_arguments(arguments)?;
                self.apply_operation(operation, &values)
            }
        }
    }

    fn read_arguments(&self, operands: &[Operand]) -> Result<Vec<MachineValue>, Fault> {
        let mut values = Vec::with_capacity(operands.len());
        for operand in operands {
            values.push(self.read_operand(operand)?);
        }
        Ok(values)
    }

    fn apply_operation(&self, name: &str, values: &[MachineValue]) -> Result<MachineValue, Fault> {
        if let Some(op) = self.ops.get(name).cloned() {
            return op(values);
        }
        Err(Fault::UndefinedOperation(name.to_owned()))
    }
}

/// The operation table every lesson machine draws on.
#[must_use]
pub fn standard_operations() -> HashMap<String, OpHandler> {
    let mut ops: HashMap<String, OpHandler> = HashMap::new();
    ops.insert("add".to_owned(), std::rc::Rc::new(op_add));
    ops.insert("add1".to_owned(), std::rc::Rc::new(op_add1));
    ops.insert("sub1".to_owned(), std::rc::Rc::new(op_sub1));
    ops.insert("sub2".to_owned(), std::rc::Rc::new(op_sub2));
    ops.insert("mul".to_owned(), std::rc::Rc::new(op_mul));
    ops.insert("rem".to_owned(), std::rc::Rc::new(op_rem));
    ops.insert("=".to_owned(), std::rc::Rc::new(op_eq));
    ops.insert("<".to_owned(), std::rc::Rc::new(op_lt));
    ops.insert(">".to_owned(), std::rc::Rc::new(op_gt));
    ops.insert("print".to_owned(), std::rc::Rc::new(op_print));
    ops
}

fn op_add(values: &[MachineValue]) -> Result<MachineValue, Fault> {
    let [a, b] = values else {
        return Err(Fault::UndefinedOperation("add".to_owned()));
    };
    a.checked_add(*b).ok_or(Fault::Overflow("add"))
}

fn op_add1(values: &[MachineValue]) -> Result<MachineValue, Fault> {
    let [a] = values else {
        return Err(Fault::UndefinedOperation("add1".to_owned()));
    };
    // `add1` mutates its argument in the book's controller; the
    // simulator models it as producing the incremented value, and the
    // assignment above stores it.
    a.checked_add(1).ok_or(Fault::Overflow("add1"))
}

fn op_sub1(values: &[MachineValue]) -> Result<MachineValue, Fault> {
    let [a] = values else {
        return Err(Fault::UndefinedOperation("sub1".to_owned()));
    };
    a.checked_sub(1).ok_or(Fault::Overflow("sub1"))
}

fn op_sub2(values: &[MachineValue]) -> Result<MachineValue, Fault> {
    let [a] = values else {
        return Err(Fault::UndefinedOperation("sub2".to_owned()));
    };
    a.checked_sub(2).ok_or(Fault::Overflow("sub2"))
}

fn op_mul(values: &[MachineValue]) -> Result<MachineValue, Fault> {
    let [a, b] = values else {
        return Err(Fault::UndefinedOperation("mul".to_owned()));
    };
    a.checked_mul(*b).ok_or(Fault::Overflow("multiply"))
}

fn op_rem(values: &[MachineValue]) -> Result<MachineValue, Fault> {
    let [a, b] = values else {
        return Err(Fault::UndefinedOperation("rem".to_owned()));
    };
    if *b == 0 {
        return Err(Fault::DivByZero);
    }
    a.checked_rem(*b).ok_or(Fault::Overflow("rem"))
}

fn op_eq(values: &[MachineValue]) -> Result<MachineValue, Fault> {
    let [a, b] = values else {
        return Err(Fault::UndefinedOperation("=".to_owned()));
    };
    Ok(i64::from(a == b))
}

fn op_lt(values: &[MachineValue]) -> Result<MachineValue, Fault> {
    let [a, b] = values else {
        return Err(Fault::UndefinedOperation("<".to_owned()));
    };
    Ok(i64::from(a < b))
}

fn op_gt(values: &[MachineValue]) -> Result<MachineValue, Fault> {
    let [a, b] = values else {
        return Err(Fault::UndefinedOperation(">".to_owned()));
    };
    Ok(i64::from(a > b))
}

fn op_print(values: &[MachineValue]) -> Result<MachineValue, Fault> {
    let [a] = values else {
        return Err(Fault::UndefinedOperation("print".to_owned()));
    };
    Ok(*a)
}
