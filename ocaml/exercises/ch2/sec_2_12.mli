(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program make-center-width in SICP section
   2.1 exercise 2.12 *)

(** Exercise 2.12: a center-and-percentage-tolerance constructor and
    selector. The stubs raise [Sicp_common.Pending.Pending_solution]
    until they are solved.

    Addition 2.12a: [add_interval] restated from center and width
    instead of from endpoints, compared against the endpoint-based
    version. *)

type interval = float * float

val make_interval : float -> float -> interval
val lower_bound : interval -> float
val upper_bound : interval -> float
val center : interval -> float
val width : interval -> float

(** [make_center_width c w] is the interval [c - w] to [c + w], given
    by the book's own narrative. *)
val make_center_width : float -> float -> interval

(** [make_center_percent c p] is the interval centered at [c] with
    tolerance [p] percent of [c]. *)
val make_center_percent : float -> float -> interval

(** [percent i] is [i]'s tolerance, as a percentage of its center. *)
val percent : interval -> float

(** [ex_2_12 ()] is [(center, percent)] of a 6.8-ohm 10%-tolerance
    resistor built with [make_center_percent]. *)
val ex_2_12 : unit -> float * float

(** [add_interval x y] is @ref{2.1.4}'s endpoint-based interval sum,
    given here only as a cross-check for the center-width version
    below. *)
val add_interval : interval -> interval -> interval

(** [add_interval_by_center_width x y] is the interval whose center is
    the sum of [x] and [y]'s centers and whose width is the sum of
    their widths, exercise 2.9's proven algebra restated as a
    constructor. *)
val add_interval_by_center_width : interval -> interval -> interval

(** [ex_2_12a ()] is [true] exactly when [add_interval] and
    [add_interval_by_center_width] agree, within floating-point
    tolerance, on a handful of representative interval pairs. *)
val ex_2_12a : unit -> bool
