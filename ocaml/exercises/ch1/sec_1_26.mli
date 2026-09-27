(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program expmod in SICP section 1.2 exercise
   1.26 *)

(** Exercise 1.26: why Louis's [expmod] runs in @math{{\Theta(n)}}
    instead of @math{{\Theta(\log n)}}. The stubs raise
    [Sicp_common.Pending.Pending_solution] until they are solved. *)

(** [expmod_square_calls base exp m] is the number of calls the
    section's [expmod] (one recursive call per level) makes. *)
val expmod_square_calls : int -> int -> int -> int

(** [expmod_double_calls base exp m] is the number of calls Louis's
    [expmod] (two recursive calls on an even level) makes. *)
val expmod_double_calls : int -> int -> int -> int

(** [ex_1_26 ()] is [(expmod_square_calls 4 64 97,
    expmod_double_calls 4 64 97)]. *)
val ex_1_26 : unit -> int * int
