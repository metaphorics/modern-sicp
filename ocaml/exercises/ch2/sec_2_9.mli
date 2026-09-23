(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program width in SICP section 2.1
   exercise 2.9 *)

(** Exercise 2.9: an interval's width, and whether the width of a
    combination is a function of the widths of its arguments alone.
    The stubs raise [Sicp_common.Pending.Pending_solution] until they
    are solved. *)

type interval = float * float

val make_interval : float -> float -> interval
val lower_bound : interval -> float
val upper_bound : interval -> float

(** [width i] is half the difference between [i]'s bounds. *)
val width : interval -> float

val add_interval : interval -> interval -> interval
val mul_interval : interval -> interval -> interval

(** [ex_2_09 ()] is [(width (mul_interval a1 b), width (mul_interval a2
    b))] for two width-1 intervals [a1] and [a2] multiplied by the
    same [b]: unlike addition, where every same-width pair of
    arguments gives the same result width, these two differ, showing
    multiplication's result width is not a function of the argument
    widths alone. *)
val ex_2_09 : unit -> float * float
