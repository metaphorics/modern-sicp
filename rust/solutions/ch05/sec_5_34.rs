// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.34: the loop's depth stays
//! flat while its answers grow.
//!
//! The tail-recursive `iter` of the book becomes the host `while`
//! loop: no continuation is parked across iterations, so the counted
//! compiler stack reaches the same maximum depth at n = 3, 4, and 5
//! while the answers run 6, 24, 120. Both engines agree on every row,
//! which is the transfer the exercise asks about made observable: the
//! loop reuses its frame instead of saving a return address.

use sicp_runtime::host::CheckedProgram;

fn admitted(source: &str) -> CheckedProgram {
    match sicp_runtime::host::admit(source) {
        Ok(program) => program,
        Err(diag) => panic!("admitted: {}", diag.message),
    }
}

fn depth_at(template: &str, n: u32) -> (String, usize) {
    let program = admitted(&template.replace("{n}", &n.to_string()));
    let (outcome, stats) = ch05::sec_5_5::compiled_run_counted(&program);
    assert!(outcome.trap.is_none(), "trapped at {n}: {outcome:?}");
    (outcome.stdout, stats.max_depth)
}

mod ex_5_34 {
    //! Exercise 5.34: iteration transfers without parking state, so
    //! the maximum depth is the same across inputs.

    use super::*;

    const SINGLE: &str = "fn factorial(n: i64) -> i64 {\n    let mut product = 1;\n    let mut counter = 1;\n    while counter <= n {\n        product = product * counter;\n        counter += 1;\n    }\n    product\n}\n\nfn main() {\n    println!(\"{}\", factorial({n}));\n}\n";

    const SESSION: &str = "fn factorial(n: i64) -> i64 {\n    let mut product = 1;\n    let mut counter = 1;\n    while counter <= n {\n        product = product * counter;\n        counter += 1;\n    }\n    product\n}\n\nfn main() {\n    println!(\"{}\", factorial(3));\n    println!(\"{}\", factorial(4));\n    println!(\"{}\", factorial(5));\n}\n";

    /// Each input answers on both engines, and the counted depth at
    /// n = 3, 4, and 5 is one unchanging number.
    #[test]
    fn ex_5_34_tail_transfer_holds_depth_flat() {
        let program = admitted(SESSION);
        let interpreted = ch05::sec_5_4::Eceval::run(&program);
        assert!(interpreted.trap.is_none(), "{interpreted:?}");
        assert_eq!(interpreted.stdout, "6\n24\n120\n");
        let first = depth_at(SINGLE, 3);
        let second = depth_at(SINGLE, 4);
        let third = depth_at(SINGLE, 5);
        assert_eq!(
            (first.0.as_str(), second.0.as_str(), third.0.as_str()),
            ("6\n", "24\n", "120\n")
        );
        assert_eq!(first.1, second.1, "depth at 3 and 4");
        assert_eq!(second.1, third.1, "depth at 4 and 5");
    }
}
