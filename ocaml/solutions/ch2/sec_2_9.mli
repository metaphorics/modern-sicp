(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program width in SICP section 2.1
   exercise 2.9 *)

(** Exercise 2.9: interval width and its algebra under addition versus
    multiplication. *)

type interval = float * float

val make_interval : float -> float -> interval
val lower_bound : interval -> float
val upper_bound : interval -> float
val width : interval -> float
val add_interval : interval -> interval -> interval
val mul_interval : interval -> interval -> interval
val ex_2_09 : unit -> float * float
