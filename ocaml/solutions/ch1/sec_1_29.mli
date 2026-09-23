(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program integral in SICP section 1.3
   exercise 1.29 *)

(** Reference solution of exercise 1.29. *)

val simpson : (float -> float) -> float -> float -> int -> float

(** [ex_1_29 ()] is [(0.25000000000000006, 0.25000000000000006)]:
    Simpson's Rule integrates a cubic exactly, and [cube] is one, so
    both the coarse and the fine grid land within a rounding error of
    the true 1/4, unlike the section's [integral]. *)
val ex_1_29 : unit -> float * float
