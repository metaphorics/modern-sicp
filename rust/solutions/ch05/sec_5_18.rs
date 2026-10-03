// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.18: tracing the stores of
//! chosen registers.

use std::collections::{BTreeSet, HashMap};

use ch05::sec_5_1::{MachineProgram, factorial_recursive};
use ch05::sec_5_2::{Fault, Machine, MachineValue, assemble};

/// One reported store: the instruction count when it happened, the
/// register, and the values before and after.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Store {
    /// The instructions executed when the store happened.
    pub step: usize,
    /// The stored register.
    pub register: String,
    /// The value the store replaced.
    pub before: MachineValue,
    /// The value the store wrote.
    pub after: MachineValue,
}

/// A per-register store log: only the traced registers report, and
/// every store path — the host's load, each `assign`, and each
/// `restore` — reports through the one register store the machine
/// has.
pub struct StoreLog {
    traced: BTreeSet<String>,
    entries: Vec<Store>,
    step: usize,
}

impl StoreLog {
    /// Traces the named registers.
    #[must_use]
    pub fn new(traced: &[&str]) -> Self {
        Self {
            traced: traced.iter().map(|name| (*name).to_owned()).collect(),
            entries: Vec::new(),
            step: 0,
        }
    }

    /// Stops tracing one register.
    pub fn untrace(&mut self, register: &str) {
        self.traced.remove(register);
    }

    /// The reported stores.
    #[must_use]
    pub fn entries(&self) -> &[Store] {
        &self.entries
    }

    /// Loads one register from the host and reports the store.
    /// # Errors
    /// Returns the fault raised when `register` is not present in the machine.
    pub fn load(
        &mut self,
        machine: &mut Machine,
        register: &str,
        value: MachineValue,
    ) -> Result<(), Fault> {
        let before = machine.get_register(register)?;
        machine.set_register(register, value)?;
        self.report(register, before, value);
        Ok(())
    }

    /// Steps the machine to its last instruction, reporting every
    /// store to a traced register.
    /// # Errors
    /// Returns the first fault raised by a machine transition.
    pub fn run(&mut self, machine: &mut Machine) -> Result<(), Fault> {
        while machine.pc() < machine.assembled().instructions.len() {
            let before = self.snapshot(machine.registers());
            machine.step()?;
            self.step += 1;
            self.report_changes(machine.registers(), &before);
        }
        Ok(())
    }

    fn snapshot(&self, registers: &HashMap<String, MachineValue>) -> Vec<MachineValue> {
        self.traced
            .iter()
            .map(|name| registers.get(name).copied().unwrap_or(0))
            .collect()
    }

    fn report_changes(
        &mut self,
        registers: &HashMap<String, MachineValue>,
        before: &[MachineValue],
    ) {
        let traced: Vec<String> = self.traced.iter().cloned().collect();
        for (name, old) in traced.iter().zip(before) {
            let new = registers.get(name).copied().unwrap_or(0);
            if *old != new {
                self.report(name, *old, new);
            }
        }
    }

    fn report(&mut self, register: &str, before: MachineValue, after: MachineValue) {
        if self.traced.contains(register) {
            self.entries.push(Store {
                step: self.step,
                register: register.to_owned(),
                before,
                after,
            });
        }
    }
}

/// Runs one machine under the store log.
fn run_traced(
    program: &MachineProgram,
    inputs: &[(&str, i64)],
    traced: &[&str],
) -> Result<StoreLog, Fault> {
    let mut machine = Machine::new(assemble(program)?);
    let mut log = StoreLog::new(traced);
    for (name, value) in inputs {
        log.load(&mut machine, name, *value)?;
    }
    log.run(&mut machine)?;
    Ok(log)
}

mod ex_5_18 {
    //! Exercise 5.18: make the machine's registers trace their own
    //! assignments, per register.

    use super::*;

    /// The factorial machine on n = 3 with `n` and `val` traced: the
    /// host's initial load, every assign, and every restore report,
    /// because the machine has exactly one store path.
    #[test]
    fn ex_5_18_traced_stores_report() -> Result<(), Fault> {
        let log = run_traced(&factorial_recursive(), &[("n", 3)], &["n", "val"])?;
        let expect = [
            (0, "n", 0, 3),
            (6, "n", 3, 2),
            (13, "n", 2, 1),
            (18, "val", 0, 1),
            (20, "n", 1, 2),
            (22, "val", 1, 2),
            (24, "n", 2, 3),
            (26, "val", 2, 6),
        ];
        assert_eq!(log.entries().len(), expect.len());
        for (entry, (step, register, before, after)) in log.entries().iter().zip(expect) {
            assert_eq!(entry.step, step);
            assert_eq!(entry.register, register);
            assert_eq!(entry.before, before);
            assert_eq!(entry.after, after);
        }
        Ok(())
    }

    /// Untracing silences a register; the answer is unaffected
    /// either way.
    #[test]
    fn ex_5_18_untrace_silences() -> Result<(), Fault> {
        let mut machine = Machine::new(assemble(&factorial_recursive())?);
        let mut log = StoreLog::new(&["n", "val"]);
        log.load(&mut machine, "n", 3)?;
        log.untrace("val");
        log.run(&mut machine)?;
        assert!(log.entries().iter().all(|entry| entry.register == "n"));
        assert_eq!(log.entries().len(), 5);
        assert_eq!(machine.get_register("val")?, 6);
        Ok(())
    }

    /// Tracing one register reports exactly its own stores and
    /// nothing else's.
    #[test]
    fn ex_5_18_one_register_traced() -> Result<(), Fault> {
        let log = run_traced(&factorial_recursive(), &[("n", 3)], &["val"])?;
        assert_eq!(log.entries().len(), 3);
        assert!(log.entries().iter().all(|entry| entry.register == "val"));
        assert_eq!(log.entries().last().map(|entry| entry.after), Some(6));
        Ok(())
    }
}
