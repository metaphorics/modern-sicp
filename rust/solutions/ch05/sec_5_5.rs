// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solutions of exercises 5.5 and 5.5a: hand-simulated
//! traces of the factorial and Fibonacci machines, and the edition's
//! restore annotations.

use std::collections::HashMap;
use std::fmt::Write as _;

use ch05::sec_5_1::{
    Instruction, MachineProgram, Operand, Register, factorial_recursive, fibonacci_machine,
};
use ch05::sec_5_2::{Fault, Machine, assemble};

/// One stack event of a hand simulation: its step number, the action,
/// the register, the value it moved, and the stack afterwards.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Event {
    step: usize,
    action: &'static str,
    reg: String,
    value: i64,
    stack: Vec<(String, i64)>,
}

/// One completed hand simulation: its events, the registers each
/// executed instruction read, and the instruction count.
struct Simulation {
    events: Vec<Event>,
    reads: Vec<Vec<String>>,
    steps: usize,
}

/// The registers one operand tree reads.
fn operand_registers(operand: &Operand, out: &mut Vec<String>) {
    match operand {
        Operand::Register(Register(name)) => out.push(name.clone()),
        Operand::Operation { arguments, .. } => {
            for argument in arguments {
                operand_registers(argument, out);
            }
        }
        Operand::Constant(_) | Operand::Label(_) => {}
    }
}

/// The registers one instruction reads before it executes.
fn registers_read(instruction: &Instruction) -> Vec<String> {
    let mut out = Vec::new();
    match instruction {
        Instruction::Assign { value, .. } => operand_registers(value, &mut out),
        Instruction::Test { arguments, .. } | Instruction::Perform { arguments, .. } => {
            for argument in arguments {
                operand_registers(argument, &mut out);
            }
        }
        Instruction::Goto(operand) => operand_registers(operand, &mut out),
        Instruction::Save(Register(name)) => out.push(name.clone()),
        Instruction::Branch(_) | Instruction::Restore(_) => {}
    }
    out
}

/// What one instruction does to the stack.
enum Effect {
    Save(String),
    Restore(String),
    Plain,
}

/// Classifies one instruction's stack effect.
fn effect_of(instruction: &Instruction) -> Effect {
    match instruction {
        Instruction::Save(Register(name)) => Effect::Save(name.clone()),
        Instruction::Restore(Register(name)) => Effect::Restore(name.clone()),
        _ => Effect::Plain,
    }
}

/// Runs one machine one instruction at a time, recording every save
/// and restore together with the stack it leaves. The shadow stack
/// mirrors the machine's discipline, so a mismatch would surface as a
/// wrong depth in the recorded trace.
fn simulate(
    program: &MachineProgram,
    inputs: &[(&str, i64)],
) -> Result<(Simulation, Machine), Fault> {
    let mut machine = Machine::new(assemble(program)?);
    for (name, value) in inputs {
        machine.set_register(name, *value)?;
    }
    let mut events = Vec::new();
    let mut reads = Vec::new();
    let mut shadow: Vec<(String, i64)> = Vec::new();
    let mut steps = 0_usize;
    while machine.pc() < machine.assembled().instructions.len() {
        let (effect, read) = {
            let instruction = &machine.assembled().instructions[machine.pc()];
            (effect_of(instruction), registers_read(instruction))
        };
        steps += 1;
        reads.push(read);
        match effect {
            Effect::Save(name) => {
                let value = machine.get_register(&name)?;
                machine.step()?;
                shadow.push((name.clone(), value));
                events.push(Event {
                    step: steps,
                    action: "save",
                    reg: name,
                    value,
                    stack: shadow.clone(),
                });
            }
            Effect::Restore(name) => {
                machine.step()?;
                let value = machine.get_register(&name)?;
                shadow.pop();
                events.push(Event {
                    step: steps,
                    action: "restore",
                    reg: name,
                    value,
                    stack: shadow.clone(),
                });
            }
            Effect::Plain => machine.step()?,
        }
    }
    Ok((
        Simulation {
            events,
            reads,
            steps,
        },
        machine,
    ))
}

/// The label names of one machine, indexed the way `goto` and `assign`
/// store them: a label operand's value is its instruction index.
fn label_names(machine: &Machine) -> HashMap<usize, String> {
    machine
        .assembled()
        .labels
        .iter()
        .map(|(name, index)| (*index, name.clone()))
        .collect()
}

/// Renders one value the way the book's hand simulation writes it:
/// a `continue` register shows its label, everything else its number.
fn symbolic(reg: &str, value: i64, labels: &HashMap<usize, String>) -> String {
    if reg == "continue" {
        let index = usize::try_from(value).ok();
        if let Some(name) = index.and_then(|index| labels.get(&index)) {
            return name.clone();
        }
    }
    value.to_string()
}

