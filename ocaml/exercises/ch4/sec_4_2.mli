(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.2 *)

(** Exercise 4.2: the dispatch order of [eval] and a call-prefixed
    application language.  The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** [subexpressions e] is the immediate subexpressions of [e] in source
    order, empty for a literal or a variable. *)
val subexpressions : Sicp_common.Ast.expr -> Sicp_common.Ast.expr list

(** [louis_eval] is Louis's reordered evaluator.  Its first clause
    recognizes an application by asking only whether the node has
    subexpressions, the analogue of testing for a pair; it then
    evaluates the first subexpression as the operator and applies it to
    the rest.  Only literals and variables reach the standard clauses. *)
val louis_eval : Sicp_ch4.Sec_4_1.eval_t

(** [eval] is the evaluator of part (b): an application headed by the
    variable [call] applies its first operand to the others, and any
    other application is an [Invalid_form] error.  Guest programs make
    the syntax typed OCaml by defining [let call f = f]. *)
val eval : Sicp_ch4.Sec_4_1.eval_t

(** [ex_4_02 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_02 : unit -> string list
