(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program div-interval in SICP section 2.1
   exercise 2.10 *)

(** Exercise 2.10: division checked against a zero-spanning divisor. *)

type interval = float * float
type interval_error = Spans_zero of float * float

val make_interval : float -> float -> interval
val lower_bound : interval -> float
val upper_bound : interval -> float
val mul_interval : interval -> interval -> interval
val div_interval : interval -> interval -> (interval, interval_error) result
val ex_2_10 : unit -> (interval, interval_error) result
