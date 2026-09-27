// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.51: `permanent-set!`. The
//! engine's assignment clause takes a trail kind: `set!` records the
//! old value on the undo trail -- the book's `*1*`/`*2*` pair -- while
//! `permanent-set!` assigns and skips the record, so no backtracking
//! ever restores it. With `permanent-set!` the book's counting program
//! answers (a b 2), (a c 3), ... : every trial, failed or not, stays
//! counted. With `set!` each answer reads 1, because the try-again that
//! resumes the search passes back through the assignment interception
//! and restores the count before the next alternative runs.

use ch04::eval_support::{AMB_SEED, Amb, first_amb_answers, with_eval_stack};

mod ex_4_51 {
    use super::*;

    /// The book's program, parameterized over the assignment form.
    fn program(assign: &str) -> String {
        format!(
            r"
(define (require p) (if (not p) (amb)))
(define (an-element-of items)
  (require (not (null? items)))
  (amb (car items) (an-element-of (cdr items))))
(define count 0)
(let ((x (an-element-of '(a b c))) (y (an-element-of '(a b c))))
  ({assign} count (+ count 1))
  (require (not (eq? x y)))
  (list x y count))
"
        )
    }

    /// The first six answers under one assignment form.
    ///
    /// # Panics
    /// Panics when the search raises an object error.
    #[must_use]
    pub fn answers(assign: &str) -> Vec<String> {
        let program = program(assign);
        with_eval_stack(move || {
            let amb = Amb::new(AMB_SEED).expect("the seed is nonzero");
            first_amb_answers(&amb, &program, 6)
        })
    }
}

#[test]
fn ex_4_51() {
    // permanent-set! counts every trial; the failed (a a) and (b b)
    // pairs are why each answer's count runs ahead of its position.
    assert_eq!(
        ex_4_51::answers("permanent-set!"),
        vec![
            "(a b 2)", "(a c 3)", "(b a 4)", "(b c 6)", "(c a 7)", "(c b 8)"
        ]
    );
    // set! undoes on backtrack: the count always reads the failed
    // trials plus this one, restored -- so each answer reads 1.
    assert_eq!(
        ex_4_51::answers("set!"),
        vec![
            "(a b 1)", "(a c 1)", "(b a 1)", "(b c 1)", "(c a 1)", "(c b 1)"
        ]
    );
}
