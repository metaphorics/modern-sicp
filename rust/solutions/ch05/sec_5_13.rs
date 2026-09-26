// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.13: machines whose register
//! sets are read out of the controller text.

use ch05::sec_5_2::{OpHandler, make_machine_with_derived_registers, op};
use sicp_runtime::Value;

fn gcd_operations() -> Vec<(&'static str, OpHandler)> {
    vec![
        ("=", op("=").expect("shared")),
        ("rem", op("rem").expect("shared")),
    ]
}

fn fib_operations() -> Vec<(&'static str, OpHandler)> {
    vec![
        ("<", op("<").expect("shared")),
        ("+", op("+").expect("shared")),
        ("-", op("-").expect("shared")),
    ]
}

/// The GCD controller of Figure 5.4 without the read loop.
const GCD_CONTROLLER: &str = "
test-b
  (test (op =) (reg b) (const 0))
  (branch (label gcd-done))
  (assign t (op rem) (reg a) (reg b))
  (assign a (reg b))
  (assign b (reg t))
  (goto (label test-b))
gcd-done";

/// The Fibonacci controller of Figure 5.12.
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

mod ex_5_13 {
    //! Exercise 5.13: determine the registers from the controller
    //! sequence instead of a supplied list.

    use super::*;

    /// The GCD controller derives exactly `a`, `b`, and `t`, and the
    /// derived machine answers the text's session, gcd(206, 40) = 2.
    #[test]
    fn ex_5_13_derived_gcd_runs() {
        let mut machine =
            make_machine_with_derived_registers(&gcd_operations(), GCD_CONTROLLER).unwrap();
        assert_eq!(machine.register_names(), ["a", "b", "t"]);
        machine.set_register("a", Value::Int(206)).unwrap();
        machine.set_register("b", Value::Int(40)).unwrap();
        machine.start().unwrap();
        assert_eq!(machine.get_register("a").unwrap(), Value::Int(2));
    }

    /// The Fibonacci controller derives `continue`, `n`, and `val`
    /// (and never the machine's own `flag`, which no controller
    /// names), and the derived machine answers fib(6) = 8.
    #[test]
    fn ex_5_13_derived_fibonacci_runs() {
        let mut machine =
            make_machine_with_derived_registers(&fib_operations(), FIB_CONTROLLER).unwrap();
        assert_eq!(machine.register_names(), ["continue", "n", "val"]);
        machine.set_register("n", Value::Int(6)).unwrap();
        machine.start().unwrap();
        assert_eq!(machine.get_register("val").unwrap(), Value::Int(8));
    }

    /// A controller that names an operation the machine lacks still
    /// fails the assembly, derived registers or not.
    #[test]
    fn ex_5_13_unknown_operation_still_refused() {
        let controller = "
  (assign a (op mystery) (reg a))";
        let fault = make_machine_with_derived_registers(&gcd_operations(), controller).unwrap_err();
        assert!(matches!(
            fault,
            ch05::sec_5_2::Fault::UnknownOperation { .. }
        ));
    }

    /// A controller that names no register derives an empty register
    /// set, and the machine still runs its constant instruction.
    #[test]
    fn ex_5_13_constant_only_controller_derives_nothing() {
        let controller = "
  (perform (op print) (const 3))";
        let mut machine =
            make_machine_with_derived_registers(&gcd_operations(), controller).unwrap();
        assert!(machine.register_names().is_empty());
        machine.start().unwrap();
        assert_eq!(machine.transcript(), ["3".to_owned()]);
    }
}
