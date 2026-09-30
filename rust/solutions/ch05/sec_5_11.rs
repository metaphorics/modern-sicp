// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.11: the three restore
//! disciplines as typed alternatives, and the pruned Fibonacci
//! machine the book's own discipline allows.

use std::collections::HashMap;

use ch05::sec_5_1::{
    Instruction, MachineProgram, Operand, Register, constant, fibonacci_machine, reg,
};
use ch05::sec_5_2::{Fault, Machine, assemble};

/// The three meanings of `restore` the exercise contrasts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Policy {
    /// `restore` takes whatever the stack top holds, whatever
    /// register it was saved from.
    Plain,
    /// `restore` demands the saved register match its own name.
    Strict,
    /// Every register keeps its own stack, so a restore always finds
    /// its own last save.
    PerRegister,
}

/// One typed restore fault, naming the step that raised it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestoreFault {
    /// A strict restore met a different register's save.
    MismatchedRestore {
        /// The register the restore named.
        reg: String,
        /// The register the save named.
        saved: String,
        /// The step number of the refused restore.
        step: usize,
    },
    /// A per-register restore found its own stack empty.
    StackUnderflow {
        /// The register whose stack was empty.
        reg: String,
        /// The step number of the refused restore.
        step: usize,
    },
}

/// The stack one discipline uses.
enum Stack {
    /// One stack of name-tagged saves: plain and strict.
    Shared(Vec<(String, i64)>),
    /// One stack per register: per-register.
    PerRegister(HashMap<String, Vec<i64>>),
}

impl Stack {
    /// The stack the policy dictates.
    fn new(policy: Policy) -> Self {
        match policy {
            Policy::PerRegister => Self::PerRegister(HashMap::new()),
            Policy::Plain | Policy::Strict => Self::Shared(Vec::new()),
        }
    }

    /// Pushes one saved value.
    fn push(&mut self, name: &str, value: i64) {
        match self {
            Self::Shared(rows) => rows.push((name.to_owned(), value)),
            Self::PerRegister(stacks) => stacks.entry(name.to_owned()).or_default().push(value),
        }
    }

    /// Pops one value into `name`, refusing an empty stack or a
    /// mismatched save the way the policy dictates.
    fn pop(&mut self, name: &str, policy: Policy, step: usize) -> Result<i64, RestoreFault> {
        let underflow = || RestoreFault::StackUnderflow {
            reg: name.to_owned(),
            step,
        };
        match self {
            Self::Shared(rows) => {
                let (saved, value) = rows.pop().ok_or_else(underflow)?;
                if policy == Policy::Strict && saved != name {
                    return Err(RestoreFault::MismatchedRestore {
                        reg: name.to_owned(),
                        saved,
                        step,
                    });
                }
                Ok(value)
            }
            Self::PerRegister(stacks) => stacks
                .get_mut(name)
                .and_then(Vec::pop)
                .ok_or_else(underflow),
        }
    }

    /// The number of saved values, the machine's stack depth.
    fn depth(&self) -> usize {
        match self {
            Self::Shared(rows) => rows.len(),
            Self::PerRegister(stacks) => stacks.values().map(Vec::len).sum(),
        }
    }
}

/// One run of a machine under one policy: its registers, transcript,
/// step count, pushes, and peak depth.
#[derive(Debug)]
pub struct Outcome {
    /// The register file at the end.
    pub registers: HashMap<String, i64>,
    /// The `print` operations' results in order.
    pub output: Vec<i64>,
    /// The executed-instruction count.
    pub steps: usize,
    /// The save count.
    pub pushes: usize,
    /// The peak stack depth.
    pub max_depth: usize,
}