/// Renders the stack event trace.
fn render(simulation: &Simulation, machine: &Machine) -> String {
    let labels = label_names(machine);
    let mut out = String::new();
    for event in &simulation.events {
        let stack: Vec<String> = event
            .stack
            .iter()
            .map(|(reg, value)| format!("{}={}", reg, symbolic(reg, *value, &labels)))
            .collect();
        // Writing into a `String` is infallible.
        let _ = writeln!(
            &mut out,
            "{:>3} {:<7} {}={} stack [{}] depth {}",
            event.step,
            event.action,
            event.reg,
            symbolic(&event.reg, event.value, &labels),
            stack.join(", "),
            event.stack.len()
        );
    }
    out
}

/// Renders the same trace with every restore annotated by the save it
/// matches, the age of the value it returns, and the older saves of
/// the same register still beneath that save.
fn annotate(simulation: &Simulation, machine: &Machine) -> String {
    let labels = label_names(machine);
    let mut pending: Vec<(usize, String)> = Vec::new();
    let mut out = String::new();
    for event in &simulation.events {
        let stack: Vec<String> = event
            .stack
            .iter()
            .map(|(reg, value)| format!("{}={}", reg, symbolic(reg, *value, &labels)))
            .collect();
        let _ = write!(
            &mut out,
            "{:>3} {:<7} {}={} stack [{}] depth {}",
            event.step,
            event.action,
            event.reg,
            symbolic(&event.reg, event.value, &labels),
            stack.join(", "),
            event.stack.len()
        );
        if event.action == "save" {
            pending.push((event.step, event.reg.clone()));
        } else {
            let matched = pending
                .iter()
                .rposition(|(_, reg)| *reg == event.reg)
                .map(|position| pending.remove(position));
            if let Some((saved_at, _)) = matched {
                let beneath = pending.iter().filter(|(_, reg)| *reg == event.reg).count();
                let _ = write!(
                    &mut out,
                    "  matches save #{saved_at}, age {}",
                    event.step - saved_at
                );
                if beneath > 0 {
                    let _ = write!(&mut out, ", {beneath} older save of {} beneath", event.reg);
                }
            }
        }
        out.push('\n');
    }
    out
}

/// The number of restores whose value is pushed back unchanged by the
/// next save of the same register, with no instruction in between
/// reading that register: the pure round trips the annotations expose.
fn round_trip_pairs(simulation: &Simulation) -> usize {
    let mut count = 0;
    for (index, event) in simulation.events.iter().enumerate() {
        if event.action != "restore" {
            continue;
        }
        let Some(next) = simulation.events[index + 1..]
            .iter()
            .find(|candidate| candidate.reg == event.reg)
        else {
            continue;
        };
        if next.action != "save" || next.value != event.value {
            continue;
        }
        let between = &simulation.reads[event.step..next.step - 1];
        if between.iter().any(|read| read.contains(&event.reg)) {
            continue;
        }
        count += 1;
    }
    count
}

mod ex_5_05 {
    //! Exercise 5.5: hand-simulate the factorial and Fibonacci
    //! machines on a nontrivial input, showing the stack at each
    //! significant point.

    use super::*;

    /// The factorial machine (the book's Figure 5.11) on n = 3: four
    /// saves and four restores, the stack peaking at depth 4.
    #[test]
    fn ex_5_05_factorial() {
        let (simulation, machine) = simulate(&factorial_recursive(), &[("n", 3)]).expect("run");
        assert_eq!(machine.get_register("val").expect("val"), 6);
        assert_eq!(simulation.steps, 28);
        assert_eq!(machine.stack_statistics().pushes, 4);
        assert_eq!(machine.stack_statistics().pops, 4);
        assert_eq!(machine.stack_statistics().max_depth, 4);
        assert_eq!(render(&simulation, &machine), FACT_N3_TRACE);
    }

    /// The Fibonacci machine (the book's Figure 5.12) on n = 3: both
    /// recursive calls execute, eight saves against eight restores.
    #[test]
    fn ex_5_05_fibonacci() {
        let (simulation, machine) = simulate(&fibonacci_machine(), &[("n", 3)]).expect("run");
        assert_eq!(machine.get_register("val").expect("val"), 2);
        assert_eq!(simulation.steps, 52);
        assert_eq!(machine.stack_statistics().pushes, 8);
        assert_eq!(machine.stack_statistics().pops, 8);
        assert_eq!(machine.stack_statistics().max_depth, 4);
        assert_eq!(render(&simulation, &machine), FIB_N3_TRACE);
    }

