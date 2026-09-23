(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program count-change in SICP section 1.2
   exercise 1.14 *)

(** Reference solution of exercises 1.14 and 1.14a; the tree drawing for
    1.14 is in [ex_1_14.md]. *)

val first_denomination : int -> int

(** [cc_with_count amount] is [(count_change amount, calls)]. *)
val cc_with_count : int -> int * int

(** [ex_1_14 ()] is the call count for 11 cents: 55. *)
val ex_1_14 : unit -> int

(** [ex_1_14a ()] is [[(50, calls); (100, calls); (200, calls); (400,
    calls)]], the measured call count at each doubling. *)
val ex_1_14a : unit -> (int * int) list
