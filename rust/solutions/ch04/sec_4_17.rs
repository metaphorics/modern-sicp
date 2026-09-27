// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.17: the extra frame of the
//! scanned-out program, shown on the body the text transforms.

use ch04::eval_support::*;

/// Answers the value `f` gives under both mechanisms and the printed
/// scanned-out body whose `let` is the extra frame.
fn answers() -> Result<(String, String), SchemeError> {
    let program = "(define (f x) (define u (* x 2)) (define v (+ u 1)) (+ u v))\n(f 5)";
    let sequential = run_base(program);
    assert_eq!(sequential.last(), Some(&"21".to_owned()));
    let (values, _) = run_with(&WithScanOut, program)?;
    let scanned = printed(&values).last().cloned().unwrap_or_default();
    let body = lambda_body(
        &read("(lambda (x) (define u (* x 2)) (define v (+ u 1)) (+ u v))").expect("read"),
    )?;
    let rewritten = scanned_body(&body)?;
    Ok((scanned, print_value(&rewritten)))
}

mod ex_4_17 {
    //! Exercise 4.17: the extra frame of the scanned-out program.

    use super::*;

    #[test]
    fn ex_4_17() {
        let (value, scanned) = answers().expect("runs");
        // A correct program behaves the same under both mechanisms.
        assert_eq!(value, "21");
        // The transform binds u and v in a `let` inside the call frame:
        // that let's frame is the extra one. It can never change a
        // correct program's behavior, because every read of a scanned
        // name happens after its set! and name resolution simply walks
        // one frame further; the frame disappears when the parameter
        // frame itself is created holding u and v unassigned beside x.
        assert_eq!(
            scanned,
            "(let ((u (quote *unassigned*)) (v (quote *unassigned*))) (begin (set! u (* x 2)) (set! v (+ u 1)) (+ u v)))"
        );
    }
}
