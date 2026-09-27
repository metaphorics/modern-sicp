// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.40: prune before choosing.
//! There are 5^5 = 3125 assignments of five people to five floors
//! before the distinctness requirement, and 5! = 120 after it -- the
//! test pins both counts from a plain host enumeration. The efficient
//! procedure interleaves restrictions with the choices, so most rejects
//! happen after one or two `amb`s instead of five; the measurement
//! reports 300 Fail deliveries where the unpruned search needs 3670,
//! for the same unique answer.

use ch04::eval_support::{AMB_SEED, Amb, setup_amb_environment, with_eval_stack};

mod ex_4_40 {
    use super::*;

    /// The book's procedure beside the pruned one.
    const LIBRARY: &str = r"
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
(define (multiple-dwelling)
  (let ((baker (amb 1 2 3 4 5)) (cooper (amb 1 2 3 4 5))
        (fletcher (amb 1 2 3 4 5)) (miller (amb 1 2 3 4 5))
        (smith (amb 1 2 3 4 5)))
    (require (distinct? (list baker cooper fletcher miller smith)))
    (require (not (= baker 5)))
    (require (not (= cooper 1)))
    (require (not (= fletcher 5)))
    (require (not (= fletcher 1)))
    (require (> miller cooper))
    (require (not (= (abs (- smith fletcher)) 1)))
    (require (not (= (abs (- fletcher cooper)) 1)))
    (list (list 'baker baker) (list 'cooper cooper)
          (list 'fletcher fletcher) (list 'miller miller)
          (list 'smith smith))))
(define (multiple-dwelling-faster)
  (let ((cooper (amb 1 2 3 4 5)))
    (require (not (= cooper 1)))
    (let ((fletcher (amb 1 2 3 4 5)))
      (require (not (= fletcher 1)))
      (require (not (= fletcher 5)))
      (require (not (= (abs (- fletcher cooper)) 1)))
      (let ((miller (amb 1 2 3 4 5)))
        (require (> miller cooper))
        (let ((baker (amb 1 2 3 4 5)))
          (require (not (= baker 5)))
          (let ((smith (amb 1 2 3 4 5)))
            (require (distinct? (list baker cooper fletcher miller smith)))
            (require (not (= (abs (- smith fletcher)) 1)))
            (list (list 'baker baker) (list 'cooper cooper)
                  (list 'fletcher fletcher) (list 'miller miller)
                  (list 'smith smith))))))))";

    /// The first answer of one procedure with its failure count.
    ///
    /// # Panics
    /// Panics when the procedure finds no solution, which only a broken
    /// restriction set causes.
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
            let first = amb.run(&form, &env).expect("a solution exists");
            (sicp_runtime::print_value(&first), amb.failures() - before)
        })
    }

    /// The assignment counts the exercise asks for: before and after
    /// the distinctness requirement.
    #[must_use]
    pub fn assignment_counts() -> (usize, usize) {
        let before = 5_usize.pow(5);
        let after = (1..=5).product();
        (before, after)
    }
}

#[test]
fn ex_4_40() {
    // The exercise's two counts.
    assert_eq!(ex_4_40::assignment_counts(), (3125, 120));
    // Both procedures answer the same unique assignment.
    let (naive_answer, naive_failures) = ex_4_40::first_with_failures("(multiple-dwelling)");
    let (pruned_answer, pruned_failures) =
        ex_4_40::first_with_failures("(multiple-dwelling-faster)");
    assert_eq!(
        naive_answer,
        "((baker 3) (cooper 2) (fletcher 4) (miller 5) (smith 1))"
    );
    assert_eq!(pruned_answer, naive_answer);
    // The pruned search delivers a fraction of the Fails.
    assert_eq!(naive_failures, 3670);
    assert_eq!(pruned_failures, 300);
}
