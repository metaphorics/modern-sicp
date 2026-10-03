(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.22 *)

(** Exercise 4.22: [let] in the analyzing evaluator. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** [lower_lets e] is [e] with every non-recursive [let] replaced by its
    combination. *)
val lower_lets : Sicp_common.Ast.expr -> Sicp_common.Ast.expr

(** [analyze e] analyzes [lower_lets e] with the section's analyzer and
    answers its execution procedure. *)
val analyze
  :  Sicp_common.Ast.expr
  -> Sicp_common.Env.t
  -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

(** [eval] runs [analyze e] in the environment. *)
val eval : Sicp_ch4.Sec_4_1.eval_t

(** [count_lets e] is the number of non-recursive [let] nodes in [e]. *)
val count_lets : Sicp_common.Ast.expr -> int

(** [ex_4_22 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_22 : unit -> string list
