(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.4 *)

(** Exercise 5.25: normal-order evaluation in the controller, based on
    the lazy evaluator of 4.2.  Operands of a procedure application are
    delayed into thunk cells; a variable read forces and memoizes its
    thunk; primitives force their arguments first. *)

(** [controller] is the lazy evaluator: the base controller with the
    argument loop replaced by thunk construction, [ev-variable]
    extended to force and memoize a delayed binding, and
    [primitive-apply] extended to force its arguments first. *)
val controller : Sec_5_23.controller

(** The exercise's operations: the thunk constructor [delay-operand],
    the [delayed?] test and thunk selectors, [memoize-thunk], the
    argument-forcing operations of [primitive-apply], and
    [settle-first-argument], which writes a forced argument back into
    the list so the forcing loop terminates whether or not the thunk
    memoized. *)
val operations : Sec_5_23.operations

(** [without_memoization] is [operations] with [memoize-thunk] a no-op:
    the same lazy evaluator, except a thunk is forced again at every
    reference. *)
val without_memoization : Sec_5_23.operations

(** [run source] is the output of [source] on the lazy evaluator. *)
val run : string -> (string list, Sicp_common.Eval_error.t) result

(** [memo_session] is the memoization proof program: [use_twice (bump
    1)] prints both tuple components, then the counter. *)
val memo_session : string

(** [ex_5_25 ()] runs three proofs on the lazy evaluator: [factorial 5]
    still answers [120]; [always_42 (1 / 0)] answers [42] where the
    strict base evaluator stops with a division by zero; and
    [use_twice (bump 1)] answers [1 1] with the counter left at [1].
    The same session on [without_memoization] answers [1 2] with the
    counter at [2], so the counter singles out memoization: without it
    the thunk runs once per reference. *)
val ex_5_25 : unit -> (string list, Sicp_common.Eval_error.t) result
