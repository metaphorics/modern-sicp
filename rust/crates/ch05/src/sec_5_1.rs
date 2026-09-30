// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 5.1

//! Section 5.1: register-machine design. The machines of the section
//! are values in the data language of grammar §7 — registers, labels,
//! operands, and instructions — not parsed controller text. Each
//! lesson machine below is built with those constructors and names the
//! registers and operations its data-path diagram describes.

pub use sicp_runtime::host::machine::{Instruction, Label, MachineProgram, Operand, Register};

/// Builds one register declaration.
#[must_use]
pub fn reg(name: &str) -> Register {
    Register(name.to_owned())
}

/// Builds one label reference.
#[must_use]
pub fn label(name: &str) -> Label {
    Label(name.to_owned())
}

/// Builds one register operand.
#[must_use]
pub fn reg_op(name: &str) -> Operand {
    Operand::Register(reg(name))
}

/// Builds one constant operand.
#[must_use]
pub fn constant(value: i64) -> Operand {
    Operand::Constant(value)
}

/// The GCD machine of section 5.1.1: `a` and `b`, a `test` on `b`,
/// and the remainder loop.
#[must_use]
pub fn gcd_machine() -> MachineProgram {
    MachineProgram::new(
        vec![reg("a"), reg("b"), reg("t")],
        vec![
            (None, Instruction::Goto(Operand::Label(label("loop")))),
            (
                Some(label("loop")),
                Instruction::Test {
                    predicate: "=".to_owned(),
                    arguments: vec![reg_op("b"), constant(0)],
                },
            ),
            (None, Instruction::Branch(label("done"))),
            (
                None,
                Instruction::Assign {
                    target: reg("t"),
                    value: Operand::Operation {
                        operation: "rem".to_owned(),
                        arguments: vec![reg_op("a"), reg_op("b")],
                    },
                },
            ),
            (
                None,
                Instruction::Assign {
                    target: reg("a"),
                    value: reg_op("b"),
                },
            ),
            (
                None,
                Instruction::Assign {
                    target: reg("b"),
                    value: reg_op("t"),
                },
            ),
            (None, Instruction::Goto(Operand::Label(label("loop")))),
            (
                Some(label("done")),
                Instruction::Perform {
                    operation: "print".to_owned(),
                    arguments: vec![reg_op("a")],
                },
            ),
        ],
    )
}

/// The iterative factorial machine of exercise 5.1: `product` and
/// `counter` with the `>` test.
#[must_use]
pub fn factorial_iterative() -> MachineProgram {
    MachineProgram::new(
        vec![reg("n"), reg("product"), reg("counter")],
        vec![
            (
                None,
                Instruction::Assign {
                    target: reg("product"),
                    value: constant(1),
                },
            ),
            (
                None,
                Instruction::Assign {
                    target: reg("counter"),
                    value: constant(1),
                },
            ),
            (
                Some(label("loop")),
                Instruction::Test {
                    predicate: ">".to_owned(),
                    arguments: vec![reg_op("counter"), reg_op("n")],
                },
            ),
            (None, Instruction::Branch(label("done"))),
            (
                None,
                Instruction::Assign {
                    target: reg("product"),
                    value: Operand::Operation {
                        operation: "mul".to_owned(),
                        arguments: vec![reg_op("product"), reg_op("counter")],
                    },
                },
            ),
            (
                None,
                Instruction::Assign {
                    target: reg("counter"),
                    value: Operand::Operation {
                        operation: "add1".to_owned(),
                        arguments: vec![reg_op("counter")],
                    },
                },
            ),
            (None, Instruction::Goto(Operand::Label(label("loop")))),
            (
                Some(label("done")),
                Instruction::Perform {
                    operation: "print".to_owned(),
                    arguments: vec![reg_op("product")],
                },
            ),
        ],
    )
}

