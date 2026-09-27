(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program par2 in SICP section 2.1
   exercise 2.15 *)

(** Exercise 2.15: [par2]'s tighter bound, measured. *)

type interval = float * float

val make_interval : float -> float -> interval
val lower_bound : interval -> float
val upper_bound : interval -> float
val center : interval -> float
val width : interval -> float
val make_center_percent : float -> float -> interval
val percent : interval -> float
val add_interval : interval -> interval -> interval
val mul_interval : interval -> interval -> interval
val div_interval : interval -> interval -> interval
val par1 : interval -> interval -> interval
val par2 : interval -> interval -> interval
val ex_2_15 : unit -> float * float
