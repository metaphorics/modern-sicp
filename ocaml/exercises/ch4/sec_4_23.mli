(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.23 *)

(** Exercise 4.23: two analyses of a sequence. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** An execution procedure. *)
type execution =
  Sicp_common.Env.t -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

(** The sequencing work done during analysis and during execution. *)
type counts =
  { mutable analysis_steps : int
  ; mutable runtime_steps : int
  }

(** Whose sequence analysis runs. *)
type style =
  | Book
  | Alyssa

(** [flatten e] is the expressions of the sequence [e] in order; an
    expression that is not a sequence is a sequence of one. *)
val flatten : Sicp_common.Ast.expr -> Sicp_common.Ast.expr list

(** [book_sequence counts procs] chains [procs] into one procedure,
    counting each chaining step as analysis work.  An empty list is an
    [Invalid_form] error. *)
val book_sequence
  :  counts
  -> execution list
  -> (execution, Sicp_common.Eval_error.t) result

(** [alyssa_sequence counts procs] is a procedure that walks [procs] each
    time it runs, counting each list step as runtime work.  An empty
    list is an [Invalid_form] error. *)
val alyssa_sequence
  :  counts
  -> execution list
  -> (execution, Sicp_common.Eval_error.t) result

(** [analyze_sequence style counts e] analyzes each expression of the
    sequence [e] and combines the procedures in [style]. *)
val analyze_sequence
  :  style
  -> counts
  -> Sicp_common.Ast.expr
  -> (execution, Sicp_common.Eval_error.t) result

(** [measure style source ~runs] analyzes the admitted sequence [source]
    once, runs it [runs] times, and answers the printed output and the
    counts. *)
val measure
  :  style
  -> string
  -> runs:int
  -> (string * counts, Sicp_common.Eval_error.t) result

(** [ex_4_23 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_23 : unit -> string list
