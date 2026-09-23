(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program expmod in SICP section 1.2 exercise
   1.25 *)

(** Exercise 1.25: is Alyssa's shortcut [expmod] correct? The stubs
    raise [Sicp_common.Pending.Pending_solution] until they are solved. *)

(** [expmod_correct base exp m] is the section's [expmod], reducing
    modulo [m] at every step. *)
val expmod_correct : int -> int -> int -> int

(** [expmod_naive base exp m] is Alyssa's shortcut: [fast_expt base exp
    mod m], building the full power first. *)
val expmod_naive : int -> int -> int -> int

(** [ex_1_25 ()] is [(expmod_correct 7 200 13, expmod_naive 7 200
    13)]. *)
val ex_1_25 : unit -> int * int
