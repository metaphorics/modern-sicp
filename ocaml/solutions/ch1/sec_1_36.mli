(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program fixed-point in SICP section 1.3
   exercise 1.36 *)

(** Reference solution of exercise 1.36. *)

val fixed_point_traced : (float -> float) -> float -> float * int
val x_to_the_x_eq_1000 : bool -> float * int

(** [ex_1_36 ()] is [((4.555532270803653, 34), (4.555537551999825, 9))]:
    average damping more than halves the number of guesses. *)
val ex_1_36 : unit -> (float * int) * (float * int)
