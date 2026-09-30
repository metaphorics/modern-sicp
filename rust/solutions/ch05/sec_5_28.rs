// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.28: tail calls hold constant
//! stack space.
//!
//! The counted compiler run shows the consequence the book's
//! counterfactual is about: the loop factorial's maximum depth is the
//! same at n = 3, 4, and 5, while the recursive factorial's depth
//! grows at every step. Both answer correctly on both engines, so the
//! difference is control, not arithmetic. The removed-optimization
//! evaluator itself — a machine that parks continuations across tail
//! calls — needs engine support the published API does not expose
//! (`Eceval::new` is private), so the counterfactual run is reported
//! as a capability gap rather than simulated.

use sicp_runtime::host::CheckedProgram;

fn admitted(source: &str) -> CheckedProgram {
    match sicp_runtime::host::admit(source) {
        Ok(program) => program,
        Err(diag) => panic!("admitted: {}", diag.message),
    }
}

fn counted_at(source: &str, n: u32) -> (String, u64, usize) {
    let program = admitted(&source.replace("{n}", &n.to_string()));
    let (outcome, stats) = ch05::sec_5_5::compiled_run_counted(&program);
    assert!(outcome.trap.is_none(), "trapped: {outcome:?}");
    (outcome.stdout, stats.pushes, stats.max_depth)
}

mod ex_5_28 {
    //! Exercise 5.28: with tail calls the loop's depth is constant in
    //! n; without them it would grow like the recursion's. The
    //! without-them machine is not constructible from the published
    //! API, so the test shows the with-them half and the doc names
    //! the missing half.

    use super::*;

    const LOOP: &str = "fn factorial(n: i64) -> i64 {\n    let mut product = 1;\n    let mut counter = 1;\n    while counter <= n {\n        product = product * counter;\n        counter += 1;\n    }\n    product\n}\n\nfn main() {\n    println!(\"{}\", factorial({n}));\n}\n";

    const RECURSIVE: &str = "fn factorial(n: i64) -> i64 {\n    if n == 1 {\n        1\n    } else {\n        n * factorial(n - 1)\n    }\n}\n\nfn main() {\n    println!(\"{}\", factorial({n}));\n}\n";

    /// The loop's counted depth is the same at n = 3, 4, and 5 while
    /// its answers run 6, 24, 120: iteration reuses the frame.
    #[test]
    fn ex_5_28_loop_depth_is_constant() {
        let first = counted_at(LOOP, 3);
        let second = counted_at(LOOP, 4);
        let third = counted_at(LOOP, 5);
        assert_eq!(
            (first.0.as_str(), second.0.as_str(), third.0.as_str()),
            ("6\n", "24\n", "120\n")
        );
        assert_eq!(first.2, second.2, "loop depth at 3 and 4");
        assert_eq!(second.2, third.2, "loop depth at 4 and 5");
    }

    /// The recursion's counted depth grows at every step over the
    /// same inputs: each call parks a frame the loop never needs.
    #[test]
    fn ex_5_28_recursive_depth_grows() {
        let first = counted_at(RECURSIVE, 3);
        let second = counted_at(RECURSIVE, 4);
        let third = counted_at(RECURSIVE, 5);
        assert_eq!(
            (first.0.as_str(), second.0.as_str(), third.0.as_str()),
            ("6\n", "24\n", "120\n")
        );
        assert!(first.2 < second.2, "depth grows from 3 to 4");
        assert!(second.2 < third.2, "depth grows from 4 to 5");
    }
}
