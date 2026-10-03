(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.4 *)

(** Exercise 5.30: errors signaled inside the evaluator.  A runtime
    failure of a primitive or an operator becomes a condition code that
    the controller tests and routes to [signal-error], which reports it
    and ends its program; [run] then goes on to the next interaction.
    The unbound variables and wrong operand counts of part (a) never
    reach the machine: admission rejects them. *)

(** [controller] is the base controller with a [condition?] test after
    [apply-primitive-procedure] and after [apply-binary], and the
    [signal-error] entry before [done]. *)
val controller : Sec_5_23.controller

(** [operations ~emit] is the exercise's operations:
    [apply-primitive-procedure] and [apply-binary] answering a condition
    code for a runtime failure, the [condition?] test, and the
    [signal-error] action, which writes the report through [emit] and
    ends the interaction. *)
val operations : emit:(string -> unit) -> Sec_5_23.operations

(** [condition_detail w] is the failure a condition code [w] carries,
    or [None] when [w] is not a condition code. *)
val condition_detail : Sicp_ch5.Sec_5_4.word -> string option

(** [run sources] runs each source as one interaction on one
    error-signaling evaluator and is the output of all of them.  A
    signaled interaction ends its own program; the next source still
    runs. *)
val run : string list -> (string list, Sicp_common.Eval_error.t) result

(** [depth_after_failure ()] runs the division-by-zero interaction on
    one evaluator and is the saved words its failed call leaves on the
    stack. *)
val depth_after_failure : unit -> (int, Sicp_common.Eval_error.t) result

(** [ex_5_30 ()] runs the caught failures on one evaluator (an array
    index out of bounds, a division by zero, a remainder by zero, a
    division by zero inside a [List.map] callback, and a failure with a
    second item after it, which never runs), then a clean factorial
    that still answers [120]; it ends with the saved frames the failed
    call left, the contrast that the base evaluator stops at the
    division by zero, and the admission rejections of an unbound
    variable and a wrong operand count. *)
val ex_5_30 : unit -> (string list, Sicp_common.Eval_error.t) result
