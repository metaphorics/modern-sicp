(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program sub-interval in SICP section 2.1
   exercise 2.8 *)

(** Exercise 2.8: interval subtraction. *)

type interval = float * float

val make_interval : float -> float -> interval
val lower_bound : interval -> float
val upper_bound : interval -> float
val sub_interval : interval -> interval -> interval
val ex_2_08 : unit -> interval
