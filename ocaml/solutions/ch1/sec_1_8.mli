(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme cube-root program of SICP section 1.1
   exercise 1.8 *)

(** Reference solution of exercise 1.8. *)

(** [ex_1_08_cube_root x] is the cube root of [x] by Newton's method,
    improving a guess [y] to [x / y^2 + 2y] over [3], with the
    relative change end test of exercise 1.7 and a guard returning
    [0.0] for [x = 0.0]. *)
val ex_1_08_cube_root : float -> float
