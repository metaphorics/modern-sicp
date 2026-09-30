(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.20 *)

(** Exercise 4.20: [let rec] as a derived expression.

    (a) A [let rec] group derives into a [let] of reference cells
    holding the unassigned marker, one assignment per definition, and
    the body, every read of a group name becoming a dereference: the
    transformation of [Sec_4_16], applied to every [let rec] rather than
    only to procedure bodies.

    (b) Louis writes the group with a plain [let ... and ...].  A plain
    group evaluates its right-hand sides in the enclosing environment,
    so the closure made for [even] captures an environment with no
    [odd].  Under [let rec], the closures capture the frame that holds
    the group's own names, so each finds the other.  OCaml checks this
    before running anything: Louis's program is rejected because [odd]
    is unbound where [even]'s body mentions it. *)

(** [letrec_to_lets e] is the derived form of the [let rec] group [e],
    or an [Invalid_form] error when [e] is not a [let rec]. *)
val letrec_to_lets
  :  Sicp_common.Ast.expr
  -> (Sicp_common.Ast.expr, Sicp_common.Eval_error.t) result

(** [eval] is the evaluator that derives every [let rec] with
    [letrec_to_lets] and reports a read of the unassigned marker as an
    error. *)
val eval : Sicp_ch4.Sec_4_1.eval_t

(** [parity binder] is the source of Louis's example, [f 5], whose
    internal group is written [let binder ... and ...]. *)
val parity : string -> string

(** [ex_4_20 ()] runs the parity example with [let rec], a recursive
    factorial, and Louis's version with a plain [let]. *)
val ex_4_20 : unit -> string list
