// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.3: the square-root machine in
//! two stages, both built and run.
//!
//! The machine's value model is `i64`, so the iteration runs on the
//! edition's fixed-point representation: every value is the true
//! value scaled by [`SCALE`] (2^20). The book's tolerance `0.001`
//! becomes [`TOLERANCE`] scale units.

use std::rc::Rc;

use ch05::sec_5_1::{Instruction, Label, MachineProgram, Operand, constant, label, reg, reg_op};
use ch05::sec_5_2::{Fault, Machine, OpHandler, assemble};

/// The fixed-point scale: one, as the machine stores it.
const SCALE: i64 = 1_048_576;

/// The book's `0.001` in scale units, truncated: a residual of at
/// most this many units is `|guess^2 - x| <= 0.001`.
const TOLERANCE: i64 = 1048;

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

/// The `improve` primitive of the first stage: one Newton step on the
/// scaled representation, `guess' = (guess + x / guess) / 2`.
fn improve_operation() -> OpHandler {
    Rc::new(|args: &[i64]| {
        let [guess, x] = args else {
            return Err(Fault::UndefinedOperation("improve".to_owned()));
        };
        Ok((guess + (x * SCALE) / guess) / 2)
    })
}

/// The `good-enough?` primitive of the first stage: `|guess^2 - x|`
/// at or below the tolerance, all in scale units.
fn good_enough_operation() -> OpHandler {
    Rc::new(|args: &[i64]| {
        let [guess, x] = args else {
            return Err(Fault::UndefinedOperation("good-enough?".to_owned()));
        };
        let residual = ((guess * guess) / SCALE - x).abs();
        Ok(i64::from(residual <= TOLERANCE))
    })
}

/// The arithmetic the expanded stage drives: integer division,
/// subtraction, and absolute value join the standard `mul` and `add`.
fn install_arithmetic(machine: &mut Machine) {
    machine.install_operation(
        "div",
        Rc::new(|args: &[i64]| {
            let [a, b] = args else {
                return Err(Fault::UndefinedOperation("div".to_owned()));
            };
            if *b == 0 {
                return Err(Fault::DivByZero);
            }
            Ok(a / b)
        }),
    );
    machine.install_operation(
        "sub",
        Rc::new(|args: &[i64]| {
            let [a, b] = args else {
                return Err(Fault::UndefinedOperation("sub".to_owned()));
            };
            Ok(a - b)
        }),
    );
    machine.install_operation(
        "abs",
        Rc::new(|args: &[i64]| {
            let [a] = args else {
                return Err(Fault::UndefinedOperation("abs".to_owned()));
            };
            Ok(a.abs())
        }),
    );
}

mod ex_5_03 {
    //! Exercise 5.3: design a square-root machine on Newton's method,
    //! with the compound operations first primitive, then expanded.

    use super::*;

    /// The data-path descriptions the exercise asks to draw, in the
    /// book's notation.
    ///
    /// ```text
    /// First stage, the two compound operations as primitive boxes:
    ///
    ///     guess, x --> (good-enough?) --> to the controller (test)
    ///     guess, x --> (improve) -------> guess
    ///
    /// Second stage, arithmetic only, with t holding each
    /// intermediate value:
    ///
    ///     guess, guess --> (*) ---> t
    ///     t, S          --> (/) ---> t
    ///     t, x          --> (-) ---> t
    ///     t             --> (abs) -> t
    ///     t, tol        --> (<) ---> to the controller (test)
    ///     x, S          --> (*) ---> t
    ///     t, guess      --> (/) ---> t
    ///     t, guess      --> (+) ---> t
    ///     t, 2          --> (/) ---> guess
    /// ```
    ///
    /// First-stage controller: `good-enough?` and `improve` as
    /// primitive machine operations.
    fn sqrt_primitive() -> MachineProgram {
        MachineProgram::new(
            vec![reg("x"), reg("guess")],
            vec![
                row(assign("guess", constant(SCALE))),
                at(
                    "loop",
                    Instruction::Test {
                        predicate: "good-enough?".to_owned(),
                        arguments: vec![source("guess"), source("x")],
                    },
                ),
                row(Instruction::Branch(label("done"))),
                row(assign(
                    "guess",
                    operation("improve", &[source("guess"), source("x")]),
                )),
                row(Instruction::Goto(Operand::Label(label("loop")))),
                at(
                    "done",
                    Instruction::Perform {
                        operation: "print".to_owned(),
                        arguments: vec![source("guess")],
                    },
                ),
            ],
        )
    }

