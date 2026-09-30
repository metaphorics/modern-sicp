(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.24 *)

(** Exercise 4.24: analysis against direct evaluation.

    The benchmark is a doubly recursive [fib], whose run is dominated by
    procedure calls: the direct evaluator re-dispatches on the syntax of
    the body at every call, while the analyzed evaluator dispatched once,
    during analysis.  Times are processor time from [Sys.time], each the
    best of five calls after one warm-up call.  The analysis itself is
    timed separately to show how small a share of the run it is. *)

(** [benchmark] is the source of the timed program. *)
val benchmark : string

(** [runs] is the number of timed calls after the warm-up. *)
val runs : int

(** [timings ()] is the best time in seconds of the direct run, of the
    analyzed execution alone, and of the analysis alone. *)
val timings : unit -> (float * float * float, Sicp_common.Eval_error.t) result

(** [ex_4_24 ()] reports the three timings and the ratio of the direct
    time to the analyzed execution time. *)
val ex_4_24 : unit -> (string list, Sicp_common.Eval_error.t) result
