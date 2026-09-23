(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program smallest-divisor in SICP section 1.2
   exercise 1.23 *)

(** Exercise 1.23: skip the even test divisors above 2, and remeasure
    against the naive search. The stubs raise
    [Sicp_common.Pending.Pending_solution] until they are solved. *)

val square : int -> int

(** [next_test_divisor d] is 3 if [d] is 2, else [d + 2]. *)
val next_test_divisor : int -> int

val find_divisor_fast : int -> int -> int

(** [ex_1_23 n] is [n]'s smallest divisor, skipping even test
    divisors above 2. *)
val ex_1_23 : int -> int
