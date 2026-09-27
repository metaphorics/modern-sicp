(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.4 *)

(** Exercise 5.25: normal-order evaluation in the controller, based on
    the lazy evaluator of 4.2. *)

(** [controller] is the lazy evaluator: the base controller with the
    argument loop replaced by thunk construction, [ev-variable]
    extended to force and memoize thunked bindings, and
    [primitive-apply] extended to force the arguments first. *)
val controller : string

(** [lazy_operations ()] builds the exercise's operations table: the
    thunk constructor and selectors plus the placeholder
    [extend-environment]/[lookup-variable-value] overrides.  A fresh
    call answers a fresh machine's side table. *)
val lazy_operations : unit -> (string * Sicp_ch5.Sec_5_4.op) list

(** [run source] builds a fresh lazy machine over [source] and answers
    its transcript; the queue-dry end of the driver loop is the normal
    end. *)
val run : string -> (string list, Sicp_ch5.Sec_5_4.error) result

(** [ex_5_25 ()] runs the lazy evaluator and answers three transcript
    groups: the factorial session, the laziness proof, and the
    memoization proof, as the rationale describes. *)
val ex_5_25 : unit -> (string list, Sicp_ch5.Sec_5_4.error) result
