(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program par1 in SICP section 2.1
   exercise 2.14 *)

(** Exercise 2.14: Lem's complaint, investigated. *)

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
val ex_2_14 : unit -> float * float * float * float
