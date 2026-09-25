(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.52: [if-fail]. The form evaluates its first expression
    and succeeds with its value; the first failure of that expression
    is caught once and the second expression succeeds in its place.
    Failures after the catch -- including the failure that follows the
    replacement value when the driver's [try_again] demands another
    answer -- propagate, which is why the all-odd case reports
    ``no more values'' after [all-odd] while the even case answers [8]
    and then, on [try_again], falls back to [all-odd]. *)

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
  (amb (car items) (an-element-of (cdr items))))|}
;;

let all_odd =
  "(if-fail (let ((x (an-element-of '(1 3 5)))) (require (even? x)) x) 'all-odd)"
;;

let has_eight =
  "(if-fail (let ((x (an-element-of '(1 3 5 8)))) (require (even? x)) x) 'all-odd)"
;;

let ex_4_52 () =
  let env = Eval.the_global_environment () in
  let (_ : (Value.t, Eval_error.t) result) = Eval.run_program env program in
  let first = Eval.run env all_odd in
  let first_again = Eval.try_again () in
  let second = Eval.run env has_eight in
  let second_again = Eval.try_again () in
  let second_again_again = Eval.try_again () in
  List.map show [ first; first_again; second; second_again; second_again_again ]
;;
