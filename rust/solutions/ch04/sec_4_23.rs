// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solutions of exercise 4.23 and the edition's addition
//! 4.23a: the two analyze-sequence versions, and the counter that shows
//! where the sequencing work lands.

use ch04::eval_support::*;

/// How many analysis invocations a one-expression body costs under each
/// version -- the same single pass -- and the shared values of one- and
/// two-expression bodies.
fn answers() -> Result<(u64, u64), SchemeError> {
    let text = Counting::new(false);
    let alyssa = Counting::new(true);
    let one = read("(lambda () (+ 3 4))").expect("read");
    let body = lambda_body(&one)?;
    text.analyze_sequence(&body)?;
    alyssa.analyze_sequence(&body)?;
    let defines = "(define (f) (+ 3 4))\n(define (g) 4 3)";
    let (values, _) = run_analyzed(
        &AlyssaAnalyzer::default(),
        &format!("{defines}\n(f)\n(f)\n(g)\n(g)"),
    )?;
    let base_answers: Vec<String> = run_base(&format!("{defines}\n(f)\n(f)\n(g)\n(g)"))
        .into_iter()
        .filter(|answer| answer != "ok")
        .collect();
    let answers: Vec<String> = printed(&values)
        .into_iter()
        .filter(|answer| answer != "ok")
        .collect();
    assert_eq!(base_answers, ["7", "7", "3", "3"]);
    assert_eq!(answers, vec!["7", "7", "3", "3"]);
    Ok((text.analyze_calls(), alyssa.analyze_calls()))
}

/// The sequence-execution counts for a one-expression body executed
/// twice under each style.
fn counting_answers() -> Result<(u64, u64), SchemeError> {
    let program = "(define (f) (+ 3 4))\n(define exec (lambda () (f)))\n(exec)\n(exec)";
    let text = Counting::new(false);
    let (values, _) = run_analyzed(&text, program)?;
    assert_eq!(printed(&values).last(), Some(&"7".to_owned()));
    let alyssa = Counting::new(true);
    let (values, _) = run_analyzed(&alyssa, program)?;
    assert_eq!(printed(&values).last(), Some(&"7".to_owned()));
    assert_eq!(text.analyze_calls(), alyssa.analyze_calls());
    Ok((text.sequence_execs(), alyssa.sequence_execs()))
}

mod ex_4_23 {
    //! Exercise 4.23: the two analyze-sequence versions.

    use super::*;

    #[test]
    fn ex_4_23() {
        let (text_analyses, alyssa_analyses) = answers().expect("runs");
        // Both versions analyze the body in the same single pass: the
        // analysis invocation counts are identical.
        assert_eq!(text_analyses, alyssa_analyses);
    }
}

mod ex_4_23a {
    //! Exercise 4.23a (this edition): counting the sequencing work.

    use super::*;

    #[test]
    fn ex_4_23a() {
        let (text_execs, alyssa_execs) = counting_answers().expect("runs");
        // Two executions of each of the two one-expression bodies: the
        // text's version ran no sequence wrapper at all; Alyssa's ran
        // hers once per body execution.
        assert_eq!((text_execs, alyssa_execs), (0, 4));
    }
}