/// Runs one machine program one instruction at a time under one
/// restore discipline. The save, restore, test, and branch rows are
/// executed exactly as written; only the meaning of `restore`
/// changes.
/// # Errors
/// Returns the first [`RestoreFault`] raised by an invalid restore.
pub fn run_policy(
    program: &MachineProgram,
    policy: Policy,
    inputs: &[(&str, i64)],
) -> Result<Outcome, RestoreFault> {
    let labels = label_table(program);
    let mut registers = HashMap::new();
    for (name, value) in inputs {
        registers.insert((*name).to_owned(), *value);
    }
    let mut stack = Stack::new(policy);
    let mut output = Vec::new();
    let mut pushes = 0;
    let mut max_depth = 0;
    let mut pc = 0;
    let mut steps = 0;
    while pc < program.instructions.len() {
        steps += 1;
        let instruction = &program.instructions[pc].1;
        pc = match instruction {
            Instruction::Assign {
                target: Register(name),
                value,
            } => {
                let value = read(value, &registers, &labels);
                registers.insert(name.clone(), value);
                pc + 1
            }
            Instruction::Test {
                predicate,
                arguments,
            } => {
                let values: Vec<i64> = arguments
                    .iter()
                    .map(|argument| read(argument, &registers, &labels))
                    .collect();
                let flag = apply(predicate, &values);
                registers.insert("flag".to_owned(), flag);
                pc + 1
            }
            Instruction::Branch(target) => {
                let flag = registers.get("flag").copied().unwrap_or(0);
                if flag != 0 { labels[&target.0] } else { pc + 1 }
            }
            Instruction::Goto(operand) => match operand {
                Operand::Label(target) => labels[&target.0],
                Operand::Register(Register(name)) => to_step(registers[name]),
                Operand::Constant(index) => to_step(*index),
                Operand::Operation { .. } => to_step(read(operand, &registers, &labels)),
            },
            Instruction::Save(Register(name)) => {
                let value = registers.get(name).copied().unwrap_or(0);
                stack.push(name, value);
                pushes += 1;
                max_depth = max_depth.max(stack.depth());
                pc + 1
            }
            Instruction::Restore(Register(name)) => {
                let value = stack.pop(name, policy, steps)?;
                registers.insert(name.clone(), value);
                pc + 1
            }
            Instruction::Perform {
                operation,
                arguments,
            } => {
                let values: Vec<i64> = arguments
                    .iter()
                    .map(|argument| read(argument, &registers, &labels))
                    .collect();
                output.push(apply(operation, &values));
                pc + 1
            }
        };
    }
    Ok(Outcome {
        registers,
        output,
        steps,
        pushes,
        max_depth,
    })
}

/// A label index widened to the register's value type.
fn to_i64(index: usize) -> i64 {
    i64::try_from(index).unwrap_or(0)
}

/// A stored program counter. A negative or oversized value lands
/// past the last instruction, which stops the run.
fn to_step(value: i64) -> usize {
    usize::try_from(value).unwrap_or(usize::MAX)
}

/// The label table of one program: label name to instruction index.
fn label_table(program: &MachineProgram) -> HashMap<String, usize> {
    program
        .instructions
        .iter()
        .enumerate()
        .filter_map(|(index, (label_row, _))| {
            label_row.as_ref().map(|label| (label.0.clone(), index))
        })
        .collect()
}

/// Reads one operand: constants and registers plainly, a label as
/// its instruction index, and an operation recursively.
fn read(
    operand: &Operand,
    registers: &HashMap<String, i64>,
    labels: &HashMap<String, usize>,
) -> i64 {
    match operand {
        Operand::Constant(value) => *value,
        Operand::Register(Register(name)) => registers.get(name).copied().unwrap_or(0),
        Operand::Label(label) => to_i64(labels.get(&label.0).copied().unwrap_or(0)),
        Operand::Operation {
            operation,
            arguments,
        } => {
            let values: Vec<i64> = arguments
                .iter()
                .map(|argument| read(argument, registers, labels))
                .collect();
            apply(operation, &values)
        }
    }
}

/// The machine's operation table: the arithmetic the Fibonacci and
/// probe machines need.
fn apply(operation: &str, values: &[i64]) -> i64 {
    match (operation, values) {
        ("add", [a, b]) => a + b,
        ("sub1", [a]) => a - 1,
        ("sub2", [a]) => a - 2,
        ("mul", [a, b]) => a * b,
        ("=", [a, b]) => i64::from(a == b),
        ("<", [a, b]) => i64::from(a < b),
        (">", [a, b]) => i64::from(a > b),
        ("print", [a]) => *a,
        _ => 0,
    }
}

/// The book's probe: restore a register that is not the last one
/// saved.
fn out_of_order_program() -> MachineProgram {
    MachineProgram::new(
        vec![reg("x"), reg("y")],
        vec![
            (
                None,
                Instruction::Assign {
                    target: reg("y"),
                    value: constant(7),
                },
            ),
            (
                None,
                Instruction::Assign {
                    target: reg("x"),
                    value: constant(8),
                },
            ),
            (None, Instruction::Save(reg("y"))),
            (None, Instruction::Save(reg("x"))),
            (None, Instruction::Restore(reg("y"))),
        ],
    )
}

/// The pruning the book's own plain discipline allows: at
/// `afterfibn-2` the pair `assign n <- val` / `restore val` collapses
/// into one `restore n`, which picks up the value `val` saved.
fn fibonacci_pruned() -> MachineProgram {
    let MachineProgram {
        registers,
        instructions,
    } = fibonacci_machine();
    let mut rows = Vec::with_capacity(instructions.len());
    let mut pending = instructions.into_iter().peekable();
    while let Some((label_row, instruction)) = pending.next() {
        if writes_n_from_val(&instruction) && restores_val(pending.peek()) {
            rows.push((label_row, Instruction::Restore(reg("n"))));
            pending.next();
            continue;
        }
        rows.push((label_row, instruction));
    }
    MachineProgram::new(registers, rows)
}

/// Whether one instruction is `assign n <- val`.
fn writes_n_from_val(instruction: &Instruction) -> bool {
    matches!(
        instruction,
        Instruction::Assign { target: Register(target), value: Operand::Register(Register(source)), .. }
            if target == "n" && source == "val"
    )
}

