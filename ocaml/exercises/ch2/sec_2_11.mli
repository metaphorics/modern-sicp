(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program mul-interval in SICP section 2.1
   exercise 2.11 *)

(** Exercise 2.11: rewrite [mul_interval] by testing endpoint signs
    instead of computing all four products and taking their min and
    max. The stubs raise [Sicp_common.Pending.Pending_solution] until
    they are solved. *)

type interval = float * float

val make_interval : float -> float -> interval
val lower_bound : interval -> float
val upper_bound : interval -> float

(** [mul_interval_naive] is @ref{2.1.4}'s four-multiplication version,
    given here only as a cross-check for the nine-case rewrite below. *)
val mul_interval_naive : interval -> interval -> interval

(** [mul_interval x y] is Ben's nine-case rewrite: at most two
    multiplications once the signs of [x]'s and [y]'s endpoints are
    known, one of the nine sign combinations needs all four. *)
val mul_interval : interval -> interval -> interval

(** [ex_2_11 ()] is [true] exactly when [mul_interval] and
    [mul_interval_naive] agree on a representative interval from each
    of the nine sign combinations. *)
val ex_2_11 : unit -> bool
