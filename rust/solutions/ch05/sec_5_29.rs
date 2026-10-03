// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.29: tree recursion grows
//! work super-additively and depth linearly.
//!
//! The Fibonacci call tree doubles its avenues at every non-base
//! level, so the counted pushes outgrow any linear fit while the
//! maximum depth climbs one frame per level. The solution measures
//! n = 2..=8 on the compiler's counter and checks the shape, not
//! pinned constants: depths strictly increase, each push count tops
//! the sum of its two predecessors, and the answers are the Fibonacci
//! numbers themselves on both engines.

use sicp_runtime::host::CheckedProgram;

fn admitted(source: &str) -> CheckedProgram {
    match sicp_runtime::host::admit(source) {
        Ok(program) => program,
        Err(diag) => panic!("admitted: {}", diag.message),
    }
}

mod ex_5_29 {
    //! Exercise 5.29: fib's stack depth is linear in n while its
    //! pushes grow with the call tree.

    use super::*;

    const FIB: &str = "fn fib(n: i64) -> i64 {\n    if n < 2 {\n        n\n    } else {\n        fib(n - 1) + fib(n - 2)\n    }\n}\n\nfn main() {\n    println!(\"{}\", fib(2));\n    println!(\"{}\", fib(3));\n    println!(\"{}\", fib(4));\n    println!(\"{}\", fib(5));\n    println!(\"{}\", fib(6));\n    println!(\"{}\", fib(7));\n    println!(\"{}\", fib(8));\n}\n";

    fn single(n: u32) -> (u64, usize) {
        let source = format!(
            "fn fib(n: i64) -> i64 {{\n    if n < 2 {{\n        n\n    }} else {{\n        fib(n - 1) + fib(n - 2)\n    }}\n}}\n\nfn main() {{\n    println!(\"{{}}\", fib({n}));\n}}\n"
        );
        let program = admitted(&source);
        let (outcome, stats) = ch05::sec_5_5::compiled_run_counted(&program);
        assert!(outcome.trap.is_none(), "trapped at {n}: {outcome:?}");
        (stats.pushes, stats.max_depth)
    }

    /// The full session answers the Fibonacci numbers on both
    /// engines: `1, 2, 3, 5, 8, 13, 21`.
    #[test]
    fn ex_5_29_fib_answers() {
        let program = admitted(FIB);
        let interpreted = ch05::sec_5_4::Eceval::run(&program);
        let compiled = ch05::sec_5_5::compiled_run(&program);
        assert!(interpreted.trap.is_none(), "{interpreted:?}");
        assert_eq!(interpreted.stdout, "1\n2\n3\n5\n8\n13\n21\n");
        assert_eq!(interpreted.stdout, compiled.stdout, "engines agree");
    }

    /// Depths strictly increase with n while pushes outgrow the sum
    /// of their two predecessors: linear space, tree-shaped work.
    #[test]
    fn ex_5_29_tree_shape() {
        let mut rows = Vec::new();
        for n in 2..=8 {
            rows.push(single(n));
        }
        for window in rows.windows(2) {
            assert!(window[0].1 < window[1].1, "depth grows: {rows:?}");
        }
        for window in rows.windows(3) {
            assert!(
                window[2].0 > window[0].0 + window[1].0,
                "pushes outgrow their predecessors: {rows:?}"
            );
        }
    }
}
