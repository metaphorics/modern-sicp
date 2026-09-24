(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.22: let in the analyzed evaluator. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** [analyze] compiles the language, let included, to execution
      procedures; [eval] analyzes and runs. *)
val analyze
  :  Sicp_common.Ast.expr
  -> ( Sicp_common.Value.env -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result
       , Sicp_common.Eval_error.t )
       result

val eval
  :  Sicp_common.Ast.expr
  -> Sicp_common.Value.env
  -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

(** [ex_4_22 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_22 : unit -> string list
