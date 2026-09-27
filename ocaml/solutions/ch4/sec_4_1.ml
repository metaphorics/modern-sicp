(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 4.1 exercise 4.1 *)

(** Exercise 4.1: the two operand orders of [list-of-values], written
    out explicitly. The metacircular evaluator's order is whatever its
    [list_of_values] does; in this edition the host fixes nothing,
    because both orders are ordinary recursive functions. Each version
    answers the same values; the order of evaluation differs. *)

let ( >>= ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

(** [eval] is the standard evaluator: its [list_of_values] evaluates
    the first operand, then the rest, so the order is left to right by
    construction, not inherited from the host. *)
let eval : Sicp_ch4.Sec_4_1.eval_t = Sicp_ch4.Sec_4_1.eval

(** [list_of_values_left_to_right eval exps env] evaluates the first
      operand, then the rest. *)
let rec list_of_values_left_to_right (eval : Sicp_ch4.Sec_4_1.eval_t) exps env =
  match exps with
  | [] -> Ok []
  | exp :: rest ->
    eval exp env
    >>= fun value ->
    list_of_values_left_to_right eval rest env >>= fun values -> Ok (value :: values)
;;

(** [list_of_values_right_to_left eval exps env] evaluates the rest
      first, then the first operand, and assembles the values in
      operand order. *)
let rec list_of_values_right_to_left (eval : Sicp_ch4.Sec_4_1.eval_t) exps env =
  match exps with
  | [] -> Ok []
  | exp :: rest ->
    list_of_values_right_to_left eval rest env
    >>= fun values -> eval exp env >>= fun value -> Ok (value :: values)
;;

(** [trace_order list_of_values env operands] evaluates the operands
      with an instrumented evaluator that logs every evaluation, and
      answers the log in evaluation order. *)
let trace_order list_of_values env operands =
  let log = ref [] in
  let instrumented exp env =
    let value = Sicp_ch4.Sec_4_1.eval exp env in
    let shown =
      match value with
      | Ok v -> Value.to_string v
      | Error e -> "Error: " ^ Eval_error.to_string e
    in
    log := !log @ [ shown ];
    value
  in
  let _values = list_of_values instrumented operands env in
  !log
;;

(** [ex_4_01 ()] evaluates the operands of one application with two
      operands, [1] and an unbound variable, first left to right, then
      right to left. The left-to-right version evaluates [1] and fails
      on the second operand; the right-to-left version fails before
      [1] is ever evaluated. The answer is the two logs. *)
let ex_4_01 () =
  let env = Sicp_ch4.Sec_4_1.the_global_environment () in
  let operands = [ Ast.int 1; Ast.variable "not-there" ] in
  let left = trace_order list_of_values_left_to_right env operands in
  let right = trace_order list_of_values_right_to_left env operands in
  left, right
;;
