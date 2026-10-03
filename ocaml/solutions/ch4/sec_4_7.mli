(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.7 *)

(** Exercise 4.7: sequential binding as nested [let]s.

    A sequential binding form binds its names from left to right, each
    init seeing every earlier name.  It rewrites into one single-binding
    [let] per name, nested in order.  OCaml's own [let ... in let ... in]
    already has this shape; the form here is an extension node beside
    the checked syntax, derived into it. *)

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

(** [ex_4_07 ()] evaluates the book's example, [x = 3], [y = x + 2],
    [z = x + y + 5], body [x * z], through [Sec_4_6.eval] and through the
    standard evaluator, then a form that rebinds [x] from its own
    earlier value. *)
val ex_4_07 : unit -> (string list, Sicp_common.Eval_error.t) result
