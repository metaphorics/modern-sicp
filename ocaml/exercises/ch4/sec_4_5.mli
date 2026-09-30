(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.5 *)

(** Exercise 4.5: the arrow clause of a multi-way conditional. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** One clause of the conditional. *)
type clause =
  | Test of Sicp_common.Ast.expr * Sicp_common.Ast.expr
  (** [Test (test, result)] answers [result] when [test] is [true]. *)
  | Arrow of Sicp_common.Ast.expr * Sicp_common.Ast.expr
  (** [Arrow (test, recipient)] applies [recipient] to [v] when [test]
      answers [Some v]. *)
  | Else of Sicp_common.Ast.expr (** [Else result] is the final default. *)

(** [cond_to_expr clauses] is the nested conditional of [clauses], or an
    [Invalid_form] error unless exactly the last clause is [Else]. *)
val cond_to_expr : clause list -> (Sicp_common.Ast.expr, Sicp_common.Eval_error.t) result

(** [eval_cond clauses env] derives [clauses] and evaluates the result
    in [env] with the standard evaluator. *)
val eval_cond
  :  clause list
  -> Sicp_common.Env.t
  -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

(** [ex_4_05 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_05 : unit -> (string list, Sicp_common.Eval_error.t) result
