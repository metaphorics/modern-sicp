(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program par1 in SICP section 2.1
   exercise 2.14 *)

(** Exercise 2.14: investigate Lem's complaint that [par1] and [par2]
    disagree, and that [A / A] is not the exact interval [[1, 1]] the
    algebra would suggest. The stub raises
    [Sicp_common.Pending.Pending_solution] until it is solved. *)

type interval = float * float

val make_interval : float -> float -> interval
val lower_bound : interval -> float
val upper_bound : interval -> float
val center : interval -> float
val width : interval -> float
val make_center_percent : float -> float -> interval
val percent : interval -> float
val add_interval : interval -> interval -> interval
val mul_interval : interval -> interval -> interval
val div_interval : interval -> interval -> interval

(** [par1]/[par2], Lem's two algebraically equivalent programs, given
    by the book. *)
val par1 : interval -> interval -> interval

val par2 : interval -> interval -> interval

(** [ex_2_14 ()] is [(percent (div_interval a a), percent
    (div_interval a b), percent (par1 r1 r2), percent (par2 r1 r2))]
    for small-tolerance intervals [a], [b], [r1], [r2]. *)
val ex_2_14 : unit -> float * float * float * float
