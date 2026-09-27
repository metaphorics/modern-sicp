(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program smooth in SICP section 1.3
   exercise 1.44 *)

(** Exercise 1.44: [smooth] averages a function's neighbors;
    [n_fold_smoothed] repeats that with [repeated]. The stubs raise
    [Sicp_common.Pending.Pending_solution] until they are solved. *)

(** [smooth f x] is the average of [f (x -. dx)], [f x], and
    [f (x +. dx)] for a small fixed [dx]. *)
val smooth : (float -> float) -> float -> float

(** [n_fold_smoothed f n] is [f] smoothed [n] times. *)
val n_fold_smoothed : (float -> float) -> int -> float -> float

(** [noisy x] is [x *. x] plus a small oscillating perturbation, a
    stand-in for a signal with measurement noise. *)
val noisy : float -> float

(** [ex_1_44 ()] is [(noisy 2., n_fold_smoothed noisy 5 2.)]. *)
val ex_1_44 : unit -> float * float
