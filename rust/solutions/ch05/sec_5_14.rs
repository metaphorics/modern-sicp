// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.14: the measured factorial
//! machine, with the stack initialized and the statistics printed by
//! the controller.

use ch05::sec_5_2::{Machine, OpHandler, make_machine, op};
use sicp_runtime::Value;

mod ex_5_14 {
    //! Exercise 5.14: measure the pushes and the maximum stack depth
    //! of the Figure 5.11 machine and read off the formulas.

    use super::*;

    /// The exercise's augmented controller: the machine operations
    /// `initialize-stack` and `print-stack-statistics` are installed
    /// by the simulator, so the controller only performs them at the
    /// right moments.
    const MEASURED_CONTROLLER: &str = "
  (perform (op initialize-stack))
  (assign continue (label fact-done))
fact-loop
  (test (op =) (reg n) (const 1))
  (branch (label base-case))
  (save continue)
  (save n)
  (assign n (op -) (reg n) (const 1))
  (assign continue (label after-fact))
  (goto (label fact-loop))
after-fact
  (restore n)
  (restore continue)
  (assign val (op *) (reg n) (reg val))
  (goto (reg continue))
base-case
  (assign val (const 1))
  (goto (reg continue))
fact-done
  (perform (op print-stack-statistics))";

    fn measured_machine() -> Machine {
        let arithmetic: Vec<(&'static str, OpHandler)> = vec![
            ("=", op("=").expect("shared")),
            ("*", op("*").expect("shared")),
            ("-", op("-").expect("shared")),
        ];
        make_machine(&["n", "val", "continue"], &arithmetic, MEASURED_CONTROLLER)
            .expect("assembles")
    }

    /// Runs the measured machine on `n` and returns
    /// `(answer, total-pushes, maximum-depth, printed-line)`.
    fn measure(n: i128) -> (i128, u64, u64, String) {
        let mut machine = measured_machine();
        machine.set_register("n", Value::Int(n)).unwrap();
        machine.start().unwrap();
        let Value::Int(answer) = machine.get_register("val").unwrap() else {
            unreachable!("factorial answers an integer")
        };
        let (pushes, depth) = machine.stack_statistics();
        let printed = machine.transcript().last().cloned().unwrap_or_default();
        (answer, pushes, depth, printed)
    }

    /// The measured table for n = 1..=7: both quantities are 2n - 2
    /// (zero only at n = 1), the slope 2 counting the save and the
    /// restore of each of the n - 1 non-base levels, the intercept
    /// -2 discounting the base level. The answers are the host's
    /// factorials.
    #[test]
    fn ex_5_14_measured_table() {
        for n in 1..=7 {
            let (answer, pushes, depth, _) = measure(n);
            assert_eq!(answer, (1..=n).product::<i128>());
            assert_eq!(u64::try_from(2 * (n - 1)).expect("small n"), pushes);
            assert_eq!(u64::try_from(2 * (n - 1)).expect("small n"), depth);
        }
    }

    /// The printed statistics of the controller run, and the book's
    /// worked n = 6 pin: ten pushes, maximum depth ten.
    #[test]
    fn ex_5_14_printed_statistics() {
        let (answer, _, _, printed) = measure(6);
        assert_eq!(answer, 720);
        assert_eq!(printed, "(total-pushes = 10 maximum-depth = 10)");
    }
}
