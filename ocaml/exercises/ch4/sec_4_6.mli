(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.6 *)

(** Exercise 4.6: [let] as a derived expression. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** [let_to_combination e] is the combination equivalent to the
    non-recursive [let] expression [e].  It is an [Invalid_form] error
    for any other expression, including a [let rec], whose names must be
    in scope before its right-hand sides run. *)
val let_to_combination
  :  Sicp_common.Ast.expr
  -> (Sicp_common.Ast.expr, Sicp_common.Eval_error.t) result

(** [eval] is the standard evaluator with a [let] clause that evaluates
    [let_to_combination] of the node; the clause fires at every depth. *)
val eval : Sicp_ch4.Sec_4_1.eval_t

(** [ex_4_06 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_06 : unit -> string list
