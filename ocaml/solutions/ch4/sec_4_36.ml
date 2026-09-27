(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.36: unbounded Pythagorean triples. Plain depth-first
    search is not fair over unbounded ranges: replacing
    [an-integer-between] by [an-integer-starting-from] in the 4.35
    procedure would descend into the first coordinate and never return.
    This edition's rule keeps the evaluator's search unchanged and
    restores fairness in the enumeration order: the procedure grows the
    hypotenuse without bound and searches each finite hypotenuse
    completely, so every triple is reached in finitely many
    [try_again] steps. The demonstration pins the first six triples. *)

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
(define (an-integer-between low high)
  (require (not (> low high)))
  (amb low (an-integer-between (+ low 1) high)))
(define (an-integer-starting-from n)
  (amb n (an-integer-starting-from (+ n 1))))
(define (a-pythagorean-triple-from low)
  (let ((k (an-integer-starting-from low)))
    (let ((i (an-integer-between low (- k 1))))
      (let ((j (an-integer-between i (- k 1))))
        (require (= (+ (* i i) (* j j)) (* k k)))
        (list i j k)))))|}
;;

let ex_4_36 () =
  let env = Eval.the_global_environment () in
  let (_ : (Value.t, Eval_error.t) result) = Eval.run_program env program in
  let first = Eval.run env "(a-pythagorean-triple-from 1)" in
  let second = Eval.try_again () in
  let third = Eval.try_again () in
  let fourth = Eval.try_again () in
  let fifth = Eval.try_again () in
  let sixth = Eval.try_again () in
  List.map show [ first; second; third; fourth; fifth; sixth ]
;;
