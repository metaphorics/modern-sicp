// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.8: duplicate labels fail the
//! assembly instead of silently picking one site.

use ch05::sec_5_1::{Instruction, Label, MachineProgram, Operand, Register, constant, label, reg};
use ch05::sec_5_2::{Fault, Machine, assemble};

/// The repair the exercise asks for: a duplicate label is a typed
/// assembly error naming the repeated label.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssemblyError {
    /// One label marks two sites.
    DuplicateLabel {
        /// The repeated label's name.
        label: String,
    },
    /// The underlying assembler's refusal.
    Machine(Fault),
}

/// The book's ambiguous controller as typed data: the label `here`
/// marks two different locations.
fn ambiguous_program() -> MachineProgram {
    MachineProgram::new(
        vec![reg("a")],
        vec![
            (None, Instruction::Goto(Operand::Label(label("here")))),
            (
                Some(label("here")),
                Instruction::Assign {
                    target: reg("a"),
                    value: constant(3),
                },
            ),
            (None, Instruction::Goto(Operand::Label(label("there")))),
            (
                Some(label("here")),
                Instruction::Assign {
                    target: reg("a"),
                    value: constant(4),
                },
            ),
            (None, Instruction::Goto(Operand::Label(label("there")))),
            (
                Some(label("there")),
                Instruction::Perform {
                    operation: "print".to_owned(),
                    arguments: vec![Operand::Register(reg("a"))],
                },
            ),
        ],
    )
}

/// The ambiguous controller with one `here` renamed, so a run
/// resolves the duplicate the way one candidate label table would.
/// `rename` names the site to shadow; the other keeps `here`.
fn with_shadowed_site(rename_second_here: bool) -> MachineProgram {
    let program = ambiguous_program();
    let mut seen_here = 0;
    let instructions = program
        .instructions
        .into_iter()
        .map(|(label_row, instruction)| {
            let Some(Label(name)) = label_row else {
                return (label_row, instruction);
            };
            if name != "here" {
                return (Some(Label(name)), instruction);
            }
            seen_here += 1;
            let shadowed = seen_here == 2;
            if shadowed == rename_second_here {
                (Some(Label("here-shadow".to_owned())), instruction)
            } else {
                (Some(Label(name)), instruction)
            }
        })
        .collect();
    MachineProgram::new(vec![reg("a")], instructions)
}

/// The controller whose `here` marks the `a <- 3` site: the lookup
/// keeps the first occurrence.
fn first_here_wins() -> MachineProgram {
    with_shadowed_site(true)
}

/// The controller whose `here` marks the `a <- 4` site: the lookup
/// keeps the last occurrence.
fn last_here_wins() -> MachineProgram {
    with_shadowed_site(false)
}

/// The repair: assemble only after every label name is unique.
fn assemble_checked(program: &MachineProgram) -> Result<Machine, AssemblyError> {
    let mut seen: Vec<&str> = Vec::new();
    for (label_row, _) in &program.instructions {
        let Some(Label(name)) = label_row else {
            continue;
        };
        if seen.contains(&name.as_str()) {
            return Err(AssemblyError::DuplicateLabel {
                label: name.clone(),
            });
        }
        seen.push(name);
    }
    assemble(program)
        .map(Machine::new)
        .map_err(AssemblyError::Machine)
}

/// Runs one program to completion and answers its `a` register and
/// the transcript the machine printed.
fn run(program: &MachineProgram) -> Result<(i64, Vec<String>), Fault> {
    let mut machine = Machine::new(assemble(program)?);
    let outcome = machine.run()?;
    Ok((machine.get_register("a")?, outcome.output))
}

mod ex_5_08 {
    //! Exercise 5.8: with the simulator as written, what does
    //! register `a` hold when control reaches `there`? Then make the
    //! assembler reject the duplicate label.

    use super::*;

    /// The answer depends on which site the lookup finds. If the
    /// lookup keeps the first `here`, control runs the `a <- 3` block
    /// and `a` holds 3 when it reaches `there`; if it keeps the last,
    /// the `a <- 4` block runs and `a` holds 4. The machine is
    /// ambiguous until the assembler refuses it.
    #[test]
    fn ex_5_08_as_written_the_value_is_ambiguous() -> Result<(), Fault> {
        let (first, first_output) = run(&first_here_wins())?;
        let (last, last_output) = run(&last_here_wins())?;
        assert_eq!(
            (first, first_output.as_slice()),
            (3, ["3".to_owned()].as_slice())
        );
        assert_eq!(
            (last, last_output.as_slice()),
            (4, ["4".to_owned()].as_slice())
        );
        Ok(())
    }

    /// The repaired assembler refuses the whole controller: the
    /// typed fault names the repeated label, and no machine is
    /// returned at all.
    #[test]
    fn ex_5_08_duplicate_label_fails_assembly() {
        let error = assemble_checked(&ambiguous_program()).expect_err("duplicate is refused");
        assert_eq!(
            error,
            AssemblyError::DuplicateLabel {
                label: "here".to_owned()
            }
        );
    }

    /// The repair refuses nothing else: both unambiguous variants
    /// assemble, and their register declaration is untouched.
    #[test]
    fn ex_5_08_unambiguous_variants_still_assemble() -> Result<(), AssemblyError> {
        for program in [first_here_wins(), last_here_wins()] {
            assert_eq!(program.registers, vec![Register("a".to_owned())]);
            assemble_checked(&program)?;
        }
        Ok(())
    }
}
