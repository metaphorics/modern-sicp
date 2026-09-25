// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.32: the extra laziness of lazy
//! lists against the chapter 3 streams, and the eager comparison the
//! edition keeps beside them (D18). A lazy pair constructs without
//! computing either slot; only a demand forces, so the armed slot of a
//! pair stays dormant until it is the one demanded. The chapter 3
//! constructor evaluates the car eagerly, and so does the strict
//! `cons` primitive beside this edition's evaluator.

use ch04::eval_support::*;

mod ex_4_32 {
    use super::*;

    /// The 4.2.3 procedural pairs: `cons` delays both slots by wrapping
    /// them in a procedure.
    const LAZY_PAIRS: &str = "\
(define (cons x y) (lambda (m) (m x y)))
(define (car z) (z (lambda (p q) p)))
(define (cdr z) (z (lambda (p q) q)))";

    /// The three behaviors: the lazy skip of the armed tail, the demand
    /// that forces it, and the eager chapter 3 analog under the strict
    /// constructor.
    ///
    /// # Errors
    /// The armed demand and the eager constructor raise by design; the
    /// messages travel in the answer.
    pub fn answers() -> Result<(String, String, String, String), SchemeError> {
        let (values, _) = run_lazy(&Lazy, &format!("{LAZY_PAIRS}\n(car (cons 7 (/ 1 0)))"))?;
        let armed_tail_skipped = printed(&values).last().cloned().unwrap_or_default();

        let armed_tail_forced = run_lazy(&Lazy, &format!("{LAZY_PAIRS}\n(cdr (cons 7 (/ 1 0)))"))
            .expect_err("demanding the armed tail forces it")
            .to_string();

        let eager_car = run_with(&Base, "(car (cons (/ 1 0) 7))")
            .expect_err("the strict constructor evaluates the car")
            .to_string();

        // Construction without computation: the self-referential `ones`
        // defines without looping, which the chapter 3 stream could
        // only do through the explicit `delay` of `cons-stream`.
        let (values, _) = run_lazy(
            &Lazy,
            &format!("{LAZY_PAIRS}\n(define ones (cons 1 ones))\n'ok"),
        )?;
        let infinite_defines = printed(&values).last().cloned().unwrap_or_default();

        Ok((
            armed_tail_skipped,
            armed_tail_forced,
            eager_car,
            infinite_defines,
        ))
    }
}

#[test]
fn ex_4_32() {
    let (skipped, forced, eager, infinite) = ex_4_32::answers().expect("lazy runs succeed");
    // The lazy pair answers its car while the armed tail stays a
    // thunk: the extra laziness over the chapter 3 stream.
    assert_eq!(skipped, "7");
    // The dormant slot computes exactly when demanded.
    assert_eq!(forced, "division by zero");
    // The eager comparison: under the strict constructor the car
    // evaluates at cons time, before any demand.
    assert_eq!(eager, "division by zero");
    // And construction without computation: `ones` defines in a step.
    assert_eq!(infinite, "ok");
}

mod ex_4_32a {
    use super::*;

    /// The lazy tree of the addition: each node is a lazy pair whose
    /// slots are delayed subtrees or delayed counting bodies, so
    /// building the tree computes nothing and a path forces only what
    /// it touches.
    const TREE_SESSION: &[&str] = &[
        "(define count 0)",
        "(define (tick x) (set! count (+ count 1)) x)",
        "(define tree (cons (cons (tick 1) (cons (tick 2) (tick 3))) (tick 4)))",
        "(car (cdr (car tree)))",
        "count",
        "(cdr tree)",
        "count",
    ];

    /// The printable transcript of the selective forcing.
    #[must_use]
    pub fn transcript() -> String {
        printable_driver_transcript(&LazyPrintable, TREE_SESSION)
    }
}

#[test]
fn ex_4_32a() {
    let transcript = ex_4_32a::transcript();
    let values: Vec<&str> = transcript
        .lines()
        .filter_map(|line| line.strip_prefix(";;; L-Eval value: "))
        .collect();
    // Three defines, then the tree: the defines answer `ok`, and the
    // `tree` define computes nothing because every slot is delayed.
    assert_eq!(values.first(), Some(&"ok"));
    assert_eq!(&values[..3], &["ok", "ok", "ok"]);
    // The left-left path forces the subtrees on it and the leaf 2;
    // the counter answers 1.
    assert_eq!(values.get(3), Some(&"2"));
    assert_eq!(values.get(4), Some(&"1"));
    // The right branch forces only the leaf 4; the counter answers 2,
    // which pins the untouched leaves 1 and 3 as never computed.
    assert_eq!(values.get(5), Some(&"4"));
    assert_eq!(values.last(), Some(&"2"));
}
