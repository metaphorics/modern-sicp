(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.24 *)

(** Exercise 4.24: analysis against direct evaluation. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** [timings ()] is the best time in seconds of the direct run, of the
    analyzed execution alone, and of the analysis alone. *)
val timings : unit -> (float * float * float, Sicp_common.Eval_error.t) result

(** [ex_4_24 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_24 : unit -> (string list, Sicp_common.Eval_error.t) result
