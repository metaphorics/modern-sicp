// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.44: eight queens with `amb`.
//! Each row draws a column from the board, `require` rejects a column
//! that a placed queen attacks, and the recursion descends row by row;
//! the answer list accumulates in cons order, last row first.

use ch04::eval_support::{AMB_SEED, Amb, SchemeError, setup_amb_environment, with_eval_stack};

mod ex_4_44 {
    use super::*;

    /// The queens program: helpers, `queens`, and one query per board
    /// the test answers.
    const PROGRAM: &str = r"
(define (require p) (if (not p) (amb)))
(define (an-element-of items)
  (require (not (null? items)))
  (amb (car items) (an-element-of (cdr items))))
(define (integers low high)
  (if (> low high) '() (cons low (integers (+ low 1) high))))
(define (safe col placed)
  (define (check dist cols)
    (cond ((null? cols) #t)
          ((= col (car cols)) #f)
          ((= dist (abs (- col (car cols)))) #f)
          (else (check (+ dist 1) (cdr cols)))))
  (check 1 placed))
(define (queens n)
  (define (place k placed)
    (if (> k n)
        placed
        (let ((col (an-element-of (integers 1 n))))
          (require (safe col placed))
          (place (+ k 1) (cons col placed)))))
  (place 1 '()))";

    /// The first solution on a board of `n` rows and columns.
    ///
    /// # Panics
    /// Panics when the board has no solution at all, which the square
    /// boards 4 and up never hit.
    #[must_use]
    pub fn first_solution(n: usize) -> String {
        let query = format!("(queens {n})");
        with_eval_stack(move || {
            let amb = Amb::new(AMB_SEED).expect("the seed is nonzero");
            let env = setup_amb_environment();
            for form in sicp_runtime::read_program(PROGRAM).expect("parses") {
                let _ = amb.run_form(&form, &env);
            }
            match amb.run(&query, &env) {
                Ok(value) => sicp_runtime::print_value(&value),
                Err(SchemeError::Backtrack) => String::from("<no solution>"),
                Err(error) => panic!("the search raised: {error}"),
            }
        })
    }
}

#[test]
fn ex_4_44() {
    // The first solutions on the three boards, columns in row order.
    assert_eq!(ex_4_44::first_solution(4), "(3 1 4 2)");
    assert_eq!(ex_4_44::first_solution(6), "(5 3 1 6 4 2)");
    assert_eq!(ex_4_44::first_solution(8), "(4 2 7 3 6 8 5 1)");
}
