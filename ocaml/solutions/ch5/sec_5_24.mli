(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.4 *)

(** Exercise 5.24: the subset's [cond], the literal [match], as a basic
    controller form, not reduced to [if]; and Exercise 5.24a, added by
    this edition: chains of [&&] and [||] as basic controller forms in
    the same style. *)

(** [controller] is the base controller with an [ev-cond] clause loop
    and [&&]/[||] chain loops ahead of the base dispatch. *)
val controller : Sec_5_23.controller

(** The exercise's operations: the [cond?] test, the clause-loop
    operations [last-case?], [case-selects?], and [case-environment],
    and the chain flatteners [and-operands] and [or-operands]. *)
val operations : Sec_5_23.operations

(** [run source] is the output of [source] on the exercise's
    evaluator. *)
val run : string -> (string list, Sicp_common.Eval_error.t) result

(** [ex_5_24 ()] runs the 5.23 classify session through the basic
    [ev-cond] and ends with the cost of [classify 7] through [ev-cond]
    beside its cost through 5.23's [cond->if]. *)
val ex_5_24 : unit -> (string list, Sicp_common.Eval_error.t) result

(** [ex_5_24a ()] runs the chain session (all-true and first-false
    chains, short circuits past a division by zero in both directions,
    nesting, and the [within] predicate) and ends with the cost of a
    four-operand [&&] through the chain loop beside the base [ev-and]. *)
val ex_5_24a : unit -> (string list, Sicp_common.Eval_error.t) result
