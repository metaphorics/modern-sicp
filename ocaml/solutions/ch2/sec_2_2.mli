(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program print-point in SICP section 2.1
   exercise 2.2 *)

(** Exercise 2.2: points, segments, and a segment's midpoint. *)

type point = float * float
type segment = point * point

val make_point : float -> float -> point
val x_point : point -> float
val y_point : point -> float
val make_segment : point -> point -> segment
val start_segment : segment -> point
val end_segment : segment -> point
val midpoint_segment : segment -> point
val print_point : point -> unit
val ex_2_02 : unit -> point
