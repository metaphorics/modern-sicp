// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.13: machines whose register
//! sets are read out of the typed controller.

use ch05::sec_5_1::{Instruction, MachineProgram, Operand, Register, constant, label, reg, reg_op};
use ch05::sec_5_2::{Fault, Machine, assemble};

/// The registers one program names, in first-seen order: the derived
/// declaration list the exercise asks for. The machine's own `flag`
/// is never derived, because no controller names it.
pub fn registers_in(program: &MachineProgram) -> Vec<Register> {
    let mut names: Vec<String> = Vec::new();
    for (_, instruction) in &program.instructions {
        for name in names_of(instruction) {
            if !names.contains(&name) {
                names.push(name);
            }
        }
    }
    names.into_iter().map(Register).collect()
}

/// Every register one instruction names, in appearance order.
fn names_of(instruction: &Instruction) -> Vec<String> {
    let mut names = Vec::new();
    let note = |names: &mut Vec<String>, name: &str| names.push(name.to_owned());
    let note_operand = |names: &mut Vec<String>, operand: &Operand| {
        for source in operands_of(operand) {
            if let Operand::Register(Register(name)) = source {
                names.push(name.clone());
            }
        }
    };
    match instruction {
        Instruction::Assign {
            target: Register(name),
            value,
        } => {
            note(&mut names, name);
            note_operand(&mut names, value);
        }
        Instruction::Test { arguments, .. } | Instruction::Perform { arguments, .. } => {
            for argument in arguments {
                note_operand(&mut names, argument);
            }
        }
        Instruction::Goto(operand) => note_operand(&mut names, operand),
        Instruction::Save(Register(name)) | Instruction::Restore(Register(name)) => {
            note(&mut names, name);
        }
        Instruction::Branch(_) => {}
    }
    names
}

/// The operand tree of one value, flattened.
fn operands_of(value: &Operand) -> Vec<&Operand> {
    match value {
        Operand::Operation { arguments, .. } => arguments.iter().flat_map(operands_of).collect(),
        other => vec![other],
    }
}

/// Builds one machine from its controller alone: the register list
/// is derived, never supplied.
/// # Errors
/// Returns the assembly fault if the controller references an unbound register or label.
pub fn machine_from_controller(program: &MachineProgram) -> Result<Machine, Fault> {
    let derived = MachineProgram::new(registers_in(program), program.instructions.clone());
    assemble(&derived).map(Machine::new)
}

/// The GCD controller of Figure 5.4 without the read loop.
fn gcd_program() -> MachineProgram {
    let instructions = vec![
        (
            Some(label("test-b")),
            Instruction::Test {
                predicate: "=".to_owned(),
                arguments: vec![reg_op("b"), constant(0)],
            },
        ),
        (None, Instruction::Branch(label("gcd-done"))),
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
        (None, Instruction::Goto(Operand::Label(label("test-b")))),
        (
            Some(label("gcd-done")),
            Instruction::Perform {
                operation: "print".to_owned(),
                arguments: vec![reg_op("a")],
            },
        ),
    ];
    MachineProgram::new(Vec::new(), instructions)
}

/// The Fibonacci controller of Figure 5.12.
fn fibonacci_program() -> MachineProgram {
    let instructions = vec![
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
    ];
    MachineProgram::new(Vec::new(), instructions)
}

mod ex_5_13 {
    //! Exercise 5.13: determine the registers from the controller
    //! sequence instead of a supplied list.

    use super::*;

    /// The register names of one program, derived in first-seen
    /// order.
    fn register_names(program: &MachineProgram) -> Vec<String> {
        registers_in(program)
            .iter()
            .map(|Register(name)| name.clone())
            .collect()
    }

    /// The GCD controller derives exactly `a`, `b`, and `t` — in
    /// first-seen order `b`, `t`, `a`, since `b` opens the test — and
    /// the derived machine answers the text's session,
    /// gcd(206, 40) = 2.
    #[test]
    fn ex_5_13_derived_gcd_runs() {
        let mut machine = machine_from_controller(&gcd_program()).expect("assembles");
        assert_eq!(register_names(&gcd_program()), ["b", "t", "a"]);
        machine.set_register("a", 206).expect("register");
        machine.set_register("b", 40).expect("register");
        machine.run().expect("run");
        assert_eq!(machine.get_register("a").expect("a"), 2);
    }

    /// The Fibonacci controller derives `continue`, `n`, and `val`
    /// (and never the machine's own `flag`, which no controller
    /// names), and the derived machine answers fib(6) = 8.
    #[test]
    fn ex_5_13_derived_fibonacci_runs() {
        let mut machine = machine_from_controller(&fibonacci_program()).expect("assembles");
        assert_eq!(
            register_names(&fibonacci_program()),
            ["continue", "n", "val"]
        );
        machine.set_register("n", 6).expect("register");
        machine.run().expect("run");
        assert_eq!(machine.get_register("val").expect("val"), 8);
    }

    /// A controller that names an operation the machine lacks still
    /// fails at run time, derived registers or not: the typed fault
    /// names the missing operation.
    #[test]
    fn ex_5_13_unknown_operation_still_refused() {
        let program = MachineProgram::new(
            Vec::new(),
            vec![(
                None,
                Instruction::Assign {
                    target: reg("a"),
                    value: Operand::Operation {
                        operation: "mystery".to_owned(),
                        arguments: vec![reg_op("a")],
                    },
                },
            )],
        );
        let mut machine = machine_from_controller(&program).expect("assembles");
        let fault = machine.run().expect_err("the operation is missing");
        assert_eq!(fault, Fault::UndefinedOperation("mystery".to_owned()));
    }

    /// A controller that names no register derives an empty register
    /// set, and the machine still runs its constant instruction.
    #[test]
    fn ex_5_13_constant_only_controller_derives_nothing() {
        let program = MachineProgram::new(
            Vec::new(),
            vec![(
                None,
                Instruction::Perform {
                    operation: "print".to_owned(),
                    arguments: vec![constant(3)],
                },
            )],
        );
        assert!(registers_in(&program).is_empty());
        let mut machine = machine_from_controller(&program).expect("assembles");
        let outcome = machine.run().expect("run");
        assert_eq!(outcome.output, ["3".to_owned()]);
    }
}
