// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.18: the alternative scan-out
//! strategy enforces the restriction the text's strategy only states.

use ch04::eval_support::*;

/// Answers what the two strategies give for a body whose second
/// initializer reads the first defined name.
fn answers() -> Result<(String, String), SchemeError> {
    let program = "(define (f) (define u 5) (define v (* u 2)) v)\n(f)";
    let (values, _) = run_with(&WithScanOut, program)?;
    let text_strategy = printed(&values).last().cloned().unwrap_or_default();
    let alternative = run_with(&WithScanOutAlt, program).expect_err("v reads u too early");
    Ok((text_strategy, alternative.to_string()))
}

mod ex_4_18 {
    //! Exercise 4.18: the alternative scan-out strategy.

    use super::*;

    #[test]
    fn ex_4_18() {
        let (text_strategy, alternative) = answers().expect("runs");
        // The text's strategy runs the program, for the accidental
        // reason the IEEE footnote names: the set!s run in order, so u
        // is 5 by the time v's set! reads it.
        assert_eq!(text_strategy, "10");
        // The alternative evaluates the initializers first, while every
        // name is still unassigned, and so refuses the program.
        assert!(
            alternative.contains("before its define runs"),
            "{alternative}"
        );
    }
}
