(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.1 *)

(** Exercise 4.1: left-to-right and right-to-left operand evaluation. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** [list_of_values_left_to_right eval exps env] evaluates the first
    operand of [exps], then the rest, and answers the values in operand
    order.  It stops at the first failure. *)
val list_of_values_left_to_right
  :  Sicp_ch4.Sec_4_1.eval_t
  -> Sicp_common.Ast.expr list
  -> Sicp_common.Env.t
  -> (Sicp_common.Value.t list, Sicp_common.Eval_error.t) result

(** [list_of_values_right_to_left eval exps env] evaluates the rest of
    [exps] first, then the first operand, and answers the values in
    operand order.  It stops at the first failure. *)
val list_of_values_right_to_left
  :  Sicp_ch4.Sec_4_1.eval_t
  -> Sicp_common.Ast.expr list
  -> Sicp_common.Env.t
  -> (Sicp_common.Value.t list, Sicp_common.Eval_error.t) result

(** [eval_left_to_right] is the direct evaluator whose application
    clause evaluates the operator, then the operands through
    [list_of_values_left_to_right]. *)
val eval_left_to_right : Sicp_ch4.Sec_4_1.eval_t

(** [eval_right_to_left] is the direct evaluator whose application
    clause evaluates the operator, then the operands through
    [list_of_values_right_to_left]. *)
val eval_right_to_left : Sicp_ch4.Sec_4_1.eval_t

(** [run_source eval source] admits the single expression [source] and
    evaluates it with [eval] in a fresh global environment.  It is the
    printed output followed by the printed value, the output followed by
    [error: ...] when evaluation fails, or [rejected: ...] when
    admission fails. *)
val run_source : Sicp_ch4.Sec_4_1.eval_t -> string -> string

(** [open_expression names source] admits [source] with the variables
    [names] in scope and answers its syntax.  The names are admitted as
    the parameters of an enclosing function, so the body may refer to
    them freely; the enclosing function is not part of the answer. *)
val open_expression
  :  string list
  -> string
  -> (Sicp_common.Ast.expr, Sicp_common.Eval_error.t) result

(** [ex_4_01 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_01 : unit -> string list
