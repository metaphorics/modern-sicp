(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program sub-interval in SICP section 2.1
   exercise 2.8 *)

(** Exercise 2.8: interval subtraction. The stubs raise
    [Sicp_common.Pending.Pending_solution] until they are solved. *)

type interval = float * float

val make_interval : float -> float -> interval
val lower_bound : interval -> float
val upper_bound : interval -> float

(** [sub_interval x y] is the interval of every [a - b] for [a] in [x]
    and [b] in [y]. *)
val sub_interval : interval -> interval -> interval

(** [ex_2_08 ()] is the 10%-tolerance 6.8-ohm resistor's interval minus
    the 5%-tolerance 4.7-ohm resistor's interval. *)
val ex_2_08 : unit -> interval
