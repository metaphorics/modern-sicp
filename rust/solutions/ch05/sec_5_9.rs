// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.9: an operation input may be
//! a register or a constant, never a label; the grammar refuses the
//! label at assembly time.

use ch05::sec_5_2::{Fault, Machine, OpHandler, make_machine, op};
use sicp_runtime::Value;

fn arithmetic() -> Vec<(&'static str, OpHandler)> {
    vec![("+", op("+").expect("shared"))]
}

/// Reads `a` after one pass of the machine.
fn run(registers: &[&str], operations: &[(&str, OpHandler)], controller: &str) -> Machine {
    make_machine(registers, operations, controller).expect("assembles")
}

mod ex_5_09 {
    //! Exercise 5.9: modify the expression-processing procedures so
    //! operations can be used only with registers and constants.

    use super::*;

    /// The book's permissive reading: `(op +)` over `(reg b)` and
    /// `(reg c)` still answers 5.
    #[test]
    fn ex_5_09_registers_and_constants_still_work() {
        let controller = "
  (assign a (op +) (reg b) (reg c))
  (assign d (op +) (const 2) (const 3))";
        let mut machine = run(&["a", "b", "c", "d"], &arithmetic(), controller);
        machine.set_register("b", Value::Int(2)).unwrap();
        machine.set_register("c", Value::Int(3)).unwrap();
        machine.start().unwrap();
        assert_eq!(machine.get_register("a").unwrap(), Value::Int(5));
        assert_eq!(machine.get_register("d").unwrap(), Value::Int(5));
    }

    /// A label in operand position now fails the assembly with the
    /// typed [`Fault::LabelOperand`], naming the operation and the
    /// refused label.
    #[test]
    fn ex_5_09_label_operand_fails_assembly() {
        let controller = "
  (assign b (op +) (label there))
there";
        let fault = make_machine(&["b"], &arithmetic(), controller).unwrap_err();
        assert_eq!(
            fault,
            Fault::LabelOperand {
                op: "+".to_owned(),
                label: "there".to_owned()
            }
        );
    }

    /// The instruction's other operand does not excuse the label:
    /// the check is per input, at assembly, before the machine can
    /// start.
    #[test]
    fn ex_5_09_label_among_valid_inputs_still_refused() {
        let controller = "
  (assign b (op +) (reg b) (label there))
there";
        let fault = make_machine(&["b"], &arithmetic(), controller).unwrap_err();
        assert!(matches!(fault, Fault::LabelOperand { .. }));
    }
}
