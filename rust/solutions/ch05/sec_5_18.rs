// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.18: traced registers report
//! every change through the machine's single store path.

use ch05::sec_5_2::factorial_machine;
use sicp_runtime::Value;

mod ex_5_18 {
    //! Exercise 5.18: registers accept trace-on and trace-off; a
    //! traced register prints its name, old contents, and new
    //! contents on every assignment.

    use super::*;

    /// The factorial machine on n = 3 with `n` and `val` traced: the
    /// host's initial load, every assign, and every restore report,
    /// because the machine has exactly one store path.
    #[test]
    fn ex_5_18_traced_stores_report() {
        let mut machine = factorial_machine();
        machine.trace_register("n").unwrap();
        machine.trace_register("val").unwrap();
        machine.set_register("n", Value::Int(3)).unwrap();
        machine.start().unwrap();
        assert_eq!(machine.get_register("val").unwrap(), Value::Int(6));
        assert_eq!(
            machine.transcript(),
            [
                "n: *unassigned* -> 3".to_owned(),
                "n: 3 -> 2".to_owned(),
                "n: 2 -> 1".to_owned(),
                "val: *unassigned* -> 1".to_owned(),
                "n: 1 -> 2".to_owned(),
                "val: 1 -> 2".to_owned(),
                "n: 2 -> 3".to_owned(),
                "val: 2 -> 6".to_owned(),
            ]
        );
    }

    /// Untracing silences a register; the answer is unaffected
    /// either way.
    #[test]
    fn ex_5_18_untrace_silences() {
        let mut machine = factorial_machine();
        machine.trace_register("n").unwrap();
        machine.trace_register("val").unwrap();
        machine.untrace_register("val").unwrap();
        machine.set_register("n", Value::Int(3)).unwrap();
        machine.start().unwrap();
        let reported: Vec<&String> = machine
            .transcript()
            .iter()
            .filter(|line| line.starts_with("val:"))
            .collect();
        assert!(reported.is_empty());
        assert_eq!(machine.get_register("val").unwrap(), Value::Int(6));
        assert_eq!(machine.transcript().len(), 5);
    }
}
