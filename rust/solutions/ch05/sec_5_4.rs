// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.4: recursive and iterative
//! exponentiation machines, both built and run.

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

/// `goto` by label name.
fn jump(name: &str) -> Instruction {
    Instruction::Goto(Operand::Label(label(name)))
}

/// `goto` through a register.
fn jump_register(name: &str) -> Instruction {
    Instruction::Goto(Operand::Register(reg(name)))
}

/// The linear-recursive exponentiation machine: `n` counts down, each
/// level saves `continue`, and the pending multiplications ride the
/// stack. `n` itself is dead after the recursive call, so it needs no
/// save: the answer is `b * expt(b, n-1)`, and only the return label
/// crosses the call. The data paths the exercise asks to draw:
///
/// ```text
///     n, 0  --> (=) ----> to the controller (test)
///     n     --> (-1) --> n                    button n <- n-1
///     val, b --> (*) --> val                  button val <- b*val
///     continue --> (save)/(restore) --> the stack
/// ```
#[must_use]
pub fn expt_recursive() -> MachineProgram {
    let registers: Vec<Register> = ["b", "n", "val", "continue"]
        .iter()
        .map(|name| reg(name))
        .collect();
    let instructions = vec![
        row(assign("continue", Operand::Label(label("expt-done")))),
        at(
            "loop",
            Instruction::Test {
                predicate: "=".to_owned(),
                arguments: vec![source("n"), constant(0)],
            },
        ),
        row(Instruction::Branch(label("base"))),
        row(Instruction::Save(reg("continue"))),
        row(assign("n", operation("sub1", &[source("n")]))),
        row(assign("continue", Operand::Label(label("after")))),
        row(jump("loop")),
        at("after", Instruction::Restore(reg("continue"))),
        row(assign(
            "val",
            operation("mul", &[source("val"), source("b")]),
        )),
        row(jump_register("continue")),
        at("base", assign("val", constant(1))),
        row(jump_register("continue")),
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

/// The iterative exponentiation machine: a product accumulator and a
/// countdown `n`, no stack. Its controller:
///
/// ```text
///   assign product <- 1
/// loop
///   test =(n, 0);  branch expt-done
///   assign product <- product * b
///   assign n <- n-1
///   goto loop
/// expt-done
///   perform print(product)
/// ```
#[must_use]
pub fn expt_iterative() -> MachineProgram {
    let registers: Vec<Register> = ["b", "n", "product"].iter().map(|name| reg(name)).collect();
    let instructions = vec![
        row(assign("product", constant(1))),
        at(
            "loop",
            Instruction::Test {
                predicate: "=".to_owned(),
                arguments: vec![source("n"), constant(0)],
            },
        ),
        row(Instruction::Branch(label("expt-done"))),
        row(assign(
            "product",
            operation("mul", &[source("product"), source("b")]),
        )),
        row(assign("n", operation("sub1", &[source("n")]))),
        row(jump("loop")),
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

/// Runs one machine and answers its value register, push count,
/// maximum stack depth, and executed-instruction count.
fn run_machine(
    program: &MachineProgram,
    result: &str,
    b: i64,
    n: i64,
) -> Result<(i64, u64, usize, u64), Fault> {
    let mut machine = Machine::new(assemble(program)?);
    machine.set_register("b", b)?;
    machine.set_register("n", n)?;
    let run = machine.run()?;
    let stats = machine.stack_statistics();
    let value = machine.get_register(result)?;
    Ok((value, stats.pushes, stats.max_depth, run.steps))
}

mod ex_5_04 {
    //! Exercise 5.4: controller sequences for recursive and iterative
    //! exponentiation.

    use super::*;

    #[test]
    fn ex_5_04_both_machines_compute_exponentiation() -> Result<(), Fault> {
        for (b, n, power) in [(2, 5, 32), (3, 4, 81), (2, 0, 1)] {
            let (recursive, ..) = run_machine(&expt_recursive(), "val", b, n)?;
            let (iterative, ..) = run_machine(&expt_iterative(), "product", b, n)?;
            assert_eq!(recursive, power, "recursive expt({b}, {n})");
            assert_eq!(iterative, power, "iterative expt({b}, {n})");
        }
        Ok(())
    }

    #[test]
    fn ex_5_04_recursive_machine_pays_one_push_per_level() -> Result<(), Fault> {
        // Each level saves only the return label: five pushes and
        // depth five for n = 5, four of each for n = 4.
        let (_, pushes, depth, _) = run_machine(&expt_recursive(), "val", 2, 5)?;
        assert_eq!((pushes, depth), (5, 5));
        let (_, pushes, depth, _) = run_machine(&expt_recursive(), "val", 3, 4)?;
        assert_eq!((pushes, depth), (4, 4));
        Ok(())
    }

    #[test]
    fn ex_5_04_iterative_machine_never_touches_the_stack() -> Result<(), Fault> {
        for (b, n) in [(2, 5), (3, 4), (2, 0)] {
            let (_, pushes, depth, _) = run_machine(&expt_iterative(), "product", b, n)?;
            assert_eq!(pushes, 0, "expt({b}, {n})");
            assert_eq!(depth, 0, "expt({b}, {n})");
        }
        Ok(())
    }

    #[test]
    fn ex_5_04_machines_run_on_the_simulator() -> Result<(), Fault> {
        // The recursive machine executes 51 instructions for n = 5
        // and 42 for n = 4; the iterative machine answers n = 5 in 29.
        let (.., recursive_five) = run_machine(&expt_recursive(), "val", 2, 5)?;
        let (.., recursive_four) = run_machine(&expt_recursive(), "val", 3, 4)?;
        let (.., iterative_five) = run_machine(&expt_iterative(), "product", 2, 5)?;
        assert_eq!(
            (recursive_five, recursive_four, iterative_five),
            (51, 42, 29)
        );
        Ok(())
    }
}
