// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.43: the yacht puzzle. The
//! program assigns each daughter a father, keeps the two facts the
//! story states (Melissa is Sir Barnacle's daughter; Mary Ann's father
//! is Mr. Moore when we are told her surname), requires the five yacht
//! names distinct with the four fixed namings, and holds the last
//! sentence: Gabrielle's father owns the yacht named after Dr.
//! Parker's daughter. Told that Mary Ann is a Moore, Lorna's father is
//! Downing and the solution is unique; without the surname, Parker
//! also works.

use ch04::eval_support::{AMB_SEED, Amb, collect_amb_answers, with_eval_stack};

mod ex_4_43 {
    use super::*;

    /// The puzzle program; the `told` flag stands for the sentence the
    /// exercise varies.
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
(define (lookup key alist)
  (cond ((null? alist) #f)
        ((eq? key (car (car alist))) (car (cdr (car alist))))
        (else (lookup key (cdr alist)))))
(define (yacht-solve told)
  (let ((mary-ann (an-element-of '(moore downing hall barnacle parker)))
        (gabrielle (an-element-of '(moore downing hall barnacle parker)))
        (lorna (an-element-of '(moore downing hall barnacle parker)))
        (rosalind (an-element-of '(moore downing hall barnacle parker)))
        (melissa (an-element-of '(moore downing hall barnacle parker)))
        (parker-yacht (an-element-of '(mary-ann gabrielle lorna rosalind melissa))))
    (require (distinct? (list mary-ann gabrielle lorna rosalind melissa)))
    (require (eq? melissa 'barnacle))
    (if told (require (eq? mary-ann 'moore)) #t)
    (require (not (member parker-yacht '(lorna melissa rosalind gabrielle))))
    (require (not (eq? lorna 'moore)))
    (require (not (eq? rosalind 'hall)))
    (require (not (eq? gabrielle 'barnacle)))
    (let ((parker-daughter
           (cond ((eq? mary-ann 'parker) 'mary-ann)
                 ((eq? gabrielle 'parker) 'gabrielle)
                 ((eq? lorna 'parker) 'lorna)
                 ((eq? rosalind 'parker) 'rosalind)
                 (else 'melissa))))
      (let ((yachts (list (list 'moore 'lorna) (list 'downing 'melissa)
                          (list 'hall 'rosalind) (list 'barnacle 'gabrielle)
                          (list 'parker parker-yacht))))
        (require (eq? (lookup gabrielle yachts) parker-daughter))
        (list 'lornas-father lorna)))))
(define (an-element-of items)
  (require (not (null? items)))
  (amb (car items) (an-element-of (cdr items))))
(yacht-solve #t)";

    /// Lorna's father for one value of the told flag: every solution.
    #[must_use]
    pub fn solutions(told: bool) -> Vec<String> {
        let program = PROGRAM.replace("(yacht-solve #t)", &format!("(yacht-solve {told})"));
        with_eval_stack(move || {
            let amb = Amb::new(AMB_SEED).expect("the seed is nonzero");
            collect_amb_answers(&amb, &program)
        })
    }
}

#[test]
fn ex_4_43() {
    // Told that Mary Ann's last name is Moore: one solution.
    assert_eq!(ex_4_43::solutions(true), vec!["(lornas-father downing)"]);
    // Not told: Downing and Parker both work.
    assert_eq!(
        ex_4_43::solutions(false),
        vec!["(lornas-father downing)", "(lornas-father parker)"]
    );
}
