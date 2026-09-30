// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.7: the two exponentiation
//! machines of exercise 5.4, run on the section's simulator.

use ch05::sec_5_1::{
    Instruction, Label, MachineProgram, Operand, Register, constant, label, reg, reg_op,
};
use ch05::sec_5_2::{Fault, Machine, assemble};

/// One row of a controller: an optional leading label and its
/// instruction.
type Row = (Option<Label>, Instruction);

/// One unlabeled instruction row.
fn row(instruction: Instruction) -> Row {
    (None, instruction)
}

/// One labeled instruction row.
fn at(name: &str, instruction: Instruction) -> Row {
    (Some(label(name)), instruction)
}

/// `assign` from an operand.
fn assign(name: &str, value: Operand) -> Instruction {
    Instruction::Assign {
        target: reg(name),
        value,
    }
}

/// The named register operand shorthand.
fn source(name: &str) -> Operand {
    reg_op(name)
}

/// The operation-valued operand the book writes
/// `(assign target (op name) args...)` as.
fn operation(name: &str, arguments: &[Operand]) -> Operand {
    Operand::Operation {
        operation: name.to_owned(),
        arguments: arguments.to_vec(),
    }
}

/// The linear-recursive exponentiation machine of exercise 5.4: `n`
/// counts down, each level saves `continue`, and the pending
/// multiplications ride the stack.
fn recursive_exponent_machine() -> MachineProgram {
    let registers: Vec<Register> = ["b", "n", "val", "continue"]
        .iter()
        .map(|name| reg(name))
        .collect();
    let instructions = vec![
        row(assign("continue", Operand::Label(label("expt-done")))),
        at(
            "expt-loop",
            Instruction::Test {
                predicate: "=".to_owned(),
                arguments: vec![source("n"), constant(0)],
            },
        ),
        row(Instruction::Branch(label("base-expt"))),
        row(Instruction::Save(reg("continue"))),
        row(assign("n", operation("sub1", &[source("n")]))),
        row(assign("continue", Operand::Label(label("multiply")))),
        row(Instruction::Goto(Operand::Label(label("expt-loop")))),
        at(
            "multiply",
            assign("val", operation("mul", &[source("b"), source("val")])),
        ),
        row(Instruction::Restore(reg("continue"))),
        row(Instruction::Goto(Operand::Register(reg("continue")))),
        at("base-expt", assign("val", constant(1))),
        row(Instruction::Goto(Operand::Register(reg("continue")))),
        at(
            "expt-done",
            Instruction::Perform {
                operation: "print".to_owned(),
                arguments: vec![source("val")],
            },
        ),
    ];
    MachineProgram::new(registers, instructions)
}

/// The iterative exponentiation machine of exercise 5.4: a product
/// accumulator and a counter, no stack.
fn iterative_exponent_machine() -> MachineProgram {
    let registers: Vec<Register> = ["b", "n", "counter", "product"]
        .iter()
        .map(|name| reg(name))
        .collect();
    let instructions = vec![
        row(assign("counter", source("n"))),
        row(assign("product", constant(1))),
        at(
            "expt-iter",
            Instruction::Test {
                predicate: "=".to_owned(),
                arguments: vec![source("counter"), constant(0)],
            },
        ),
        row(Instruction::Branch(label("expt-done"))),
        row(assign(
            "product",
            operation("mul", &[source("product"), source("b")]),
        )),
        row(assign("counter", operation("sub1", &[source("counter")]))),
        row(Instruction::Goto(Operand::Label(label("expt-iter")))),
        at(
            "expt-done",
            Instruction::Perform {
                operation: "print".to_owned(),
                arguments: vec![source("product")],
            },
        ),
    ];
    MachineProgram::new(registers, instructions)
}

/// Runs one machine to completion and answers its result register,
/// push count, and maximum stack depth.
fn run_machine(
    program: &MachineProgram,
    result: &str,
    b: i64,
    n: i64,
) -> Result<(i64, u64, usize), Fault> {
    let mut machine = Machine::new(assemble(program)?);
    machine.set_register("b", b)?;
    machine.set_register("n", n)?;
    machine.run()?;
    let stats = machine.stack_statistics();
    let value = machine.get_register(result)?;
    Ok((value, stats.pushes, stats.max_depth))
}

mod ex_5_07 {
    //! Exercise 5.7: use the simulator to test the machines designed
    //! in exercise 5.4.

    use super::*;

    /// The recursive machine answers `b^n` and pays one save and one
    /// restore per level: pushes and depth are both `n`.
    #[test]
    fn ex_5_07_recursive() -> Result<(), Fault> {
        for b in 2..=5 {
            for n in 0..=9 {
                let (value, pushes, depth) =
                    run_machine(&recursive_exponent_machine(), "val", b, n)?;
                let levels = u32::try_from(n).expect("small exponent");
                assert_eq!(value, b.pow(levels), "recursive expt({b}, {n})");
                assert_eq!(pushes, u64::from(levels), "recursive expt({b}, {n}) pushes");
                assert_eq!(depth, levels as usize, "recursive expt({b}, {n}) depth");
            }
        }
        Ok(())
    }

    /// The iterative machine answers the same values and never
    /// touches the stack.
    #[test]
    fn ex_5_07_iterative() -> Result<(), Fault> {
        for b in 2..=5 {
            for n in 0..=9 {
                let (value, pushes, depth) =
                    run_machine(&iterative_exponent_machine(), "product", b, n)?;
                let levels = u32::try_from(n).expect("small exponent");
                assert_eq!(value, b.pow(levels), "iterative expt({b}, {n})");
                assert_eq!(pushes, 0, "iterative expt({b}, {n}) pushes");
                assert_eq!(depth, 0, "iterative expt({b}, {n}) depth");
            }
        }
        Ok(())
    }

    /// The book's worked inputs, pinned literally: 2^10 = 1024 and
    /// 3^5 = 243 on both machines.
    #[test]
    fn ex_5_07_pinned_inputs() -> Result<(), Fault> {
        let (recursive, ..) = run_machine(&recursive_exponent_machine(), "val", 2, 10)?;
        assert_eq!(recursive, 1024);
        let (iterative, ..) = run_machine(&iterative_exponent_machine(), "product", 3, 5)?;
        assert_eq!(iterative, 243);
        Ok(())
    }
}
