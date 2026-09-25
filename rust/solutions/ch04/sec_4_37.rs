// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.37: Ben's generator. His claim
//! holds. Both procedures answer the same first triple, but Ben's
//! `sqrt` filter rejects each candidate `(i, j)` in one step, where the
//! exercise-4.35 order walks a third choice point over every `k` below
//! the hypotenuse. The engine's failure counter makes the comparison:
//! it counts one Fail delivery per failed alternative replay and one
//! per choice-frame exhaustion, over the identical search bounds.

use ch04::eval_support::{
    AMB_SEED, Amb, collect_amb_answers, setup_amb_environment, with_eval_stack,
};

mod ex_4_37 {
    use super::*;

    /// The shared library plus both generators.
    const LIBRARY: &str = r"
(define (require p) (if (not p) (amb)))
(define (an-integer-between low high)
  (require (<= low high))
  (amb low (an-integer-between (+ low 1) high)))
(define (square x) (* x x))
(define (plain-triple low high)
  (let ((i (an-integer-between low high)))
    (let ((j (an-integer-between i high)))
      (let ((k (an-integer-between j high)))
        (require (= (+ (* i i) (* j j)) (* k k)))
        (list i j k)))))
(define (isqrt n)
  (if (< n 2) n
      (let ((r (* 2 (isqrt (quotient n 4)))))
        (if (<= (square (+ r 1)) n) (+ r 1) r))))
(define (ben-triple low high)
  (let ((i (an-integer-between low high))
        (hsq (* high high)))
    (let ((j (an-integer-between i high)))
      (let ((ksq (+ (* i i) (* j j))))
        (require (>= hsq ksq))
        (let ((k (isqrt ksq)))
          (require (= ksq (* k k)))
          (list i j k))))))";

    /// The first triple of one generator with its failure count.
    ///
    /// # Panics
    /// Panics when the generator finds no triple, which only a broken
    /// program causes.
    #[must_use]
    pub fn first_with_failures(form: &str) -> (String, u64) {
        let form = form.to_owned();
        with_eval_stack(move || {
            let amb = Amb::new(AMB_SEED).expect("the seed is nonzero");
            let env = setup_amb_environment();
            for form in sicp_runtime::read_program(LIBRARY).expect("parses") {
                let _ = amb.run_form(&form, &env);
            }
            let before = amb.failures();
            let first = amb.run(&form, &env).expect("a triple exists");
            (sicp_runtime::print_value(&first), amb.failures() - before)
        })
    }

    /// Every triple of the plain order, for the answer cross-check.
    #[must_use]
    pub fn plain_triples() -> Vec<String> {
        with_eval_stack(move || {
            let amb = Amb::new(AMB_SEED).expect("the seed is nonzero");
            collect_amb_answers(&amb, &format!("{LIBRARY}\n(plain-triple 1 20)\n"))
        })
    }
}

#[test]
fn ex_4_37() {
    // Both generators answer the same first triple.
    let (plain_first, plain_failures) = ex_4_37::first_with_failures("(plain-triple 1 20)");
    let (ben_first, ben_failures) = ex_4_37::first_with_failures("(ben-triple 1 20)");
    assert_eq!(plain_first, "(3 4 5)");
    assert_eq!(ben_first, "(3 4 5)");
    // The measured comparison over identical bounds: the 4.35 order
    // delivers 1836 Fails to choice frames, Ben's filter 162.
    assert_eq!(plain_failures, 1836);
    assert_eq!(ben_failures, 162);
    // And the plain order's full answer set matches exercise 4.35's.
    assert_eq!(
        ex_4_37::plain_triples(),
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
