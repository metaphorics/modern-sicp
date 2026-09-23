(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme cube-root program of SICP section 1.1
   exercise 1.8 *)

(** Exercise 1.8: cube roots by Newton's method. The stub raises
    [Sicp_common.Pending.Pending_solution] until it is solved. *)

(** [ex_1_08_cube_root x] is the cube root of [x] by Newton's method,
    improving a guess [y] to [x / y^2 + 2y] over [3]. *)
val ex_1_08_cube_root : float -> float
