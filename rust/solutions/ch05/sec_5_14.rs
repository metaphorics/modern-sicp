// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.14: the measured factorial
//! machine, with the stack statistics read off the simulator.

use ch05::sec_5_1::factorial_recursive;
use ch05::sec_5_2::{Fault, Machine, StackStats, assemble};

/// Runs the Figure 5.11 machine on `n` and answers its statistics
/// together with the book's rendered statistics line.
fn measure(n: i64) -> Result<(i64, StackStats, String), Fault> {
    let mut machine = Machine::new(assemble(&factorial_recursive())?);
    machine.set_register("n", n)?;
    machine.run()?;
    let answer = machine.get_register("val")?;
    let stats = machine.stack_statistics();
    let printed = format!(
        "(total-pushes = {} maximum-depth = {})",
        stats.pushes, stats.max_depth
    );
    Ok((answer, stats, printed))
}

mod ex_5_14 {
    //! Exercise 5.14: measure the pushes and the maximum stack depth
    //! of the Figure 5.11 machine and read off the formulas.

    use super::*;

    /// The measured table for n = 1..=7: both quantities are 2n - 2
    /// (zero only at n = 1), the slope 2 counting the save and the
    /// restore of each of the n - 1 non-base levels, the intercept
    /// -2 discounting the base level. The answers are the host's
    /// factorials.
    #[test]
    fn ex_5_14_measured_table() -> Result<(), Fault> {
        for n in 1..=7 {
            let (answer, stats, _) = measure(n)?;
            let expected = (1..=n).product::<i64>();
            let formula = u64::try_from(2 * (n - 1)).expect("small n");
            assert_eq!(answer, expected, "factorial({n})");
            assert_eq!(stats.pushes, formula, "factorial({n}) pushes");
            assert_eq!(
                u64::try_from(stats.max_depth).expect("small n"),
                formula,
                "factorial({n}) depth"
            );
        }
        Ok(())
    }

    /// The book's worked n = 6 pin: ten pushes, maximum depth ten,
    /// and the statistics line the augmented controller prints.
    #[test]
    fn ex_5_14_printed_statistics() -> Result<(), Fault> {
        let (answer, _, printed) = measure(6)?;
        assert_eq!(answer, 720);
        assert_eq!(printed, "(total-pushes = 10 maximum-depth = 10)");
        Ok(())
    }
}
