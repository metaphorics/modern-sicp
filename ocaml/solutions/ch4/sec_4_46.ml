(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.46: left-to-right operands. The section's
    [list_of_values] evaluates the operands left to right by
    construction, and the parsing programs depend on that order: each
    [parse-word] consumes the head of [*unparsed*], so the noun phrase
    must consume ``the professor'' before the verb phrase consumes
    ``lectures''. The demonstration marks each operand with a
    [note!] that records the moment of its evaluation under a shared
    trace, and the trace of the first answer shows the left operand's
    choice before the right one. *)

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
(define trace '())
(define (note! x) (set! trace (cons x trace)) x)|}
;;

let ex_4_46 () =
  let env = Eval.the_global_environment () in
  let (_ : (Value.t, Eval_error.t) result) = Eval.run_program env program in
  let first_answer = Eval.run env "(list (note! (amb 1 2)) (note! (amb 3 4)))" in
  let trace_after = Eval.run env "trace" in
  (* A fresh problem repeats the order with fresh choices. *)
  let second_problem =
    Eval.run env "(begin (set! trace '()) (list (note! (amb 5 6)) (note! (amb 7 8))))"
  in
  let trace_after_second = Eval.run env "trace" in
  List.map show [ first_answer; trace_after; second_problem; trace_after_second ]
;;
