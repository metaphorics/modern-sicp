(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program expmod in SICP section 1.2 exercise
   1.26 *)

(** Reference solution of exercise 1.26; the doubling argument is in
    [ex_1_26.md]. *)

(** [expmod_square_calls base exp m] is the section's [expmod]'s call
    count. *)
val expmod_square_calls : int -> int -> int -> int

(** [expmod_double_calls base exp m] is Louis's [expmod]'s call
    count. *)
val expmod_double_calls : int -> int -> int -> int

(** [ex_1_26 ()] is [(8, 191)]: both compute the same value, 35, at
    very different cost. *)
val ex_1_26 : unit -> int * int
