(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise
   SICP section 2.1 exercise 2.16 *)

(** Exercise 2.16: the repeated-variable problem, demonstrated. *)

type interval = float * float

val make_interval : float -> float -> interval
val lower_bound : interval -> float
val upper_bound : interval -> float
val sub_interval : interval -> interval -> interval
val ex_2_16 : unit -> interval
