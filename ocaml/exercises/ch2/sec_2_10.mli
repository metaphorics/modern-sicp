(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program div-interval in SICP section 2.1
   exercise 2.10 *)

(** Exercise 2.10: refuse to divide by an interval spanning zero. The
    stubs raise [Sicp_common.Pending.Pending_solution] until they are
    solved. *)

type interval = float * float
type interval_error = Spans_zero of float * float (** the offending divisor's bounds *)

val make_interval : float -> float -> interval
val lower_bound : interval -> float
val upper_bound : interval -> float
val mul_interval : interval -> interval -> interval

(** [div_interval x y] is [x] multiplied by [y]'s reciprocal interval,
    or [Error (Spans_zero (lower_bound y, upper_bound y))] when [y]'s
    bounds have opposite signs (or either is zero), since [1.0 /. 0.0]
    is a silent [infinity] rather than an error in OCaml, exactly as
    it is in Scheme's own inexact division. *)
val div_interval : interval -> interval -> (interval, interval_error) result

(** [ex_2_10 ()] divides [[1, 2]] by [[-1, 1]], an interval spanning
    zero. *)
val ex_2_10 : unit -> (interval, interval_error) result
