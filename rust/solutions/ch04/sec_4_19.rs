// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.19: the scoping debate, with
//! each implementable position run under its own mechanism.

use ch04::eval_support::*;

/// The book's expression, evaluated under one evaluator.
fn evaluate(ev: &impl Evaluator) -> Result<String, SchemeError> {
    let program = "(let ((a 1))\n  (define (f x)\n    (define b (+ a x))\n    (define a 5)\n    (+ a b))\n  (f 10))";
    let (values, _) = run_with(ev, program)?;
    Ok(printed(&values).last().cloned().unwrap_or_default())
}

/// Answers Ben's sequential result and Alyssa's error.
fn answers() -> Result<(String, String), SchemeError> {
    let ben = evaluate(&Base)?;
    let alyssa = evaluate(&WithScanOut).expect_err("a is unassigned when b is computed");
    Ok((ben, alyssa.to_string()))
}

mod ex_4_19 {
    //! Exercise 4.19: the internal-definition scoping debate.

    use super::*;

    #[test]
    fn ex_4_19() {
        let (ben, alyssa) = answers().expect("runs");
        // Ben's sequential rule: b is the outer a plus x, 11, then a is
        // 5, and the answer is 16.
        assert_eq!(ben, "16");
        // Alyssa's mechanism: a is unassigned when b's value is
        // computed, so the program is an error rather than a wrong
        // answer.
        assert!(alyssa.contains("before its define runs"), "{alyssa}");
    }
}
