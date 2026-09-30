(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.22 *)

(** Exercise 4.22: [let] in the analyzing evaluator.

    The analyzer handles a non-recursive [let] as the derived expression
    of Exercise 4.6: before analysis, every such [let] at every depth is
    rewritten into its combination, so the analyzer sees only a function
    applied to the inits.  The rewrite is syntactic and happens once,
    before any execution procedure exists. *)

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

(** [programs] is the sources [ex_4_22] runs. *)
val programs : string list

(** [ex_4_22 ()] runs each of [programs] through [eval] and reports how
    many [let] nodes it had before and after lowering. *)
val ex_4_22 : unit -> string list
