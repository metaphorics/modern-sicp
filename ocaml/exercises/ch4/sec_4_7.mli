(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.7 *)

(** Exercise 4.7: sequential binding as nested [let]s. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** A sequential binding form: its bindings in order and its body.  Each
    init may refer to the names bound before it, and the body to all of
    them. *)
type let_star =
  { bindings : (string * Sicp_common.Ast.expr) list
  ; body : Sicp_common.Ast.expr
  }

(** [let_star_to_nested_lets shape] is the nest of single-binding [let]
    expressions equivalent to [shape]; with no bindings it is the body. *)
val let_star_to_nested_lets : let_star -> Sicp_common.Ast.expr

(** [eval_let_star eval shape env] is the clause the statement proposes:
    it evaluates [let_star_to_nested_lets shape] with [eval].  With
    [Sec_4_6.eval] every inner [let] is itself derived, and no
    non-derived expression is needed. *)
val eval_let_star
  :  Sicp_ch4.Sec_4_1.eval_t
  -> let_star
  -> Sicp_common.Env.t
  -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

(** [ex_4_07 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_07 : unit -> (string list, Sicp_common.Eval_error.t) result
