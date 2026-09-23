(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program count-change in SICP section 1.2
   exercise 1.14 *)

(** Exercise 1.14: the tree that [count_change] generates for 11 cents,
    and its orders of growth in space and steps. Exercise 1.14a checks
    the growth prediction by measurement. The stubs raise
    [Sicp_common.Pending.Pending_solution] until they are solved. *)

val first_denomination : int -> int

(** [cc_with_count amount] is [(count_change amount, calls)], the
    number of ways to change [amount] paired with the number of calls
    [cc] made computing it. *)
val cc_with_count : int -> int * int

(** [ex_1_14 ()] is the call count for 11 cents. *)
val ex_1_14 : unit -> int

(** [ex_1_14a ()] is the call count at 50, 100, 200, and 400 cents,
    each paired with its amount. *)
val ex_1_14a : unit -> (int * int) list
