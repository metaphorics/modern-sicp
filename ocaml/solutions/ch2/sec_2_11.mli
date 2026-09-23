(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program mul-interval in SICP section 2.1
   exercise 2.11 *)

(** Exercise 2.11: the nine-case [mul_interval]. *)

type interval = float * float

val make_interval : float -> float -> interval
val lower_bound : interval -> float
val upper_bound : interval -> float
val mul_interval_naive : interval -> interval -> interval
val mul_interval : interval -> interval -> interval
val ex_2_11 : unit -> bool
