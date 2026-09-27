(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program A in SICP section 1.2 exercise 1.10 *)

(** Exercise 1.10: Ackermann's function, and concise definitions for the
    functions it computes at [x] = 0, 1, 2. The stubs raise
    [Sicp_common.Pending.Pending_solution] until they are solved. *)

val ackermann : int -> int -> int
val f : int -> int
val g : int -> int
val h : int -> int
val k : int -> int

(** [ex_1_10 ()] is [(ackermann 1 10, ackermann 2 4, ackermann 3 3)]. *)
val ex_1_10 : unit -> int * int * int