    const FACT_N3_TRACE: &str = "  4 save    continue=done stack [continue=done] depth 1
  5 save    n=3 stack [continue=done, n=3] depth 2
 11 save    continue=after stack [continue=done, n=3, continue=after] depth 3
 12 save    n=2 stack [continue=done, n=3, continue=after, n=2] depth 4
 20 restore n=2 stack [continue=done, n=3, continue=after] depth 3
 21 restore continue=after stack [continue=done, n=3] depth 2
 24 restore n=3 stack [continue=done] depth 1
 25 restore continue=done stack [] depth 0
";

    const FIB_N3_TRACE: &str = "  4 save    continue=done stack [continue=done] depth 1
  6 save    n=3 stack [continue=done, n=3] depth 2
 11 save    continue=afterfibn-1 stack [continue=done, n=3, continue=afterfibn-1] depth 3
 13 save    n=2 stack [continue=done, n=3, continue=afterfibn-1, n=2] depth 4
 20 restore n=2 stack [continue=done, n=3, continue=afterfibn-1] depth 3
 21 restore continue=afterfibn-1 stack [continue=done, n=3] depth 2
 22 save    continue=afterfibn-1 stack [continue=done, n=3, continue=afterfibn-1] depth 3
 24 save    val=1 stack [continue=done, n=3, continue=afterfibn-1, val=1] depth 4
 32 restore val=1 stack [continue=done, n=3, continue=afterfibn-1] depth 3
 33 restore continue=afterfibn-1 stack [continue=done, n=3] depth 2
 36 restore n=3 stack [continue=done] depth 1
 37 restore continue=done stack [] depth 0
 38 save    continue=done stack [continue=done] depth 1
 40 save    val=1 stack [continue=done, val=1] depth 2
 48 restore val=1 stack [continue=done] depth 1
 49 restore continue=done stack [] depth 0
";
}

mod ex_5_05a {
    //! Exercise 5.5a (this edition): annotate every restore in the
    //! Fibonacci trace with the save it matches and the age of the
    //! value it returns, and read the redundant pair of exercise 5.6
    //! off the annotations.

    use super::*;

    /// The annotated trace of the n = 3 Fibonacci run. The
    /// annotations show the pattern: at step 21 the restore of
    /// `continue` returns the value `afterfibn-1`, and at step 22 the
    /// very next save of `continue` pushes that same value back;
    /// steps 37 and 38 repeat the pair at the outer level. A restore
    /// whose value is pushed back unchanged, with no use in between,
    /// can be removed together with that save.
    #[test]
    fn ex_5_05a() {
        let (simulation, machine) = simulate(&fibonacci_machine(), &[("n", 3)]).expect("run");
        assert_eq!(annotate(&simulation, &machine), FIB_N3_ANNOTATED);
        // One pure round trip per internal call: the two levels of
        // the n = 3 computation. These pairs are exactly the
        // redundant save and restore of exercise 5.6.
        assert_eq!(round_trip_pairs(&simulation), 2);
    }

    const FIB_N3_ANNOTATED: &str = "  4 save    continue=done stack [continue=done] depth 1
  6 save    n=3 stack [continue=done, n=3] depth 2
 11 save    continue=afterfibn-1 stack [continue=done, n=3, continue=afterfibn-1] depth 3
 13 save    n=2 stack [continue=done, n=3, continue=afterfibn-1, n=2] depth 4
 20 restore n=2 stack [continue=done, n=3, continue=afterfibn-1] depth 3  matches save #13, age 7, 1 older save of n beneath
 21 restore continue=afterfibn-1 stack [continue=done, n=3] depth 2  matches save #11, age 10, 1 older save of continue beneath
 22 save    continue=afterfibn-1 stack [continue=done, n=3, continue=afterfibn-1] depth 3
 24 save    val=1 stack [continue=done, n=3, continue=afterfibn-1, val=1] depth 4
 32 restore val=1 stack [continue=done, n=3, continue=afterfibn-1] depth 3  matches save #24, age 8
 33 restore continue=afterfibn-1 stack [continue=done, n=3] depth 2  matches save #22, age 11, 1 older save of continue beneath
 36 restore n=3 stack [continue=done] depth 1  matches save #6, age 30
 37 restore continue=done stack [] depth 0  matches save #4, age 33
 38 save    continue=done stack [continue=done] depth 1
 40 save    val=1 stack [continue=done, val=1] depth 2
 48 restore val=1 stack [continue=done] depth 1  matches save #40, age 8
 49 restore continue=done stack [] depth 0  matches save #38, age 11
";
}
