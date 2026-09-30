// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.45: the special-purpose
//! machine parks the least stack.
//!
//! The recursive factorial runs two ways at n = 5 and 10: the counted
//! compiler run and the special-purpose Figure 5.11 machine. The
//! hand-tuned machine parks exactly `2n - 2` pushes, and its depth
//! never exceeds the compiled run's. Every implementation answers
//! correctly, so the rows compare control, not arithmetic.

use ch05::sec_5_1::factorial_recursive;
use ch05::sec_5_2::{Machine, assemble};

fn admitted(source: &str) -> sicp_runtime::host::CheckedProgram {
    match sicp_runtime::host::admit(source) {
        Ok(program) => program,
        Err(diag) => panic!("admitted: {}", diag.message),
    }
}

fn render(source: &str) -> (String, u64, usize) {
    let program = admitted(source);
    let (outcome, stats) = ch05::sec_5_5::compiled_run_counted(&program);
    assert!(outcome.trap.is_none(), "trapped: {outcome:?}");
    (outcome.stdout, stats.pushes, stats.max_depth)
}

mod ex_5_45 {
    //! Exercise 5.45: the special-purpose machine parks the least
    //! stack; every implementation answers correctly.

    use super::*;

    const TEMPLATE: &str = "fn factorial(n: i64) -> i64 {\n    if n == 1 {\n        1\n    } else {\n        n * factorial(n - 1)\n    }\n}\n\nfn main() {\n    println!(\"{}\", factorial({n}));\n}\n";

    fn machine_at(n: i64) -> (u64, usize) {
        let mut machine = Machine::new(assemble(&factorial_recursive()).expect("assembles"));
        machine.set_register("n", n).expect("register n");
        machine.run().expect("runs");
        let stats = machine.stack_statistics();
        (stats.pushes, stats.max_depth)
    }

    /// At n = 5 and 10 the answers are `120` and `3628800`, the
    /// machine parks exactly `2n - 2` pushes, and its depth never
    /// exceeds the compiled run's.
    #[test]
    fn ex_5_45_compilation_cuts_stack() {
        for (n, answer) in [(5i64, "120\n"), (10, "3628800\n")] {
            let source = TEMPLATE.replace("{n}", &n.to_string());
            let (stdout, _, compiled_depth) = render(&source);
            let (pushes, depth) = machine_at(n);
            assert_eq!(stdout, answer, "compiled answer at {n}");
            assert_eq!(pushes, u64::try_from(2 * (n - 1)).expect("small n"));
            assert!(depth <= compiled_depth, "hand-tuned machine wins at {n}");
        }
    }
}
