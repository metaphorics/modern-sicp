(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise
   SICP section 2.1 exercise 2.13 *)

(** Exercise 2.13: for small percentage tolerances and positive
    centers, the product of two intervals has approximately the sum
    of their percentage tolerances. The stubs raise
    [Sicp_common.Pending.Pending_solution] until they are solved. *)

type interval = float * float

val make_interval : float -> float -> interval
val lower_bound : interval -> float
val upper_bound : interval -> float
val center : interval -> float
val width : interval -> float
val make_center_percent : float -> float -> interval
val percent : interval -> float
val mul_interval : interval -> interval -> interval

(** [ex_2_13 ()] is [(percent (mul_interval x y), percent x +. percent
    y)] for two small-tolerance, positive-centered intervals [x] and
    [y], demonstrating the approximate formula the exercise asks for. *)
val ex_2_13 : unit -> float * float
