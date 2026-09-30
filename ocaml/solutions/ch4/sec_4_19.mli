(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.19 *)

(** Exercise 4.19: three readings of internal definitions.

    In [let a = 1 in let f x = let rec b = (a, x) and a = 5 in ...], the
    pair [b] reads [a] while it is built.  Ben evaluates the definitions
    in order as ordinary bindings, so [b] sees the outer [a] and [f 10]
    is 16.  Alyssa scans the definitions out, so [b] reads the
    unassigned [a], an error.  Eva wants true simultaneity, so [b] sees
    the inner [a] and the answer is 20.  Eva's reading is implementable
    by evaluating each definition after the definitions it reads at
    once; a cycle of such reads cannot be ordered.  OCaml itself, and the
    standard evaluator with it, give Eva's 20. *)

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

(** [program] is the source of the book's example. *)
val program : string

(** [ex_4_19 ()] runs [program] under Ben's, Alyssa's, and Eva's
    evaluators and under the standard one. *)
val ex_4_19 : unit -> string list
