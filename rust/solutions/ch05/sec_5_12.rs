// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.12: the assembler's
//! instruction-use summary over the section's two recursive machines.

use std::collections::{BTreeMap, BTreeSet};

use ch05::sec_5_1::{
    Instruction, MachineProgram, Operand, Register, factorial_recursive, fibonacci_machine,
};

/// One register's summary: the distinct sources its assignments
/// read, in the source rendering the book uses.
#[derive(Debug, Default)]
pub struct RegisterUse {
    /// The rendered source of each `assign` to the register.
    pub sources: BTreeSet<String>,
}

/// One machine's instruction-use summary.
#[derive(Debug, Default)]
pub struct Summary {
    /// The distinct instruction counts, by kind.
    pub counts: BTreeMap<String, usize>,
    /// The registers named in `goto` operand position: the entry
    /// points a computed jump can reach.
    pub entry_points: BTreeSet<String>,
    /// The registers the stack sees: the `save` and `restore` names.
    pub stack_registers: BTreeSet<String>,
    /// Each register's distinct assignment sources.
    pub registers: BTreeMap<String, RegisterUse>,
}

/// Collects one program's instruction-use summary.
#[must_use]
pub fn summarize(program: &MachineProgram) -> Summary {
    let mut summary = Summary::default();
    for (_, instruction) in &program.instructions {
        let kind = summary
            .counts
            .entry(kind_of(instruction).to_owned())
            .or_insert(0);
        *kind += 1;
        note_entry_point(instruction, &mut summary);
        note_stack_register(instruction, &mut summary);
        note_sources(instruction, &mut summary);
    }
    summary
}

/// The book's name for one instruction kind.
fn kind_of(instruction: &Instruction) -> &'static str {
    match instruction {
        Instruction::Assign { .. } => "assign",
        Instruction::Test { .. } => "test",
        Instruction::Branch(_) => "branch",
        Instruction::Goto(_) => "goto",
        Instruction::Save(_) => "save",
        Instruction::Restore(_) => "restore",
        Instruction::Perform { .. } => "perform",
    }
}

/// Records a `goto` through a register as an entry point.
fn note_entry_point(instruction: &Instruction, summary: &mut Summary) {
    if let Instruction::Goto(Operand::Register(Register(name))) = instruction {
        summary.entry_points.insert(name.clone());
    }
}

/// Records the registers one stack instruction touches.
fn note_stack_register(instruction: &Instruction, summary: &mut Summary) {
    let (Instruction::Save(Register(name)) | Instruction::Restore(Register(name))) = instruction
    else {
        return;
    };
    summary.stack_registers.insert(name.clone());
}

/// Records the sources of one `assign`.
fn note_sources(instruction: &Instruction, summary: &mut Summary) {
    let Instruction::Assign {
        target: Register(name),
        value,
    } = instruction
    else {
        return;
    };
    summary
        .registers
        .entry(name.clone())
        .or_default()
        .sources
        .insert(render(value));
}

/// Renders one operand the way the book prints a source: a constant
/// as `(const n)`, a register as `(reg r)`, a label as `(label l)`,
/// and an operation as `(op name args...)`.
fn render(operand: &Operand) -> String {
    match operand {
        Operand::Constant(value) => format!("(const {value})"),
        Operand::Register(Register(name)) => format!("(reg {name})"),
        Operand::Label(label) => format!("(label {})", label.0),
        Operand::Operation {
            operation,
            arguments,
        } => {
            let rendered: Vec<String> = arguments.iter().map(render).collect();
            format!("(op {operation} {})", rendered.join(" "))
        }
    }
}

mod ex_5_12 {
    //! Exercise 5.12: extend the assembler to collect the
    //! instruction types, the entry-point registers, the stacked
    //! registers, and each register's assign sources, then examine
    //! the lists for the Fibonacci machine of Figure 5.12.

    use super::*;

    /// The sources of one register, as rendered strings.
    fn sources_of(summary: &Summary, register: &str) -> Vec<String> {
        summary.registers[register]
            .sources
            .iter()
            .cloned()
            .collect()
    }

    /// The Fibonacci machine's summary: eight assigns, one test, one
    /// branch, four gotos, four saves, four restores, and one
    /// perform. `continue` is the only entry-point register and the
    /// stack sees `continue`, `n`, and `val`. The sources of `n` are
    /// its two decrements and `val`; the sources of `val` are the
    /// addition and `n` itself; the sources of `continue` are the
    /// three labels.
    #[test]
    fn ex_5_12_fibonacci_summary() {
        let summary = summarize(&fibonacci_machine());
        assert_eq!(
            summary.counts,
            [
                ("assign", 8),
                ("branch", 1),
                ("goto", 4),
                ("perform", 1),
                ("restore", 4),
                ("save", 4),
                ("test", 1),
            ]
            .into_iter()
            .map(|(kind, count)| (kind.to_owned(), count))
            .collect::<BTreeMap<String, usize>>()
        );
        assert_eq!(
            summary.entry_points,
            ["continue".to_owned()].into_iter().collect::<BTreeSet<_>>()
        );
        assert_eq!(
            summary.stack_registers,
            ["continue".to_owned(), "n".to_owned(), "val".to_owned()]
                .into_iter()
                .collect::<BTreeSet<_>>()
        );
        assert_eq!(
            sources_of(&summary, "n"),
            [
                "(op sub1 (reg n))".to_owned(),
                "(op sub2 (reg n))".to_owned(),
                "(reg val)".to_owned(),
            ]
        );
        assert_eq!(
            sources_of(&summary, "val"),
            [
                "(op add (reg val) (reg n))".to_owned(),
                "(reg n)".to_owned(),
            ]
        );
        assert_eq!(
            sources_of(&summary, "continue"),
            [
                "(label afterfibn-1)".to_owned(),
                "(label afterfibn-2)".to_owned(),
                "(label done)".to_owned(),
            ]
        );
    }

    /// The factorial machine's summary, cross-checking the same
    /// machinery: fifteen rows, five assigns, and `val`'s sources are
    /// exactly the book's example, the constant 1 and the
    /// multiplication; `n` has its single decrement; `continue` has
    /// its two labels.
    #[test]
    fn ex_5_12_factorial_summary() {
        let summary = summarize(&factorial_recursive());
        let rows: usize = summary.counts.values().sum();
        assert_eq!(rows, 15);
        assert_eq!(
            summary.entry_points,
            ["continue".to_owned()].into_iter().collect::<BTreeSet<_>>()
        );
        assert_eq!(
            summary.stack_registers,
            ["continue".to_owned(), "n".to_owned()]
                .into_iter()
                .collect::<BTreeSet<_>>()
        );
        assert_eq!(
            sources_of(&summary, "val"),
            [
                "(const 1)".to_owned(),
                "(op mul (reg val) (reg n))".to_owned(),
            ]
        );
        assert_eq!(sources_of(&summary, "n"), ["(op sub1 (reg n))".to_owned()]);
        assert_eq!(
            sources_of(&summary, "continue"),
            ["(label after)".to_owned(), "(label done)".to_owned()]
        );
    }
}
