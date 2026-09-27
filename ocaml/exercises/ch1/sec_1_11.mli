(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program f in SICP section 1.2 exercise 1.11 *)

(** Exercise 1.11: [f] computed by a recursive process and by an
    iterative process. The stubs raise
    [Sicp_common.Pending.Pending_solution] until they are solved. *)

val f_recursive : int -> int
val f_iterative : int -> int

(** [ex_1_11 n] is [(f_recursive n, f_iterative n)]. *)
val ex_1_11 : int -> int * int
