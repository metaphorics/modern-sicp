(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.42: the ``Liars'' puzzle. Each of the five girls makes
    one true and one untrue statement, so [exactly-one] requires the
    two statements of a girl to disagree; the positions are chosen with
    [an-integer-between] under a distinctness requirement, and the
    evaluator's single answer is the order the puzzle asks for. *)

module Eval = Sicp_ch4.Sec_4_3
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

let show = function
  | Ok v -> Value.to_string v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

let program =
  {|
(define (require p) (if (not p) (amb)))
(define (distinct? items)
  (cond ((null? items) #t)
        ((null? (cdr items)) #t)
        ((member (car items) (cdr items)) #f)
        (else (distinct? (cdr items)))))
(define (an-integer-between low high)
  (require (not (> low high)))
  (amb low (an-integer-between (+ low 1) high)))
(define (exactly-one x y) (if x (require (not y)) (require y)))
(define (liars)
  (let ((betty (an-integer-between 1 5))
        (ethel (an-integer-between 1 5))
        (joan (an-integer-between 1 5))
        (kitty (an-integer-between 1 5))
        (mary (an-integer-between 1 5)))
    (require (distinct? (list betty ethel joan kitty mary)))
    (exactly-one (= kitty 2) (= betty 3))
    (exactly-one (= ethel 1) (= joan 2))
    (exactly-one (= joan 3) (= ethel 5))
    (exactly-one (= kitty 2) (= mary 4))
    (exactly-one (= mary 4) (= betty 1))
    (list (list 'betty betty) (list 'ethel ethel) (list 'joan joan)
          (list 'kitty kitty) (list 'mary mary))))|}
;;

let ex_4_42 () =
  let env = Eval.the_global_environment () in
  let (_ : (Value.t, Eval_error.t) result) = Eval.run_program env program in
  [ show (Eval.run env "(liars)"); show (Eval.try_again ()) ]
;;
