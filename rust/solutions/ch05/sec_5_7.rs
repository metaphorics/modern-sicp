// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.7: the two exponentiation
//! machines of exercise 5.4, run on the section's simulator.

use ch05::sec_5_2::{Machine, OpHandler, make_machine, op};
use sicp_runtime::Value;

/// The linear-recursive exponentiation machine of exercise 5.4:
/// `n` counts down, each level saves `continue`, and the pending
/// multiplications ride the stack.
const EXPONENT_RECURSIVE_CONTROLLER: &str = "
  (assign continue (label expt-done))
expt-loop
  (test (op =) (reg n) (const 0))
  (branch (label base-expt))
  (save continue)
  (assign n (op -) (reg n) (const 1))
  (assign continue (label multiply))
  (goto (label expt-loop))
multiply
  (assign val (op *) (reg b) (reg val))
  (restore continue)
  (goto (reg continue))
base-expt
  (assign val (const 1))
  (goto (reg continue))
expt-done";

/// The iterative exponentiation machine of exercise 5.4: a product
/// accumulator and a counter, no stack.
const EXPONENT_ITERATIVE_CONTROLLER: &str = "
  (assign counter (reg n))
  (assign product (const 1))
expt-iter
  (test (op =) (reg counter) (const 0))
  (branch (label expt-done))
  (assign product (op *) (reg product) (reg b))
  (assign counter (op -) (reg counter) (const 1))
  (goto (label expt-iter))
expt-done";

fn exponent_operations() -> Vec<(&'static str, OpHandler)> {
    vec![
        ("=", op("=").expect("shared")),
        ("*", op("*").expect("shared")),
        ("-", op("-").expect("shared")),
    ]
}

fn recursive_exponent_machine() -> Machine {
    make_machine(
        &["b", "n", "val", "continue"],
        &exponent_operations(),
        EXPONENT_RECURSIVE_CONTROLLER,
    )
    .expect("assembles")
}

fn iterative_exponent_machine() -> Machine {
    make_machine(
        &["b", "n", "counter", "product"],
        &exponent_operations(),
        EXPONENT_ITERATIVE_CONTROLLER,
    )
    .expect("assembles")
}

mod ex_5_07 {
    //! Exercise 5.7: use the simulator to test the machines designed
    //! in exercise 5.4.

    use super::*;

    /// The recursive machine answers `b^n` and pays one save and one
    /// restore per level: pushes and depth are both `n`.
    #[test]
    fn ex_5_07_recursive() {
        for b in 2i128..=5 {
            for n in 0u32..=9 {
                let mut machine = recursive_exponent_machine();
                machine.set_register("b", Value::Int(b)).unwrap();
                machine
                    .set_register("n", Value::Int(i128::from(n)))
                    .unwrap();
                machine.start().unwrap();
                assert_eq!(machine.get_register("val").unwrap(), Value::Int(b.pow(n)));
                let (pushes, depth) = machine.stack_statistics();
                assert_eq!(pushes, u64::from(n));
                assert_eq!(depth, u64::from(n));
            }
        }
    }

    /// The iterative machine answers the same values and never
    /// touches the stack.
    #[test]
    fn ex_5_07_iterative() {
        for b in 2i128..=5 {
            for n in 0u32..=9 {
                let mut machine = iterative_exponent_machine();
                machine.set_register("b", Value::Int(b)).unwrap();
                machine
                    .set_register("n", Value::Int(i128::from(n)))
                    .unwrap();
                machine.start().unwrap();
                let answer = machine.get_register("product").unwrap();
                assert_eq!(answer, Value::Int(b.pow(n)));
                assert_eq!(machine.stack_statistics(), (0, 0));
            }
        }
    }

    /// The book's worked inputs, pinned literally: 2^10 = 1024 and
    /// 3^5 = 243 on both machines.
    #[test]
    fn ex_5_07_pinned_inputs() {
        let mut machine = recursive_exponent_machine();
        machine.set_register("b", Value::Int(2)).unwrap();
        machine.set_register("n", Value::Int(10)).unwrap();
        machine.start().unwrap();
        assert_eq!(machine.get_register("val").unwrap(), Value::Int(1024));

        let mut machine = iterative_exponent_machine();
        machine.set_register("b", Value::Int(3)).unwrap();
        machine.set_register("n", Value::Int(5)).unwrap();
        machine.start().unwrap();
        assert_eq!(machine.get_register("product").unwrap(), Value::Int(243));
    }
}