/// Whether the next row is `restore val`.
fn restores_val(next: Option<&(Option<ch05::sec_5_1::Label>, Instruction)>) -> bool {
    matches!(
        next,
        Some((_, Instruction::Restore(Register(restored)))) if restored == "val"
    )
}

/// The host's own Fibonacci, for the direct comparison.
fn fib_direct(n: i64) -> i64 {
    if n < 2 {
        n
    } else {
        fib_direct(n - 1) + fib_direct(n - 2)
    }
}

mod ex_5_11 {
    //! Exercise 5.11: the three meanings of restore, and the
    //! instruction the book's own meaning lets us drop.

    use super::*;

    /// Part (a) demonstrated: under the plain discipline the pruned
    /// machine equals the host Fibonacci on n = 0..=10, with exactly
    /// one instruction saved per internal call.
    #[test]
    fn ex_5_11_part_a_pruned_machine_matches_the_host() {
        for n in 0..=10 {
            let outcome = run_policy(&fibonacci_pruned(), Policy::Plain, &[("n", n)]).expect("run");
            assert_eq!(
                outcome.registers["val"],
                fib_direct(n),
                "pruned fib({n}) under plain restore"
            );
        }
        let original = run_policy(&fibonacci_machine(), Policy::Plain, &[("n", 6)]).expect("run");
        assert_eq!(original.registers["val"], 8);
    }

    /// The count at n = 6: the original runs 282 instructions, the
    /// pruned machine 270, twelve internal calls each one shorter;
    /// pushes are untouched at 48 because the pruning removes an
    /// assignment, not a save.
    #[test]
    fn ex_5_11_part_a_pruned_counts() {
        let original = run_policy(&fibonacci_machine(), Policy::Plain, &[("n", 6)]).expect("run");
        let pruned = run_policy(&fibonacci_pruned(), Policy::Plain, &[("n", 6)]).expect("run");
        assert_eq!((original.steps, original.pushes), (282, 48));
        assert_eq!((pruned.steps, pruned.pushes), (270, 48));
        assert_eq!((original.max_depth, pruned.max_depth), (10, 10));
    }

    /// Part (b): under the strict discipline the out-of-order
    /// restore is a typed fault naming both registers, and the
    /// original machine (whose restores are all matched) runs
    /// unharmed while the pruned machine's exploit is caught.
    #[test]
    fn ex_5_11_part_b_strict_discipline() {
        let fault = run_policy(&out_of_order_program(), Policy::Strict, &[])
            .expect_err("the out-of-order restore is refused");
        assert_eq!(
            fault,
            RestoreFault::MismatchedRestore {
                reg: "y".to_owned(),
                saved: "x".to_owned(),
                step: 5
            }
        );
        let original = run_policy(&fibonacci_machine(), Policy::Strict, &[("n", 6)]).expect("run");
        assert_eq!(original.registers["val"], 8);
        let fault = run_policy(&fibonacci_pruned(), Policy::Strict, &[("n", 6)])
            .expect_err("the pruned machine's restore is refused");
        assert_eq!(
            fault,
            RestoreFault::MismatchedRestore {
                reg: "n".to_owned(),
                saved: "val".to_owned(),
                step: 52
            }
        );
    }

    /// The strict discipline is the published simulator's: the same
    /// probe under `Machine` raises its typed `InvalidRestore`, and
    /// the original Fibonacci machine answers 8 through it.
    #[test]
    fn ex_5_11_part_b_matches_the_published_simulator() {
        let mut probe = Machine::new(assemble(&out_of_order_program()).expect("assembles"));
        let fault = probe
            .run()
            .expect_err("the out-of-order restore is refused");
        assert_eq!(fault, Fault::InvalidRestore);

        let mut original = Machine::new(assemble(&fibonacci_machine()).expect("assembles"));
        original.set_register("n", 6).expect("register");
        original.run().expect("run");
        assert_eq!(original.get_register("val").expect("val"), 8);
    }

    /// Part (c): under the per-register discipline every restore
    /// finds its own register's last save, so the probe returns 7 and
    /// the original machine still answers 8 at n = 6; the pruned
    /// machine is refused, since `n`'s stack is empty where its
    /// exploit fires.
    #[test]
    fn ex_5_11_part_c_per_register_discipline() {
        let probe = run_policy(&out_of_order_program(), Policy::PerRegister, &[]).expect("run");
        assert_eq!(probe.registers["y"], 7);
        let original =
            run_policy(&fibonacci_machine(), Policy::PerRegister, &[("n", 6)]).expect("run");
        assert_eq!(original.registers["val"], 8);
        let fault = run_policy(&fibonacci_pruned(), Policy::PerRegister, &[("n", 6)])
            .expect_err("the pruned machine's restore is refused");
        assert_eq!(
            fault,
            RestoreFault::StackUnderflow {
                reg: "n".to_owned(),
                step: 93
            }
        );
    }
}