    /// Second-stage controller: the same iteration with every
    /// compound operation expanded into arithmetic on `t`.
    fn sqrt_expanded() -> MachineProgram {
        MachineProgram::new(
            vec![reg("x"), reg("guess"), reg("t")],
            vec![
                row(assign("guess", constant(SCALE))),
                at(
                    "loop",
                    assign("t", operation("mul", &[source("guess"), source("guess")])),
                ),
                row(assign(
                    "t",
                    operation("div", &[source("t"), constant(SCALE)]),
                )),
                row(assign("t", operation("sub", &[source("t"), source("x")]))),
                row(assign("t", operation("abs", &[source("t")]))),
                row(Instruction::Test {
                    predicate: "<".to_owned(),
                    arguments: vec![source("t"), constant(TOLERANCE + 1)],
                }),
                row(Instruction::Branch(label("done"))),
                row(assign(
                    "t",
                    operation("mul", &[source("x"), constant(SCALE)]),
                )),
                row(assign(
                    "t",
                    operation("div", &[source("t"), source("guess")]),
                )),
                row(assign(
                    "t",
                    operation("add", &[source("t"), source("guess")]),
                )),
                row(assign(
                    "guess",
                    operation("div", &[source("t"), constant(2)]),
                )),
                row(Instruction::Goto(Operand::Label(label("loop")))),
                at(
                    "done",
                    Instruction::Perform {
                        operation: "print".to_owned(),
                        arguments: vec![source("guess")],
                    },
                ),
            ],
        )
    }

    /// Runs one stage on one scaled input and answers its guess, the
    /// number of executed steps, and its push count.
    fn run_stage(program: &MachineProgram, x: i64) -> Result<(i64, u64, u64), Fault> {
        let mut machine = Machine::new(assemble(program)?);
        install_arithmetic(&mut machine);
        machine.install_operation("improve", improve_operation());
        machine.install_operation("good-enough?", good_enough_operation());
        machine.set_register("x", x)?;
        let run = machine.run()?;
        let guess = machine.get_register("guess")?;
        Ok((guess, run.steps, machine.stack_statistics().pushes))
    }

    #[test]
    fn ex_5_03_stages_agree_and_converge() -> Result<(), Fault> {
        for (x, scaled_x, root) in [(2, 2 * SCALE, 1_482_912), (9, 9 * SCALE, 3_145_823)] {
            let (primitive, ..) = run_stage(&sqrt_primitive(), scaled_x)?;
            let (expanded, ..) = run_stage(&sqrt_expanded(), scaled_x)?;
            assert_eq!(primitive, expanded, "the two stages on x = {x}");
            assert_eq!(primitive, root, "the root of {x}");
            let residual = ((root * root) / SCALE - scaled_x).abs();
            assert!(residual <= TOLERANCE, "residual {residual} on x = {x}");
        }
        Ok(())
    }

    #[test]
    fn ex_5_03_both_stages_run_without_the_stack() -> Result<(), Fault> {
        for x in [2 * SCALE, 9 * SCALE] {
            let (.., primitive_pushes) = run_stage(&sqrt_primitive(), x)?;
            let (.., expanded_pushes) = run_stage(&sqrt_expanded(), x)?;
            assert_eq!(primitive_pushes, 0);
            assert_eq!(expanded_pushes, 0);
        }
        Ok(())
    }

    #[test]
    fn ex_5_03_expanded_stage_costs_more_instructions() -> Result<(), Fault> {
        // The expansion is visible in the work: three Newton steps on
        // x = 2 cost 16 instructions as primitives and 41 expanded;
        // four steps on x = 9 cost 20 and 52.
        let (_, primitive_two, _) = run_stage(&sqrt_primitive(), 2 * SCALE)?;
        let (_, expanded_two, _) = run_stage(&sqrt_expanded(), 2 * SCALE)?;
        let (_, primitive_nine, _) = run_stage(&sqrt_primitive(), 9 * SCALE)?;
        let (_, expanded_nine, _) = run_stage(&sqrt_expanded(), 9 * SCALE)?;
        assert_eq!((primitive_two, expanded_two), (16, 41));
        assert_eq!((primitive_nine, expanded_nine), (20, 52));
        Ok(())
    }
}
