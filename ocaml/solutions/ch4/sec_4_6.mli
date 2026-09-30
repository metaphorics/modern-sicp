(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.6 *)

(** Exercise 4.6: [let] as a derived expression.

    [let x1 = e1 and ... and xn = en in body] is equivalent to the
    combination [(fun x1 ... xn -> body) e1 ... en].  The inits are
    operands of the combination, so they run in the enclosing scope,
    which is exactly the scoping of a parallel [let]. *)

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

(** [ex_4_06 ()] evaluates one [let] through the clause, the same
    combination through the standard evaluator, a shadowing program
    whose inner inits see the outer binding, a [let _] that still runs
    its init, and a [let rec], which the clause leaves to the standard
    dispatch. *)
val ex_4_06 : unit -> string list
