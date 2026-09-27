// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.53: `permanent-set!` inside
//! `if-fail`. The search walks every prime-sum pair; each one is
//! prepended to `pairs` by a `permanent-set!` that no backtrack can
//! undo, the trailing `(amb)` fails the branch to force the next pair,
//! and when the pairs run dry the `if-fail` alternative answers with
//! the accumulated list, newest pair first.

use ch04::eval_support::{AMB_SEED, Amb, SchemeError, setup_amb_environment, with_eval_stack};

mod ex_4_53 {
    use super::*;

    /// The book's program with its library.
    const PROGRAM: &str = r"
(define (require p) (if (not p) (amb)))
(define (an-element-of items)
  (require (not (null? items)))
  (amb (car items) (an-element-of (cdr items))))
(define (prime? n)
  (define (smallest-divisor test)
    (if (> (* test test) n) n
        (if (= (remainder n test) 0) test
            (smallest-divisor (+ test 1)))))
  (= (smallest-divisor 2) n))
(define (prime-sum-pair list1 list2)
  (let ((a (an-element-of list1)) (b (an-element-of list2)))
    (require (prime? (+ a b)))
    (list a b)))
(let ((pairs '()))
  (if-fail
   (let ((p (prime-sum-pair '(1 3 5 8) '(20 35 110))))
     (permanent-set! pairs (cons p pairs))
     (amb))
   pairs))";

    /// The program's value and whether the search then runs dry.
    ///
    /// # Panics
    /// Panics when the program raises an object error.
    #[must_use]
    pub fn result() -> (String, bool) {
        with_eval_stack(move || {
            let amb = Amb::new(AMB_SEED).expect("the seed is nonzero");
            let env = setup_amb_environment();
            let forms = sicp_runtime::read_program(PROGRAM).expect("parses");
            for form in &forms[..forms.len() - 1] {
                let _ = amb.run_form(form, &env);
            }
            let mut exhausted = false;
            let value = match amb.run_form(forms.last().expect("a form"), &env) {
                Ok(value) => sicp_runtime::print_value(&value),
                Err(SchemeError::Backtrack) => {
                    exhausted = true;
                    String::from("<no values>")
                }
                Err(error) => panic!("the program raised: {error}"),
            };
            if matches!(amb.try_again(), Err(SchemeError::Backtrack)) {
                exhausted = true;
            }
            (value, exhausted)
        })
    }
}

#[test]
fn ex_4_53() {
    // The pairs accumulate across the failed branches, newest first,
    // and after the answer the search runs dry.
    let (value, exhausted) = ex_4_53::result();
    assert_eq!(value, "((8 35) (3 110) (3 20))");
    assert!(exhausted);
}
