(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.19 *)

(** Exercise 4.19: three readings of internal definitions. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** [sequential e] is Ben's reading of the [let rec] group [e]: one
    ordinary [let] per definition, in source order. *)
val sequential : Sicp_common.Ast.expr -> Sicp_common.Ast.expr

(** [immediate_uses names e] is the names of [names] that [e] may read
    while it runs, not counting references inside a [fun]. *)
val immediate_uses : string list -> Sicp_common.Ast.expr -> string list

(** [order bindings] is [bindings] rearranged so that each definition
    comes after every definition it reads at once, or [None] when those
    reads form a cycle. *)
val order : Sicp_common.Ast.binding list -> Sicp_common.Ast.binding list option

(** [simultaneous e] is Eva's reading of the [let rec] group [e]: the
    group scanned out in the order [order] finds, or in source order
    when there is none. *)
val simultaneous : Sicp_common.Ast.expr -> Sicp_common.Ast.expr

(** [ben], [alyssa], and [eva] are the evaluators that apply each reading
    to the internal definitions of every procedure they make. *)
val ben : Sicp_ch4.Sec_4_1.eval_t

val alyssa : Sicp_ch4.Sec_4_1.eval_t
val eva : Sicp_ch4.Sec_4_1.eval_t

(** [ex_4_19 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_19 : unit -> string list
