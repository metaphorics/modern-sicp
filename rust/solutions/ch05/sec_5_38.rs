// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.38: open-coded primitive
//! operations over `arg1` and `arg2`.

use ch05::sec_5_2::Fault;
use ch05::sec_5_5::{Config, Linkage, compile_and_go, compile_program, new_state};

mod ex_5_38 {
    //! Exercise 5.38: open-coded factorial shrinks the emitted code;
    //! open-coded n-ary addition and multiplication fold through one
    //! register.

    use super::*;

    const FACTORIAL: &str = "(define (factorial n) (if (= n 1) 1 (* (factorial (- n 1)) n)))";

    fn statement_count(cfg: &Config, source: &str) -> Result<usize, Fault> {
        Ok(compile_program(cfg, &new_state(), source, &Linkage::Next)?
            .stmts
            .len())
    }

    /// Compiles every form of `source` onto the machine and runs them:
    /// the answers the compiled block prints, in order.
    fn compiled_answers(cfg: &Config, source: &str) -> Result<Vec<String>, Fault> {
        let mut evaluator = compile_and_go(cfg, &new_state(), source, "")?;
        evaluator.run()?;
        Ok(evaluator.transcript())
    }

    /// The compiled open-coded callers whose later operand is itself a
    /// compound call, with the answer each must print: the callee's
    /// open-coded body trashes the caller's `arg1` and `val` scratch,
    /// so the operand shields keyed on the call's claimed registers
    /// must fire. Before the call claimed `arg1` and `arg2` (the
    /// defect the TypeScript 5.5 review caught), this path answered
    /// 11, 14, and 103; the fourth shape shields the folded
    /// accumulator in `val`.
    const SHIELDED_CASES: [(&str, &str); 4] = [
        ("(define (f y) (* y 10))\n(define x 4)\n(+ x (f 1))", "14"),
        ("(define (f y) (* y 10))\n(define x 4)\n(+ x (f 1) 3)", "17"),
        (
            "(define (f y) (* y 10))\n(define (g y) (+ y 100))\n(+ (+ (f 1) (+ 2 3)) (+ (* 2 2) (g 1)))",
            "120",
        ),
        ("(define (f y) (* y 10))\n(+ 2 3 (f 1))", "15"),
    ];

    /// The nested n-ary shapes of the Rereview 2 finding: an inner
    /// three-plus-operand `+`/`*` in a non-`val` operand position.
    /// The fold answers in `val`, so the generator must copy it into
    /// the operand's requested target; before `target` was threaded
    /// the outer spread read the stale `arg1` (for example
    /// `(+ (+ 1 2 3) 4)` answered 7 and `(* (+ 1 2 3) (+ 4 5))`
    /// answered 27).
    const NARY_TARGET_CASES: [(&str, &str); 4] = [
        ("(+ (+ 1 2 3) 4)", "10"),
        ("(+ 1 (+ 2 3 4))", "10"),
        ("(* (+ 1 2 3) (+ 4 5))", "54"),
        ("(+ (+ 1 2 3) (+ 4 5) 7)", "22"),
    ];

    pub fn ex_5_38() -> Result<Vec<String>, Fault> {
        let open = Config {
            open_code: true,
            ..ch05::sec_5_5::default_config()
        };
        let plain_count = statement_count(&ch05::sec_5_5::default_config(), FACTORIAL)?;
        let open_count = statement_count(&open, FACTORIAL)?;
        let source = "(factorial 5)\n(+ 1 2 3 4)\n(< 1 2)\n(- 10 3)";
        let mut evaluator = compile_and_go(&open, &new_state(), FACTORIAL, source)?;
        evaluator.run()?;
        let transcript = evaluator.transcript();
        assert!(transcript.contains(&"120".to_owned()), "{transcript:?}");
        assert!(transcript.contains(&"10".to_owned()), "{transcript:?}");
        assert!(transcript.contains(&"#t".to_owned()), "{transcript:?}");
        assert!(
            open_count < plain_count,
            "open={open_count}, plain={plain_count}"
        );
        // An open-coded call whose operands mix a compound call with
        // variable references: the call operand rebinds `env` and
        // answers in `val`, so the later operands must read the
        // caller's frame and the compound result must reach the call
        // target register.
        let mixed = Config {
            open_code: true,
            compound_calls: true,
            ..ch05::sec_5_5::default_config()
        };
        let mut nary_evaluator = compile_and_go(
            &mixed,
            &new_state(),
            "(define (f x) (+ (h x) x 1))",
            "(define (h y) (* y 10))\n(f 4)",
        )?;
        nary_evaluator.run()?;
        let nary_transcript = nary_evaluator.transcript();
        assert!(
            nary_transcript.contains(&"45".to_owned()),
            "{nary_transcript:?}"
        );
        let mut two_evaluator = compile_and_go(
            &mixed,
            &new_state(),
            "(define (f x) (+ (h x) x))",
            "(define (h y) (* y 10))\n(f 4)",
        )?;
        two_evaluator.run()?;
        let two_transcript = two_evaluator.transcript();
        assert!(
            two_transcript.contains(&"44".to_owned()),
            "{two_transcript:?}"
        );
        let compound_sessions = [nary_transcript, two_transcript]
            .map(|t| t.join(" "))
            .join(" | ");
        let mut shielded_answers = Vec::new();
        for (source, wanted) in SHIELDED_CASES {
            let transcript = compiled_answers(&open, source)?;
            let wanted = wanted.to_owned();
            assert!(transcript.contains(&wanted), "{source} -> {transcript:?}");
            shielded_answers.push(wanted);
        }
        let mut nary_answers = Vec::new();
        for (source, wanted) in NARY_TARGET_CASES {
            let transcript = compiled_answers(&open, source)?;
            let wanted = wanted.to_owned();
            assert!(transcript.contains(&wanted), "{source} -> {transcript:?}");
            nary_answers.push(wanted);
        }
        Ok(vec![
            format!("plain factorial: {plain_count} statements"),
            format!("open-coded factorial: {open_count} statements"),
            format!("open-coded answers: {}", transcript.join(" ")),
            "n-ary + folds left through val; spread preserves both the remaining argument registers and env".to_owned(),
            format!("open-coded compound calls: {compound_sessions}"),
            format!("shielded call operands: {}", shielded_answers.join(" ")),
            format!("n-ary operand targets: {}", nary_answers.join(" ")),
        ])
    }

    #[test]
    fn ex_5_38_check() -> Result<(), Fault> {
        let lines = ex_5_38()?;
        assert!(lines[0].contains("79"), "{lines:?}");
        assert!(lines[2].contains("120"));
        assert!(lines[4].contains("45"), "{lines:?}");
        assert!(lines[4].contains("44"), "{lines:?}");
        assert_eq!(lines[5], "shielded call operands: 14 17 120 15");
        assert_eq!(lines[6], "n-ary operand targets: 10 10 54 22");
        Ok(())
    }
}
