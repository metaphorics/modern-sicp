(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program tan-cf in SICP section 1.3
   exercise 1.39 *)

(** Exercise 1.39: Lambert's continued-fraction approximation to
    [tan]. The stub raises [Sicp_common.Pending.Pending_solution] until
    it is solved. *)

(** [tan_cf x k] approximates [tan x] (radians) with a [k]-term
    continued fraction. *)
val tan_cf : float -> int -> float

(** [ex_1_39 ()] is [(tan_cf 0.1 10, tan_cf 1.0 20)]. *)
val ex_1_39 : unit -> float * float
