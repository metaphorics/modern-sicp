(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.24: analyzed versus direct evaluation. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** The medians of the timed runs, in milliseconds, of the direct
      and the analyzed evaluator on the same recursive program:
      [(direct_median, analyzed_median)]. *)
val timings : unit -> float * float

(** [ex_4_24 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_24 : unit -> string list
