// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.11: the three restore
//! disciplines as typed alternatives, and the pruned Fibonacci
//! machine the book's own discipline allows.

use ch05::sec_5_2::{Fault, Machine, OpHandler, RestoreDiscipline, make_machine, op};
use sicp_runtime::Value;

fn fib_operations() -> Vec<(&'static str, OpHandler)> {
    vec![
        ("<", op("<").expect("shared")),
        ("+", op("+").expect("shared")),
        ("-", op("-").expect("shared")),
    ]
}

/// The book's probe: restore a register that is not the last one
/// saved.
const OUT_OF_ORDER_CONTROLLER: &str = "
  (assign y (const 7))
  (assign x (const 8))
  (save y)
  (save x)
  (restore y)";

/// The machine of Figure 5.12.
const FIB_CONTROLLER: &str = "
  (assign continue (label fib-done))
fib-loop
  (test (op <) (reg n) (const 2))
  (branch (label immediate-answer))
  (save continue)
  (assign continue (label afterfib-n-1))
  (save n)
  (assign n (op -) (reg n) (const 1))
  (goto (label fib-loop))
afterfib-n-1
  (restore n)
  (restore continue)
  (assign n (op -) (reg n) (const 2))
  (save continue)
  (assign continue (label afterfib-n-2))
  (save val)
  (goto (label fib-loop))
afterfib-n-2
  (assign n (reg val))
  (restore val)
  (restore continue)
  (assign val (op +) (reg val) (reg n))
  (goto (reg continue))
immediate-answer
  (assign val (reg n))
  (goto (reg continue))
fib-done";

/// The pruned machine of 5.11a: `afterfib-n-2` opens with a plain
/// `(restore n)`, which picks up the value saved from `val`, so the
/// pair `(assign n (reg val))` and `(restore val)` collapses into
/// that one instruction.
const FIB_PRUNED_CONTROLLER: &str = "
  (assign continue (label fib-done))
fib-loop
  (test (op <) (reg n) (const 2))
  (branch (label immediate-answer))
  (save continue)
  (assign continue (label afterfib-n-1))
  (save n)
  (assign n (op -) (reg n) (const 1))
  (goto (label fib-loop))
afterfib-n-1
  (restore n)
  (restore continue)
  (assign n (op -) (reg n) (const 2))
  (save continue)
  (assign continue (label afterfib-n-2))
  (save val)
  (goto (label fib-loop))
afterfib-n-2
  (restore n)
  (restore continue)
  (assign val (op +) (reg val) (reg n))
  (goto (reg continue))
immediate-answer
  (assign val (reg n))
  (goto (reg continue))
fib-done";

fn fib_machine(discipline: RestoreDiscipline, controller: &str) -> Machine {
    let mut machine =
        make_machine(&["n", "val", "continue"], &fib_operations(), controller).expect("assembles");
    machine.set_restore_discipline(discipline);
    machine
}

fn run_fib(discipline: RestoreDiscipline, controller: &str, n: i128) -> Result<i128, Fault> {
    let mut machine = fib_machine(discipline, controller);
    machine.set_register("n", Value::Int(n)).unwrap();
    machine.start()?;
    match machine.get_register("val")? {
        Value::Int(answer) => Ok(answer),
        other => unreachable!("fib answers an integer, got {other:?}"),
    }
}

fn host_fib(n: i128) -> i128 {
    if n < 2 {
        n
    } else {
        host_fib(n - 1) + host_fib(n - 2)
    }
}

mod ex_5_11 {
    //! Exercise 5.11: the three meanings of restore, and the
    //! instruction the book's own meaning lets us drop.

    use super::*;

    /// Part (a) demonstrated: under the plain discipline the pruned
    /// machine equals the host Fibonacci on n = 0..=10, with exactly
    /// one instruction saved per return.
    #[test]
    fn part_a_pruned_machine_matches_the_host() {
        for n in 0..=10 {
            assert_eq!(
                run_fib(RestoreDiscipline::Plain, FIB_PRUNED_CONTROLLER, n).unwrap(),
                host_fib(n)
            );
        }
        let original = run_fib(RestoreDiscipline::Plain, FIB_CONTROLLER, 6).unwrap();
        assert_eq!(original, 8);
    }

    /// The count at n = 6: the original runs 281 instructions, the
    /// pruned machine 269, twelve internal calls each one shorter;
    /// pushes are untouched at 48 because the pruning removes an
    /// assign, not a save.
    #[test]
    fn part_a_pruned_counts() {
        let mut original = fib_machine(RestoreDiscipline::Plain, FIB_CONTROLLER);
        original.set_register("n", Value::Int(6)).unwrap();
        original.start().unwrap();
        assert_eq!(original.instruction_count(), 281);
        assert_eq!(original.stack_statistics(), (48, 10));

        let mut pruned = fib_machine(RestoreDiscipline::Plain, FIB_PRUNED_CONTROLLER);
        pruned.set_register("n", Value::Int(6)).unwrap();
        pruned.start().unwrap();
        assert_eq!(pruned.instruction_count(), 269);
        assert_eq!(pruned.stack_statistics(), (48, 10));
    }

    /// Part (b): under the tagged discipline the out-of-order
    /// restore is a typed fault naming both registers, and the
    /// original machine (whose restores are all matched) runs
    /// unharmed while the pruned machine's exploit is caught.
    #[test]
    fn part_b_tagged_discipline() {
        let mut machine = make_machine(&["x", "y"], &[], OUT_OF_ORDER_CONTROLLER).unwrap();
        machine.set_restore_discipline(RestoreDiscipline::Tagged);
        let fault = machine.start().unwrap_err();
        assert_eq!(
            fault,
            Fault::MismatchedRestore {
                reg: "y".to_owned(),
                saved: "x".to_owned(),
                step: 5
            }
        );
        assert_eq!(run_fib(RestoreDiscipline::Tagged, FIB_CONTROLLER, 6), Ok(8));
        assert_eq!(
            run_fib(RestoreDiscipline::Tagged, FIB_PRUNED_CONTROLLER, 6),
            Err(Fault::MismatchedRestore {
                reg: "n".to_owned(),
                saved: "val".to_owned(),
                step: 52
            })
        );
    }

    /// Part (c): under the per-register discipline every restore
    /// finds its own register's last save, so the probe returns 7
    /// and the original machine still answers 8 at n = 6; the
    /// pruned machine is refused, since `n`'s stack is empty where
    /// its exploit fires.
    #[test]
    fn part_c_per_register_discipline() {
        let mut machine = make_machine(&["x", "y"], &[], OUT_OF_ORDER_CONTROLLER).unwrap();
        machine.set_restore_discipline(RestoreDiscipline::PerRegister);
        machine.start().unwrap();
        assert_eq!(machine.get_register("y").unwrap(), Value::Int(7));
        assert_eq!(
            run_fib(RestoreDiscipline::PerRegister, FIB_CONTROLLER, 6),
            Ok(8)
        );
        assert_eq!(
            run_fib(RestoreDiscipline::PerRegister, FIB_PRUNED_CONTROLLER, 6),
            Err(Fault::StackUnderflow {
                reg: "n".to_owned(),
                step: 93
            })
        );
    }
}
