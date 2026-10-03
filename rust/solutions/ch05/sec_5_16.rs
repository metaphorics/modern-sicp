// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.16: switching instruction
//! tracing on and off.

use ch05::sec_5_1::gcd_machine;
use ch05::sec_5_2::{Fault, Machine, Run, assemble};

mod ex_5_16 {
    //! Exercise 5.16: the machine accepts `trace-on` and `trace-off`
    //! messages. Here tracing is a simulator facility: it records one
    //! line per executed instruction and never touches the answer.

    use super::*;

    /// Tracing the GCD machine on 12 and 8 records one line per
    /// executed instruction — sixteen, the run of the loop plus the
    /// final test, branch, and print — and the answer is untouched.
    #[test]
    fn ex_5_16_traced_gcd() -> Result<(), Fault> {
        let mut machine = Machine::new(assemble(&gcd_machine())?);
        machine.set_register("a", 12)?;
        machine.set_register("b", 8)?;
        machine.set_trace(true);
        let outcome: Run = machine.run()?;
        assert_eq!(outcome.registers["a"], 4);
        assert_eq!(outcome.steps, 16);
        assert_eq!(machine.trace().len(), 16);
        assert!(machine.trace()[0].starts_with("   0: "));
        Ok(())
    }

    /// With tracing off the same run records nothing; and a machine
    /// whose tracing is switched off after four instructions stays
    /// quiet from there on, the answer unaffected either way.
    #[test]
    fn ex_5_16_trace_off_prints_nothing() -> Result<(), Fault> {
        let mut machine = Machine::new(assemble(&gcd_machine())?);
        machine.set_register("a", 12)?;
        machine.set_register("b", 8)?;
        machine.set_trace(false);
        let outcome: Run = machine.run()?;
        assert_eq!(outcome.registers["a"], 4);
        assert!(machine.trace().is_empty());

        let mut machine = Machine::new(assemble(&gcd_machine())?);
        machine.set_register("a", 12)?;
        machine.set_register("b", 8)?;
        machine.set_trace(true);
        for _ in 0..4 {
            machine.step()?;
        }
        machine.set_trace(false);
        let outcome: Run = machine.resume()?;
        assert_eq!(outcome.registers["a"], 4);
        assert_eq!(machine.trace().len(), 4);
        Ok(())
    }
}
