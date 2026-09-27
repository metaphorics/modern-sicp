(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program make-interval in SICP section 2.1
   exercise 2.7 *)

(** Exercise 2.7: complete Alyssa's interval abstraction with
    [lower_bound] and [upper_bound]. The stubs raise
    [Sicp_common.Pending.Pending_solution] until they are solved. *)

type interval = float * float

(** [make_interval a b] is the interval from [a] to [b], given by the
    book. *)
val make_interval : float -> float -> interval

val lower_bound : interval -> float
val upper_bound : interval -> float

(** [ex_2_07 ()] is [(lower_bound, upper_bound)] of the 10%-tolerance
    6.8-ohm resistor from the section's introduction. *)
val ex_2_07 : unit -> float * float
