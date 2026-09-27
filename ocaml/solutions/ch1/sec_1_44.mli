(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program smooth in SICP section 1.3
   exercise 1.44 *)

(** Reference solution of exercise 1.44. *)

val smooth : (float -> float) -> float -> float
val n_fold_smoothed : (float -> float) -> int -> float -> float
val noisy : float -> float

(** [ex_1_44 ()] is [(4.0093003950441615, 4.009298845427903)]: five
    rounds of smoothing pull [noisy]'s value at 2 much closer to the
    underlying [x *. x = 4.]. *)
val ex_1_44 : unit -> float * float
