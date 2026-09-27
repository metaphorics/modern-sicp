(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program timed-prime-test in SICP section 1.2
   exercise 1.24 *)

(** Reference solution of exercise 1.24; the timing comparison is in
    [ex_1_24.md]. *)

val expmod : int -> int -> int -> int
val fermat_test : int -> Sicp_common.Random.t -> bool
val fast_prime : int -> int -> Sicp_common.Random.t -> bool

(** The 12 primes exercise 1.22 found, restated here so this file needs
    no other exercise file to run. *)
val twelve_primes : int list

(** [ex_1_24 ()] is [fast_prime] at 20 rounds over [twelve_primes],
    every entry [true]. *)
val ex_1_24 : unit -> bool list
