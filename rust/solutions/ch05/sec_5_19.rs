// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.19: breakpoints with
//! proceed and cancel.

use ch05::sec_5_1::gcd_machine;
use ch05::sec_5_2::{Fault, Machine, Run, assemble};

/// Runs the GCD machine on `a` and `b` up to the `n`th visit of the
/// loop label: the machine stops before that visit's instruction.
fn stop_at_loop(a: i64, b: i64, n: usize) -> Result<(Run, Machine), Fault> {
    let mut machine = Machine::new(assemble(&gcd_machine())?);
    machine.set_register("a", a)?;
    machine.set_register("b", b)?;
    machine.set_breakpoint("loop", n)?;
    let outcome = machine.run()?;
    Ok((outcome, machine))
}

mod ex_5_19 {
    //! Exercise 5.19: set a breakpoint, hold the machine at it,
    //! proceed on request, and cancel breakpoints.

    use super::*;

    /// A breakpoint at the first visit of `loop` holds the machine
    /// just before the test, with the inputs still in `a` and `b`.
    /// Proceeding runs the loop to its end: the run reports no
    /// breakpoint and a = 4.
    #[test]
    fn ex_5_19_breakpoint_holds_and_proceeds() -> Result<(), Fault> {
        let (held, mut machine) = stop_at_loop(12, 8, 1)?;
        assert_eq!(held.at_breakpoint.as_deref(), Some("loop"));
        assert_eq!(machine.pc(), 1);
        assert_eq!(held.registers["a"], 12);
        assert_eq!(held.registers["b"], 8);

        let outcome = machine.resume()?;
        assert_eq!(outcome.at_breakpoint, None);
        assert_eq!(outcome.registers["a"], 4);
        Ok(())
    }

    /// A breakpoint names a visit: the second holds after the first
    /// pass (a = 8, b = 4), the third after the second (a = 4,
    /// b = 0), and the third pass ends the run.
    #[test]
    fn ex_5_19_breakpoint_names_a_visit() -> Result<(), Fault> {
        let (held, _) = stop_at_loop(12, 8, 2)?;
        assert_eq!(held.at_breakpoint.as_deref(), Some("loop"));
        assert_eq!(held.registers["a"], 8);
        assert_eq!(held.registers["b"], 4);

        let (held, _) = stop_at_loop(12, 8, 3)?;
        assert_eq!(held.at_breakpoint.as_deref(), Some("loop"));
        assert_eq!(held.registers["a"], 4);
        assert_eq!(held.registers["b"], 0);
        Ok(())
    }

    /// Cancelling the breakpoints lets the next run go straight
    /// through, and the answer is unaffected.
    #[test]
    fn ex_5_19_cancel_breakpoint() -> Result<(), Fault> {
        let mut machine = Machine::new(assemble(&gcd_machine())?);
        machine.set_register("a", 12)?;
        machine.set_register("b", 8)?;
        machine.set_breakpoint("loop", 1)?;
        machine.clear_breakpoints();
        let outcome = machine.run()?;
        assert_eq!(outcome.at_breakpoint, None);
        assert_eq!(outcome.registers["a"], 4);
        Ok(())
    }

    /// A breakpoint names a declared label: an unknown one is
    /// refused, and no run is started.
    #[test]
    fn ex_5_19_unknown_label_refused() -> Result<(), Fault> {
        let mut machine = Machine::new(assemble(&gcd_machine())?);
        let fault = machine
            .set_breakpoint("nosuch", 1)
            .expect_err("unknown label");
        assert_eq!(fault, Fault::UnboundLabel("nosuch".to_owned()));
        Ok(())
    }
}
