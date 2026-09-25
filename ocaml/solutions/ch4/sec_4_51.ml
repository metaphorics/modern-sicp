(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.51: [permanent-set!]. The variant of [set!] assigns
    without installing the undo trail, so a later backtracking step
    leaves the assignment in place. In the statement's counting
    example the first answer reports 2 trials -- the rejected first
    choice of [y] still raised the counter -- and [try_again] reports
    3; the same program under plain [set!] undoes the rejected trial
    and reports 1 and then 2. The demonstration pins all four answers
    and the surviving counter value. *)

module Eval = Sicp_ch4.Sec_4_3
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

let show = function
  | Ok v -> Value.to_string v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

let permanent_demo =
  {|
(define (require p) (if (not p) (amb)))
(define (an-element-of items)
  (require (not (null? items)))
  (amb (car items) (an-element-of (cdr items))))
(define count 0)
(let ((x (an-element-of '(a b c)))
      (y (an-element-of '(a b c))))
  (permanent-set! count (+ count 1))
  (require (not (eq? x y)))
  (list x y count))|}
;;

let set_demo =
  {|
(define (require p) (if (not p) (amb)))
(define (an-element-of items)
  (require (not (null? items)))
  (amb (car items) (an-element-of (cdr items))))
(define count2 0)
(let ((x (an-element-of '(a b c)))
      (y (an-element-of '(a b c))))
  (set! count2 (+ count2 1))
  (require (not (eq? x y)))
  (list x y count2))|}
;;

let ex_4_51 () =
  let env = Eval.the_global_environment () in
  let first = Eval.run_program env permanent_demo in
  let second = Eval.try_again () in
  let surviving_count = Eval.run env "count" in
  let set_env = Eval.the_global_environment () in
  let set_first = Eval.run_program set_env set_demo in
  let set_second = Eval.try_again () in
  List.map show [ first; second; surviving_count; set_first; set_second ]
;;
