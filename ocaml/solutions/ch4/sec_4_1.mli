(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.1 *)

(** Exercise 4.1: left-to-right and right-to-left operand evaluation.

    OCaml leaves the evaluation order of a native application's
    arguments unspecified, so neither version inherits an order from the
    host: each one sequences its operand evaluations explicitly.  The
    module also carries the small harness the later exercises of
    section 4.1 share: running one admitted expression under an
    extended evaluator. *)

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

(** [ex_4_01 ()] runs one application whose two operands each print a
    word before answering a number, first under [eval_left_to_right] and
    then under [eval_right_to_left], then the same pair of runs on an
    application whose first operand divides by zero.  Each line is the
    run's output followed by its value or its error. *)
val ex_4_01 : unit -> string list
