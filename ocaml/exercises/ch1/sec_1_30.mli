(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program sum in SICP section 1.3
   exercise 1.30 *)

(** Exercise 1.30: rewrite [sum] as a tail-recursive loop instead of a
    linear recursion. The stubs raise
    [Sicp_common.Pending.Pending_solution] until they are solved. *)

(** [sum_iterative term a next b] is [Sec_1_3.Sum_abstraction.sum term
    a next b], carried by a loop instead of a linear recursion. *)
val sum_iterative : (float -> float) -> float -> (float -> float) -> float -> float

(** [ex_1_30 ()] is [sum_iterative identity 1. inc 10.]. *)
val ex_1_30 : unit -> float
