(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program make-interval in SICP section 2.1
   exercise 2.7 *)

(** Exercise 2.7: the interval abstraction's selectors. *)

type interval = float * float

val make_interval : float -> float -> interval
val lower_bound : interval -> float
val upper_bound : interval -> float
val ex_2_07 : unit -> float * float
