// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.17: a traced instruction is
//! announced by the labels that immediately precede it, and the
//! count of exercise 5.15 is untouched.

use ch05::sec_5_2::fibonacci_machine;
use sicp_runtime::Value;

mod ex_5_17 {
    //! Exercise 5.17: print the labels preceding a traced
    //! instruction, in a way that does not interfere with counting.

    use super::*;

    /// The label-traced fib(3) run: the trace carries 63 lines,
    /// 51 instruction lines and 12 label lines, and the instruction
    /// count of exercise 5.15 is still exactly 51.
    #[test]
    fn ex_5_17_labels_announced() {
        let mut machine = fibonacci_machine();
        machine.set_register("n", Value::Int(3)).unwrap();
        machine.set_trace(true);
        machine.set_label_trace(true);
        machine.start().unwrap();
        assert_eq!(machine.instruction_count(), 51);
        assert_eq!(machine.transcript().len(), 63);
        let label_lines: Vec<&String> = machine
            .transcript()
            .iter()
            .filter(|line| line.ends_with(':'))
            .collect();
        let names: Vec<&str> = label_lines
            .iter()
            .map(|line| line.trim_end_matches(':'))
            .collect();
        assert_eq!(
            names,
            [
                "fib-loop",
                "fib-loop",
                "fib-loop",
                "immediate-answer",
                "afterfib-n-1",
                "fib-loop",
                "immediate-answer",
                "afterfib-n-2",
                "afterfib-n-1",
                "fib-loop",
                "immediate-answer",
                "afterfib-n-2",
            ]
        );
    }

    /// Each label line comes immediately before its instruction, and
    /// the first instruction of the sequence (before any label) is
    /// announced by nothing.
    #[test]
    fn ex_5_17_label_precedes_its_instruction() {
        let mut machine = fibonacci_machine();
        machine.set_register("n", Value::Int(3)).unwrap();
        machine.set_trace(true);
        machine.set_label_trace(true);
        machine.start().unwrap();
        let transcript = machine.transcript();
        assert_eq!(transcript[0], "(assign continue (label fib-done))");
        assert_eq!(transcript[1], "fib-loop:");
        assert_eq!(transcript[2], "(test (op <) (reg n) (const 2))");
        assert_eq!(transcript[3], "(branch (label immediate-answer))");
        assert_eq!(transcript[20], "immediate-answer:");
        assert_eq!(transcript[21], "(assign val (reg n))");
    }
}
