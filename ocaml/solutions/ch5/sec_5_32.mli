(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

(** Exercise 5.32: the explicit-control evaluator's fast path for calls
    whose operator is a variable.

    (a) The modified [ev-application] tests [symbol-operator?] first.  A
    variable operator is looked up in place: [env] and [unev] are not
    saved and control does not pass through [eval-dispatch]; the fast
    path joins the ordinary one with [proc] about to be set.  Every
    other operator runs the base code.

    (b) Alyssa's plan does not remove the advantage of compilation.
    Each special case the evaluator recognizes is a test the evaluator
    repeats on every evaluation of every expression, while the compiler
    decides once; and the register analysis of [preserving] depends on
    what the surrounding code needs, which the evaluator's dispatch on
    the current expression alone cannot see. *)

(** [ev_application_fast] is the replacement [ev-application]
    fragment. *)
val ev_application_fast : Sicp_ch5.Sec_5_4.word Sicp_ch5.Sec_5_1.instruction list

(** [controller] is the base controller with [ev_application_fast]. *)
val controller : Sicp_ch5.Sec_5_4.word Sicp_ch5.Sec_5_1.instruction list

(** [symbol_operator] is the [symbol-operator?] test. *)
val symbol_operator : string * Sicp_ch5.Sec_5_4.word Sicp_ch5.Sec_5_1.op

(** [ex_5_32 ()] runs the factorial of [5] and a call of an anonymous
    function on the base and the fast-path evaluators and reports each
    answer with its pushes and maximum depth. *)
val ex_5_32 : unit -> (string list, Sicp_common.Eval_error.t) result
