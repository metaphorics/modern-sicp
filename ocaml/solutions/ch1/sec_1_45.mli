(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program nth-root in SICP section 1.3
   exercise 1.45 *)

(** Reference solution of exercise 1.45. *)

val nth_root : int -> int -> float -> float
val damps_needed : int -> int

(** [ex_1_45 n] is the [n]th root of [2.]. [ex_1_45 2 = 1.4142135623746899],
    [ex_1_45 4 = 1.189207115002721], [ex_1_45 16 = 1.0442737824274144]. *)
val ex_1_45 : int -> float
