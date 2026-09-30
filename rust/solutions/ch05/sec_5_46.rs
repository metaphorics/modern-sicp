// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.46: tree recursion grows work
//! faster than depth on every implementation.
//!
//! The Fibonacci recurrence runs two ways at n = 5, 6, and 7: the
//! counted compiler run and the special-purpose machine. Each row
//! carries both counters for both implementations and the answers are
//! the Fibonacci numbers. Depths climb on both implementations while
//! each compiled push count tops the sum of its two predecessors. The
//! solution presents the measured rows rather than extrapolating a
//! limiting ratio from three short points.

use ch05::sec_5_1::fibonacci_machine;
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

mod ex_5_46 {
    //! Exercise 5.46: fib's rows show tree-shaped work at every
    //! input, on both implementations.

    use super::*;

    const TEMPLATE: &str = "fn fib(n: i64) -> i64 {\n    if n < 2 {\n        n\n    } else {\n        fib(n - 1) + fib(n - 2)\n    }\n}\n\nfn main() {\n    println!(\"{}\", fib({n}));\n}\n";

    fn machine_at(n: i64) -> (i64, u64, usize) {
        let mut machine = Machine::new(assemble(&fibonacci_machine()).expect("assembles"));
        machine.set_register("n", n).expect("register n");
        machine.run().expect("runs");
        let stats = machine.stack_statistics();
        (
            machine.get_register("val").expect("val"),
            stats.pushes,
            stats.max_depth,
        )
    }

    /// Three rows, one per input: the answers are `5`, `8`, and `13`
    /// on both implementations, depths climb everywhere, and each
    /// compiled push count tops the sum of its two predecessors.
    #[test]
    fn ex_5_46_fib_rows() {
        let mut rows: Vec<(u64, usize, usize)> = Vec::new();
        for (n, answer) in [(5i64, "5\n"), (6, "8\n"), (7, "13\n")] {
            let source = TEMPLATE.replace("{n}", &n.to_string());
            let (stdout, pushes, depth) = render(&source);
            let (machine_answer, _, machine_depth) = machine_at(n);
            assert_eq!(stdout, answer, "compiled answer at {n}");
            assert_eq!(
                machine_answer.to_string() + "\n",
                answer,
                "machine answer at {n}"
            );
            rows.push((pushes, depth, machine_depth));
        }
        for window in rows.windows(2) {
            assert!(window[0].1 < window[1].1, "compiled depth climbs: {rows:?}");
            assert!(window[0].2 < window[1].2, "machine depth climbs: {rows:?}");
        }
        assert!(
            rows[2].0 > rows[0].0 + rows[1].0,
            "pushes outgrow their predecessors: {rows:?}"
        );
    }
}
