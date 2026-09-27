// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.36: unbounded Pythagorean
//! triples. Simply replacing `an-integer-between` with
//! `an-integer-starting-from` in the upper-bound positions fails to
//! terminate: the `j` choice would wander upward forever before the
//! first triple could complete. The edition's fair enumeration rule
//! grows one bound -- the hypotenuse `k` -- through
//! `an-integer-starting-from`, and searches each finite `k` completely
//! with the bounded generator of exercise 4.35, so every triple is
//! reached after finitely many `try-again`s and each `try-again` makes
//! progress.

use ch04::eval_support::{AMB_SEED, Amb, first_amb_answers, with_eval_stack};

mod ex_4_36 {
    use super::*;

    /// The unbounded program: the hypotenuse grows, the legs stay
    /// bounded by the hypotenuse being drawn.
    const PROGRAM: &str = r"
(define (require p) (if (not p) (amb)))
(define (an-integer-starting-from n)
  (amb n (an-integer-starting-from (+ n 1))))
(define (an-integer-between low high)
  (require (<= low high))
  (amb low (an-integer-between (+ low 1) high)))
(define (a-pythagorean-triple)
  (let ((k (an-integer-starting-from 1)))
    (let ((i (an-integer-between 1 k)))
      (let ((j (an-integer-between i k)))
        (require (= (+ (* i i) (* j j)) (* k k)))
        (list i j k)))))
(a-pythagorean-triple)";

    /// The first six triples the unbounded search produces.
    #[must_use]
    pub fn first_triples() -> Vec<String> {
        with_eval_stack(move || {
            let amb = Amb::new(AMB_SEED).expect("the seed is nonzero");
            first_amb_answers(&amb, PROGRAM, 6)
        })
    }
}

#[test]
fn ex_4_36() {
    // Ordered by the growing hypotenuse: 5, 10, 13, 15, 17, 20.
    assert_eq!(
        ex_4_36::first_triples(),
        vec![
            "(3 4 5)",
            "(6 8 10)",
            "(5 12 13)",
            "(9 12 15)",
            "(8 15 17)",
            "(12 16 20)"
        ]
    );
}
