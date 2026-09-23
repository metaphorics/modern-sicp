(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program expmod in SICP section 1.2 exercise
   1.28 *)

(** Exercise 1.28: the Miller-Rabin test, which the Carmichael numbers
    cannot fool. The stubs raise
    [Sicp_common.Pending.Pending_solution] until they are solved. *)

(** [miller_rabin_expmod base exp m] is [expmod], returning 0 the
    moment it discovers a nontrivial square root of 1 modulo [m]. *)
val miller_rabin_expmod : int -> int -> int -> int

val miller_rabin_test : int -> Sicp_common.Random.t -> bool
val miller_rabin_prime : int -> int -> Sicp_common.Random.t -> bool

(** [ex_1_28 ()] pairs each of the six Carmichael numbers with 30
    rounds of the Miller-Rabin test. *)
val ex_1_28 : unit -> (int * bool) list
