(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program timed-prime-test in SICP section 1.2
   exercise 1.24 *)

(** Exercise 1.24: retime the primes of exercise 1.22 with the Fermat
    method. The stubs raise [Sicp_common.Pending.Pending_solution]
    until they are solved. *)

val expmod : int -> int -> int -> int
val fermat_test : int -> Sicp_common.Random.t -> bool
val fast_prime : int -> int -> Sicp_common.Random.t -> bool

(** [ex_1_24 ()] runs [fast_prime] with 20 rounds, one fixed-seed
    generator threaded through every call, over the 12 primes exercise
    1.22 found. *)
val ex_1_24 : unit -> bool list
