(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.3 *)

(** Exercise 4.3: data-directed dispatch in [eval]. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** A table entry: given the evaluator to recurse through, an evaluator
    for its kind of expression. *)
type handler = Sicp_ch4.Sec_4_1.eval_t -> Sicp_ch4.Sec_4_1.eval_t

(** A dispatch table from kind names to handlers. *)
type table

(** [kind e] is the table key of [e]'s syntax kind, such as ["if"] or
    ["application"]. *)
val kind : Sicp_common.Ast.expr -> string

(** [put table name handler] installs [handler] for the kind [name],
    replacing any earlier entry. *)
val put : table -> string -> handler -> unit

(** [find table name] is the handler of [name], or [None] if [table]
    has none.  It is the book's [get]. *)
val find : table -> string -> handler option

(** [eval_if], [eval_and], [eval_or], [eval_sequence], and [eval_fun]
    are the handlers of the conditional, the two short-circuit
    connectives, the sequence, and the function expression. *)
val eval_if : handler

val eval_and : handler
val eval_or : handler
val eval_sequence : handler
val eval_fun : handler

(** [standard ()] is a fresh table holding the five handlers above. *)
val standard : unit -> table

(** [eval table] dispatches through [table], then the standard
    clauses; every subexpression goes through the same dispatch. *)
val eval : table -> Sicp_ch4.Sec_4_1.eval_t

(** [ex_4_03 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_03 : unit -> string list
