// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solutions of exercises 5.15 and 5.15a: instruction
//! counting with a print-and-reset message, and this edition's
//! instruction budget.

use ch05::sec_5_2::{Fault, Machine, OpHandler, make_machine, op};
use sicp_runtime::Value;

fn fib_operations() -> Vec<(&'static str, OpHandler)> {
    vec![
        ("<", op("<").expect("shared")),
        ("+", op("+").expect("shared")),
        ("-", op("-").expect("shared")),
    ]
}

mod ex_5_15 {
    //! Exercise 5.15: have the machine keep track of the number of
    //! instructions executed and accept a message that prints the
    //! count and resets it to zero.

    use super::*;

    fn counted_fib(n: i128) -> u64 {
        let mut machine = ch05::sec_5_2::fibonacci_machine();
        machine.set_register("n", Value::Int(n)).unwrap();
        machine.start().unwrap();
        machine.instruction_count()
    }

    /// Every executed instruction counts, transfers included. The
    /// base case runs five instructions (the initial assign, the
    /// test, the branch, the base assign, and the return goto); the
    /// whole computation of fib(6) runs 281.
    #[test]
    fn ex_5_15_counts() {
        assert_eq!(counted_fib(0), 5);
        assert_eq!(counted_fib(1), 5);
        assert_eq!(counted_fib(2), 28);
        assert_eq!(counted_fib(6), 281);
    }

    /// The message: the count is returned and printed, and the
    /// counter is back to zero afterwards.
    #[test]
    fn ex_5_15_print_and_reset_message() {
        let mut machine = ch05::sec_5_2::fibonacci_machine();
        machine.set_register("n", Value::Int(6)).unwrap();
        machine.start().unwrap();
        assert_eq!(machine.print_instruction_count(), 281);
        assert_eq!(machine.instruction_count(), 0);
        assert_eq!(machine.transcript(), ["281".to_owned()]);
    }

    /// The same message is reachable from a controller as the
    /// machine operation `print-instruction-count`: a run that ends
    /// by performing it prints its own count and leaves the counter
    /// at zero.
    #[test]
    fn ex_5_15_controller_visible_message() {
        let controller = "
  (assign continue (label fib-done))
fib-loop
  (test (op <) (reg n) (const 2))
  (branch (label immediate-answer))
  (save continue)
  (assign continue (label afterfib))
  (save n)
  (assign n (op -) (reg n) (const 1))
  (goto (label fib-loop))
afterfib
  (restore n)
  (restore continue)
immediate-answer
  (assign val (reg n))
  (goto (reg continue))
fib-done
  (perform (op print-instruction-count))";
        let mut machine: Machine =
            make_machine(&["n", "val", "continue"], &fib_operations(), controller)
                .expect("assembles");
        machine.set_register("n", Value::Int(3)).unwrap();
        machine.start().unwrap();
        assert_eq!(machine.transcript(), ["28".to_owned()]);
        assert_eq!(machine.instruction_count(), 0);
    }
}

mod ex_5_15a {
    //! Exercise 5.15a (this edition): the counting machine accepts a
    //! budget; a run due to execute an instruction past it halts
    //! with a typed fault carrying the count and the program
    //! counter.

    use super::*;

    fn fib_with_budget(n: i128, budget: Option<u64>) -> Result<(), Fault> {
        let mut machine = ch05::sec_5_2::fibonacci_machine();
        machine.set_instruction_budget(budget);
        machine.set_register("n", Value::Int(n)).unwrap();
        machine.start().map(|_| ())
    }

    /// A budget far below the demand halts the run at the count
    /// itself, naming the instruction that would have run: fib(6)
    /// stopped after ten instructions at pc 3, the loop's assign.
    #[test]
    fn ex_5_15a_run_halts_at_the_budget() {
        assert_eq!(
            fib_with_budget(6, Some(10)),
            Err(Fault::BudgetExceeded { count: 10, pc: 3 })
        );
    }

    /// One instruction short of the demand still faults; the exact
    /// demand finishes cleanly. fib(6) needs 281 instructions.
    #[test]
    fn ex_5_15a_exact_demand_is_the_boundary() {
        assert_eq!(
            fib_with_budget(6, Some(280)),
            Err(Fault::BudgetExceeded { count: 280, pc: 19 })
        );
        assert_eq!(fib_with_budget(6, Some(281)), Ok(()));
        assert_eq!(fib_with_budget(6, None), Ok(()));
    }

    /// The fault leaves the machine inspectable and continuable:
    /// clearing the budget and proceeding completes the very run,
    /// and the total count accounts the halted instruction too.
    #[test]
    fn ex_5_15a_halted_machine_continues() {
        let mut machine = ch05::sec_5_2::fibonacci_machine();
        machine.set_instruction_budget(Some(280));
        machine.set_register("n", Value::Int(6)).unwrap();
        let fault = machine.start().unwrap_err();
        assert_eq!(fault, Fault::BudgetExceeded { count: 280, pc: 19 });
        assert_eq!(machine.instruction_count(), 280);
        machine.set_instruction_budget(None);
        machine.proceed().unwrap();
        assert_eq!(machine.get_register("val").unwrap(), Value::Int(8));
        assert_eq!(machine.instruction_count(), 281);
    }
}
