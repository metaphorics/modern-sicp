(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise
   SICP section 2.1 exercise 2.13 *)

(** Exercise 2.13: the approximate percentage-tolerance formula for a
    product. *)

type interval = float * float

val make_interval : float -> float -> interval
val lower_bound : interval -> float
val upper_bound : interval -> float
val center : interval -> float
val width : interval -> float
val make_center_percent : float -> float -> interval
val percent : interval -> float
val mul_interval : interval -> interval -> interval
val ex_2_13 : unit -> float * float
