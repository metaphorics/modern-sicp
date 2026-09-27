(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program integral in SICP section 1.3
   exercise 1.29 *)

(** Exercise 1.29: Simpson's Rule, a more accurate integral than the
    section's midpoint-sum [integral]. The stubs raise
    [Sicp_common.Pending.Pending_solution] until they are solved. *)

(** [simpson f a b n] approximates [∫ₐᵇ f] with Simpson's Rule over [n]
    equal slices; [n] must be even. *)
val simpson : (float -> float) -> float -> float -> int -> float

(** [ex_1_29 ()] is [(simpson cube 0. 1. 100, simpson cube 0. 1. 1000)]. *)
val ex_1_29 : unit -> float * float
