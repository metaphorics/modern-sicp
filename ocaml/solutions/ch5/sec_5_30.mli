(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.4 *)

(** Exercise 5.30: errors signaled inside the evaluator: unbound
    variables and primitive failures become distinguished condition
    codes the controller tests and routes to [signal-error]. *)

(** [controller] is the checking evaluator: the base controller with
    [ev-variable] and [primitive-apply] testing condition codes. *)
val controller : string

(** [operations] are the condition-code operations: the checking
    lookup and primitive application, and the two tests. *)
val operations : (string * Sicp_ch5.Sec_5_4.op) list

(** [run source] evaluates [source]; the answered lines are the
    transcript plus a leading note naming how the run ended: the
    queue-dry end, or the caught error's message. *)
val run : string -> (string list, Sicp_ch5.Sec_5_4.error) result

(** [ex_5_30 ()] runs the caught failures and one clean computation:
    an unbound variable, [car] of a symbol, division by zero, a wrong
    operand count, and a factorial that must still answer [120]. *)
val ex_5_30 : unit -> (string list, Sicp_ch5.Sec_5_4.error) result
