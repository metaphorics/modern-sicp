// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.2: the iterative factorial
//! controller written out in the register-machine data language.

use ch05::sec_5_1::{
    Instruction, Label, MachineProgram, Operand, Register, constant, factorial_iterative, label,
    reg, reg_op,
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

mod ex_5_02 {
    //! Exercise 5.2: describe the iterative factorial machine of
    //! exercise 5.1 in the register-machine language.

    use super::*;

    /// The controller the exercise asks the reader to write, in the
    /// edition's data language: registers, label, and instruction
    /// constructors in controller order. Each computed assignment
    /// sources an operation operand, so `mul(product, counter)` is
    /// the book's `product <- counter * product` and `add1(counter)`
    /// is `counter <- counter + 1`.
    ///
    /// ```text
    ///   assign product <- 1
    ///   assign counter <- 1
    /// test-counter
    ///   test >(counter, n)
    ///   branch factorial-done
    ///   assign product <- mul(product, counter)
    ///   assign counter <- add1(counter)
    ///   goto test-counter
    /// factorial-done
    ///   perform print(product)
    /// ```
    fn controller() -> MachineProgram {
        let registers: Vec<Register> = ["n", "product", "counter"].iter().map(|n| reg(n)).collect();
        let instructions = vec![
            row(assign("product", constant(1))),
            row(assign("counter", constant(1))),
            at(
                "test-counter",
                Instruction::Test {
                    predicate: ">".to_owned(),
                    arguments: vec![source("counter"), source("n")],
                },
            ),
            row(Instruction::Branch(label("factorial-done"))),
            row(assign(
                "product",
                operation("mul", &[source("product"), source("counter")]),
            )),
            row(assign("counter", operation("add1", &[source("counter")]))),
            row(Instruction::Goto(Operand::Label(label("test-counter")))),
            at(
                "factorial-done",
                Instruction::Perform {
                    operation: "print".to_owned(),
                    arguments: vec![source("product")],
                },
            ),
        ];
        MachineProgram::new(registers, instructions)
    }

    /// Runs one machine to completion and answers its product
    /// register and push count.
    fn run_of(program: &MachineProgram, n: i64) -> Result<(i64, u64), Fault> {
        let mut machine = Machine::new(assemble(program)?);
        machine.set_register("n", n)?;
        machine.run()?;
        let product = machine.get_register("product")?;
        Ok((product, machine.stack_statistics().pushes))
    }

    #[test]
    fn ex_5_02_written_controller_agrees_with_the_machine() -> Result<(), Fault> {
        // The written-out controller is the section's machine: same
        // answer and same stack discipline on every input tried, the
        // same loop instruction for instruction.
        for n in 0..=10 {
            let expected = (1..=n).product::<i64>().max(1);
            let (written, written_pushes) = run_of(&controller(), n)?;
            let (lesson, lesson_pushes) = run_of(&factorial_iterative(), n)?;
            assert_eq!(written, expected, "written controller on n = {n}");
            assert_eq!(lesson, expected, "lesson machine on n = {n}");
            assert_eq!(written_pushes, 0);
            assert_eq!(lesson_pushes, 0);
        }
        Ok(())
    }

    #[test]
    fn ex_5_02_written_controller_runs() -> Result<(), Fault> {
        for (n, factorial) in [(4, 24), (10, 3_628_800)] {
            let (product, pushes) = run_of(&controller(), n)?;
            assert_eq!(product, factorial);
            assert_eq!(pushes, 0);
        }
        Ok(())
    }
}