/// The recursive factorial machine of section 5.1.2: the explicit
/// stack with `save` and `restore`.
#[must_use]
pub fn factorial_recursive() -> MachineProgram {
    MachineProgram::new(
        vec![reg("n"), reg("val"), reg("continue")],
        vec![
            (
                None,
                Instruction::Assign {
                    target: reg("continue"),
                    value: Operand::Label(label("done")),
                },
            ),
            (
                Some(label("loop")),
                Instruction::Test {
                    predicate: "=".to_owned(),
                    arguments: vec![reg_op("n"), constant(1)],
                },
            ),
            (None, Instruction::Branch(label("base"))),
            (None, Instruction::Save(reg("continue"))),
            (None, Instruction::Save(reg("n"))),
            (
                None,
                Instruction::Assign {
                    target: reg("n"),
                    value: Operand::Operation {
                        operation: "sub1".to_owned(),
                        arguments: vec![reg_op("n")],
                    },
                },
            ),
            (
                None,
                Instruction::Assign {
                    target: reg("continue"),
                    value: Operand::Label(label("after")),
                },
            ),
            (None, Instruction::Goto(Operand::Label(label("loop")))),
            (Some(label("after")), Instruction::Restore(reg("n"))),
            (None, Instruction::Restore(reg("continue"))),
            (
                None,
                Instruction::Assign {
                    target: reg("val"),
                    value: Operand::Operation {
                        operation: "mul".to_owned(),
                        arguments: vec![reg_op("val"), reg_op("n")],
                    },
                },
            ),
            (None, Instruction::Goto(Operand::Register(reg("continue")))),
            (
                Some(label("base")),
                Instruction::Assign {
                    target: reg("val"),
                    value: constant(1),
                },
            ),
            (None, Instruction::Goto(Operand::Register(reg("continue")))),
            (
                Some(label("done")),
                Instruction::Perform {
                    operation: "print".to_owned(),
                    arguments: vec![reg_op("val")],
                },
            ),
        ],
    )
}

/// The Fibonacci machine of section 5.1.3.
#[must_use]
pub fn fibonacci_machine() -> MachineProgram {
    MachineProgram::new(
        vec![reg("n"), reg("val"), reg("continue")],
        vec![
            (
                None,
                Instruction::Assign {
                    target: reg("continue"),
                    value: Operand::Label(label("done")),
                },
            ),
            (
                Some(label("loop")),
                Instruction::Test {
                    predicate: "<".to_owned(),
                    arguments: vec![reg_op("n"), constant(2)],
                },
            ),
            (None, Instruction::Branch(label("base"))),
            (None, Instruction::Save(reg("continue"))),
            (
                None,
                Instruction::Assign {
                    target: reg("continue"),
                    value: Operand::Label(label("afterfibn-1")),
                },
            ),
            (None, Instruction::Save(reg("n"))),
            (
                None,
                Instruction::Assign {
                    target: reg("n"),
                    value: Operand::Operation {
                        operation: "sub1".to_owned(),
                        arguments: vec![reg_op("n")],
                    },
                },
            ),
            (None, Instruction::Goto(Operand::Label(label("loop")))),
            (Some(label("afterfibn-1")), Instruction::Restore(reg("n"))),
            (None, Instruction::Restore(reg("continue"))),
            (None, Instruction::Save(reg("continue"))),
            (
                None,
                Instruction::Assign {
                    target: reg("continue"),
                    value: Operand::Label(label("afterfibn-2")),
                },
            ),
            (None, Instruction::Save(reg("val"))),
            (
                None,
                Instruction::Assign {
                    target: reg("n"),
                    value: Operand::Operation {
                        operation: "sub2".to_owned(),
                        arguments: vec![reg_op("n")],
                    },
                },
            ),
            (None, Instruction::Goto(Operand::Label(label("loop")))),
            (
                Some(label("afterfibn-2")),
                Instruction::Assign {
                    target: reg("n"),
                    value: reg_op("val"),
                },
            ),
            (None, Instruction::Restore(reg("val"))),
            (None, Instruction::Restore(reg("continue"))),
            (
                None,
                Instruction::Assign {
                    target: reg("val"),
                    value: Operand::Operation {
                        operation: "add".to_owned(),
                        arguments: vec![reg_op("val"), reg_op("n")],
                    },
                },
            ),
            (None, Instruction::Goto(Operand::Register(reg("continue")))),
            (
                Some(label("base")),
                Instruction::Assign {
                    target: reg("val"),
                    value: reg_op("n"),
                },
            ),
            (None, Instruction::Goto(Operand::Register(reg("continue")))),
            (
                Some(label("done")),
                Instruction::Perform {
                    operation: "print".to_owned(),
                    arguments: vec![reg_op("val")],
                },
            ),
        ],
    )
}
