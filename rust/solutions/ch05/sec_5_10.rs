// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.10: a new surface syntax
//! installed as one translator feeding the untouched assembler.

use ch05::sec_5_1::{Instruction, Label, MachineProgram, Operand, Register, gcd_machine, reg};
use ch05::sec_5_2::{Fault, Machine, assemble};

/// One line of the new syntax: no `(reg x)` wrappers, bare names for
/// registers, bare numbers for constants, and an operation call in
/// function form.
pub enum Line {
    /// A label site.
    Mark(&'static str),
    /// `assign TARGET <- SOURCE` with a plain source name or number.
    Move(&'static str, Source),
    /// `assign TARGET <- (op ARGS...)`.
    Compute(&'static str, &'static str, Vec<Source>),
    /// `test (op ARGS...)`.
    Check(&'static str, Vec<Source>),
    /// `branch LABEL`.
    Branch(&'static str),
    /// `goto LABEL`.
    To(&'static str),
    /// `goto (reg REGISTER)`.
    ToReg(&'static str),
    /// `assign TARGET <- (label LABEL)`.
    SetLabel(&'static str, &'static str),
    /// `save REGISTER`.
    Keep(&'static str),
    /// `restore REGISTER`.
    Take(&'static str),
    /// `perform (op ARGS...)`.
    Act(&'static str, Vec<Source>),
}

/// One operand of the new syntax: a bare number is a constant and a
/// bare name is a register.
pub enum Source {
    /// An integer constant.
    Number(i64),
    /// A register name.
    Name(&'static str),
}

/// One new-syntax operand to the edition's operand.
fn translate_source(source: &Source) -> Operand {
    match source {
        Source::Number(value) => Operand::Constant(*value),
        Source::Name(name) => Operand::Register(reg(name)),
    }
}

/// One new-syntax line to the edition's instruction. A label line is
/// not an instruction: `translate` attaches it to the next line.
fn translate_line(line: &Line) -> Option<Instruction> {
    let instruction = match line {
        Line::Mark(_) => return None,
        Line::Move(target, source) => Instruction::Assign {
            target: reg(target),
            value: translate_source(source),
        },
        Line::Compute(target, operation, arguments) => Instruction::Assign {
            target: reg(target),
            value: Operand::Operation {
                operation: (*operation).to_owned(),
                arguments: arguments.iter().map(translate_source).collect(),
            },
        },
        Line::Check(predicate, arguments) => Instruction::Test {
            predicate: (*predicate).to_owned(),
            arguments: arguments.iter().map(translate_source).collect(),
        },
        Line::Branch(name) => Instruction::Branch(Label((*name).to_owned())),
        Line::To(name) => Instruction::Goto(Operand::Label(Label((*name).to_owned()))),
        Line::ToReg(name) => Instruction::Goto(Operand::Register(reg(name))),
        Line::SetLabel(target, name) => Instruction::Assign {
            target: reg(target),
            value: Operand::Label(Label((*name).to_owned())),
        },
        Line::Keep(name) => Instruction::Save(reg(name)),
        Line::Take(name) => Instruction::Restore(reg(name)),
        Line::Act(operation, arguments) => Instruction::Perform {
            operation: (*operation).to_owned(),
            arguments: arguments.iter().map(translate_source).collect(),
        },
    };
    Some(instruction)
}

/// Translates a whole new-syntax program: a label line becomes the
/// label site of the next instruction, exactly as the book's
/// `extract-labels` attaches a label to the instruction that follows.
fn translate(lines: &[Line]) -> MachineProgram {
    let mut rows = Vec::new();
    let mut pending: Option<Label> = None;
    for line in lines {
        if let Line::Mark(name) = line {
            pending = Some(Label((*name).to_owned()));
            continue;
        }
        let Some(instruction) = translate_line(line) else {
            continue;
        };
        rows.push((pending.take(), instruction));
    }
    let registers = derived_registers(&rows);
    MachineProgram::new(registers, rows)
}

/// The registers a program names, in sorted order: a canonical list
/// that does not depend on the instruction traversal.
fn derived_registers(rows: &[(Option<Label>, Instruction)]) -> Vec<Register> {
    let mut names: Vec<String> = Vec::new();
    let mut note = |name: &str| {
        if !names.iter().any(|seen| seen == name) {
            names.push(name.to_owned());
        }
    };
    for (_, instruction) in rows {
        match instruction {
            Instruction::Assign {
                target: Register(name),
                value,
            } => {
                note(name);
                for operand in operands_of(value) {
                    if let Operand::Register(Register(source)) = operand {
                        note(source);
                    }
                }
            }
            Instruction::Test { arguments, .. } | Instruction::Perform { arguments, .. } => {
                for operand in arguments {
                    if let Operand::Register(Register(source)) = operand {
                        note(source);
                    }
                }
            }
            Instruction::Goto(Operand::Register(Register(name)))
            | Instruction::Save(Register(name))
            | Instruction::Restore(Register(name)) => note(name),
            _ => {}
        }
    }
    names.sort();
    names.into_iter().map(Register).collect()
}

/// The operand tree of one value, flattened.
fn operands_of(value: &Operand) -> Vec<&Operand> {
    match value {
        Operand::Operation { arguments, .. } => arguments.iter().flat_map(operands_of).collect(),
        other => vec![other],
    }
}

mod ex_5_10 {
    //! Exercise 5.10: design a new syntax for the machine
    //! instructions and modify the simulator to use it.

    use super::*;

    /// The GCD machine in the new syntax: the labels `loop` and
    /// `done`, and the operation calls in function form.
    fn gcd_lines() -> Vec<Line> {
        use Source::*;
        vec![
            Line::To("loop"),
            Line::Mark("loop"),
            Line::Check("=", vec![Name("b"), Number(0)]),
            Line::Branch("done"),
            Line::Compute("t", "rem", vec![Name("a"), Name("b")]),
            Line::Move("a", Name("b")),
            Line::Move("b", Name("t")),
            Line::To("loop"),
            Line::Mark("done"),
            Line::Act("print", vec![Name("a")]),
        ]
    }

    /// The recursive exponentiation machine of exercise 5.4 in the
    /// new syntax, the second test of the translation: it exercises
    /// the label-valued assignment, save/restore, and `goto` through
    /// a register.
    fn expt_lines() -> Vec<Line> {
        use Source::*;
        vec![
            Line::SetLabel("continue", "expt-done"),
            Line::Mark("loop"),
            Line::Check("=", vec![Name("n"), Number(0)]),
            Line::Branch("base"),
            Line::Keep("continue"),
            Line::Compute("n", "sub1", vec![Name("n")]),
            Line::SetLabel("continue", "after"),
            Line::To("loop"),
            Line::Mark("after"),
            Line::Take("continue"),
            Line::Compute("val", "mul", vec![Name("val"), Name("b")]),
            Line::ToReg("continue"),
            Line::Mark("base"),
            Line::Move("val", Number(1)),
            Line::ToReg("continue"),
            Line::Mark("expt-done"),
            Line::Act("print", vec![Name("val")]),
        ]
    }

    /// The translation of the new-syntax GCD controller equals the
    /// section's GCD machine, instruction for instruction: the new
    /// syntax is a translator's business, not the assembler's. Only
    /// the declaration order differs, since the translator derives
    /// its register list in sorted order.
    #[test]
    fn ex_5_10_translation_matches_the_lesson_machine() {
        let translated = translate(&gcd_lines());
        let lesson = gcd_machine();
        assert_eq!(translated.instructions, lesson.instructions);
        let mut derived: Vec<String> = translated
            .registers
            .iter()
            .map(|Register(name)| name.clone())
            .collect();
        derived.sort();
        let mut declared: Vec<String> = lesson
            .registers
            .iter()
            .map(|Register(name)| name.clone())
            .collect();
        declared.sort();
        assert_eq!(derived, declared);
    }

    /// The translated GCD answers 2 on 206 and 40, the text's
    /// session.
    #[test]
    fn ex_5_10_translated_gcd_runs() -> Result<(), Fault> {
        let mut machine = Machine::new(assemble(&translate(&gcd_lines()))?);
        machine.set_register("a", 206)?;
        machine.set_register("b", 40)?;
        machine.run()?;
        assert_eq!(machine.get_register("a")?, 2);
        Ok(())
    }

    /// The translated exponentiation machine answers 2^5 = 32,
    /// through the same untouched assembler.
    #[test]
    fn ex_5_10_translated_exponent_runs() -> Result<(), Fault> {
        let mut machine = Machine::new(assemble(&translate(&expt_lines()))?);
        machine.set_register("b", 2)?;
        machine.set_register("n", 5)?;
        machine.run()?;
        assert_eq!(machine.get_register("val")?, 32);
        Ok(())
    }

    /// The translator derives its register list from the code it
    /// translated, so no register list is supplied anywhere.
    #[test]
    fn ex_5_10_registers_come_from_the_translation() {
        let program = translate(&gcd_lines());
        let names: Vec<String> = program
            .registers
            .iter()
            .map(|Register(name)| name.clone())
            .collect();
        assert_eq!(names, ["a", "b", "t"]);
    }
}
