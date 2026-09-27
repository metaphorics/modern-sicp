(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.23: Alyssa's analyze-sequence. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** [analysis_count_book] and [analysis_count_alyssa] count how many
      times the statement's program is analyzed, in whole or in part,
      when it runs once under each evaluator. *)
val analysis_count_book : unit -> int

val analysis_count_alyssa : unit -> int

(** [ex_4_23 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_23 : unit -> string list
