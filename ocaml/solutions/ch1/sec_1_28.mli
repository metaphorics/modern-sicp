(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program expmod in SICP section 1.2 exercise
   1.28 *)

(** Reference solution of exercise 1.28. *)

(** [miller_rabin_expmod base exp m] is [expmod], signaling a
    nontrivial square root of 1 modulo [m] by returning 0. *)
val miller_rabin_expmod : int -> int -> int -> int

val miller_rabin_test : int -> Sicp_common.Random.t -> bool
val miller_rabin_prime : int -> int -> Sicp_common.Random.t -> bool
val carmichael_numbers : int list

(** [ex_1_28 ()] pairs each of [carmichael_numbers] with 30 rounds of
    [miller_rabin_prime], every entry [false]: Miller-Rabin is not
    fooled where Fermat is. *)
val ex_1_28 : unit -> (int * bool) list
