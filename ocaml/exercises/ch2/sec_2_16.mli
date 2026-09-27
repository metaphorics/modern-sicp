(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise
   SICP section 2.1 exercise 2.16 *)

(** Exercise 2.16: why do algebraically equivalent expressions give
    different interval answers, and can a package avoid this? The
    stub raises [Sicp_common.Pending.Pending_solution] until it is
    solved. *)

type interval = float * float

val make_interval : float -> float -> interval
val lower_bound : interval -> float
val upper_bound : interval -> float
val sub_interval : interval -> interval -> interval

(** [ex_2_16 ()] is [sub_interval a a] for a nonzero-width interval
    [a]: algebraically [A - A = 0], but the endpoint-only
    representation cannot see that both occurrences name the same
    quantity. *)
val ex_2_16 : unit -> interval
