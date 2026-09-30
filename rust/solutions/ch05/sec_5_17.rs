// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.17: announcing the label
//! before each traced instruction.

use ch05::sec_5_1::fibonacci_machine;
use ch05::sec_5_2::{Fault, Machine, assemble};

/// Drives one machine with tracing on and annotates every executed
/// instruction with the labels its program counter carries: a label
/// line precedes its instruction line, and the very first
/// instruction — which follows no label — is announced by nothing.
fn labelled_trace(machine: &mut Machine) -> Result<Vec<String>, Fault> {
    let mut lines = Vec::new();
    while machine.pc() < machine.assembled().instructions.len() {
        let here = machine.pc();
        for (label, index) in &machine.assembled().labels {
            if *index == here {
                lines.push(format!("{label}:"));
            }
        }
        machine.step()?;
        if let Some(recorded) = machine.trace().last() {
            lines.push(recorded.to_owned());
        }
    }
    Ok(lines)
}

/// Counts how often one label was announced.
fn announced(lines: &[String], label: &str) -> usize {
    let head = format!("{label}:");
    lines.iter().filter(|line| **line == head).count()
}

mod ex_5_17 {
    //! Exercise 5.17: modify the tracing so that it labels each
    //! instruction with the label at which it starts.

    use super::*;

    /// The label-traced fib(3) run: the trace carries 65 lines, 52
    /// instruction lines and 13 label lines — `loop` five times,
    /// `base` three, `afterfibn-1` and `afterfibn-2` twice each, and
    /// `done` once — and the instruction count of exercise 5.15 is
    /// still exactly 52.
    #[test]
    fn ex_5_17_labels_announced() -> Result<(), Fault> {
        let mut machine = Machine::new(assemble(&fibonacci_machine())?);
        machine.set_register("n", 3)?;
        machine.set_trace(true);
        let lines = labelled_trace(&mut machine)?;
        let labels = lines.iter().filter(|line| line.ends_with(':')).count();
        assert_eq!(lines.len(), 65);
        assert_eq!(lines.len() - labels, 52);
        assert_eq!(labels, 13);
        assert_eq!(announced(&lines, "loop"), 5);
        assert_eq!(announced(&lines, "base"), 3);
        assert_eq!(announced(&lines, "afterfibn-1"), 2);
        assert_eq!(announced(&lines, "afterfibn-2"), 2);
        assert_eq!(announced(&lines, "done"), 1);
        Ok(())
    }

    /// Each label line comes immediately before its own instruction,
    /// and the first line of the trace is an instruction — the
    /// controller's opening assignment, before any label.
    #[test]
    fn ex_5_17_label_precedes_its_instruction() -> Result<(), Fault> {
        let mut machine = Machine::new(assemble(&fibonacci_machine())?);
        machine.set_register("n", 3)?;
        machine.set_trace(true);
        let lines = labelled_trace(&mut machine)?;
        let first = lines.first().expect("the trace has lines");
        assert!(!first.ends_with(':'));
        assert_eq!(lines[1], "loop:");
        assert!(!lines[2].ends_with(':'));
        assert!(lines[2].contains("Test"));
        let done = lines
            .iter()
            .position(|line| line == "done:")
            .expect("announced");
        let instruction = lines.get(done + 1).expect("an instruction follows");
        assert!(instruction.contains("Perform"));
        assert!(!instruction.ends_with(':'));
        Ok(())
    }
}
