(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.4 *)

(** Exercise 5.23: derived expressions.  The subset's multi-way
    conditional (a [match] whose cases are literals, a variable, or the
    wildcard) and the parallel [let] enter the evaluator through
    transformer machine operations.

    The module also holds the session harness the section's exercises
    share: admission, output capture, stack measurement, and controller
    composition over the kept fragments of [Sicp_ch5.Sec_5_4]. *)

(** An evaluator controller. *)
type controller = Sicp_ch5.Sec_5_4.word Sicp_ch5.Sec_5_1.instruction list

(** An evaluator operation table. *)
type operations = (string * Sicp_ch5.Sec_5_4.word Sicp_ch5.Sec_5_1.op) list

(** [admit source] is the admitted program of [source], or an
    [Invalid_form] naming the admission diagnostic. *)
val admit : string -> (Sicp_common.Check.program, Sicp_common.Eval_error.t) result

(** [lines_of text] is [text] split at newlines, without the empty line
    a final newline leaves. *)
val lines_of : string -> string list

(** [session ?operations ~controller source] admits [source], runs
    every top-level item on a fresh evaluator over [controller] and
    [operations], and is the guest output split into lines. *)
val session
  :  ?operations:operations
  -> controller:controller
  -> string
  -> (string list, Sicp_common.Eval_error.t) result

(** One measured evaluation: the monitored stack's total pushes and
    maximum depth, and the instructions executed. *)
type stats =
  { pushes : int
  ; depth : int
  ; instructions : int
  }

(** [statistics ?operations ~controller source] runs [source] like
    [session] and is the measurement of its last top-level
    evaluation. *)
val statistics
  :  ?operations:operations
  -> controller:controller
  -> string
  -> (stats, Sicp_common.Eval_error.t) result

(** [extend ~dispatch ~entries] is the base controller with each
    [(test, label)] of [dispatch] tested on [exp], in order, ahead of
    the base dispatch tests, and with [entries] placed before [done]. *)
val extend : dispatch:(string * string) list -> entries:controller -> controller

(** [splice ~from ~until replacement controller] is [controller] with
    the instructions from the label [from] up to the label [until]
    replaced by [replacement]; [until] and what follows it are kept.
    It raises [Invalid_argument] when [from] does not precede [until]. *)
val splice : from:string -> until:string -> controller -> controller -> controller

(** [is_cond e] is [true] when [e] is a [match] whose every case
    pattern is a literal, a variable, or the wildcard: the subset's
    [cond]. *)
val is_cond : Sicp_common.Ast.expr -> bool

(** [cond_to_if e] is the book's [cond->if] for the literal match [e]:
    a function of the scrutinee applied once to it, whose body is a
    chain of [if]s testing equality with each literal.  A variable case
    binds the scrutinee, and the final case needs no test because
    admission proved the match exhaustive. *)
val cond_to_if
  :  Sicp_common.Ast.expr
  -> (Sicp_common.Ast.expr, Sicp_common.Eval_error.t) result

(** [let_to_combination e] is the book's [let->combination] for the
    parallel [let] [e]: a function of the bound names applied to the
    right-hand sides. *)
val let_to_combination
  :  Sicp_common.Ast.expr
  -> (Sicp_common.Ast.expr, Sicp_common.Eval_error.t) result

(** The exercise's operations: the [cond?] test and the two
    transformers [cond->if] and [let->combination]. *)
val operations : operations

(** [controller] is the base controller with [cond?] and [let?] tests
    ahead of the base dispatch, each entering a transformer entry that
    rewrites [exp] and re-enters [eval-dispatch]. *)
val controller : controller

(** [run source] is the output of [source] on the exercise's
    evaluator. *)
val run : string -> (string list, Sicp_common.Eval_error.t) result

(** [classify_session] is the exercise's guest program: the literal
    classify of three clauses, an exhaustive Boolean match, a variable
    case, and a parallel [let], each printing its answer. *)
val classify_session : string

(** [classify_cost] defines classify and evaluates [classify 7] last,
    for measurement. *)
val classify_cost : string

(** [ex_5_23 ()] runs the classify session through the derived forms
    and ends with the cost of [classify 7] through [cond->if] beside
    its cost through the base [ev-match]. *)
val ex_5_23 : unit -> (string list, Sicp_common.Eval_error.t) result
