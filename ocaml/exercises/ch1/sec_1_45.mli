(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program nth-root in SICP section 1.3
   exercise 1.45 *)

(** Exercise 1.45: [n]th roots as a fixed point of a
    [n_damps]-times-average-damped [y -> x / y^(n - 1)]. The stubs
    raise [Sicp_common.Pending.Pending_solution] until they are
    solved. *)

(** [nth_root n n_damps x] is the [n]th root of [x], found as a fixed
    point of [y -> x / y^(n - 1)], damped [n_damps] times. *)
val nth_root : int -> int -> float -> float

(** [damps_needed n] is the fewest average damps this edition found by
    experiment that make [nth_root n damps 2.] converge to within
    [0.001] of the true [n]th root of [2.]. *)
val damps_needed : int -> int

(** [ex_1_45 n] is [nth_root n (damps_needed n) 2.]. *)
val ex_1_45 : int -> float
