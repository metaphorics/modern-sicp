// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.38: the multiple-dwelling
//! procedure with the requirement that Smith and Fletcher not live on
//! adjacent floors omitted. The modified puzzle has five solutions; the
//! test counts them two ways, by the amb search and by a brute-force
//! enumeration of all 5! assignments, so the count does not rest on the
//! evaluator alone.

use ch04::eval_support::{AMB_SEED, Amb, collect_amb_answers, with_eval_stack};

mod ex_4_38 {
    use super::*;

    /// The book's procedure with one `require` dropped.
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
    (require (not (= (abs (- fletcher cooper)) 1)))
    (list (list 'baker baker) (list 'cooper cooper)
          (list 'fletcher fletcher) (list 'miller miller)
          (list 'smith smith))))
(multiple-dwelling)";

    /// Every solution of the modified puzzle, printed, in search order.
    #[must_use]
    pub fn solutions() -> Vec<String> {
        with_eval_stack(move || {
            let amb = Amb::new(AMB_SEED).expect("the seed is nonzero");
            collect_amb_answers(&amb, PROGRAM)
        })
    }

    /// The independent count: a plain host loop over the 120 floor
    /// assignments applying the remaining restrictions.
    #[must_use]
    pub fn brute_force_count() -> usize {
        let mut count = 0;
        for baker in 1..=5 {
            for cooper in 1..=5 {
                for fletcher in 1..=5 {
                    for miller in 1..=5 {
                        for smith in 1..=5 {
                            let mut sorted = [baker, cooper, fletcher, miller, smith];
                            sorted.sort_unstable();
                            let distinct = sorted == [1, 2, 3, 4, 5];
                            if distinct
                                && baker != 5
                                && cooper != 1
                                && fletcher != 5
                                && fletcher != 1
                                && miller > cooper
                                && fletcher != cooper + 1
                                && fletcher != cooper - 1
                            {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
        count
    }
}

#[test]
fn ex_4_38() {
    // The book's answer, without the Smith-Fletcher clause.
    assert_eq!(
        ex_4_38::solutions(),
        vec![
            "((baker 1) (cooper 2) (fletcher 4) (miller 3) (smith 5))",
            "((baker 1) (cooper 2) (fletcher 4) (miller 5) (smith 3))",
            "((baker 1) (cooper 4) (fletcher 2) (miller 5) (smith 3))",
            "((baker 3) (cooper 2) (fletcher 4) (miller 5) (smith 1))",
            "((baker 3) (cooper 4) (fletcher 2) (miller 5) (smith 1))",
        ]
    );
    // The original solution is one of them, and the brute force
    // agrees with the search.
    assert!(
        ex_4_38::solutions()
            .contains(&"((baker 3) (cooper 2) (fletcher 4) (miller 5) (smith 1))".to_owned())
    );
    assert_eq!(ex_4_38::solutions().len(), ex_4_38::brute_force_count());
}
