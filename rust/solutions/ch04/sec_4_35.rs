// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.35: `an-integer-between`, the
//! bounded choice the Pythagorean-triple search draws from. The
//! procedure is the book's: require a nonempty inclusive range, then
//! offer the low end or the range above it -- the second alternative
//! only evaluates when the first fails, so the recursion descends one
//! bound at a time.

use ch04::eval_support::{AMB_SEED, Amb, collect_amb_answers, with_eval_stack};

mod ex_4_35 {
    use super::*;

    /// The exercise's program: the generator and the triple search over
    /// the book's range.
    const PROGRAM: &str = r"
(define (require p) (if (not p) (amb)))
(define (an-integer-between low high)
  (require (<= low high))
  (amb low (an-integer-between (+ low 1) high)))
(define (a-pythagorean-triple-between low high)
  (let ((i (an-integer-between low high)))
    (let ((j (an-integer-between i high)))
      (let ((k (an-integer-between j high)))
        (require (= (+ (* i i) (* j j)) (* k k)))
        (list i j k)))))
(a-pythagorean-triple-between 1 20)";

    /// Every triple the search finds over 1..=20, printed.
    #[must_use]
    pub fn triples() -> Vec<String> {
        with_eval_stack(move || {
            let amb = Amb::new(AMB_SEED).expect("the seed is nonzero");
            collect_amb_answers(&amb, PROGRAM)
        })
    }
}

#[test]
fn ex_4_35() {
    // Six triples between 1 and 20, then the search runs dry.
    assert_eq!(
        ex_4_35::triples(),
        vec![
            "(3 4 5)",
            "(5 12 13)",
            "(6 8 10)",
            "(8 15 17)",
            "(9 12 15)",
            "(12 16 20)"
        ]
    );
}
