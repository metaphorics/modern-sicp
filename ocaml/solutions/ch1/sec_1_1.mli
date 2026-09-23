(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program ex_1_01 in SICP section 1.1 *)

(** Reference solution of exercise 1.1. *)

(** [ex_1_01 ()] is the results of exercise 1.1's value-producing
    interactions in source order, paired with the result of its
    equality test: [[10; 12; 8; 3; 6; 19; 4; 16; 6; 16]] and [false].
    The two definitions in the sequence contribute no results. *)
val ex_1_01 : unit -> int list * bool
