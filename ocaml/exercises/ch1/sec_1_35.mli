(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program fixed-point in SICP section 1.3
   exercise 1.35 *)

(** Exercise 1.35: the golden ratio as a fixed point of
    [x -> 1 + 1 / x]. The stub raises
    [Sicp_common.Pending.Pending_solution] until it is solved. *)

(** [golden_ratio ()] is the fixed point of [x -> 1 + 1 / x] found from
    an initial guess of [1.]. *)
val golden_ratio : unit -> float

val ex_1_35 : unit -> float
