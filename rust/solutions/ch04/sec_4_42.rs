// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.42: the Liars puzzle. Each girl
//! makes one true and one untrue statement, so each pair of claims
//! satisfies `one-true` -- exclusive or. The search assigns the five
//! places and keeps the assignment where every girl's pair holds.

use ch04::eval_support::{AMB_SEED, Amb, collect_amb_answers, with_eval_stack};

mod ex_4_42 {
    use super::*;

    /// The puzzle program: one `require` per girl, in letter order.
    const PROGRAM: &str = r"
(define (require p) (if (not p) (amb)))
(define (distinct? items)
  (cond ((null? items) #t)
        ((null? (cdr items)) #t)
        ((member (car items) (cdr items)) #f)
        (else (distinct? (cdr items)))))
(define (member x xs)
  (cond ((null? xs) #f)
        ((equal? x (car xs)) xs)
        (else (member x (cdr xs)))))
(define (one-true p q) (if p (not q) q))
(let ((betty (amb 1 2 3 4 5)) (ethel (amb 1 2 3 4 5))
      (joan (amb 1 2 3 4 5)) (kitty (amb 1 2 3 4 5))
      (mary (amb 1 2 3 4 5)))
  (require (distinct? (list betty ethel joan kitty mary)))
  (require (one-true (= kitty 2) (= betty 3)))
  (require (one-true (= ethel 1) (= joan 2)))
  (require (one-true (= joan 3) (= ethel 5)))
  (require (one-true (= kitty 2) (= mary 4)))
  (require (one-true (= mary 4) (= betty 1)))
  (list (list 'betty betty) (list 'ethel ethel) (list 'joan joan)
        (list 'kitty kitty) (list 'mary mary)))";

    /// Every consistent placement, printed.
    #[must_use]
    pub fn solutions() -> Vec<String> {
        with_eval_stack(move || {
            let amb = Amb::new(AMB_SEED).expect("the seed is nonzero");
            collect_amb_answers(&amb, PROGRAM)
        })
    }
}

#[test]
fn ex_4_42() {
    // One ordering survives every girl's letter, and the search then
    // runs dry.
    assert_eq!(
        ex_4_42::solutions(),
        vec!["((betty 3) (ethel 5) (joan 2) (kitty 1) (mary 4))"]
    );
}
