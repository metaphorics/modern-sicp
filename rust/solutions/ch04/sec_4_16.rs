// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.16: scan out internal
//! definitions, installed where the procedure is constructed; reading a
//! scanned name before its define runs is the unassigned error.

use ch04::eval_support::*;

/// The printed answers: the mutually recursive `f` under the scanned
/// evaluator and the premature-read error on `g`.
fn answers() -> Result<(String, String), SchemeError> {
    let f = "(define (f x)\n  (define (even? n) (if (= n 0) true (odd? (- n 1))))\n  (define (odd? n) (if (= n 0) false (even? (- n 1))))\n  (even? x))\n(f 10)";
    let g = "(define (g) (define a (* b 2)) (define b 3) a)\n(g)";
    let (values, _) = run_with(&WithScanOut, f)?;
    let mutual = printed(&values).last().cloned().unwrap_or_default();
    let premature = run_with(&WithScanOut, g).expect_err("a is read too early");
    Ok((mutual, premature.to_string()))
}

mod ex_4_16 {
    //! Exercise 4.16: scan out internal definitions.

    use super::*;

    #[test]
    fn ex_4_16() {
        let (mutual, premature) = answers().expect("runs");
        // Mutual recursion works under the scan-out...
        assert_eq!(mutual, "#t");
        // ...and a premature read names the unassigned binding, the
        // scan-out failure, where the sequential mechanism says unbound.
        assert!(premature.contains("before its define runs"), "{premature}");
    }
}
