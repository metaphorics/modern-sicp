(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program smallest-divisor in SICP section 1.2
   exercise 1.21 *)

(** Exercise 1.21: the smallest divisor of 199, 1999, and 19999. The
    stubs raise [Sicp_common.Pending.Pending_solution] until they are
    solved. *)

val square : int -> int
val find_divisor : int -> int -> int

(** [smallest_divisor n] is [n]'s smallest divisor greater than 1. *)
val smallest_divisor : int -> int

(** [ex_1_21 ()] is [(smallest_divisor 199, smallest_divisor 1999,
    smallest_divisor 19999)]. *)
val ex_1_21 : unit -> int * int * int
