(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.35: [an-integer-between] and the bounded Pythagorean
    triples. The procedure of the statement enumerates the triples
    between the bounds under the evaluator's depth-first search, and
    [try_again] walks them in the chronological order the search
    defines: by ascending [i], then [j], then [k]. The demonstration
    pins the first six triples between 1 and 20 and the exhaustion that
    follows. *)

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
(define (an-element-of items)
  (require (not (null? items)))
  (amb (car items) (an-element-of (cdr items))))
(define (an-integer-between low high)
  (require (not (> low high)))
  (amb low (an-integer-between (+ low 1) high)))
(define (a-pythagorean-triple-between low high)
  (let ((i (an-integer-between low high)))
    (let ((j (an-integer-between i high)))
      (let ((k (an-integer-between j high)))
        (require (= (+ (* i i) (* j j)) (* k k)))
        (list i j k)))))|}
;;

let ex_4_35 () =
  let env = Eval.the_global_environment () in
  let (_ : (Value.t, Eval_error.t) result) = Eval.run_program env program in
  let first = Eval.run env "(a-pythagorean-triple-between 1 20)" in
  let second = Eval.try_again () in
  let third = Eval.try_again () in
  let fourth = Eval.try_again () in
  let fifth = Eval.try_again () in
  let sixth = Eval.try_again () in
  let seventh = Eval.try_again () in
  List.map show [ first; second; third; fourth; fifth; sixth; seventh ]
;;
