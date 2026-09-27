(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program print-point in SICP section 2.1
   exercise 2.2 *)

(** Exercise 2.2: points, segments built from two points, and a
    segment's midpoint. The stubs raise
    [Sicp_common.Pending.Pending_solution] until they are solved. *)

type point = float * float
type segment = point * point

val make_point : float -> float -> point
val x_point : point -> float
val y_point : point -> float
val make_segment : point -> point -> segment
val start_segment : segment -> point
val end_segment : segment -> point

(** [midpoint_segment s] is the point whose coordinates are the
    average of [s]'s two endpoints. *)
val midpoint_segment : segment -> point

(** [print_point p] prints [p] as ["(x,y)"] followed by a newline,
    the book's given helper for trying these procedures. *)
val print_point : point -> unit

(** [ex_2_02 ()] is the midpoint of the segment from [(0, 0)] to
    [(4, 6)]. *)
val ex_2_02 : unit -> point
