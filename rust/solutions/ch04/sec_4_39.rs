// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.39: the order of the
//! restrictions. Neither the answer nor the work changes here: in the
//! book's procedure every restriction follows every choice, so the
//! search tree is the same shape whichever `require` rejects a branch
//! first, and the same assignments are rejected at the same depth. The
//! measurement makes that exact -- both orders deliver the identical
//! failure count to the choice frames -- and exercise 4.40 shows where
//! ordering does pay: interleaved with the choices.

use ch04::eval_support::{AMB_SEED, Amb, setup_amb_environment, with_eval_stack};

mod ex_4_39 {
    use super::*;

    /// The library plus the book's restriction order.
    pub const BOOK: &str = r"
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
(define (dwelling)
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
(dwelling)";

    /// The library plus the most selective restrictions first.
    pub const REORDERED: &str = r"
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
(define (dwelling)
  (let ((baker (amb 1 2 3 4 5)) (cooper (amb 1 2 3 4 5))
        (fletcher (amb 1 2 3 4 5)) (miller (amb 1 2 3 4 5))
        (smith (amb 1 2 3 4 5)))
    (require (not (= fletcher 5)))
    (require (not (= fletcher 1)))
    (require (not (= cooper 1)))
    (require (not (= baker 5)))
    (require (not (= (abs (- fletcher cooper)) 1)))
    (require (> miller cooper))
    (require (distinct? (list baker cooper fletcher miller smith)))
    (require (not (= (abs (- smith fletcher)) 1)))
    (list (list 'baker baker) (list 'cooper cooper)
          (list 'fletcher fletcher) (list 'miller miller)
          (list 'smith smith))))
(dwelling)";

    /// The first answer of one program with its failure count.
    ///
    /// # Panics
    /// Panics when the program finds no solution, which only a broken
    /// restriction set causes.
    #[must_use]
    pub fn first_with_failures(program: &str) -> (String, u64) {
        let program = program.to_owned();
        with_eval_stack(move || {
            let amb = Amb::new(AMB_SEED).expect("the seed is nonzero");
            let env = setup_amb_environment();
            let forms = sicp_runtime::read_program(&program).expect("parses");
            for form in &forms[..forms.len() - 1] {
                let _ = amb.run_form(form, &env);
            }
            let before = amb.failures();
            let first = amb
                .run_form(forms.last().expect("a form"), &env)
                .expect("a solution exists");
            (sicp_runtime::print_value(&first), amb.failures() - before)
        })
    }
}

#[test]
fn ex_4_39() {
    let (book_answer, book_failures) = ex_4_39::first_with_failures(ex_4_39::BOOK);
    let (fast_answer, fast_failures) = ex_4_39::first_with_failures(ex_4_39::REORDERED);
    // The answer is the same either way, and so is the work: both
    // orders deliver exactly 3670 Fails to the choice frames, because
    // every restriction runs after every choice in this procedure.
    assert_eq!(
        book_answer,
        "((baker 3) (cooper 2) (fletcher 4) (miller 5) (smith 1))"
    );
    assert_eq!(fast_answer, book_answer);
    assert_eq!(book_failures, 3670);
    assert_eq!(fast_failures, 3670);
}
