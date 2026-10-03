// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.9: an operation input may be
//! a register or a constant, never a label; the grammar refuses the
//! label at assembly time.

use ch05::sec_5_1::{Instruction, Label, MachineProgram, Operand, constant, label, reg, reg_op};
use ch05::sec_5_2::{Fault, Machine, assemble};

/// The repair the exercise asks for: a label in operation-operand
/// position is a typed assembly error naming the operation and the
/// refused label.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssemblyError {
    /// An operation was given a label as one of its inputs.
    LabelOperand {
        /// The operation that refused the input.
        operation: String,
        /// The refused label's name.
        label: String,
    },
    /// The underlying assembler's refusal.
    Machine(Fault),
}

/// The operation-valued operand the book writes
/// `(assign target (op name) args...)` as.
fn operation(name: &str, arguments: &[Operand]) -> Operand {
    Operand::Operation {
        operation: name.to_owned(),
        arguments: arguments.to_vec(),
    }
}

/// `assign` from an operand.
fn assign(name: &str, value: Operand) -> Instruction {
    Instruction::Assign {
        target: reg(name),
        value,
    }
}

/// The exercise's controller: the two sums of its first question.
fn registers_and_constants() -> MachineProgram {
    let instructions = sum_rows(&[constant(2), constant(3)]);
    MachineProgram::new(vec![reg("a"), reg("b"), reg("c"), reg("d")], instructions)
}

/// The two `add` rows and the `there` target shared by both
/// variants; `second_sum` supplies the second `add`'s inputs.
fn sum_rows(second_sum: &[Operand]) -> Vec<(Option<Label>, Instruction)> {
    vec![
        (
            None,
            assign("a", operation("add", &[reg_op("b"), reg_op("c")])),
        ),
        (None, assign("d", operation("add", second_sum))),
        (
            Some(label("there")),
            Instruction::Perform {
                operation: "print".to_owned(),
                arguments: vec![reg_op("a")],
            },
        ),
    ]
}

/// The controller with one label-typed operation input, alone.
fn lone_label_input() -> MachineProgram {
    MachineProgram::new(
        vec![reg("b")],
        vec![
            (
                None,
                assign("b", operation("add", &[Operand::Label(label("there"))])),
            ),
            (
                Some(label("there")),
                Instruction::Perform {
                    operation: "print".to_owned(),
                    arguments: vec![reg_op("b")],
                },
            ),
        ],
    )
}

/// The controller with a label among otherwise valid inputs.
fn label_among_valid_inputs() -> MachineProgram {
    MachineProgram::new(
        vec![reg("b")],
        vec![
            (
                None,
                assign(
                    "b",
                    operation("add", &[reg_op("b"), Operand::Label(label("there"))]),
                ),
            ),
            (
                Some(label("there")),
                Instruction::Perform {
                    operation: "print".to_owned(),
                    arguments: vec![reg_op("b")],
                },
            ),
        ],
    )
}

/// The repair: every operation input is checked before the machine
/// is built, at assembly, before it can start.
fn assemble_checked(program: &MachineProgram) -> Result<Machine, AssemblyError> {
    for (_, instruction) in &program.instructions {
        let arguments = match instruction {
            Instruction::Assign { value, .. } => operand_arguments(value),
            Instruction::Test { arguments, .. } | Instruction::Perform { arguments, .. } => {
                arguments.clone()
            }
            _ => Vec::new(),
        };
        for argument in &arguments {
            if let Operand::Label(Label(name)) = argument {
                return Err(AssemblyError::LabelOperand {
                    operation: operation_of(instruction),
                    label: name.clone(),
                });
            }
        }
    }
    assemble(program)
        .map(Machine::new)
        .map_err(AssemblyError::Machine)
}

/// The operation arguments one assignment's value carries, if any.
fn operand_arguments(value: &Operand) -> Vec<Operand> {
    match value {
        Operand::Operation { arguments, .. } => arguments.clone(),
        _ => Vec::new(),
    }
}

/// The name of the operation one instruction runs.
fn operation_of(instruction: &Instruction) -> String {
    match instruction {
        Instruction::Assign {
            value: Operand::Operation { operation, .. },
            ..
        }
        | Instruction::Test {
            predicate: operation,
            ..
        }
        | Instruction::Perform { operation, .. } => operation.clone(),
        _ => String::new(),
    }
}

mod ex_5_09 {
    //! Exercise 5.9: modify the expression-processing procedures so
    //! operations can be used only with registers and constants.

    use super::*;

    /// The book's permissive reading: `add` over two registers and
    /// over two constants both answer 5.
    #[test]
    fn ex_5_09_registers_and_constants_still_work() -> Result<(), Fault> {
        let mut machine = Machine::new(assemble(&registers_and_constants())?);
        machine.set_register("b", 2)?;
        machine.set_register("c", 3)?;
        machine.run()?;
        assert_eq!(machine.get_register("a")?, 5);
        assert_eq!(machine.get_register("d")?, 5);
        Ok(())
    }

    /// A label in operand position now fails the assembly with the
    /// typed [`AssemblyError::LabelOperand`], naming the operation and
    /// the refused label.
    #[test]
    fn ex_5_09_label_operand_fails_assembly() {
        let error = assemble_checked(&lone_label_input()).expect_err("label input is refused");
        assert_eq!(
            error,
            AssemblyError::LabelOperand {
                operation: "add".to_owned(),
                label: "there".to_owned()
            }
        );
    }

    /// The instruction's other operand does not excuse the label:
    /// the check is per input, at assembly, before the machine can
    /// start.
    #[test]
    fn ex_5_09_label_among_valid_inputs_still_refused() {
        let error =
            assemble_checked(&label_among_valid_inputs()).expect_err("label input is refused");
        assert!(matches!(error, AssemblyError::LabelOperand { .. }));
    }

    /// The repaired assembler accepts the registers-and-constants
    /// controller unchanged: the refusal is about labels only.
    #[test]
    fn ex_5_09_repair_accepts_the_valid_controller() -> Result<(), AssemblyError> {
        assemble_checked(&registers_and_constants())?;
        Ok(())
    }
}
