(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.23 *)

(** Exercise 4.23: two analyses of a sequence.

    Both versions analyze each expression of the sequence once.  The
    text's version also walks the list of execution procedures during
    analysis, chaining them into one procedure, so running the sequence
    only calls procedures.  Alyssa's version keeps the list and walks it
    each time the sequence runs, counting each walk step as runtime
    work.  For one expression the text's version answers that
    expression's own procedure while Alyssa's still pays one list step
    per run; for two expressions the text's version does one chaining
    step, once, and Alyssa's does two list steps per run.  Both styles
    also count one runtime step per procedure execution, so the same
    work is counted on both sides. *)

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

(** [ex_4_23 ()] measures a one-expression and a two-expression sequence,
    each run three times, under both styles. *)
val ex_4_23 : unit -> string list
