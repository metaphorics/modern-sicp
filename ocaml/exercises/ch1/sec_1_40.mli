(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program cubic in SICP section 1.3
   exercise 1.40 *)

(** Exercise 1.40: [cubic a b c] is a function usable with
    [newtons_method] to find a zero of [x^3 + a x^2 + b x + c]. The
    stub raises [Sicp_common.Pending.Pending_solution] until it is
    solved. *)

val cubic : float -> float -> float -> float -> float
val deriv : (float -> float) -> float -> float
val newton_transform : (float -> float) -> float -> float
val newtons_method : (float -> float) -> float -> float

(** [ex_1_40 ()] is a zero of [(x - 1)(x - 2)(x - 3)], found by
    [newtons_method (cubic (-6.) 11. (-6.)) 1.]. *)
val ex_1_40 : unit -> float
