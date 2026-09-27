(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program make-center-width in SICP section
   2.1 exercise 2.12 *)

(** Exercise 2.12: center-percent construction and selection. Addition
    2.12a: the endpoint and center-width interfaces agree on
    [add_interval]. *)

type interval = float * float

val make_interval : float -> float -> interval
val lower_bound : interval -> float
val upper_bound : interval -> float
val center : interval -> float
val width : interval -> float
val make_center_width : float -> float -> interval
val make_center_percent : float -> float -> interval
val percent : interval -> float
val ex_2_12 : unit -> float * float
val add_interval : interval -> interval -> interval
val add_interval_by_center_width : interval -> interval -> interval
val ex_2_12a : unit -> bool
