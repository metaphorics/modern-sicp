(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program * in SICP section 1.2 exercise 1.18 *)

(** Exercise 1.18: an iterative, logarithmic-step multiplication (the
    ``Russian peasant method''). The stub raises
    [Sicp_common.Pending.Pending_solution] until it is solved. *)

(** [mult_iter total a b] keeps [total + a * b] invariant. *)
val mult_iter : int -> int -> int -> int

(** [ex_1_18 a b] is [a * b] by [mult_iter], starting [total] at 0. *)
val ex_1_18 : int -> int -> int
